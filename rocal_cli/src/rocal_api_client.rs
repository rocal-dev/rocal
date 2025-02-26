use std::time::Duration;

use create_payment_link::CreatePaymentLink;
use create_user::CreateUser;
use login_user::LoginUser;
use oob_code_response::OobCodeResponse;
use payment_link::PaymentLink;
use reqwest::Client;
use send_email_verification::SendEmailVerification;
use send_password_reset_email::SendPasswordResetEmail;
use serde::{de::DeserializeOwned, Serialize};
use user_login_token::UserLoginToken;
use user_refresh_token::UserRefreshToken;

use crate::{
    commands::utils::{
        color::Color,
        indicator::{IndicatorLauncher, Kind},
    },
    response::ResponseWithMessage,
    token_manager::{self, TokenManager},
};

pub mod create_payment_link;
pub mod create_user;
pub mod login_user;
mod oob_code_response;
mod payment_link;
pub mod send_email_verification;
pub mod send_password_reset_email;
mod user_login_token;
pub mod user_refresh_token;

pub struct RocalAPIClient {
    client: Client,
    endpoint: String,
}

impl RocalAPIClient {
    pub fn new() -> Self {
        let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        let endpoint = if env!("BUILD_PROFILE") == "debug" {
            "http://127.0.0.1:8000"
        } else {
            "https://rocal.dev/api"
        };

        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build a http client");

        Self {
            client,
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn sign_up(&self, user: CreateUser) {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Spinner)
            .interval(100)
            .text("Signing up...")
            .color(Color::Blue)
            .start();

        match self
            .post::<CreateUser, UserLoginToken>(
                &format!("{}/v1/users/sign-up", self.endpoint),
                user,
            )
            .await
        {
            Ok(data) => {
                let _ = indicator.stop();
                if let Err(err) = TokenManager::set_token(
                    token_manager::Kind::RocalAccessToken,
                    data.get_id_token(),
                ) {
                    eprintln!("{}", err.to_string());
                }

                if let Err(err) = TokenManager::set_token(
                    token_manager::Kind::RocalRefreshToken,
                    data.get_refresh_token(),
                ) {
                    eprintln!("{}", err.to_string());
                }
            }
            Err(err) => {
                let _ = indicator.stop();
                eprintln!("{}", err);
            }
        }
    }

    pub async fn sign_in(&self, user: LoginUser) -> Result<(), String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Spinner)
            .interval(100)
            .text("Signing in...")
            .color(Color::Blue)
            .start();

