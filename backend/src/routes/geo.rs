use arrow::array::{BinaryBuilder, Float64Builder, RecordBatch, StringBuilder};
use arrow::datatypes::{DataType, Field, Schema};
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;
use quick_xml::events::Event as XmlEvent;
use quick_xml::Reader;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, Mutex, OnceLock};
use tokio_stream::wrappers::ReceiverStream;

/// In-memory cache for generated geoviewer HTML (avoids Docker volume sync issues)
static HTML_CACHE: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

fn html_cache() -> &'static Mutex<HashMap<String, String>> {
    HTML_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

use crate::state::AppState;

const GML_CACHE_DIR: &str = "/data/gml_cache";
const GEO_MAP_TEMPLATE: &str = include_str!("../templates/geo_map.html");
const MAPTILER_STYLE_URL: &str = "https://api.maptiler.com/maps/019c11e4-a378-77f3-8bb1-deabad13a05e/style.json?key=xlQ4I66qbGWwrztUO21v";

static MAPTILER_STYLE_CACHE: OnceLock<String> = OnceLock::new();

fn get_maptiler_style_json() -> &'static str {
    MAPTILER_STYLE_CACHE.get_or_init(|| {
        tracing::info!("Fetching MapTiler style from {}", MAPTILER_STYLE_URL);
        match reqwest::blocking::get(MAPTILER_STYLE_URL) {
            Ok(resp) => match resp.text() {
                Ok(text) => {
                    tracing::info!("MapTiler style fetched ({} bytes)", text.len());
                    text
                }
                Err(e) => {
                    tracing::error!("Failed to read MapTiler style response: {}", e);
                    "{}".to_string()
                }
            },
            Err(e) => {
                tracing::error!("Failed to fetch MapTiler style: {}", e);
                "{}".to_string()
            }
        }
    })
}

fn get_geo_map_template() -> String {
    if let Ok(path) = std::env::var("GEO_MAP_TEMPLATE_PATH") {
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to read template from {}: {}, falling back to embedded", path, e);
                GEO_MAP_TEMPLATE.to_string()
            })
    } else {
        GEO_MAP_TEMPLATE.to_string()
    }
}
/// Default location: Rathausplatz 2, 86150 Augsburg
const DEFAULT_LAT: f64 = 48.3696;
const DEFAULT_LNG: f64 = 10.8979;

#[derive(Deserialize)]
pub struct GeoBuildingsQuery {
    lat: f64,
    lng: f64,
    session_id: Option<String>,
    address: Option<String>,
}

struct Surface {
    gmlid: String,
    surface_type: String,
    coords: Vec<[f64; 3]>,
    flaeche: f64,          // Surface area in m² (from gen:stringAttribute "Flaeche")
    // Building-level CityGML attributes
    function: String,
    roof_type: String,
    measured_height: f64,
    gemeindeschluessel: String,
    street: String,        // From xAL: ThoroughfareName (street name only)
    house_number: String,  // From xAL: ThoroughfareNumber
    zip: String,           // From xAL: PostalCodeNumber
    city: String,          // From xAL: LocalityName
}

/// Convert WGS84 to UTM32N (EPSG:25832)
fn wgs84_to_utm32n(lat: f64, lng: f64) -> (f64, f64) {
    let a = 6378137.0_f64;
    let f = 1.0 / 298.257223563;
    let e2 = 2.0 * f - f * f;
    let e_prime2 = e2 / (1.0 - e2);
    let k0 = 0.9996;
    let lng0 = 9.0_f64;

    let lat_rad = lat.to_radians();
    let lng_rad = lng.to_radians();
    let lng0_rad = lng0.to_radians();

    let n = a / (1.0 - e2 * lat_rad.sin().powi(2)).sqrt();
    let t = lat_rad.tan().powi(2);
    let c = e_prime2 * lat_rad.cos().powi(2);
    let a_coeff = (lng_rad - lng0_rad) * lat_rad.cos();
    let m = a
        * ((1.0 - e2 / 4.0 - 3.0 * e2.powi(2) / 64.0 - 5.0 * e2.powi(3) / 256.0) * lat_rad
            - (3.0 * e2 / 8.0 + 3.0 * e2.powi(2) / 32.0 + 45.0 * e2.powi(3) / 1024.0)
                * (2.0 * lat_rad).sin()
            + (15.0 * e2.powi(2) / 256.0 + 45.0 * e2.powi(3) / 1024.0)
                * (4.0 * lat_rad).sin()
            - (35.0 * e2.powi(3) / 3072.0) * (6.0 * lat_rad).sin());

    let easting = k0 * n
        * (a_coeff
            + (1.0 - t + c) * a_coeff.powi(3) / 6.0
            + (5.0 - 18.0 * t + t.powi(2) + 72.0 * c - 58.0 * e_prime2) * a_coeff.powi(5)
                / 120.0)
        + 500000.0;

    let northing = k0
        * (m + n * lat_rad.tan()
            * (a_coeff.powi(2) / 2.0
                + (5.0 - t + 9.0 * c + 4.0 * c.powi(2)) * a_coeff.powi(4) / 24.0
                + (61.0 - 58.0 * t + t.powi(2) + 600.0 * c - 330.0 * e_prime2)
                    * a_coeff.powi(6)
                    / 720.0));

    (easting, northing)
}

/// Convert UTM32N (EPSG:25832) back to WGS84 (lon, lat)
fn utm32n_to_wgs84(easting: f64, northing: f64) -> (f64, f64) {
    let a: f64 = 6378137.0;
    let f: f64 = 1.0 / 298.257223563;
    let e2: f64 = 2.0 * f - f * f;
    let e_prime2 = e2 / (1.0 - e2);
    let k0 = 0.9996_f64;
    let lng0 = 9.0_f64;

    let x = easting - 500000.0;
    let y = northing;

    let m = y / k0;
    let mu = m / (a * (1.0 - e2 / 4.0 - 3.0 * e2.powi(2) / 64.0 - 5.0 * e2.powi(3) / 256.0));

    let e1 = (1.0 - (1.0 - e2).sqrt()) / (1.0 + (1.0 - e2).sqrt());
    let phi1 = mu
        + (3.0 * e1 / 2.0 - 27.0 * e1.powi(3) / 32.0) * (2.0 * mu).sin()
        + (21.0 * e1.powi(2) / 16.0 - 55.0 * e1.powi(4) / 32.0) * (4.0 * mu).sin()
        + (151.0 * e1.powi(3) / 96.0) * (6.0 * mu).sin()
        + (1097.0 * e1.powi(4) / 512.0) * (8.0 * mu).sin();

    let n1 = a / (1.0 - e2 * phi1.sin().powi(2)).sqrt();
    let t1 = phi1.tan().powi(2);
    let c1 = e_prime2 * phi1.cos().powi(2);
    let r1 = a * (1.0 - e2) / (1.0 - e2 * phi1.sin().powi(2)).powf(1.5);
    let d = x / (n1 * k0);

    let lat = phi1
        - (n1 * phi1.tan() / r1)
            * (d.powi(2) / 2.0
                - (5.0 + 3.0 * t1 + 10.0 * c1 - 4.0 * c1.powi(2) - 9.0 * e_prime2)
                    * d.powi(4)
                    / 24.0
                + (61.0 + 90.0 * t1 + 298.0 * c1 + 45.0 * t1.powi(2) - 252.0 * e_prime2
                    - 3.0 * c1.powi(2))
                    * d.powi(6)
                    / 720.0);

    let lng = lng0.to_radians()
        + (d - (1.0 + 2.0 * t1 + c1) * d.powi(3) / 6.0
            + (5.0 - 2.0 * c1 + 28.0 * t1 - 3.0 * c1.powi(2) + 8.0 * e_prime2
                + 24.0 * t1.powi(2))
                * d.powi(5)
                / 120.0)
            / phi1.cos();

    (lng.to_degrees(), lat.to_degrees())
}

