use std::fs::File;
use std::io::{self, Read};

use crate::utils;

pub fn read_header(file: &mut File) -> io::Result<u64> {
    let mut magic: [u8; 4] = [0; 4];
    file.read_exact(&mut magic)?;
    let magic_text: &str = std::str::from_utf8(&magic)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    println!("Magic: {magic_text}");

    let version: u32 = utils::read_u32(file)?;
    println!("Version: {}", version);

    let tensor_count: u64 = utils::read_u64(file)?;
    println!("Tensor count: {}", tensor_count);

    let metadata_count: u64 = utils::read_u64(file)?;
    println!("Metadata count: {}", metadata_count);

    Ok(metadata_count)
}