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
            eprintln!("could not refresh access token: {}", error);
            exit(1)
        }
    };

    println!("{:?}", messages_list);
}
