use crate::client;
use crate::constants;
use crate::search;
use base64::Engine;
use base64::engine::general_purpose;
use chrono::DateTime;
use chrono::Utc;
use reqwest::blocking::Client;
use search::Searchable;
use serde;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: String,

    #[serde(rename(deserialize = "threadId", serialize = "thread_id"))]
    pub thread_id: String,

    #[serde(rename(deserialize = "labelIds", serialize = "label_ids"))]
    pub label_ids: Vec<String>,

    pub payload: MessagePayload,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct MessagePayload {
    filename: String,
    headers: Vec<MessagePayloadHeaderPair>,
    body: MessagePayloadBody,
    parts: Option<Vec<MessagePayloadPart>>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct MessagePayloadHeaderPair {
    name: String,
    value: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct MessagePayloadBody {
    size: u64,
    data: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct MessagePayloadPart {
    #[serde(rename = "partId")]
    part_id: String,

    #[serde(rename = "mimeType")]
    mime_type: String,

    filename: String,
    body: MessagePayloadPartBody,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct MessagePayloadPartBody {
    size: u64,
    data: String,
}

impl Searchable for Message {
    fn to_searchable_mail(&self) -> Result<search::Mail, Box<dyn Error>> {
        let mut subject: Option<String> = None;
        let mut from: Option<String> = None;
        let mut to: Option<String> = None;
        let mut time: Option<i64> = None;
        let mut raw_body: Option<String> = None;
        let mut searchable_body: Option<String> = None;
        let mut labels: Vec<String> = Vec::new();

        let mut label_conversion: HashMap<&str, &str> = HashMap::new();
        label_conversion.insert("INBOX", search::LABEL_INBOX);
        label_conversion.insert("STARRED", search::LABEL_STARRED);
        label_conversion.insert("IMPORTANT", search::LABEL_IMPORTANT);
        label_conversion.insert("SENT", search::LABEL_SENT);
        label_conversion.insert("SCHEDULED", search::LABEL_SCHEDULED);
        label_conversion.insert("SPAM", search::LABEL_SPAM);
        label_conversion.insert("BIN", search::LABEL_BIN);

        for header in &self.payload.headers {
            match header.name.as_str() {
                "Subject" => subject = Some(header.value.to_string()),
                "From" => from = Some(header.value.to_string()),
                "To" => to = Some(header.value.to_string()),
                "Date" => {
                    time = Some(
                        DateTime::parse_from_rfc2822(&header.value)
                            .map_err(|error| {
                                format!("could not parse {} to datetime: {error}", &header.value)
                            })?
                            .with_timezone(&Utc)
                            .timestamp(),
                    )
                }
                _ => {}
            }
        }

        if let Some(parts) = &self.payload.parts {
            for part in parts {
                match part.mime_type.as_str() {
                    "text/html" => {
                        let decoded_raw_body = general_purpose::URL_SAFE
                            .decode(part.body.data.as_str())
                            .map_err(|error| format!("could not decode raw body: {error}"))?;
                        raw_body = Some(String::from_utf8(decoded_raw_body).map_err(|error| {
                            format!("could not create string from UTF-8 encoded raw body: {error}")
                        })?);
                    }
                    "text/plain" => {
                        let decoded_searchable_body = general_purpose::URL_SAFE
                            .decode(part.body.data.as_str())
                            .map_err(|error| format!("could not decode searchable body {error}"))?;
                        searchable_body = Some(String::from_utf8(decoded_searchable_body).map_err(|error|format!("could not create string from UTF-8 encoded searchable body: {error}"))?);
                    }
                    _ => {}
                }
            }
        }

        labels.extend(
            self.label_ids
                .iter()
                .filter_map(|label| label_conversion.get(label.as_str()))
                .map(|label| label.to_string()),
        );

        let subject = subject.ok_or("missing subject")?;
        let from = from.ok_or("missing from")?;
        let to = to.ok_or("missing to")?;
        let time = time.ok_or("missing time")?;
        let raw_body = raw_body.ok_or("missing raw body")?;
        let searchable_body = searchable_body.ok_or("missing searchable body")?;

        let mail = search::Mail {
            id: self.id.to_string(),
            thread_id: self.thread_id.to_string(),
            subject: subject,
            from,
            to,
            labels: labels,
            time: time,
            raw_body: raw_body,
            searchable_body: searchable_body,
        };

        Ok(mail)
    }
}

// TODO combine messages_list and message, just specify the amount, this should do the rest.
pub fn messages_list(
    client: &mut client::GmailClient,
    results: Option<u32>,
) -> Result<MessagesList, Box<dyn Error>> {
    let results = results.unwrap_or(3);

    if results > constants::MAXIMUM_MESSAGE_LIST_RESULTS {
        Err(format!(
            "maximum number of messages results is {}",
            constants::MAXIMUM_MESSAGE_LIST_RESULTS
        ))?
    }

    let url = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults={}",
        results
    );

    let messages_list: MessagesList = client.send(|client: &Client| client.get(&url))?.json()?;

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
