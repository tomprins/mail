use anyhow::Result;
use std::{fs::File, io::BufWriter};

use serde::Serialize;

pub fn write_struct_to_file<T: Serialize>(value: &T, path: &str) -> Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, value)?;

    Ok(())
}
