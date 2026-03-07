use sqlx::SqlitePool;
use std::sync::Arc;

use crate::{
    cache::SignatureCache,
    config::Config,
    services::{CaptchaService, EmailService, GeoIpService},
};

pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub signature_cache: SignatureCache,
    pub email_service: EmailService,
    pub captcha_service: CaptchaService,
    pub geoip_service: GeoIpService,
}

impl AppState {
    pub async fn new(config: Config, db: SqlitePool) -> anyhow::Result<Arc<Self>> {
        // Create email service
        let email_service = EmailService::new(
            &config.smtp_host,
            config.smtp_port,
            &config.smtp_user,
            &config.smtp_pass,
            &config.email_from,
            &config.email_to,
        )?;

        // Create captcha service
        let captcha_service = CaptchaService::new(
            config.friendly_captcha_secret.clone(),
            config.friendly_captcha_sitekey.clone(),
        );

        // Create GeoIP service
        let geoip_service = GeoIpService::new(config.geoip_db_path.as_deref());

        // Create and load signature cache
        let signature_cache = SignatureCache::new();
        signature_cache.load(&db).await?;

        Ok(Arc::new(Self {
            db,
            config,
            signature_cache,
            email_service,
            captcha_service,
            geoip_service,
        }))
    }
}
