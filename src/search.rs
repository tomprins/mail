use crate::constants;
use crate::utils;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use tokio::runtime::Runtime;
use typesense::apis::collections_api;
use typesense::apis::configuration::ApiKey;
use typesense::apis::configuration::Configuration;
use typesense::apis::documents_api::import_documents;
use typesense::collection_schema::CollectionSchema;
use typesense::field::Field;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
enum FieldType {
    String,
    StringArray,
    Int32,
    Int32Array,
    Int64,
    Int64Array,
    Float,
    FloatArray,
    Bool,
    BoolArray,
    Geopoint,
    GeopointArray,
    Geopolygon,
    Object,
    ObjectArray,
    StringPointer,
    Image,
    Auto,
}

#[allow(dead_code)]
impl FieldType {
    fn as_str(&self) -> &'static str {
        match self {
            FieldType::String => "string",
            FieldType::StringArray => "string[]",
            FieldType::Int32 => "int32",
            FieldType::Int32Array => "int32[]",
            FieldType::Int64 => "int64",
            FieldType::Int64Array => "int64[]",
            FieldType::Float => "float",
            FieldType::FloatArray => "float[]",
            FieldType::Bool => "bool",
            FieldType::BoolArray => "bool[]",
            FieldType::Geopoint => "geopoint",
            FieldType::GeopointArray => "geopoint[]",
            FieldType::Geopolygon => "geopolygon",
            FieldType::Object => "object",
            FieldType::ObjectArray => "object[]",
            FieldType::StringPointer => "stringpointer",
            FieldType::Image => "image",
            FieldType::Auto => "auto",
        }
    }

    fn from_str(label: &str) -> Option<Self> {
        match label {
            "string" => Some(FieldType::String),
            "string[]" => Some(FieldType::StringArray),
            "int32" => Some(FieldType::Int32),
            "int32[]" => Some(FieldType::Int32Array),
            "int64" => Some(FieldType::Int64),
            "int64[]" => Some(FieldType::Int64Array),
            "float" => Some(FieldType::Float),
            "float[]" => Some(FieldType::FloatArray),
            "bool" => Some(FieldType::Bool),
            "bool[]" => Some(FieldType::BoolArray),
            "geopoint" => Some(FieldType::Geopoint),
            "geopoint[]" => Some(FieldType::GeopointArray),
            "geopolygon" => Some(FieldType::Geopolygon),
            "object" => Some(FieldType::Object),
            "object[]" => Some(FieldType::ObjectArray),
            "stringpointer" => Some(FieldType::StringPointer),
            "image" => Some(FieldType::Image),
            "auto" => Some(FieldType::Auto),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mail {
    pub id: String,
    pub thread_id: String,

    pub subject: String,
    pub time: i64, // unix timestamp
    pub labels: Vec<String>,

    pub raw_body: String,        // html
    pub searchable_body: String, // cleaned text

    pub from: String,
    pub to: String,
}

#[allow(dead_code)]
enum Label {
    Inbox,
    Starred,
    Important,
    Sent,
    Scheduled,
    Spam,
    Bin,
}

#[allow(dead_code)]
impl Label {
    fn as_str(&self) -> &'static str {
        match self {
            Label::Inbox => "inbox",
            Label::Starred => "starred",
            Label::Important => "important",
            Label::Sent => "sent",
            Label::Scheduled => "scheduled",
            Label::Spam => "spam",
            Label::Bin => "bin",
        }
    }

    fn from_str(label: &str) -> Option<Self> {
        match label {
            "spam" => Some(Label::Inbox),
            "scheduled" => Some(Label::Starred),
            "sent" => Some(Label::Important),
            "important" => Some(Label::Sent),
            "starred" => Some(Label::Scheduled),
            "inbox" => Some(Label::Spam),
            _ => None,
        }
    }
}

pub const LABEL_INBOX: &str = "inbox";
pub const LABEL_STARRED: &str = "starred";
pub const LABEL_IMPORTANT: &str = "important";
pub const LABEL_SENT: &str = "sent";
pub const LABEL_SCHEDULED: &str = "scheduled";
pub const LABEL_SPAM: &str = "spam";
pub const LABEL_BIN: &str = "bin";

#[allow(dead_code)]
pub const LABELS: [&'static str; 7] = [
    LABEL_INBOX,
    LABEL_STARRED,
    LABEL_IMPORTANT,
    LABEL_SENT,
    LABEL_SCHEDULED,
    LABEL_SPAM,
    LABEL_BIN,
];

pub trait Searchable {
    fn to_searchable_mail(&self) -> Result<Mail, Box<dyn Error>>;
}

#[allow(dead_code)]
impl Mail {
    pub fn collection_schema() -> CollectionSchema {
        CollectionSchema {
            name: constants::SEARCHABLE_MAIL_COLLECTION_NAME.to_string(),
            fields: vec![
                Field {
                    name: "id".to_string(),
                    r#type: FieldType::String.as_str().to_string(),
                    ..Default::default()
                },
                Field {
                    name: "thread_id".to_string(),
                    r#type: FieldType::String.as_str().to_string(),
                    ..Default::default()
                },
                Field {
                    name: "subject".to_string(),
                    r#type: FieldType::String.as_str().to_string(),
                    optional: Some(true),
                    infix: Some(true),
                    ..Default::default()
                },
                Field {
                    name: "time".to_string(),
                    r#type: FieldType::Int64.as_str().to_string(),
                    ..Default::default()
                },
                Field {
                    name: "labels".to_string(),
                    optional: Some(true),
                    facet: Some(true),
                    r#type: FieldType::StringArray.as_str().to_string(),
                    ..Default::default()
                },
                Field {
                    name: "raw_body".to_string(),
                    r#type: FieldType::String.as_str().to_string(),
                    index: Some(false),
                    ..Default::default()
                },
                Field {
                    name: "searchable_body".to_string(),
                    r#type: FieldType::String.as_str().to_string(),
                    infix: Some(true),
                    ..Default::default()
                },
                Field {
                    name: "from".to_string(),
                    r#type: FieldType::String.as_str().to_string(),
                    infix: Some(true),
                    facet: Some(true),
                    ..Default::default()
                },
                Field {
                    name: "to".to_string(),
                    r#type: FieldType::String.as_str().to_string(),
                    infix: Some(true),
                    facet: Some(true),
                    ..Default::default()
                },
            ],
            default_sorting_field: None,
            token_separators: None,
            enable_nested_fields: None,
            symbols_to_index: None,
        }
    }
}

#[allow(dead_code)]
pub fn update_collection(
    runtime: Runtime,
    configuration: &Configuration,
) -> Result<(), Box<dyn Error>> {
    let collection = runtime
        .block_on(collections_api::create_collection(
            configuration,
            Mail::collection_schema(),
        ))
        .map_err(|error| format!("could not update mail collection schema: {error}"))?;

    println!("{:?}", collection);

    Ok(())
}

// TODO shoul probaly be embedded in the runtime clinet
pub fn get_typesense_configuration() -> Result<Configuration, Box<dyn Error>> {
    #[derive(Serialize, Deserialize, Debug)]
    struct Credentials {
        url: String,
        user_agent: String,
        api_key: String,
    }

    let credentials: Credentials =
        utils::read_json(&constants::TYPESENSE_CREDENTIALS.display().to_string())
            .map_err(|error| format!("could not read typesense credentials: {error}"))?;

    let configuration = Configuration {
        base_path: credentials.url,
        user_agent: Some(credentials.user_agent),
        api_key: Some(ApiKey {
            prefix: None,
            key: credentials.api_key,
        }),
        ..Default::default()
    };

    Ok(configuration)
}

pub fn import_document<T>(
    runtime: &Runtime,
    configuration: &Configuration,
    collection_name: &str,
    document: &T,
) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    let body = serde_json::to_string(document)
        .map_err(|error| format!("could not serialize object for {collection_name}: {error}"))?;

    let result = runtime
        .block_on(import_documents(configuration, collection_name, body, None))
        .map_err(|error| format!("could not import document into {collection_name}: {error}"))?;

    println!("imported document into {}: {}", collection_name, result);

    Ok(())
}
