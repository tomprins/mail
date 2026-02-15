// use reqwest::Error;
use crate::utils;
use anyhow::Result;
use reqwest::blocking::Client;
use reqwest::blocking::get;
use reqwest::header::{HeaderMap, HeaderValue};
use serde;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct MessagesList {
    pub messages: Vec<Message>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: String,
    #[serde(rename = "resultSizeEstimate")]
    pub result_size_estimate: u64,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "threadId")]
    pub thread_id: String,
}

// TODO try defining struct in struct
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

// pub fn messages_list(client: &Client, access_token: &str) -> Result<MessagesList, Box<dyn Error>> {
pub fn messages_list(client: &Client, access_token: &str) -> Result<(), Box<dyn Error>> {
    // TODO use bearer auth for this
    let bearer = &format!("Bearer {}", access_token);
    let authorization_header = HeaderValue::from_str(bearer)?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", authorization_header);

    let response: MessagesList = client
        .get("https://gmail.googleapis.com/gmail/v1/users/me/messages")
        .headers(headers)
        .send()?
        .error_for_status()?
        .json()?;

    println!("{:?}", response);

    Ok(())
}

// pub fn message(
//     client: &Client,
//     access_token: &str,
//     message_id: &str,
// ) -> Result<(), Box<dyn Error>> {
//     let bearer = &format!("Bearer {}", access_token);
//     let authorization_header = HeaderValue::from_str(bearer)?;

//     let mut headers = HeaderMap::new();
//     headers.insert("Authorization", authorization_header);

//     let url = format!(
//         "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
//         message_id
//     );

//     let response = client.get(url).headers(headers).send()?;

//     println!("{:?}", response.text());

//     Ok(())
// }

pub fn refresh_access_token(
    client: &Client,
    mut credentials: Credentials,
) -> Result<(), Box<dyn Error>> {
    let refresh_token = credentials
        .token
        .refresh_token
        .as_ref()
        .expect("no refresh token in credentials")
        .to_string();

    let mut form: HashMap<&str, String> = HashMap::new();
    // TODO is this the best way?
    form.insert("client_id", credentials.oauth.client_id.to_string());
    form.insert("client_secret", credentials.oauth.client_secret.to_string());
    form.insert("refresh_token", refresh_token);
    form.insert("grant_type", "refresh_token".to_string());

    let response: CredentialsToken = client
        .post("https://oauth2.googleapis.com/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form)
        .send()?
        .error_for_status()?
        .json()?;

    println!("{:?}", response);

    credentials.token.access_token = response.access_token;
    utils::write_struct_to_file(
        &credentials,
        "C:\\Users\\tompr\\mail\\credentials\\gmail.json",
    )?;

    Ok(())
}

pub fn credentials() -> Result<Credentials, Box<dyn Error>> {
    let file = File::open("C:\\Users\\tompr\\mail\\credentials\\gmail.json")?;
    let reader = BufReader::new(file);

    let credentials: Credentials = serde_json::from_reader(reader)?;
    Ok(credentials)
}
