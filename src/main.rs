mod client;
mod constants;
mod gmail;
mod utils;
use client::GmailClient;
use std::process::exit;

fn main() {
    let mut gmail_client = GmailClient::new();

    let messages_list = match gmail::messages_list(&mut gmail_client) {
        Ok(message_list) => message_list,
        Err(error) => {
            eprintln!("could not get messages list: {}", error);
            exit(1)
        }
    };

    let messages_id: Vec<String> = messages_list
        .messages
        .iter()
        .map(|message: &gmail::MessageListMessage| message.id.to_string())
        .collect();

    let messages = match gmail::get_messages_batched(&mut gmail_client, &messages_id) {
        Ok(messages) => messages,
        Err(error) => {
            eprintln!("could not get messages: {}", error);
            exit(1)
        }
    };

    println!("{:?}", messages)
}
