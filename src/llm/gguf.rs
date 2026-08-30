use std::fs::File;
use std::io;

use crate::llm::header;
use crate::llm::metadata;

pub fn connect_llm(file_name: &str) -> io::Result<()> {
    println!("[connect_llm]reading {file_name}");
    let mut file: File = File::open(file_name)?;

    println!("[connect_llm]-----HEADER--------");
    let metadata_count = header::read_header(&mut file)?;

    println!("[connect_llm]----METADATA-------");
    metadata::read_meatadata(&mut file, metadata_count)?;
    Ok(())
}
