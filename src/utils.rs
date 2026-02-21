use anyhow::Result;
use std::{
    error::Error,
    fs::File,
    io::{BufReader, BufWriter},
};

use serde::{Serialize, de::DeserializeOwned};

pub fn write_struct_to_file<T: Serialize>(value: &T, path: &str) -> Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, value)?;

    Ok(())
}

pub fn read_json<T>(path: &str) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let file = File::open(path).map_err(|error| format!("could not open '{path}': {error}"))?;
    let reader = BufReader::new(file);
    let credentials: T = serde_json::from_reader(reader)
        .map_err(|error| format!("could not parse '{path}' to struct: {error}"))?;
    Ok(credentials)
}
