use std::{fs, time::Duration};

use cancel_subscription::CancelSubscription;
use create_app::CreateApp;
use create_payment_link::CreatePaymentLink;
use create_user::CreateUser;
use login_user::LoginUser;
use oob_code_response::OobCodeResponse;
use payment_link::PaymentLink;
use reqwest::{Client, RequestBuilder};
use send_email_verification::SendEmailVerification;
use send_password_reset_email::SendPasswordResetEmail;
use serde::{de::DeserializeOwned, Serialize};
use subdomain::Subdomain;
use subscription_status::SubscriptionStatus;
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

pub mod cancel_subscription;
pub mod create_app;
pub mod create_payment_link;
pub mod create_user;
pub mod login_user;
mod oob_code_response;
mod payment_link;
pub mod send_email_verification;
pub mod send_password_reset_email;
pub mod subdomain;
pub mod subscription_status;
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
            "https://www.rocal.dev"
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
            .kind(Kind::Dots)
            .interval(100)
            .text("Signing up...")
            .color(Color::White)
            .start();

        match self
            .req::<CreateUser, UserLoginToken>(
                RequestMethod::Post,
                &format!("{}/v1/users/sign-up", self.endpoint),
                Some(user),
                None,
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
            .kind(Kind::Dots)
            .interval(100)
            .text("Signing in...")
            .color(Color::White)
            .start();

        match self
            .req::<LoginUser, UserLoginToken>(
                RequestMethod::Post,
                &format!("{}/v1/users/sign-in", self.endpoint),
                Some(user),
                None,
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

    pub async fn refresh_user_login_token(&self, refresh_token: &str) -> Result<(), String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Refreshing your access token...")
            .color(Color::White)
            .start();

        match self
            .req::<UserRefreshToken, UserLoginToken>(
                RequestMethod::Post,
                &format!("{}/v1/users/refresh-token", self.endpoint),
                Some(UserRefreshToken::new(refresh_token)),
                None,
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
            .kind(Kind::Dots)
            .interval(100)
            .text("Sending an email verification...")
            .color(Color::White)
            .start();

        match self
            .req::<SendEmailVerification, OobCodeResponse>(
                RequestMethod::Post,
                &format!("{}/v1/users/send-email-verification", self.endpoint),
                Some(req),
                None,
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
            .kind(Kind::Dots)
            .interval(100)
            .text("Sending a password reset email...")
            .color(Color::White)
            .start();

        match self
            .req::<SendPasswordResetEmail, OobCodeResponse>(
                RequestMethod::Post,
                &format!("{}/v1/users/send-password-reset-email", self.endpoint),
                Some(req),
                None,
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
            .kind(Kind::Dots)
            .interval(100)
            .text("Issuing a payment link for your plan...")
            .color(Color::White)
            .start();

        match TokenManager::get_token(token_manager::Kind::RocalAccessToken) {
            Ok(token) => {
                match self
                    .req::<CreatePaymentLink, PaymentLink>(
                        RequestMethod::Post,
                        &format!("{}/v1/subscriptions", self.endpoint),
                        Some(create_payment_link),
                        Some(&token),
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
                match err {
                    keyring::Error::NoEntry => Err("Please sign in/up first.".to_string()),
                    _ => Err(format!("{}", err.to_string())),
                }
            }
        }
    }

    pub async fn get_subscription_status(&self) -> Result<SubscriptionStatus, String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Checking your subscription status...")
            .color(Color::White)
            .start();

        match TokenManager::get_token(token_manager::Kind::RocalAccessToken) {
            Ok(token) => {
                match self
                    .req::<(), SubscriptionStatus>(
                        RequestMethod::Get,
                        &format!("{}/v1/subscriptions/status", self.endpoint),
                        None,
                        Some(&token),
                    )
                    .await
                {
                    Ok(sub) => {
                        let _ = indicator.stop();
                        Ok(sub)
                    }
                    Err(err) => {
                        let _ = indicator.stop();
                        Err(format!("{}", err.to_string()))
                    }
                }
            }
            Err(err) => {
                let _ = indicator.stop();
                Err(format!("{}", err.to_string()))
            }
        }
    }

    pub async fn unsubscribe(&self, cancel_subscription: CancelSubscription) -> Result<(), String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Processing...")
            .color(Color::White)
            .start();

        let access_token = match TokenManager::get_token(token_manager::Kind::RocalAccessToken) {
            Ok(token) => token,
            Err(err) => {
                let _ = indicator.stop();
                return Err(format!("{}", err.to_string()));
            }
        };

        match self
            .req::<CancelSubscription, String>(
                RequestMethod::Patch,
                &format!("{}/v1/subscriptions/unsubscribe", self.endpoint),
                Some(cancel_subscription),
                Some(&access_token),
            )
            .await
        {
            Ok(_) => {}
            Err(err) => {
                let _ = indicator.stop();
                return Err(err);
            }
        }

        let _ = indicator.stop();

        Ok(())
    }

    pub async fn upload_app(&self, create_app: CreateApp, file_path: &str) -> Result<(), String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Uploading...")
            .color(Color::White)
            .start();

        let access_token = match TokenManager::get_token(token_manager::Kind::RocalAccessToken) {
            Ok(token) => token,
            Err(err) => {
                let _ = indicator.stop();
                return Err(format!("{}", err.to_string()));
            }
        };

        match self
            .req::<CreateApp, String>(
                RequestMethod::Post,
                &format!("{}/v1/apps", self.endpoint),
                Some(create_app),
                Some(&access_token),
            )
            .await
        {
            Ok(presigned_url) => {
                let file_bytes = if let Ok(bytes) = fs::read(file_path) {
                    bytes
                } else {
                    let _ = indicator.stop();
                    return Err("Could not find release.tar.gz to publish".to_string());
                };

                let res = match self
                    .client
                    .put(&presigned_url)
                    .body(file_bytes)
                    .header("Content-Type", "application/x-tar")
                    .send()
                    .await
                {
                    Ok(res) => res,
                    Err(err) => {
                        let _ = indicator.stop();
                        return Err(err.to_string());
                    }
                };

                if !res.status().is_success() {
                    let _ = indicator.stop();
                    return Err(format!("Upload failed with status: {}", &res.status()));
                }

                let _ = indicator.stop();
                Ok(())
            }
            Err(err) => {
                let _ = indicator.stop();
                Err(err)
            }
        }
    }

    pub async fn extract_app(&self, subdomain: &str) -> Result<(), String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Extracting...")
            .color(Color::White)
            .start();

        let access_token = match TokenManager::get_token(token_manager::Kind::RocalAccessToken) {
            Ok(token) => token,
            Err(err) => {
                let _ = indicator.stop();
                return Err(format!("{}", err.to_string()));
            }
        };

        match self
            .req::<(), bool>(
                RequestMethod::Post,
                &format!("{}/public-apps-extraction/{}", self.endpoint, subdomain),
                None,
                Some(&access_token),
            )
            .await
        {
            Ok(_) => {
                let _ = indicator.stop();
                Ok(())
            }
            Err(err) => {
                let _ = indicator.stop();
                Err(err.to_string())
            }
        }
    }

    pub async fn get_subdomain(&self, app_name: &str) -> Result<Option<Subdomain>, String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Processing...")
            .color(Color::White)
            .start();

        let access_token = match TokenManager::get_token(token_manager::Kind::RocalAccessToken) {
            Ok(token) => token,
            Err(err) => {
                let _ = indicator.stop();
                return Err(format!("{}", err.to_string()));
            }
        };

        match self
            .req::<(), Option<Subdomain>>(
                RequestMethod::Get,
                &format!("{}/v1/subdomains/{}", self.endpoint, app_name),
                None,
                Some(&access_token),
            )
            .await
        {
            Ok(subdomain) => {
                let _ = indicator.stop();
                Ok(subdomain)
            }
            Err(err) => {
                let _ = indicator.stop();

                if &err == "not exists" {
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }

    pub async fn check_subdomain_existence(&self, subdomain: &str) -> Result<bool, String> {
        let mut indicator = IndicatorLauncher::new()
            .kind(Kind::Dots)
            .interval(100)
            .text("Checking...")
            .color(Color::White)
            .start();

        let access_token = match TokenManager::get_token(token_manager::Kind::RocalAccessToken) {
            Ok(token) => token,
            Err(err) => {
                let _ = indicator.stop();
                return Err(format!("{}", err.to_string()));
            }
        };

        match self
            .req::<(), bool>(
                RequestMethod::Get,
                &format!("{}/v1/subdomains/exists/{}", self.endpoint, subdomain),
                None,
                Some(&access_token),
            )
            .await
        {
            Ok(exists) => {
                let _ = indicator.stop();
                Ok(exists)
            }
            Err(err) => {
                let _ = indicator.stop();
                Err(err)
            }
        }
    }

    async fn handle_response<T>(&self, req: RequestBuilder) -> Result<T, String>
    where
        T: DeserializeOwned + Clone,
    {
        match req.send().await {
            Ok(res) => match res.json::<ResponseWithMessage<T>>().await {
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

    async fn req<T, U>(
        &self,
        method: RequestMethod,
        path: &str,
        data: Option<T>,
        access_token: Option<&str>,
    ) -> Result<U, String>
    where
        T: Serialize,
        U: DeserializeOwned + Clone,
    {
        let req = match (method, data, access_token) {
            (RequestMethod::Get, None, None) => self.client.get(path),
            (RequestMethod::Get, None, Some(access_token)) => {
                self.client.get(path).bearer_auth(access_token)
            }
            (RequestMethod::Post, Some(data), None) => self.client.post(path).json(&data),
            (RequestMethod::Post, None, Some(access_token)) => {
                self.client.post(path).bearer_auth(access_token)
            }
            (RequestMethod::Post, Some(data), Some(access_token)) => {
                self.client.post(path).bearer_auth(access_token).json(&data)
            }
            (RequestMethod::Patch, Some(data), None) => self.client.patch(path).json(&data),
            (RequestMethod::Patch, Some(data), Some(access_token)) => self
                .client
                .patch(path)
                .bearer_auth(access_token)
                .json(&data),
            _ => return Err("Failed to construct a request".to_string()),
        };

        self.handle_response(req).await
    }
}

enum RequestMethod {
    Get,
    Post,
    Patch,
}
