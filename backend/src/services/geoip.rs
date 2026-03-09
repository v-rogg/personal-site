use maxminddb::{geoip2, Reader};
use std::net::IpAddr;
use std::path::Path;
use std::sync::Arc;

pub struct GeoIpService {
    reader: Option<Arc<Reader<Vec<u8>>>>,
}

#[derive(Debug, Clone, Default)]
pub struct GeoLocation {
    pub country_code: Option<String>,
    pub region: Option<String>,
}

impl GeoIpService {
    pub fn new(db_path: Option<&str>) -> Self {
        let reader = db_path.and_then(|path| {
            if Path::new(path).exists() {
                match Reader::open_readfile(path) {
                    Ok(r) => {
                        tracing::info!("Loaded GeoIP database from {}", path);
                        Some(Arc::new(r))
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load GeoIP database: {}", e);
                        None
                    }
                }
            } else {
                tracing::info!(
                    "GeoIP database not found at {}, geo tracking disabled",
                    path
                );
                None
            }
        });

        Self { reader }
    }

    /// Look up geographic location for an IP address
    pub fn lookup(&self, ip_str: &str) -> GeoLocation {
        let Some(reader) = &self.reader else {
            return GeoLocation::default();
        };

        // Parse IP address, handling IPv4-mapped IPv6 addresses
        let ip: IpAddr = match ip_str.parse() {
            Ok(ip) => ip,
            Err(_) => return GeoLocation::default(),
        };

        // Look up in MaxMind database
        let city: geoip2::City = match reader.lookup(ip) {
            Ok(c) => c,
            Err(_) => return GeoLocation::default(),
        };

        let country_code = city.country.and_then(|c| c.iso_code).map(|s| s.to_string());

        let region = city
            .subdivisions
            .and_then(|s| s.first().cloned())
            .and_then(|s| s.names)
            .and_then(|n| n.get("en").cloned())
            .map(|s| s.to_string());

        GeoLocation {
            country_code,
            region,
        }
    }

    /// Extract client IP from request headers (X-Forwarded-For or X-Real-IP)
    pub fn extract_client_ip(headers: &axum::http::HeaderMap) -> Option<String> {
        // Try X-Forwarded-For first (may contain multiple IPs, take the first)
        if let Some(forwarded) = headers.get("x-forwarded-for") {
            if let Ok(value) = forwarded.to_str() {
                // X-Forwarded-For may contain: "client, proxy1, proxy2"
                if let Some(client_ip) = value.split(',').next() {
                    let ip = client_ip.trim();
                    if !ip.is_empty() {
                        return Some(ip.to_string());
                    }
                }
            }
        }

        // Fallback to X-Real-IP
        if let Some(real_ip) = headers.get("x-real-ip") {
            if let Ok(value) = real_ip.to_str() {
                let ip = value.trim();
                if !ip.is_empty() {
                    return Some(ip.to_string());
                }
            }
        }

        None
    }

}
