mod client;
mod constants;
mod gmail;
mod search;
mod utils;
use crate::search::Searchable;
use client::GmailClient;
use std::process::exit;
use tokio::runtime::Runtime;

fn main() {
    let runtime = Runtime::new().unwrap();
    let typesense_configuration = search::get_typesense_configuration().unwrap();
    let mut gmail_client = GmailClient::new();

    let messages_list = match gmail::messages_list(&mut gmail_client, None) {
        Ok(message_list) => message_list,
        Err(error) => {
            eprintln!("could not get messages list: {}", error);
            exit(1)
        }
    };

    let messages_ids: Vec<String> = messages_list
        .messages
        .iter()
        .map(|message: &gmail::MessageListMessage| message.id.to_string())
        .collect();

    let messages = match gmail::get_messages_batched(&mut gmail_client, &messages_ids) {
        Ok(messages) => messages,
        Err(error) => {
            eprintln!("could not get messages: {}", error);
            exit(1)
        }
    };

    let messages: Vec<search::Mail> = messages
        .iter()
        .filter_map(|message| {
            message
                .to_searchable_mail()
                .inspect_err(|error| {
                    eprintln!("could not convert gmail message to searchable message: {error}")
                })
                .ok()
        })
        .collect();

    for message in messages {
        match search::import_document(
            &runtime,
            &typesense_configuration,
            &constants::SEARCHABLE_MAIL_COLLECTION_NAME,
            &message,
        ) {
            Err(error) => eprintln!("could not import message into typesense: {error}"),
            _ => {}
        };
    }
}