/// Compute 1-4 tiles covering a 500m box around a point
fn get_tiles(lat: f64, lng: f64) -> Vec<(u32, u32)> {
    let (easting, northing) = wgs84_to_utm32n(lat, lng);
    let radius = 250.0;

    let min_e = ((easting - radius) / 2000.0).floor() as u32 * 2;
    let max_e = ((easting + radius) / 2000.0).floor() as u32 * 2;
    let min_n = ((northing - radius) / 2000.0).floor() as u32 * 2;
    let max_n = ((northing + radius) / 2000.0).floor() as u32 * 2;

    let mut tiles = Vec::new();
    let mut e = min_e;
    while e <= max_e {
        let mut n = min_n;
        while n <= max_n {
            if e >= 400 && e <= 900 && n >= 5200 && n <= 5650 {
                tiles.push((e, n));
            }
            n += 2;
        }
        e += 2;
    }
    tiles
}

fn compute_cache_key(tiles: &[(u32, u32)], lat: f64, lng: f64) -> String {
    let mut sorted = tiles.to_vec();
    sorted.sort();
    let tiles_part = sorted
        .iter()
        .map(|(e, n)| format!("{}_{}", e, n))
        .collect::<Vec<_>>()
        .join("-");
    format!("{}-{:.4}_{:.4}", tiles_part, lat, lng)
}

