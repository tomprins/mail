use crate::client;
use reqwest::blocking::Client;
use serde;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::error::Error;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MessagesList {
    pub messages: Vec<MessageListMessage>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: String,
    #[serde(rename = "resultSizeEstimate")]
    pub result_size_estimate: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MessageListMessage {
    pub id: String,
    #[serde(rename = "threadId")]
    pub thread_id: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,

    #[serde(rename = "threadId")]
    pub thread_id: String,

    #[serde(rename = "labelIds")]
    pub label_ids: Vec<String>,

    pub payload: MessagePayload,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MessagePayload {
    filename: String,
    headers: Vec<MessagePayloadHeaderPair>,
    body: MessagePayloadBody,
    parts: Option<Vec<MessagePayloadPart>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MessagePayloadHeaderPair {
    name: String,
    value: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MessagePayloadBody {
    size: u64,
    data: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MessagePayloadPart {
    #[serde(rename = "partId")]
    part_id: String,

    #[serde(rename = "mimeType")]
    mime_type: String,

    filename: String,
    body: MessagePayloadPartBody,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MessagePayloadPartBody {
    size: u64,
    data: String,
}

// TODO combine messages_list and message, just specify the amount, this should do the rest.
pub fn messages_list(client: &mut client::GmailClient) -> Result<MessagesList, Box<dyn Error>> {
    let messages_list: MessagesList = client
        .send(|client: &Client| {
            client.get("https://gmail.googleapis.com/gmail/v1/users/me/messages")
        })?
        .json()?;

    Ok(messages_list)
}

pub fn get_messages_batched(
    client: &mut client::GmailClient,
    message_ids: &[String],
) -> Result<Vec<Message>, Box<dyn Error>> {
    // refresh token before doing batch requests.
    client.refresh_access_token()?;

    let boundary = "batch_boundary";
    let mut body = String::new();

    for index in 0..message_ids.len() {
        match message_ids.get(index) {
            Some(message_id) => {
                body = format!(
                    "{}--{}\nContent-Type: application/http\n\nGET /gmail/v1/users/me/messages/{}?full HTTP/1.1\n",
                    body, boundary, message_id,
                );
            }
            None => return Err(format!("could not get message_id with index {}", index).into()),
        }
    }

    body = format!("{}\n--{}--", body, boundary);

    let raw_batch_resonse: String = client
        .send(|client: &Client| {
            client
                .post("https://gmail.googleapis.com/batch/gmail/v1")
                .header(
                    "Content-Type",
                    format!("multipart/mixed; boundary={}", boundary),
                )
                .body(body.to_string())
        })?
        .text()?;

    let messages: Vec<Message> = deserialize_batch_response(&raw_batch_resonse)?;

    Ok(messages)
}

fn deserialize_batch_response<T>(raw_batch_response: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let batch_boundary = raw_batch_response
        .split("\r\n")
        .filter(|line| line.len() > 0)
        .next()
        .ok_or_else(|| Box::<dyn Error>::from("could not get batch boundary from resonse"))?;

    let serialized_objects: Vec<String> = raw_batch_response
        .split(batch_boundary)
        .filter_map(|line| {
            let json_string = line
                .split_once("{")
                .map(|(_, after_bracket)| after_bracket)
                .unwrap_or("");

            if json_string.is_empty() {
                None
            } else {
                // Add the '{' used for splitting back.
                Some(format!("{{{}", json_string))
            }
        })
        .collect();

    let deserialised_objects: Vec<T> = serialized_objects
        .iter()
        .filter_map(|serialized_object: &String| {
            match serde_json::from_str::<T>(serialized_object) {
                Ok(deserialized_object) => Some(deserialized_object),
                Err(_) => {
                    eprintln!("could not deserialize '{:?}'", serialized_object);
                    None
                }
            }
        })
        .collect();

    Ok(deserialised_objects)
}
