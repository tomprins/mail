use reqwest::blocking::Client;
mod gmail;
mod utils;
use reqwest::StatusCode;
use std::{io::ErrorKind, process::exit};

fn main() {
    let gmail_client = Client::new();
    // TODO can be simplified?
    let gmail_credentials = match gmail::credentials() {
        Ok(credentials) => credentials,
        Err(error) => {
            eprintln!("could not read gmail credentials from disk: {}", error);
            exit(1);
        }
    };

    let messages_list =
        match gmail::messages_list(&gmail_client, &gmail_credentials.token.access_token) {
            Ok(messages_list) => messages_list,
            Err(error) => {
                if let Some(error) = error.downcast_ref::<reqwest::Error>() {
                    if error.status() == Some(StatusCode::UNAUTHORIZED) {
                        println!("received 'unauthorized', refreshing access token");

                        if let Err(error) =
                            gmail::refresh_access_token(&gmail_client, gmail_credentials)
                        {
                            eprintln!("could not refresh access token: {}", error);
                            exit(1)
                        }
                    }
                }
            }
        };

    // let message = match messages_list.messages.first() {
    //     Some(message) => message,
    //     None => {
    //         eprintln!("no messages in messages list");
    //         exit(1);
    //     }
    // };

    // let message = match gmail::message(&gmail_client, &access_token, &message.id) {
    //     Ok(()) => println!("message"),
    //     Err(error) => eprintln!("could not get message: {}", error),
    // };
}
