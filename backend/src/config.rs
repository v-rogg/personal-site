use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,

    // SMTP settings
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_user: String,
    pub smtp_pass: String,

    // Friendly Captcha (German provider)
    pub friendly_captcha_secret: String,
    pub friendly_captcha_sitekey: String,

    // Email settings
    pub email_from: String,
    pub email_to: String,

    // App settings
    pub base_url: String,

    // GeoIP settings
    pub geoip_db_path: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "/data/app.db".to_string()),
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            smtp_host: env::var("SMTP_HOST").unwrap_or_else(|_| "smtp.resend.com".to_string()),
            smtp_port: env::var("SMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(465),
            smtp_user: env::var("SMTP_USER").unwrap_or_else(|_| "resend".to_string()),
            smtp_pass: env::var("SMTP_PASS").expect("SMTP_PASS required"),
            friendly_captcha_secret: env::var("FRIENDLY_CAPTCHA_SECRET")
                .expect("FRIENDLY_CAPTCHA_SECRET required"),
            friendly_captcha_sitekey: env::var("FRIENDLY_CAPTCHA_SITEKEY")
                .expect("FRIENDLY_CAPTCHA_SITEKEY required"),
            email_from: env::var("EMAIL_FROM")
                .unwrap_or_else(|_| "noreply@valentinrogg.de".to_string()),
            email_to: env::var("EMAIL_TO").unwrap_or_else(|_| "mail@valentinrogg.de".to_string()),
            base_url: env::var("BASE_URL")
                .unwrap_or_else(|_| "https://valentinrogg.de".to_string()),
            geoip_db_path: env::var("GEOIP_DB_PATH").ok(),
        }
    }
}
