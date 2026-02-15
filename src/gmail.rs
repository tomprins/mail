use crate::client;
use reqwest::blocking::Client;
use serde;
use serde::Deserialize;
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MessagesList {
    pub messages: Vec<Message>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: String,
    #[serde(rename = "resultSizeEstimate")]
    pub result_size_estimate: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "threadId")]
    pub thread_id: String,
}

pub fn messages_list(client: &mut client::GmailClient) -> Result<MessagesList, Box<dyn Error>> {
    let messages_list: MessagesList = client
        .send(|client: &Client| {
            client.get("https://gmail.googleapis.com/gmail/v1/users/me/messages")
        })?
        .json()?;

    Ok(messages_list)
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