        match self
            .post::<LoginUser, UserLoginToken>(&format!("{}/v1/users/sign-in", self.endpoint), user)
            .await
        {
            Ok(data) => {
                let _ = indicator.stop();
                if let Err(err) = TokenManager::set_token(
                    token_manager::Kind::RocalAccessToken,
                    data.get_id_token(),
                ) {
                    return Err(format!("{}", err.to_string()));
                }

                if let Err(err) = TokenManager::set_token(
                    token_manager::Kind::RocalRefreshToken,
                    data.get_refresh_token(),
                ) {
                    return Err(format!("{}", err.to_string()));
                }

                Ok(())
            }
            Err(err) => {
                let _ = indicator.stop();
                Err(format!("{}", err))
            }
        }
    }

    pub async fn refresh_user_login_token(&self, refresh_token: &str) -> Result<(), String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Spinner)
            .interval(100)
            .text("Refreshing your access token...")
            .color(Color::Blue)
            .start();

        match self
            .post::<UserRefreshToken, UserLoginToken>(
                &format!("{}/v1/users/refresh-token", self.endpoint),
                UserRefreshToken::new(refresh_token),
            )
            .await
        {
            Ok(data) => {
                let _ = indicator.stop();
                if let Err(err) = TokenManager::set_token(
                    token_manager::Kind::RocalAccessToken,
                    data.get_id_token(),
                ) {
                    return Err(format!("{}", err.to_string()));
                }

                if let Err(err) = TokenManager::set_token(
                    token_manager::Kind::RocalRefreshToken,
                    data.get_refresh_token(),
                ) {
                    return Err(format!("{}", err.to_string()));
                }

                Ok(())
            }
            Err(err) => {
                let _ = indicator.stop();
                Err(format!("{}", err))
            }
        }
    }

    pub async fn send_email_verification(&self, req: SendEmailVerification) {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Spinner)
            .interval(100)
            .text("Sending an email verification...")
            .color(Color::Blue)
            .start();

        match self
            .post::<SendEmailVerification, OobCodeResponse>(
                &format!("{}/v1/users/send-email-verification", self.endpoint),
                req,
            )
            .await
        {
            Ok(res) => {
                let _ = indicator.stop();

                println!(
                    "{}",
                    Color::Green.text(&format!(
                        "Sent an email verification to {}",
                        res.get_email()
                    ))
                );
            }
            Err(err) => {
                let _ = indicator.stop();

                eprintln!("{}", err);
            }
        }
    }

    pub async fn send_password_reset_email(
        &self,
        req: SendPasswordResetEmail,
    ) -> Result<(), String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Spinner)
            .interval(100)
            .text("Sending a password reset email...")
            .color(Color::Blue)
            .start();

        match self
            .post::<SendPasswordResetEmail, OobCodeResponse>(
                &format!("{}/v1/users/send-password-reset-email", self.endpoint),
                req,
            )
            .await
        {
            Ok(res) => {
                let _ = indicator.stop();

                println!(
                    "{}",
                    Color::Green.text(&format!(
                        "Sent a password reset email to {}",
                        res.get_email()
                    ))
                );
                Ok(())
            }
            Err(err) => {
                let _ = indicator.stop();

                Err(format!("{}", err))
            }
        }
    }

    pub async fn create_payment_link(
        &self,
        create_payment_link: CreatePaymentLink,
    ) -> Result<String, String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Spinner)
            .interval(100)
            .text("Issuing a payment link for your plan...")
            .color(Color::Blue)
            .start();

        match TokenManager::get_token(token_manager::Kind::RocalAccessToken) {
            Ok(token) => {
                match self
                    .authorized_post::<CreatePaymentLink, PaymentLink>(
                        &token,
                        &format!("{}/v1/subscriptions", self.endpoint),
                        create_payment_link,
                    )
                    .await
                {
                    Ok(res) => {
                        let _ = indicator.stop();
                        Ok(res.get_url().to_string())
                    }
                    Err(err) => {
                        let _ = indicator.stop();
                        Err(err)
                    }
                }
            }
            Err(err) => {
                let _ = indicator.stop();
                Err(format!("{}", err.to_string()))
            }
        }
    }

    async fn post<T, U>(&self, path: &str, data: T) -> Result<U, String>
    where
        T: Serialize,
        U: DeserializeOwned + Clone,
    {
        match self.client.post(path).json(&data).send().await {
            Ok(res) => match res.json::<ResponseWithMessage<U>>().await {
                Ok(res) => {
                    if let Some(data) = res.get_data() {
                        Ok(data.clone())
                    } else {
                        Err(res.get_message().to_string())
                    }
                }
                Err(err) => Err(format!("{}", err.to_string())),
            },
            Err(err) => Err(format!("{}", err.to_string())),
        }
    }

    async fn authorized_post<T, U>(
        &self,
        access_token: &str,
        path: &str,
        data: T,
    ) -> Result<U, String>
    where
        T: Serialize,
        U: DeserializeOwned + Clone,
    {
        match self
            .client
            .post(path)
            .bearer_auth(access_token)
            .json(&data)
            .send()
            .await
        {
            Ok(res) => match res.json::<ResponseWithMessage<U>>().await {
                Ok(res) => {
                    if let Some(data) = res.get_data() {
                        Ok(data.clone())
                    } else {
                        Err(res.get_message().to_string())
                    }
                }
                Err(err) => Err(format!("{}", err.to_string())),
            },
            Err(err) => Err(format!("{}", err.to_string())),
        }
    }
}
