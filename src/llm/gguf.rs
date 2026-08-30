use std::fs::File;
use std::io;

use crate::llm::header;
use crate::llm::metadata;

pub fn connect_llm(file_name: &str) -> io::Result<()> {
    let mut file: File = File::open(file_name)?;
    let metadata_count = header::read_header(&mut file)?;
    metadata::read_meatadata(&mut file, metadata_count)?;
    Ok(())
}
