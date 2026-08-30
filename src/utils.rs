use std::fs::File;
use std::io::{self, Read};

pub fn read_u32(file: &mut File) -> io::Result<u32> {
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

pub fn read_u64(file: &mut File) -> io::Result<u64> {
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer)?;
    Ok(u64::from_le_bytes(buffer))
}

pub fn read_string(file: &mut File) -> io::Result<String> {
    let len = read_u64(file)? as usize;

    let mut data = vec![0u8; len];
    file.read_exact(&mut data)?;

    Ok(String::from_utf8_lossy(&data).into_owned())
}