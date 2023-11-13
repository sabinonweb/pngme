#![allow(unused_variables)]
fn main() {
use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

use crate::{Error, Result};
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::{
    chunk_type::ChunkType,
    png::Png,
    chunk::Chunk
};

/// Steganography is the practice of storing a information within another object. Common approach would be to use 
/// LSB(least significant bit) steganography. In this practice, the least significant byte of the png file is changed 
/// to store information, which is not noticed by human eye but a computer program can be used to extract information 
/// from it.

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let input_bytes = fs::read(&args.input_path)?;
    let output = args.output_path.unwrap_or_else(args.input_path);
    let png = Png::try_from(input_bytes.as_slice())?;
    /// Chunk::append will add the message to the end of the PNG file, even after IEND. This means that the message will 
    /// be contained in the least significant bit, making it impossible for normal PNG decoder to decode the file and also
    /// the photo is not altered because the message is contained in the LSB.
    let chunk = Chunk::new(args.chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);
    fs::write(output, png.as_bytes())?;
    Ok(())
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let input_bytes = fs::read(&args.input_path)?;
    let png = Png::try_from(input_bytes.as_slice());
    let chunk = png.chunk_by_type(args.chunk_type)?;
    
    if let Some(c) = chunk {
        println!("{}", c);
    }
    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let input_bytes = fs::read(&args.input_path)?;
    let png = Png::try_from(input_bytes.as_slice())?;
    let removed_chunk = png.remove_chunk(args.chunk_type);

    match removed_chunk {
        Ok(removed_chunk) => {
            fs::write(args.file_path, png.as_bytes())?;
            println!("Chunk {} removed!", removed_chunk);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    } 
    Ok(())
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let input_bytes = fs::read(&args.input_path)?;
    let png = Png::try_from(input_bytes.as_slice())?;
     
    for chunk in png.chunks() {
        println!("Chunk: {}", chunk);
    }
    Ok(())
}
}