use std::fs::File;
use std::io;

use crate::llm::header;
use crate::llm::metadata;
use crate::llm::tensor;
use crate::llm::tensor::read_tensor_data;

pub fn connect_llm(file_name: &str) -> io::Result<()> {
    println!("[connect_llm]reading {file_name}");
    let mut file: File = File::open(file_name)?;

    println!("[connect_llm]-----HEADER--------");
    let (tensor_count, metadata_count) = header::read_header(&mut file)?;

    println!("[connect_llm]----METADATA-------");
    let general_alignment = metadata::read_meatadata(&mut file, metadata_count)?;

    let tensor_info_vec = tensor::read_tensor_info(&mut file, tensor_count as usize)?;
    println!("{:?}", tensor_info_vec);

    read_tensor_data(&mut file, general_alignment)?;

    Ok(())
}
