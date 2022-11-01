use std::{fs, str::FromStr};

use crate::{png::Png, chunk::Chunk, chunk_type::ChunkType};

pub fn encode(path: &str, sig: &str, message: &str) {
    let mut image = Png::try_from(
        fs::read(path)
        .unwrap()
        .as_ref()
    ).unwrap();

    let message: Vec<u8> = message.bytes().collect();
    image.append_chunk(
            Chunk::new(
                ChunkType::from_str(sig).unwrap(),
                message
            )
        );
    let result = image.as_bytes();
    fs::write(path, result).unwrap();
}

pub fn decode(path: &str, sig: &str) -> Result<String, String> {
    let image = Png::try_from(fs::read(path).unwrap().as_ref()).unwrap();
    let message = match image.chunk_by_type(sig) {
        Some(result) => {
            result
        },
        None => return Err(String::from("chunk not found")),
    };
    Ok(message.data_as_string().unwrap())
}

pub fn remove(path: &str, sig: &str) -> anyhow::Result<String> {
    let mut image = Png::try_from(fs::read(path).unwrap().as_ref()).unwrap();
    let removed_chunk = image.remove_chunk(sig).unwrap();

    let result = image.as_bytes();
    fs::write(path, result).unwrap();

    removed_chunk.data_as_string()
}

// pub fn print(path: &str, sig: &str) -> anyhow::Result<String> {
//     let mut image = Png::try_from(fs::read(path).unwrap().as_ref()).unwrap();

// }
