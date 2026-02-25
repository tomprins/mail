use once_cell::sync::Lazy;
use std::path::PathBuf;

pub static CREDENTIALS_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("C:\\Users\\tompr\\mail\\credentials"));

pub static GMAIL_CREDENTIALS: Lazy<PathBuf> = Lazy::new(|| CREDENTIALS_PATH.join("gmail.json"));
pub static TYPESENSE_CREDENTIALS: Lazy<PathBuf> =
    Lazy::new(|| CREDENTIALS_PATH.join("typesense.json"));

pub const SEARCHABLE_MAIL_COLLECTION_NAME: &'static str = "mail";
pub const MAXIMUM_MESSAGE_LIST_RESULTS: u32 = 500;
