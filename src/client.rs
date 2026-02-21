use std::process::exit;
use std::{collections::HashMap, error::Error};

use anyhow::Result;
use reqwest::{
    StatusCode,
    blocking::{Client as HttpClient, RequestBuilder, Response},
};
use serde::{Deserialize, Serialize};

use crate::{constants, utils};

pub struct GmailClient {
    pub client: HttpClient,
    credentials: Credentials,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub oauth: CredentialsOAuth,
    pub token: CredentialsToken,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialsOAuth {
    pub client_id: String,
    pub project_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialsToken {
    pub refresh_token: Option<String>,
    pub access_token: String,
    pub expires_in: Option<u64>,
    pub scope: String,
    pub token_type: String,
    pub refresh_token_expires_in: Option<u64>,
}

impl GmailClient {
    pub fn new() -> Self {
        Self {
            client: HttpClient::new(),
            credentials: match utils::read_json(&constants::CREDENTIALS_PATH.display().to_string())
            {
                Ok(credentials) => credentials,
                Err(error) => {
                    eprintln!("could not get credentials: {}", error);
                    exit(1);
                }
            },
        }
    }

    pub fn refresh_access_token(&mut self) -> Result<(), Box<dyn Error>> {
        let refresh_token = self
            .credentials
            .token
            .refresh_token
            .as_ref()
            .expect("no refresh token in credentials")
            .to_string();

        let mut form: HashMap<&str, String> = HashMap::new();
        form.insert("client_id", self.credentials.oauth.client_id.to_string());
        form.insert(
            "client_secret",
            self.credentials.oauth.client_secret.to_string(),
        );
        form.insert("refresh_token", refresh_token);
        form.insert("grant_type", "refresh_token".to_string());

        let response: CredentialsToken = self
            .client
            .post("https://oauth2.googleapis.com/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form)
            .send()?
            .error_for_status()?
            .json()?;

        self.credentials.token.access_token = response.access_token;

        utils::write_struct_to_file(
            &self.credentials,
            &constants::GMAIL_CREDENTIALS.display().to_string(),
        )?;

        Ok(())
    }

    pub fn send<F>(&mut self, build: F) -> Result<Response, Box<dyn Error>>
    where
        F: Fn(&HttpClient) -> RequestBuilder,
    {
        let response = build(&self.client)
            .bearer_auth(&self.credentials.token.access_token)
            .send()?;

        if response.status() == StatusCode::UNAUTHORIZED {
            self.refresh_access_token()?;

            let retry = build(&self.client)
                .bearer_auth(&self.credentials.token.access_token)
                .send()?;

            println!("[REQUEST] {} {}", retry.url().as_str(), retry.status());
            return Ok(retry);
        }

        println!(
            "[REQUEST] {} {}",
            response.url().as_str(),
            response.status()
        );
        Ok(response)
    }
}