async fn fetch_tile(easting: u32, northing: u32) -> Result<(String, bool), StatusCode> {
    let tile_name = format!("{}_{}", easting, northing);
    let cache_dir = std::path::Path::new(GML_CACHE_DIR);
    let cache_path = cache_dir.join(format!("{}.gml", tile_name));

    // Check disk cache
    if cache_path.exists() {
        tracing::info!("GML cache hit: {}", tile_name);
        let text = tokio::fs::read_to_string(&cache_path)
            .await
            .map_err(|e| {
                tracing::error!("Failed to read cached GML {}: {}", tile_name, e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
        return Ok((text, true));
    }

    let url = format!(
        "https://download1.bayernwolke.de/a/lod2/citygml/{}.gml",
        tile_name
    );
    tracing::info!("Fetching tile: {}", url);

    let resp = reqwest::get(&url).await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(StatusCode::NOT_FOUND);
    }
    if !resp.status().is_success() {
        return Err(StatusCode::BAD_GATEWAY);
    }
    let text = resp.text().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    // Write to disk cache
    if let Err(e) = tokio::fs::create_dir_all(cache_dir).await {
        tracing::warn!("Failed to create GML cache dir: {}", e);
    } else if let Err(e) = tokio::fs::write(&cache_path, &text).await {
        tracing::warn!("Failed to cache GML {}: {}", tile_name, e);
    } else {
        tracing::info!(
            "Cached GML: {} ({:.1} MB)",
            tile_name,
            text.len() as f64 / 1_000_000.0
        );
    }

    Ok((text, false))
}

/// Maps AdV building function codes (e.g. "31001_1000") to German labels.
fn building_function_label(code: &str) -> String {
    // Strip optional "31001_" or "51001_" etc. prefix for lookup, but also try full code
    let lookup = |c: &str| -> Option<&'static str> {
        match c {
            "1000" => Some("Wohngebäude"),
            "1010" => Some("Wohnhaus"),
            "1020" => Some("Wohnheim"),
            "1021" => Some("Kinderheim"),
            "1022" => Some("Seniorenheim"),
            "1023" => Some("Schwesternwohnheim"),
            "1024" => Some("Studenten-/Schülerwohnheim"),
            "1025" => Some("Schullandheim"),
            "1100" => Some("Gemischt genutztes Gebäude mit Wohnen"),
            "1110" => Some("Wohngebäude mit Gemeinbedarf"),
            "1120" => Some("Wohngebäude mit Handel und Dienstleistungen"),
            "1121" => Some("Wohn- und Verwaltungsgebäude"),
            "1122" => Some("Wohn- und Bürogebäude"),
            "1123" => Some("Wohn- und Geschäftsgebäude"),
            "1130" => Some("Wohngebäude mit Gewerbe und Industrie"),
            "1131" => Some("Wohn- und Betriebsgebäude"),
            "1210" => Some("Land-/forstwirtschaftliches Wohngebäude"),
            "1220" => Some("Land-/forstwirtschaftliches Wohn- und Betriebsgebäude"),
            "1221" => Some("Bauernhaus"),
            "1222" => Some("Wohn- und Wirtschaftsgebäude"),
            "1223" => Some("Forsthaus"),
            "1310" => Some("Gebäude zur Freizeitgestaltung"),
            "1311" => Some("Ferienhaus"),
            "1312" => Some("Wochenendhaus"),
            "1313" => Some("Gartenhaus"),
            "2000" => Some("Gebäude für Wirtschaft oder Gewerbe"),
            "2010" => Some("Gebäude für Handel und Dienstleistungen"),
            "2020" => Some("Bürogebäude"),
            "2030" => Some("Kreditinstitut"),
            "2040" => Some("Versicherung"),
            "2050" => Some("Geschäftsgebäude"),
            "2051" => Some("Kaufhaus"),
            "2052" => Some("Einkaufszentrum"),
            "2053" => Some("Markthalle"),
            "2054" => Some("Laden"),
            "2055" => Some("Kiosk"),
            "2056" => Some("Apotheke"),
            "2060" => Some("Messehalle"),
            "2070" => Some("Gebäude für Beherbergung"),
            "2071" => Some("Hotel, Motel, Pension"),
            "2072" => Some("Jugendherberge"),
            "2073" => Some("Hütte (mit Übernachtung)"),
            "2074" => Some("Campingplatzgebäude"),
            "2080" => Some("Gebäude für Bewirtung"),
            "2081" => Some("Gaststätte, Restaurant"),
            "2082" => Some("Hütte (ohne Übernachtung)"),
            "2083" => Some("Kantine"),
            "2090" => Some("Freizeit- und Vergnügungsstätte"),
            "2091" => Some("Festsaal"),
            "2092" => Some("Kino"),
            "2093" => Some("Kegel-/Bowlinghalle"),
            "2094" => Some("Spielkasino"),
            "2095" => Some("Spielhalle"),
            "2100" => Some("Gebäude für Gewerbe und Industrie"),
            "2110" => Some("Produktionsgebäude"),
            "2111" => Some("Fabrik"),
            "2112" => Some("Betriebsgebäude"),
            "2113" => Some("Brauerei"),
            "2114" => Some("Brennerei"),
            "2120" => Some("Werkstatt"),
            "2121" => Some("Sägewerk"),
            "2130" => Some("Tankstelle"),
            "2131" => Some("Waschanlage"),
            "2140" => Some("Gebäude für Vorratshaltung"),
            "2141" => Some("Kühlhaus"),
            "2142" => Some("Speichergebäude"),
            "2143" => Some("Lagerhalle"),
            "2150" => Some("Speditionsgebäude"),
            "2160" => Some("Gebäude für Forschungszwecke"),
            "2170" => Some("Gebäude für Grundstoffgewinnung"),
            "2171" => Some("Bergwerk"),
            "2172" => Some("Saline"),
            "2180" => Some("Betriebliche Sozialeinrichtung"),
            "2200" => Some("Sonstiges Gewerbe-/Industriegebäude"),
            "2210" => Some("Mühle"),
            "2211" => Some("Windmühle"),
            "2212" => Some("Wassermühle"),
            "2213" => Some("Schöpfwerk"),
            "2220" => Some("Wetterstation"),
            "2310" => Some("Handel/Dienstleistung mit Wohnen"),
            "2320" => Some("Gewerbe/Industrie mit Wohnen"),
            "2400" => Some("Betriebsgebäude zu Verkehrsanlagen"),
            "2410" => Some("Betriebsgebäude Straßenverkehr"),
            "2411" => Some("Straßenmeisterei"),
            "2412" => Some("Wartehalle"),
            "2420" => Some("Betriebsgebäude Schienenverkehr"),
            "2421" => Some("Bahnwärterhaus"),
            "2422" => Some("Lokschuppen"),
            "2423" => Some("Stellwerk"),
            "2424" => Some("Betriebsgebäude Güterbahnhof"),
            "2430" => Some("Betriebsgebäude Flugverkehr"),
            "2431" => Some("Flugzeughalle"),
            "2440" => Some("Betriebsgebäude Schiffsverkehr"),
            "2441" => Some("Werft"),
            "2442" => Some("Dock"),
            "2443" => Some("Betriebsgebäude Schleuse"),
            "2444" => Some("Bootshaus"),
            "2450" => Some("Betriebsgebäude Seilbahn"),
            "2460" => Some("Gebäude zum Parken"),
            "2461" => Some("Parkhaus"),
            "2462" => Some("Parkdeck"),
            "2463" => Some("Garage"),
            "2464" => Some("Fahrzeughalle"),
            "2500" => Some("Gebäude zur Versorgung"),
            "2501" => Some("Gebäude zur Energieversorgung"),
            "2510" => Some("Gebäude zur Wasserversorgung"),
            "2511" => Some("Wasserwerk"),
            "2512" => Some("Pumpstation"),
            "2513" => Some("Wasserbehälter"),
            "2520" => Some("Gebäude zur Elektrizitätsversorgung"),
            "2521" => Some("Elektrizitätswerk"),
            "2522" => Some("Umspannwerk"),
            "2527" => Some("Reaktorgebäude"),
            "2528" => Some("Turbinenhaus"),
            "2529" => Some("Kesselhaus"),
            "2540" => Some("Gebäude für Fernmeldewesen"),
            "2560" => Some("Gebäude an unterirdischen Leitungen"),
            "2570" => Some("Gebäude zur Gasversorgung"),
            "2571" => Some("Gaswerk"),
            "2580" => Some("Heizwerk"),
            "2590" => Some("Gebäude zur Versorgungsanlage"),
            "2600" => Some("Gebäude zur Entsorgung"),
            "2610" => Some("Gebäude zur Abwasserbeseitigung"),
            "2611" => Some("Kläranlage"),
            "2612" => Some("Toilette"),
            "2620" => Some("Gebäude zur Abfallbehandlung"),
            "2621" => Some("Müllbunker"),
            "2622" => Some("Müllverbrennung"),
            "2623" => Some("Abfalldeponie"),
            "2700" => Some("Gebäude für Land- und Forstwirtschaft"),
            "2720" => Some("Land-/forstwirtschaftliches Betriebsgebäude"),
            "2721" => Some("Scheune"),
            "2723" => Some("Schuppen"),
            "2724" => Some("Stall"),
            "2726" => Some("Scheune und Stall"),
            "2727" => Some("Stall für Tiergroßhaltung"),
            "2728" => Some("Reithalle"),
            "2729" => Some("Wirtschaftsgebäude"),
            "2732" => Some("Almhütte"),
            "2735" => Some("Jagdhaus"),
            "2740" => Some("Treibhaus, Gewächshaus"),
            "2741" => Some("Treibhaus"),
            "2742" => Some("Gewächshaus, verschiebbar"),
            "3000" => Some("Gebäude für öffentliche Zwecke"),
            "3010" => Some("Verwaltungsgebäude"),
            "3011" => Some("Parlament"),
            "3012" => Some("Rathaus"),
            "3013" => Some("Post"),
            "3014" => Some("Zollamt"),
            "3015" => Some("Gericht"),
            "3016" => Some("Botschaft, Konsulat"),
            "3017" => Some("Kreisverwaltung"),
            "3018" => Some("Bezirksregierung"),
            "3019" => Some("Finanzamt"),
            "3020" => Some("Gebäude für Bildung und Forschung"),
            "3021" => Some("Allgemein bildende Schule"),
            "3022" => Some("Berufsbildende Schule"),
            "3023" => Some("Hochschule, Universität"),
            "3024" => Some("Forschungsinstitut"),
            "3030" => Some("Gebäude für kulturelle Zwecke"),
            "3031" => Some("Schloss"),
            "3032" => Some("Theater, Oper"),
            "3033" => Some("Konzertgebäude"),
            "3034" => Some("Museum"),
            "3035" => Some("Rundfunk, Fernsehen"),
            "3036" => Some("Veranstaltungsgebäude"),
            "3037" => Some("Bibliothek"),
            "3038" => Some("Burg, Festung"),
            "3040" => Some("Gebäude für religiöse Zwecke"),
            "3041" => Some("Kirche"),
            "3042" => Some("Synagoge"),
            "3043" => Some("Kapelle"),
            "3044" => Some("Gemeindehaus"),
            "3045" => Some("Gotteshaus"),
            "3046" => Some("Moschee"),
            "3047" => Some("Tempel"),
            "3048" => Some("Kloster"),
            "3050" => Some("Gebäude für Gesundheitswesen"),
            "3051" => Some("Krankenhaus"),
            "3052" => Some("Heilanstalt, Pflegestation"),
            "3053" => Some("Ärztehaus, Poliklinik"),
            "3054" => Some("Rettungswache"),
            "3060" => Some("Gebäude für soziale Zwecke"),
            "3061" => Some("Jugendfreizeitheim"),
            "3062" => Some("Freizeit-/Vereinsheim"),
            "3063" => Some("Seniorenfreizeitstätte"),
            "3064" => Some("Obdachlosenheim"),
            "3065" => Some("Kindergarten, Kindertagesstätte"),
            "3066" => Some("Asylbewerberheim"),
            "3070" => Some("Gebäude für Sicherheit und Ordnung"),
            "3071" => Some("Polizei"),
            "3072" => Some("Feuerwehr"),
            "3073" => Some("Kaserne"),
            "3074" => Some("Schutzbunker"),
            "3075" => Some("Justizvollzugsanstalt"),
            "3080" => Some("Friedhofsgebäude"),
            "3081" => Some("Trauerhalle"),
            "3082" => Some("Krematorium"),
            "3090" => Some("Empfangsgebäude"),
            "3091" => Some("Bahnhofsgebäude"),
            "3092" => Some("Flughafengebäude"),
            "3094" => Some("Gebäude zum U-Bahnhof"),
            "3095" => Some("Gebäude zum S-Bahnhof"),
            "3097" => Some("Gebäude zum Busbahnhof"),
            "3098" => Some("Empfangsgebäude Schifffahrt"),
            "3100" => Some("Öffentliches Gebäude mit Wohnen"),
            "3200" => Some("Gebäude für Erholungszwecke"),
            "3210" => Some("Gebäude für Sportzwecke"),
            "3211" => Some("Sport-/Turnhalle"),
            "3212" => Some("Gebäude zum Sportplatz"),
            "3220" => Some("Badegebäude"),
            "3221" => Some("Hallenbad"),
            "3222" => Some("Gebäude im Freibad"),
            "3230" => Some("Gebäude im Stadion"),
            "3240" => Some("Gebäude für Kurbetrieb"),
            "3260" => Some("Gebäude im Zoo"),
            "3270" => Some("Gebäude im botanischen Garten"),
            "3280" => Some("Gebäude für Erholungseinrichtung"),
            "3281" => Some("Schutzhütte"),
            "3290" => Some("Touristisches Informationszentrum"),
            "9998" => Some("Nicht spezifizierbar"),
            _ => None,
        }
    };

    // Try full code first (e.g. "1000"), then strip prefix (e.g. "31001_1000" -> "1000")
    if let Some(label) = lookup(code) {
        return label.to_string();
    }
    if let Some(suffix) = code.split('_').last() {
        if suffix != code {
            if let Some(label) = lookup(suffix) {
                return label.to_string();
            }
        }
    }
    code.to_string()
}

/// Maps AdV roof type codes to German labels.
fn roof_type_label(code: &str) -> String {
    let label = match code {
        "1000" => "Flachdach",
        "2100" => "Pultdach",
        "2200" => "Versetztes Pultdach",
        "3100" => "Satteldach",
        "3200" => "Walmdach",
        "3300" => "Krüppelwalmdach",
        "3400" => "Mansardendach",
        "3500" => "Zeltdach",
        "3600" => "Kegeldach",
        "3700" => "Kuppeldach",
        "3800" => "Sheddach",
        "3900" => "Bogendach",
        "4000" => "Turmdach",
        "5000" => "Mischform",
        "9999" => "Sonstiges",
        _ => return code.to_string(),
    };
    label.to_string()
}

/// Parse CityGML XML into surfaces using quick-xml streaming parser.
/// Extracts building-level attributes: function, roofType, measuredHeight, Gemeindeschluessel.
fn parse_citygml(xml: &str) -> Vec<Surface> {
    let mut reader = Reader::from_str(xml);
    let mut surfaces = Vec::new();
    let mut buf = Vec::new();

    // Building-level state
    let mut in_building = false;
    let mut current_gmlid = String::new();
    let mut bldg_function = String::new();
    let mut bldg_roof_type = String::new();
    let mut bldg_measured_height = 0.0_f64;
    let mut bldg_gemeindeschluessel = String::new();
    let mut bldg_street = String::new();
    let mut bldg_house_number = String::new();
    let mut bldg_zip = String::new();
    let mut bldg_city = String::new();
    // Addresses come after surfaces in CityGML, so collect separately and backfill
    let mut building_streets: HashMap<String, String> = HashMap::new();
    let mut building_house_numbers: HashMap<String, String> = HashMap::new();
    let mut building_zips: HashMap<String, String> = HashMap::new();
    let mut building_cities: HashMap<String, String> = HashMap::new();

    // Surface-level state
    let mut current_surface_type = String::new();
    let mut surface_flaeche = 0.0_f64;
    let mut in_pos_list = false;
    let mut pos_text = String::new();

    // For reading text content of building-level elements
    let mut reading_function = false;
    let mut reading_roof_type = false;
    let mut reading_measured_height = false;
    let mut reading_gen_value = false;
    let mut reading_thoroughfare_name = false;
    let mut reading_thoroughfare_number = false;
    let mut reading_postal_code_number = false;
    let mut reading_locality_name = false;
    let mut current_gen_attr_name = String::new();
    let mut text_buf = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => {
                let name_bytes = e.name().as_ref().to_vec();
                let local = local_name(&name_bytes);
                match local {
                    b"Building" => {
                        in_building = true;
                        current_gmlid =
                            extract_gml_id(e).unwrap_or_else(|| "unknown".to_string());
                        bldg_function.clear();
                        bldg_roof_type.clear();
                        bldg_measured_height = 0.0;
                        bldg_gemeindeschluessel.clear();
                        bldg_street.clear();
                        bldg_house_number.clear();
                        bldg_zip.clear();
                        bldg_city.clear();
                    }
                    b"function" if in_building && current_surface_type.is_empty() => {
                        reading_function = true;
                        text_buf.clear();
                    }
                    b"roofType" if in_building && current_surface_type.is_empty() => {
                        reading_roof_type = true;
                        text_buf.clear();
                    }
                    b"measuredHeight" if in_building && current_surface_type.is_empty() => {
                        reading_measured_height = true;
                        text_buf.clear();
                    }
                    b"stringAttribute" if in_building => {
                        // Extract name attribute from gen:stringAttribute
                        current_gen_attr_name.clear();
                        for attr in e.attributes().flatten() {
                            if local_name(attr.key.as_ref()) == b"name" {
                                current_gen_attr_name =
                                    String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    }
                    b"value"
                        if in_building
                            && (current_gen_attr_name == "Gemeindeschluessel"
                                || current_gen_attr_name == "Flaeche") =>
                    {
                        reading_gen_value = true;
                        text_buf.clear();
                    }
                    b"ThoroughfareName" if in_building => {
                        reading_thoroughfare_name = true;
                        text_buf.clear();
                    }
                    b"ThoroughfareNumber" if in_building => {
                        reading_thoroughfare_number = true;
                        text_buf.clear();
                    }
                    b"PostalCodeNumber" if in_building => {
                        reading_postal_code_number = true;
                        text_buf.clear();
                    }
                    b"LocalityName" if in_building => {
                        reading_locality_name = true;
                        text_buf.clear();
                    }
                    b"WallSurface" => {
                        if in_building {
                            current_surface_type = "Wand".to_string();
                            surface_flaeche = 0.0;
                        }
                    }
                    b"RoofSurface" => {
                        if in_building {
                            current_surface_type = "Dach".to_string();
                            surface_flaeche = 0.0;
                        }
                    }
                    b"GroundSurface" => {
                        if in_building {
                            current_surface_type = "Boden".to_string();
                            surface_flaeche = 0.0;
                        }
                    }
                    b"posList" => {
                        if in_building && !current_surface_type.is_empty() {
                            in_pos_list = true;
                            pos_text.clear();
                        }
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::End(ref e)) => {
                let name_bytes = e.name().as_ref().to_vec();
                let local = local_name(&name_bytes);
                match local {
                    b"Building" => {
                        if !current_gmlid.is_empty() {
                            if !bldg_street.is_empty() {
                                building_streets
                                    .insert(current_gmlid.clone(), bldg_street.clone());
                            }
                            if !bldg_house_number.is_empty() {
                                building_house_numbers
                                    .insert(current_gmlid.clone(), bldg_house_number.clone());
                            }
                            if !bldg_zip.is_empty() {
                                building_zips
                                    .insert(current_gmlid.clone(), bldg_zip.clone());
                            }
                            if !bldg_city.is_empty() {
                                building_cities
                                    .insert(current_gmlid.clone(), bldg_city.clone());
                            }
                        }
                        in_building = false;
                        current_gmlid.clear();
                    }
                    b"function" if reading_function => {
                        reading_function = false;
                        bldg_function = building_function_label(text_buf.trim());
                    }
                    b"roofType" if reading_roof_type => {
                        reading_roof_type = false;
                        bldg_roof_type = roof_type_label(text_buf.trim());
                    }
                    b"measuredHeight" if reading_measured_height => {
                        reading_measured_height = false;
                        bldg_measured_height =
                            text_buf.trim().parse().unwrap_or(0.0);
                    }
                    b"value" if reading_gen_value => {
                        reading_gen_value = false;
                        match current_gen_attr_name.as_str() {
                            "Gemeindeschluessel" => {
                                bldg_gemeindeschluessel = text_buf.trim().to_string();
                            }
                            "Flaeche" => {
                                surface_flaeche = text_buf.trim().parse().unwrap_or(0.0);
                            }
                            _ => {}
                        }
                    }
                    b"ThoroughfareName" if reading_thoroughfare_name => {
                        reading_thoroughfare_name = false;
                        let raw = text_buf.trim().to_string();
                        // Bavarian data often embeds house number in ThoroughfareName
                        // e.g. "Nördlinger Straße 38" — split it out
                        let (street, embedded_hn) = split_street_and_number(&raw);
                        bldg_street = street;
                        if bldg_house_number.is_empty() && !embedded_hn.is_empty() {
                            bldg_house_number = embedded_hn;
                        }
                    }
                    b"ThoroughfareNumber" if reading_thoroughfare_number => {
                        reading_thoroughfare_number = false;
                        // Explicit ThoroughfareNumber takes precedence
                        let hn = text_buf.trim().to_string();
                        if !hn.is_empty() {
                            bldg_house_number = hn;
                        }
                    }
                    b"PostalCodeNumber" if reading_postal_code_number => {
                        reading_postal_code_number = false;
                        bldg_zip = text_buf.trim().to_string();
                    }
                    b"LocalityName" if reading_locality_name => {
                        reading_locality_name = false;
                        bldg_city = text_buf.trim().to_string();
                    }
                    b"stringAttribute" => {
                        current_gen_attr_name.clear();
                    }
                    b"WallSurface" | b"RoofSurface" | b"GroundSurface" => {
                        current_surface_type.clear();
                    }
                    b"posList" => {
                        if in_pos_list {
                            in_pos_list = false;
                            if let Some(coords) = parse_pos_list(&pos_text) {
                                if coords.len() >= 3 {
                                    surfaces.push(Surface {
                                        gmlid: current_gmlid.clone(),
                                        surface_type: current_surface_type.clone(),
                                        coords,
                                        flaeche: surface_flaeche,
                                        function: bldg_function.clone(),
                                        roof_type: bldg_roof_type.clone(),
                                        measured_height: bldg_measured_height,
                                        gemeindeschluessel: bldg_gemeindeschluessel
                                            .clone(),
                                        street: bldg_street.clone(),
                                        house_number: bldg_house_number.clone(),
                                        zip: bldg_zip.clone(),
                                        city: bldg_city.clone(),
                                    });
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::Text(ref e)) => {
                if let Ok(text) = e.unescape() {
                    if in_pos_list {
                        pos_text.push_str(&text);
                    } else if reading_function
                        || reading_roof_type
                        || reading_measured_height
                        || reading_gen_value
                        || reading_thoroughfare_name
                        || reading_thoroughfare_number
                        || reading_postal_code_number
                        || reading_locality_name
                    {
                        text_buf.push_str(&text);
                    }
                }
            }
            Ok(XmlEvent::Eof) => break,
            Err(e) => {
                tracing::warn!("XML parse error: {:?}", e);
                break;
            }
            _ => {}
        }
        buf.clear();
    }

    // Backfill addresses, zips, and cities (xAL data comes after surfaces in CityGML)
    for s in &mut surfaces {
        if s.street.is_empty() {
            if let Some(street) = building_streets.get(&s.gmlid) {
                s.street = street.clone();
            }
        }
        if s.house_number.is_empty() {
            if let Some(hn) = building_house_numbers.get(&s.gmlid) {
                s.house_number = hn.clone();
            }
        }
        if s.zip.is_empty() {
            if let Some(zip) = building_zips.get(&s.gmlid) {
                s.zip = zip.clone();
            }
        }
        if s.city.is_empty() {
            if let Some(city) = building_cities.get(&s.gmlid) {
                s.city = city.clone();
            }
        }
    }

    surfaces
}

fn local_name(name: &[u8]) -> &[u8] {
    if let Some(pos) = name.iter().position(|&b| b == b':') {
        &name[pos + 1..]
    } else {
        name
    }
}

fn extract_gml_id(e: &quick_xml::events::BytesStart) -> Option<String> {
    for attr in e.attributes().flatten() {
        let local = local_name(attr.key.as_ref());
        if local == b"id" {
            return String::from_utf8(attr.value.to_vec()).ok();
        }
    }
    None
}

fn parse_pos_list(text: &str) -> Option<Vec<[f64; 3]>> {
    let nums: Vec<f64> = text
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    if nums.len() < 9 {
        return None;
    }
    let mut coords = Vec::with_capacity(nums.len() / 3);
    for chunk in nums.chunks_exact(3) {
        coords.push([chunk[0], chunk[1], chunk[2]]);
    }
    Some(coords)
}

/// Encode a 3D polygon ring as WKB PolygonZ (ISO type 1003)
fn polygon_to_wkb(coords: &[[f64; 3]]) -> Vec<u8> {
    let n = coords.len();
    let mut wkb = Vec::with_capacity(1 + 4 + 4 + 4 + n * 24);
    wkb.push(1u8); // little endian
    wkb.extend_from_slice(&1003u32.to_le_bytes()); // wkbPolygonZ
    wkb.extend_from_slice(&1u32.to_le_bytes()); // 1 ring
    wkb.extend_from_slice(&(n as u32).to_le_bytes());
    for c in coords {
        wkb.extend_from_slice(&c[0].to_le_bytes());
        wkb.extend_from_slice(&c[1].to_le_bytes());
        wkb.extend_from_slice(&c[2].to_le_bytes());
    }
    wkb
}

/// Normalize address for fuzzy comparison (lowercase, trim, collapse whitespace)
fn normalize_address(addr: &str) -> String {
    addr.trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Split a string like "83229 Aschau im Chiemgau" into (zip, city_name)
fn split_zip_city(s: &str) -> (String, String) {
    let trimmed = s.trim();
    let first_word = trimmed.split_whitespace().next().unwrap_or("");
    if !first_word.is_empty() && first_word.chars().all(|c| c.is_ascii_digit()) {
        let rest = trimmed[first_word.len()..].trim().to_string();
        (first_word.to_string(), rest)
    } else {
        (String::new(), trimmed.to_string())
    }
}

/// Parse a query address like "Hauptstr. 5, 83229 Aschau im Chiemgau" into (street_name, house_number, zip, city)
fn parse_query_components(addr: &str) -> (String, String, String, String) {
    let normalized = addr.trim();
    if let Some(comma_pos) = normalized.find(',') {
        let street_part = normalized[..comma_pos].trim();
        let city_part = normalized[comma_pos + 1..].trim();
        let (zip, city) = split_zip_city(city_part);

        // Check if there's a second comma (street, zip, city)
        // or if the street part contains a house number
        let (street_name, house_number) = split_street_and_number(street_part);
        (street_name, house_number, zip, city)
    } else {
        // No comma — could be "83229 Aschau im Chiemgau" or just "Aschau im Chiemgau"
        let (zip, city) = split_zip_city(normalized);
        (String::new(), String::new(), zip, city)
    }
}

/// Split "Rathausplatz 2" into ("Rathausplatz", "2") or "Friedrich-Ebert-Straße" into ("Friedrich-Ebert-Straße", "")
fn split_street_and_number(street: &str) -> (String, String) {
    let trimmed = street.trim();
    // Try to find a trailing house number (possibly with letter suffix like "2a")
    // Pattern: last whitespace-separated token starts with a digit
    if let Some(last_space) = trimmed.rfind(' ') {
        let candidate = &trimmed[last_space + 1..];
        if candidate.starts_with(|c: char| c.is_ascii_digit()) {
            let name = trimmed[..last_space].trim().to_string();
            let number = candidate.to_string();
            return (name, number);
        }
    }
    (trimmed.to_string(), String::new())
}

/// Classify what type of address query the user entered
#[derive(Debug)]
enum AddressQueryType {
    /// Full address with street + house number + zip/city → match single building
    FullAddress { street: String, house_number: String, zip: String, city: String },
    /// Street-level with street name (no house number) + zip/city → match all buildings on that street
    StreetAddress { street: String, zip: String, city: String },
    /// City-level with just zip and/or city → match everything in that area
    CityAddress { zip: String, city: String },
}

fn classify_address_query(addr: &str) -> AddressQueryType {
    let (street, house_number, zip, city) = parse_query_components(addr);

    if !street.is_empty() && !house_number.is_empty() {
        AddressQueryType::FullAddress { street, house_number, zip, city }
    } else if !street.is_empty() {
        AddressQueryType::StreetAddress { street, zip, city }
    } else {
        AddressQueryType::CityAddress { zip, city }
    }
}

/// Apply dithering: buildings farther from query point are probabilistically removed.
/// Returns the filtered surfaces and a set of "queried" gmlids.
/// If an address is provided, matches by xAL ThoroughfareName; otherwise falls back to closest centroid.
fn apply_dithering_and_queried(
    surfaces: Vec<Surface>,
    query_easting: f64,
    query_northing: f64,
    query_address: Option<&str>,
) -> (Vec<Surface>, HashSet<String>) {
    // Group surfaces by gmlid → compute building centroid in UTM32N
    let mut building_coords: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
    let mut building_street: HashMap<String, String> = HashMap::new();
    let mut building_house_number: HashMap<String, String> = HashMap::new();
    let mut building_zip: HashMap<String, String> = HashMap::new();
    let mut building_city: HashMap<String, String> = HashMap::new();

    for s in &surfaces {
        let entry = building_coords.entry(s.gmlid.clone()).or_default();
        for c in &s.coords {
            entry.push((c[0], c[1])); // UTM32N easting, northing
        }
        if !s.street.is_empty() {
            building_street
                .entry(s.gmlid.clone())
                .or_insert_with(|| s.street.clone());
        }
        if !s.house_number.is_empty() {
            building_house_number
                .entry(s.gmlid.clone())
                .or_insert_with(|| s.house_number.clone());
        }
        if !s.zip.is_empty() {
            building_zip
                .entry(s.gmlid.clone())
                .or_insert_with(|| s.zip.clone());
        }
        if !s.city.is_empty() {
            building_city
                .entry(s.gmlid.clone())
                .or_insert_with(|| s.city.clone());
        }
    }

    // Compute centroids and distances
    let mut building_centroids: HashMap<String, (f64, f64, f64)> = HashMap::new();
    let mut closest_gmlid = String::new();
    let mut closest_dist = f64::MAX;

    for (gmlid, coords) in &building_coords {
        let n = coords.len() as f64;
        let cx = coords.iter().map(|c| c.0).sum::<f64>() / n;
        let cy = coords.iter().map(|c| c.1).sum::<f64>() / n;
        let dist = ((cx - query_easting).powi(2) + (cy - query_northing).powi(2)).sqrt();
        building_centroids.insert(gmlid.clone(), (cx, cy, dist));
        if dist < closest_dist {
            closest_dist = dist;
            closest_gmlid = gmlid.clone();
        }
    }

    // Determine queried building(s) based on address query type
    let queried_gmlids: HashSet<String> = if let Some(addr) = query_address {
        let query_type = classify_address_query(addr);
        tracing::info!("Address query type: {:?}", query_type);

        let matched: HashSet<String> = match query_type {
            // Full address: match building(s) with matching street + house number + zip/city
            AddressQueryType::FullAddress { street, house_number, zip, city } => {
                let norm_street = normalize_address(&street);
                let norm_hn = normalize_address(&house_number);
                let norm_zip = normalize_address(&zip);
                let norm_city = normalize_address(&city);

                let location_matches = |gmlid: &str| -> bool {
                    if !norm_zip.is_empty() {
                        if let Some(bz) = building_zip.get(gmlid) {
                            if normalize_address(bz) == norm_zip { return true; }
                        }
                    }
                    if !norm_city.is_empty() {
                        if let Some(bc) = building_city.get(gmlid) {
                            if normalize_address(bc) == norm_city { return true; }
                        }
                    }
                    // If no zip/city in query, don't filter by location
                    norm_zip.is_empty() && norm_city.is_empty()
                };

                // Match by street name + house number + location
                let mut result: HashSet<String> = building_street
                    .iter()
                    .filter(|(gmlid, bldg_street)| {
                        let street_matches = normalize_address(bldg_street) == norm_street;
                        let hn_matches = building_house_number
                            .get(gmlid.as_str())
                            .map(|hn| normalize_address(hn) == norm_hn)
                            .unwrap_or(false);
                        street_matches && hn_matches && location_matches(gmlid)
                    })
                    .map(|(gmlid, _)| gmlid.clone())
                    .collect();

                // Fallback: some CityGML data stores house number inside ThoroughfareName
                if result.is_empty() {
                    let full_street = normalize_address(&format!("{} {}", street, house_number));
                    result = building_street
                        .iter()
                        .filter(|(gmlid, bldg_street)| {
                            normalize_address(bldg_street) == full_street && location_matches(gmlid)
                        })
                        .map(|(gmlid, _)| gmlid.clone())
                        .collect();
                }

                if !result.is_empty() {
                    tracing::info!(
                        "Full address '{} {}, {} {}' matched {} building(s)",
                        street, house_number, zip, city, result.len()
                    );
                }
                result
            }

            // Street address: match all buildings on that street (+ zip/city)
            AddressQueryType::StreetAddress { street, zip, city } => {
                let norm_street = normalize_address(&street);
                let norm_zip = normalize_address(&zip);
                let norm_city = normalize_address(&city);

                let location_matches = |gmlid: &str| -> bool {
                    if !norm_zip.is_empty() {
                        if let Some(bz) = building_zip.get(gmlid) {
                            if normalize_address(bz) == norm_zip { return true; }
                        }
                    }
                    if !norm_city.is_empty() {
                        if let Some(bc) = building_city.get(gmlid) {
                            if normalize_address(bc) == norm_city { return true; }
                        }
                    }
                    norm_zip.is_empty() && norm_city.is_empty()
                };

                // Match by exact street name + location
                let mut result: HashSet<String> = building_street
                    .iter()
                    .filter(|(gmlid, bldg_street)| {
                        normalize_address(bldg_street) == norm_street && location_matches(gmlid)
                    })
                    .map(|(gmlid, _)| gmlid.clone())
                    .collect();

                // Try prefix match if exact didn't work
                if result.is_empty() {
                    result = building_street
                        .iter()
                        .filter(|(gmlid, bldg_street)| {
                            normalize_address(bldg_street).starts_with(&norm_street)
                                && location_matches(gmlid)
                        })
                        .map(|(gmlid, _)| gmlid.clone())
                        .collect();
                }

                if !result.is_empty() {
                    tracing::info!(
                        "Street '{}, {} {}' matched {} building(s)",
                        street, zip, city, result.len()
                    );
                }
                result
            }

            // City address: match everything in that zip/city
            AddressQueryType::CityAddress { zip, city } => {
                let mut result = HashSet::new();

                // Try zip code first
                if !zip.is_empty() {
                    result = building_zip
                        .iter()
                        .filter(|(_, bldg_zip)| *bldg_zip == &zip)
                        .map(|(gmlid, _)| gmlid.clone())
                        .collect();
                    if !result.is_empty() {
                        tracing::info!("Zip '{}' matched {} building(s)", zip, result.len());
                    }
                }

                // Try city name if zip didn't match
                if result.is_empty() && !city.is_empty() {
                    let norm_city = normalize_address(&city);
                    result = building_city
                        .iter()
                        .filter(|(_, bldg_city)| normalize_address(bldg_city) == norm_city)
                        .map(|(gmlid, _)| gmlid.clone())
                        .collect();

                    // Fuzzy: try first-word prefix (handles "Aschau i.Chiemgau")
                    if result.is_empty() {
                        let first_word = norm_city.split_whitespace().next().unwrap_or("");
                        if !first_word.is_empty() {
                            result = building_city
                                .iter()
                                .filter(|(_, bldg_city)| {
                                    normalize_address(bldg_city).starts_with(first_word)
                                })
                                .map(|(gmlid, _)| gmlid.clone())
                                .collect();
                        }
                    }

                    if !result.is_empty() {
                        tracing::info!("City '{}' matched {} building(s)", city, result.len());
                    }
                }
                result
            }
        };

        // Fallback to closest building if nothing matched
        if matched.is_empty() {
            tracing::info!(
                "No address match for '{}', falling back to closest building",
                addr
            );
            let mut set = HashSet::new();
            if !closest_gmlid.is_empty() {
                set.insert(closest_gmlid);
            }
            set
        } else {
            matched
        }
    } else if !closest_gmlid.is_empty() {
        let mut set = HashSet::new();
        set.insert(closest_gmlid);
        set
    } else {
        HashSet::new()
    };

    // Dithering: filter buildings by distance
    let inner_radius = 400.0_f64;
    let outer_radius = 800.0_f64;

    let kept_buildings: HashSet<String> = building_centroids
        .iter()
        .filter(|(gmlid, (_, _, dist))| {
            if *dist <= inner_radius {
                return true;
            }
            if *dist >= outer_radius {
                return false;
            }
            // Linear falloff
            let keep_prob = 1.0 - (*dist - inner_radius) / (outer_radius - inner_radius);
            let mut hasher = DefaultHasher::new();
            gmlid.hash(&mut hasher);
            let hash_val = hasher.finish() % 100;
            (hash_val as f64) < keep_prob * 100.0
        })
        .map(|(gmlid, _)| gmlid.clone())
        .collect();

    let filtered: Vec<Surface> = surfaces
        .into_iter()
        .filter(|s| kept_buildings.contains(&s.gmlid))
        .collect();

    (filtered, queried_gmlids)
}

/// Build GeoParquet bytes from parsed surfaces (coordinates converted to WGS84)
fn build_geoparquet(
    surfaces: &[Surface],
    queried_gmlids: &HashSet<String>,
) -> Result<Vec<u8>, String> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("gmlid", DataType::Utf8, false),
        Field::new("surface_type", DataType::Utf8, false),
        Field::new("flaeche", DataType::Float64, true),
        Field::new("function", DataType::Utf8, true),
        Field::new("roof_type", DataType::Utf8, true),
        Field::new("measured_height", DataType::Float64, true),
        Field::new("gemeindeschluessel", DataType::Utf8, true),
        Field::new("address", DataType::Utf8, true),
        Field::new("city", DataType::Utf8, true),
        Field::new("queried", DataType::Utf8, false),
        Field::new("geometry", DataType::Binary, false),
    ]));

    let mut gmlid_builder = StringBuilder::new();
    let mut stype_builder = StringBuilder::new();
    let mut flaeche_builder = Float64Builder::new();
    let mut function_builder = StringBuilder::new();
    let mut roof_type_builder = StringBuilder::new();
    let mut height_builder = Float64Builder::new();
    let mut gemeinde_builder = StringBuilder::new();
    let mut address_builder = StringBuilder::new();
    let mut city_builder = StringBuilder::new();
    let mut queried_builder = StringBuilder::new();
    let mut geom_builder = BinaryBuilder::new();

    for s in surfaces {
        gmlid_builder.append_value(&s.gmlid);
        stype_builder.append_value(&s.surface_type);
        flaeche_builder.append_value(s.flaeche);
        function_builder.append_value(&s.function);
        roof_type_builder.append_value(&s.roof_type);
        height_builder.append_value(s.measured_height);
        gemeinde_builder.append_value(&s.gemeindeschluessel);

        // Combine street, house_number, zip, and city into full address: "Street Nr, Zip City"
        let street_part = if s.house_number.is_empty() {
            s.street.clone()
        } else if s.street.is_empty() {
            String::new()
        } else {
            format!("{} {}", s.street, s.house_number)
        };
        let city_part = [s.zip.as_str(), s.city.as_str()]
            .iter()
            .filter(|p| !p.is_empty())
            .copied()
            .collect::<Vec<_>>()
            .join(" ");
        let full_address = if street_part.is_empty() {
            city_part
        } else if city_part.is_empty() {
            street_part
        } else {
            format!("{}, {}", street_part, city_part)
        };
        address_builder.append_value(&full_address);
        city_builder.append_value(&s.city);
        queried_builder.append_value(if queried_gmlids.contains(&s.gmlid) {
            "true"
        } else {
            "false"
        });

        // Convert UTM32N coords to WGS84 (lon, lat, z_unchanged)
        let wgs84_coords: Vec<[f64; 3]> = s
            .coords
            .iter()
            .map(|c| {
                let (lon, lat) = utm32n_to_wgs84(c[0], c[1]);
                [lon, lat, c[2]]
            })
            .collect();

        geom_builder.append_value(polygon_to_wkb(&wgs84_coords));
    }

    let mut metadata = HashMap::new();
    metadata.insert(
        "geo".to_string(),
        serde_json::json!({
            "version": "1.0.0",
            "primary_column": "geometry",
            "columns": {
                "geometry": {
                    "encoding": "WKB",
                    "geometry_types": ["PolygonZ"],
                    "crs": {
                        "id": { "authority": "EPSG", "code": 4326 }
                    }
                }
            }
        })
        .to_string(),
    );
    let schema_with_meta = Arc::new(schema.as_ref().clone().with_metadata(metadata));

    let batch = RecordBatch::try_new(schema_with_meta.clone(), vec![
        Arc::new(gmlid_builder.finish()),
        Arc::new(stype_builder.finish()),
        Arc::new(flaeche_builder.finish()),
        Arc::new(function_builder.finish()),
        Arc::new(roof_type_builder.finish()),
        Arc::new(height_builder.finish()),
        Arc::new(gemeinde_builder.finish()),
        Arc::new(address_builder.finish()),
        Arc::new(city_builder.finish()),
        Arc::new(queried_builder.finish()),
        Arc::new(geom_builder.finish()),
    ])
    .map_err(|e| format!("Arrow error: {:?}", e))?;

    let mut buf = Vec::new();
    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD(Default::default()))
        .build();

    let mut writer = ArrowWriter::try_new(&mut buf, batch.schema(), Some(props))
        .map_err(|e| format!("Parquet writer error: {:?}", e))?;
    writer
        .write(&batch)
        .map_err(|e| format!("Parquet write error: {:?}", e))?;
    writer
        .close()
        .map_err(|e| format!("Parquet close error: {:?}", e))?;

    Ok(buf)
}

/// Fill qq's geo_map.html template with GeoParquet data and write to disk
fn render_geoviewer_html(
    parquet_bytes: &[u8],
    surfaces_count: usize,
    duration_ms: u64,
    cache_key: &str,
) -> String {
    let b64 = BASE64.encode(parquet_bytes);
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

    get_geo_map_template()
        .replace("__GEOPARQUET_BASE64__", &b64)
        .replace("__COLUMNS_JSON__", r#"["gmlid", "surface_type", "flaeche", "function", "roof_type", "measured_height", "address", "queried"]"#)
        .replace("__DEFAULT_CATEGORY__", r#""queried""#)
        .replace("__RUN_ID__", cache_key)
        .replace("__QUERY_SQL__", "CityGML LOD2 Bavaria Open Data")
        .replace("__QUERY_ENGINE__", "Bayern Atlas")
        .replace("__QUERY_HOST__", "download1.bayernwolke.de")
        .replace("__QUERY_DATABASE__", "CityGML LOD2")
        .replace("__QUERY_USER__", "public")
        .replace("__QUERY_ROW_COUNT__", &surfaces_count.to_string())
        .replace("__QUERY_DURATION_MS__", &duration_ms.to_string())
        .replace("__QUERY_DATETIME__", &now)
        .replace("__MAPTILER_STYLE_JSON__", get_maptiler_style_json())
        .replace("__VERSION__", "valentinrogg.de")
}

/// Log a geo event to the events table
async fn log_geo_event(db: &SqlitePool, session_id: &str, event_name: &str, properties: &serde_json::Value) {
    let now = chrono::Utc::now().timestamp_millis();
    let event_id = uuid::Uuid::new_v4().to_string();
    let props_str = serde_json::to_string(properties).unwrap_or_default();

    // Upsert session
    let _ = sqlx::query(
        "INSERT INTO sessions (id, ts_created, ts_last_seen) VALUES (?, ?, ?) ON CONFLICT(id) DO UPDATE SET ts_last_seen = excluded.ts_last_seen",
    )
    .bind(session_id)
    .bind(now)
    .bind(now)
    .execute(db)
    .await;

    // Insert event
    let _ = sqlx::query(
        "INSERT INTO events (id, session_id, event_name, ts, properties) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&event_id)
    .bind(session_id)
    .bind(event_name)
    .bind(now)
    .bind(&props_str)
    .execute(db)
    .await;
}

/// SSE endpoint: GET /api/geo/buildings/stream?lat=48.3654&lng=10.8978&session_id=UUID
pub async fn stream_geo_buildings(
    axum::extract::Query(query): axum::extract::Query<GeoBuildingsQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Sse<impl futures::Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    if query.lat < 47.27 || query.lat > 50.56 || query.lng < 8.98 || query.lng > 13.84 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let tiles = get_tiles(query.lat, query.lng);
    if tiles.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let cache_key = compute_cache_key(&tiles, query.lat, query.lng);
    let session_id = query.session_id.clone();
    let db = state.db.clone();
    let query_lat = query.lat;
    let query_lng = query.lng;
    let query_address = query.address.clone();

    // Log address search event
    if let Some(ref sid) = session_id {
        let db = db.clone();
        let sid = sid.clone();
        let lat = query.lat;
        let lng = query.lng;
        tokio::spawn(async move {
            log_geo_event(
                &db,
                &sid,
                "geo.address.search",
                &serde_json::json!({"lat": lat, "lng": lng}),
            )
            .await;
        });
    }

    let is_default_location = (query.lat - DEFAULT_LAT).abs() < 1e-4
        && (query.lng - DEFAULT_LNG).abs() < 1e-4;

    // Serve from memory cache (only the default location is cached)
    if is_default_location {
        if let Some(html) = html_cache().lock().unwrap().get(&cache_key) {
            if !html.is_empty() {
                tracing::info!("Geoviewer cache hit: {}", cache_key);
                let key = cache_key.clone();
                let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(4);
                tokio::spawn(async move {
                    let _ = tx
                        .send(Ok(Event::default()
                            .event("complete")
                            .data(format!(r#"{{"key":"{}","cached":true}}"#, key))))
                        .await;
                });
                return Ok(Sse::new(ReceiverStream::new(rx)).keep_alive(KeepAlive::default()));
            }
        }
    }

    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(32);

    tokio::spawn(async move {
        let cache_html = is_default_location;
        let start_time = std::time::Instant::now();
        let tile_count = tiles.len();
        let mut all_surfaces = Vec::new();

        for (i, (e, n)) in tiles.iter().enumerate() {
            let tile_name = format!("{}_{}", e, n);

            match fetch_tile(*e, *n).await {
                Ok((gml, cache_hit)) => {
                    let bytes_mb = gml.len() as f64 / 1_000_000.0;

                    let _ = tx
                        .send(Ok(Event::default().event("progress").data(format!(
                            r#"{{"step":"fetching","current":{},"total":{},"tile":"{}","size_mb":{:.1},"cache_hit":{}}}"#,
                            i + 1,
                            tile_count,
                            tile_name,
                            bytes_mb,
                            cache_hit
                        ))))
                        .await;

                    // Log tile fetch event
                    if let Some(ref sid) = session_id {
                        log_geo_event(
                            &db,
                            sid,
                            "geo.tile.fetch",
                            &serde_json::json!({
                                "tile": tile_name,
                                "cache_hit": cache_hit,
                                "size_mb": format!("{:.1}", bytes_mb),
                            }),
                        )
                        .await;
                    }

                    let _ = tx
                        .send(Ok(Event::default().event("progress").data(format!(
                            r#"{{"step":"parsing","current":{},"total":{},"tile":"{}","size_mb":{:.1}}}"#,
                            i + 1,
                            tile_count,
                            tile_name,
                            bytes_mb
                        ))))
                        .await;

                    let surfaces = parse_citygml(&gml);
                    tracing::info!(
                        "Tile {}: {:.1} MB, {} surfaces",
                        tile_name,
                        bytes_mb,
                        surfaces.len()
                    );
                    all_surfaces.extend(surfaces);
                }
                Err(StatusCode::NOT_FOUND) => {
                    tracing::info!("Tile {} not found, skipping", tile_name);
                }
                Err(_) => {
                    let _ = tx
                        .send(Ok(Event::default().event("error").data(format!(
                            r#"{{"message":"Fehler beim Laden der Kachel {}"}}"#,
                            tile_name
                        ))))
                        .await;
                    return;
                }
            }
        }

        if all_surfaces.is_empty() {
            let _ = tx
                .send(Ok(Event::default().event("error").data(
                    r#"{"message":"Keine Gebäudedaten für diesen Bereich gefunden"}"#,
                )))
                .await;
            return;
        }

        // Apply dithering and determine queried buildings
        let (query_easting, query_northing) = wgs84_to_utm32n(query_lat, query_lng);
        let (filtered_surfaces, queried_gmlids) =
            apply_dithering_and_queried(all_surfaces, query_easting, query_northing, query_address.as_deref());

        let _ = tx
            .send(Ok(Event::default().event("progress").data(format!(
                r#"{{"step":"building","surfaces":{}}}"#,
                filtered_surfaces.len()
            ))))
            .await;

        match build_geoparquet(&filtered_surfaces, &queried_gmlids) {
            Ok(parquet_bytes) => {
                let duration_ms = start_time.elapsed().as_millis() as u64;
                let size_mb = parquet_bytes.len() as f64 / 1_000_000.0;
                tracing::info!(
                    "GeoParquet: {:.1} MB, {} surfaces, key={}",
                    size_mb,
                    filtered_surfaces.len(),
                    cache_key
                );

                let html = render_geoviewer_html(
                    &parquet_bytes,
                    filtered_surfaces.len(),
                    duration_ms,
                    &cache_key,
                );

                // Store HTML in memory; default location stays cached permanently,
                // other locations are removed after first serve.
                if cache_html {
                    tracing::info!(
                        "Geoviewer HTML cached in memory (default): {} ({:.1} MB)",
                        cache_key,
                        html.len() as f64 / 1_000_000.0
                    );
                }
                html_cache().lock().unwrap().insert(cache_key.clone(), html);

                let _ = tx
                    .send(Ok(Event::default().event("complete").data(format!(
                        r#"{{"key":"{}","surfaces":{},"size_mb":{:.1}}}"#,
                        cache_key,
                        filtered_surfaces.len(),
                        size_mb
                    ))))
                    .await;
            }
            Err(msg) => {
                tracing::error!("GeoParquet build error: {}", msg);
                let _ = tx
                    .send(Ok(Event::default().event("error").data(format!(
                        r#"{{"message":"Fehler: {}"}}"#,
                        msg
                    ))))
                    .await;
            }
        }
    });

    Ok(Sse::new(ReceiverStream::new(rx)).keep_alive(KeepAlive::default()))
}

/// GET /api/geo/buildings/{key} — serve geoviewer HTML from memory.
/// Non-default-location entries are removed after serving (single use).
pub async fn get_cached_buildings(
    axum::extract::Path(key): axum::extract::Path<String>,
) -> Result<Response, StatusCode> {
    let default_suffix = format!("-{:.4}_{:.4}", DEFAULT_LAT, DEFAULT_LNG);
    let is_default = key.ends_with(&default_suffix);

    let html = {
        let mut cache = html_cache().lock().unwrap();
        if is_default {
            cache.get(&key).cloned()
        } else {
            cache.remove(&key)
        }
    }
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "no-store"),
        ],
        html,
    )
        .into_response())
}

/// Background task: evict least-used GML tiles when cache approaches disk limit.
pub async fn gml_cache_eviction(db: SqlitePool, max_bytes: u64) {
    let dir = std::path::Path::new(GML_CACHE_DIR);
    let threshold = (max_bytes as f64 * 0.9) as u64;
    let target = (max_bytes as f64 * 0.7) as u64;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(10 * 60)).await;

        // Scan cache directory
        let Ok(mut entries) = tokio::fs::read_dir(dir).await else {
            continue;
        };

        let mut cached_tiles: Vec<(String, u64, std::time::SystemTime)> = Vec::new();
        let mut total_bytes: u64 = 0;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("gml") {
                continue;
            }
            let Ok(metadata) = entry.metadata().await else {
                continue;
            };
            let size = metadata.len();
            let modified = metadata.modified().unwrap_or(std::time::UNIX_EPOCH);
            let tile_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if !tile_name.is_empty() {
                total_bytes += size;
                cached_tiles.push((tile_name, size, modified));
            }
        }

        if total_bytes < threshold {
            continue;
        }

        tracing::info!(
            "GML cache at {:.0} MB / {:.0} MB ({:.0}%), starting eviction",
            total_bytes as f64 / 1_000_000.0,
            max_bytes as f64 / 1_000_000.0,
            total_bytes as f64 / max_bytes as f64 * 100.0,
        );

        // Query analytics for tile usage counts
        let usage: HashMap<String, i64> = sqlx::query_as::<_, (String, i64)>(
            "SELECT json_extract(properties, '$.tile') as tile, COUNT(*) as cnt \
             FROM events WHERE event_name = 'geo.tile.fetch' \
             GROUP BY tile",
        )
        .fetch_all(&db)
        .await
        .unwrap_or_default()
        .into_iter()
        .collect();

        // Sort: lowest usage first, then oldest modified first
        cached_tiles.sort_by(|a, b| {
            let usage_a = usage.get(&a.0).copied().unwrap_or(0);
            let usage_b = usage.get(&b.0).copied().unwrap_or(0);
            usage_a.cmp(&usage_b).then_with(|| a.2.cmp(&b.2))
        });

        let mut deleted = 0u32;
        for (tile_name, size, _) in &cached_tiles {
            if total_bytes < target {
                break;
            }
            let path = dir.join(format!("{}.gml", tile_name));
            if tokio::fs::remove_file(&path).await.is_ok() {
                total_bytes -= size;
                deleted += 1;
                tracing::info!(
                    "Evicted GML tile: {} ({:.1} MB, {} fetches)",
                    tile_name,
                    *size as f64 / 1_000_000.0,
                    usage.get(tile_name).copied().unwrap_or(0),
                );
            }
        }

        if deleted > 0 {
            tracing::info!(
                "GML cache eviction complete: deleted {} tile(s), now {:.0} MB",
                deleted,
                total_bytes as f64 / 1_000_000.0,
            );
        }
    }
}
