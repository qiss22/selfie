use handlebars::Handlebars;
use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::Serialize;
use std::sync::Arc;

use crate::error::AuthError;

const EMAIL_VERIFICATION_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Verify Your Email - Selfie</title>
</head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
        <h1 style="color: #2c3e50;">Welcome to Selfie!</h1>
        <p>Hello,</p>
        <p>Thank you for signing up. Please verify your email address by clicking the button below:</p>
        <p style="text-align: center;">
            <a href="{{verification_link}}" 
               style="background-color: #3498db; color: white; padding: 12px 24px; 
                      text-decoration: none; border-radius: 4px; display: inline-block;">
                Verify Email Address
            </a>
        </p>
        <p>Or copy and paste this link into your browser:</p>
        <p>{{verification_link}}</p>
        <p>This link will expire in 24 hours.</p>
        <p>If you didn't create an account, you can safely ignore this email.</p>
        <hr style="border: none; border-top: 1px solid #eee; margin: 20px 0;">
        <p style="font-size: 12px; color: #666;">
            This is an automated message, please do not reply.
        </p>
    </div>
</body>
</html>"#;

const PASSWORD_RESET_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Reset Your Password - Selfie</title>
</head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333;">
    <div style="max-width: 600px; margin: 0 auto; padding: 20px;">
        <h1 style="color: #2c3e50;">Password Reset Request</h1>
        <p>Hello,</p>
        <p>We received a request to reset your password. Click the button below to create a new password:</p>
        <p style="text-align: center;">
            <a href="{{reset_link}}" 
               style="background-color: #3498db; color: white; padding: 12px 24px; 
                      text-decoration: none; border-radius: 4px; display: inline-block;">
                Reset Password
            </a>
        </p>
        <p>Or copy and paste this link into your browser:</p>
        <p>{{reset_link}}</p>
        <p>This link will expire in 1 hour.</p>
        <p>If you didn't request a password reset, you can safely ignore this email.</p>
        <hr style="border: none; border-top: 1px solid #eee; margin: 20px 0;">
        <p style="font-size: 12px; color: #666;">
            This is an automated message, please do not reply.
        </p>
    </div>
</body>
</html>"#;

#[derive(Clone)]
pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    templates: Arc<Handlebars<'static>>,
    from_address: String,
    app_url: String,
}

impl EmailService {
    pub async fn new(
        smtp_host: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
        from_address: String,
        app_url: String,
    ) -> Result<Self, AuthError> {
        let creds = Credentials::new(smtp_username, smtp_password);
        let tls_params = TlsParameters::new(smtp_host.clone())
            .map_err(|_| AuthError::InternalError)?;

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)
            .map_err(|_| AuthError::InternalError)?
            .port(smtp_port)
            .tls(Tls::Required(tls_params))
            .credentials(creds)
            .pool_config(lettre::transport::smtp::PoolConfig::new().max_size(20))
            .build();

        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("verification", EMAIL_VERIFICATION_TEMPLATE)
            .map_err(|_| AuthError::InternalError)?;
        handlebars
            .register_template_string("reset", PASSWORD_RESET_TEMPLATE)
            .map_err(|_| AuthError::InternalError)?;

        Ok(Self {
            mailer,
            templates: Arc::new(handlebars),
            from_address,
            app_url,
        })
    }

    pub async fn send_verification_email(
        &self,
        to_email: &str,
        token: &str,
    ) -> Result<(), AuthError> {
        #[derive(Serialize)]
        struct TemplateData {
            verification_link: String,
        }

        let verification_link = format!("{}/verify-email?token={}", self.app_url, token);
        let data = TemplateData {
            verification_link,
        };

        let html = self
            .templates
            .render("verification", &data)
            .map_err(|_| AuthError::InternalError)?;

        let email = Message::builder()
            .from(self.from_address.parse().map_err(|_| AuthError::InternalError)?)
            .to(to_email.parse().map_err(|_| AuthError::InternalError)?)
            .subject("Verify Your Email - Selfie")
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(format!(
                                "Please verify your email by visiting: {}",
                                verification_link
                            )),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html),
                    ),
            )
            .map_err(|_| AuthError::InternalError)?;

        self.mailer
            .send(email)
            .await
            .map_err(|_| AuthError::InternalError)?;

        Ok(())
    }

    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        token: &str,
    ) -> Result<(), AuthError> {
        #[derive(Serialize)]
        struct TemplateData {
            reset_link: String,
        }

        let reset_link = format!("{}/reset-password?token={}", self.app_url, token);
        let data = TemplateData { reset_link };

        let html = self
            .templates
            .render("reset", &data)
            .map_err(|_| AuthError::InternalError)?;

        let email = Message::builder()
            .from(self.from_address.parse().map_err(|_| AuthError::InternalError)?)
            .to(to_email.parse().map_err(|_| AuthError::InternalError)?)
            .subject("Reset Your Password - Selfie")
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(format!("Reset your password by visiting: {}", reset_link)),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html),
                    ),
            )
            .map_err(|_| AuthError::InternalError)?;

        self.mailer
            .send(email)
            .await
            .map_err(|_| AuthError::InternalError)?;

        Ok(())
    }
}