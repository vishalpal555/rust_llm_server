use std::fs::File;
use std::io::{self, Read};

use crate::utility;

pub fn read(file_name: &str) -> Result<(), io::Error> {
    let mut file: File = File::open(file_name)?;

    let mut magic: [u8; 4] = [0; 4];
    file.read_exact(&mut magic)?;
    let magic_text: &str = std::str::from_utf8(&magic)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    println!("Magic: {magic_text}");

    let version: u32 = utility::read_u32(&mut file)?;
    println!("Version: {}", version);

    let tensor_count: u64 = utility::read_u64(&mut file)?;
    println!("Tensor count: {}", tensor_count);

    let metadata_count: u64 = utility::read_u64(&mut file)?;
    println!("Metadata count: {}", metadata_count);
    
    Ok(())
}