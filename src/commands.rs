use std::{
    fs::File,
    io::{Read, Write},
    path::Path, str::FromStr,
};

use crate::{png::Png, error::{Result, Error}, chunk_type::ChunkType, chunk::Chunk};

fn read_png(path: &Path) -> Result<Png> {
    let mut file = File::open(path)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;
    Ok(data.as_slice().try_into()?)
}

fn write_png(png: &Png, path: &Path) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(png.as_bytes().as_slice())?;
    Ok(())
}

pub fn decode(input: &Path, chunk: String) -> Result<()> {
    let mut png: Png = read_png(input)?;
    if let Some(chunk) = png.chunk_by_type(chunk.as_str()) {
        println!("Message found : {}", chunk.data_as_string()?);
    }
    else {
        return Err(Error::Other("No message found"))
    }
    Ok(())
} 

pub fn encode(input: &Path, chunk: String, message: String) -> Result<()> {
    let mut png: Png = read_png(input)?;
    let chunk_type: ChunkType = ChunkType::from_str(chunk.as_str())?;
    let data: Vec<u8> = message.into_bytes();
    let chunk = Chunk::new(chunk_type, data);
    png.append_chunk(chunk);

    write_png(&png, input)?;
    Ok(())
}

pub fn remove(input: &Path, chunk: String) -> Result<()> {
    let mut png: Png = read_png(input)?;
    png.remove_chunk(chunk.as_str());

    write_png(&png, input)?;
    Ok(())
}

pub fn print(input: &Path) -> Result<()> {
    println!("{}", read_png(input)?);
    Ok(())
}