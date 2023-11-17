#![allow(unused_variables)]

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Commands {
    #[clap(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Parser, Clone)]
pub enum PngMeArgs {
    /// Encodes the message in a specific PNG file
    Encode(EncodeArgs),
    /// Decodes the message from a specific PNG file
    Decode(DecodeArgs),
    /// Remove the message from a specific PNG file
    Remove(RemoveArgs),
    /// Print the PNG chunks that can be searched for messages
    Print(PrintArgs),
}

#[derive(Parser, Clone)]
pub struct EncodeArgs {
    /// Chunk type
    pub chunk_type: String,
    /// Input PNG file path
    pub input_path: PathBuf,
    /// Message you want to store in a PNG file
    pub message: String,
    /// If set, the file is saved in a safe place, if not the file is saved in the input file itself.
    pub output_path: Option<PathBuf>,
}

#[derive(Parser, Clone)]
pub struct DecodeArgs {
    /// Chunk type
    pub chunk_type: String,
    /// Input PNG file path
    pub input_path: PathBuf,
}

#[derive(Parser, Clone)]
pub struct RemoveArgs {
    /// Chunk type
    pub chunk_type: String,
    /// Input PNG file path
    pub input_path: PathBuf,
}

#[derive(Parser, Clone)]
pub struct PrintArgs {
    /// Input file path
    pub input_path: PathBuf,
}
