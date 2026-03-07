use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};

pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    to: Mailbox,
}

impl EmailService {
    pub fn new(
        smtp_host: &str,
        smtp_port: u16,
        smtp_user: &str,
        smtp_pass: &str,
        from: &str,
        to: &str,
    ) -> anyhow::Result<Self> {
        let creds = Credentials::new(smtp_user.to_string(), smtp_pass.to_string());

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_host)?
            .port(smtp_port)
            .credentials(creds)
            .build();

        Ok(Self {
            mailer,
            from: from.parse()?,
            to: to.parse()?,
        })
    }

    pub async fn send_signature_confirmation(
        &self,
        name: &str,
        id: &str,
        url: &str,
        reply_to: Option<&str>,
    ) -> anyhow::Result<()> {
        let subject = format!("Neue Signatur: {}", name);
        let body = format!(
            r#"<html>
            <body>
                <p>Neue Signatur von <strong>{}</strong></p>
                <p>ID: {}</p>
                <p>Link: <a href="{}">{}</a></p>
            </body>
            </html>"#,
            name, id, url, url
        );

        let mut builder = Message::builder()
            .from(self.from.clone())
            .to(self.to.clone())
            .subject(subject)
            .header(ContentType::TEXT_HTML);

        if let Some(rt) = reply_to {
            if let Ok(mailbox) = rt.parse() {
                builder = builder.reply_to(mailbox);
            }
        }

        let email = builder.body(body)?;
        self.mailer.send(email).await?;

        Ok(())
    }

    pub async fn send_contact_request(&self, from_email: &str, message: &str) -> anyhow::Result<()> {
        let body = format!(
            r#"<html>
            <body>
                <p>Von: <strong>{}</strong></p>
                <p>{}</p>
            </body>
            </html>"#,
            from_email,
            message.replace('\n', "<br>")
        );

        let email = Message::builder()
            .from(self.from.clone())
            .to(self.to.clone())
            .reply_to(from_email.parse()?)
            .subject("Neue Arbeitsanfrage")
            .header(ContentType::TEXT_HTML)
            .body(body)?;

        self.mailer.send(email).await?;

        Ok(())
    }
}
