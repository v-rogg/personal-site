use crate::error::AppError;
use serde::Deserialize;

pub struct CaptchaService {
    client: reqwest::Client,
    secret: String,
    sitekey: String,
}

#[derive(Deserialize)]
struct FriendlyCaptchaResponse {
    success: bool,
    #[serde(default)]
    errors: Vec<String>,
}

impl CaptchaService {
    pub fn new(secret: String, sitekey: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            secret,
            sitekey,
        }
    }

    pub async fn verify(&self, solution: &str) -> Result<(), AppError> {
        let response: FriendlyCaptchaResponse = self
            .client
            .post("https://api.friendlycaptcha.com/api/v1/siteverify")
            .json(&serde_json::json!({
                "solution": solution,
                "secret": self.secret,
                "sitekey": self.sitekey,
            }))
            .send()
            .await?
            .json()
            .await?;

        if response.success {
            Ok(())
        } else {
            tracing::warn!(
                "Friendly Captcha verification failed: {:?}",
                response.errors
            );
            Err(AppError::CaptchaFailed)
        }
    }
}
