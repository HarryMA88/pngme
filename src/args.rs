use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PngmeArgs {
    #[clap(subcommand)]
    pub feature: Feature,
}

#[derive(Debug, Subcommand)]
pub enum Feature {
    /// Encode a message in a png file
    Encode(EncodeCommand),

    /// Decode a message in a png file
    Decode(DecodeCommand),

    /// Remove a message in a png file
    Remove(RemoveCommand),

    /// Print a message in a png file
    Print(PrintCommand),
}

#[derive(Debug, Args)]
pub struct EncodeCommand {
    /// File path of the png file
    pub file_path: PathBuf,
    /// Chunk type of the chunk you want to encode the message in
    pub chunk_type: String,
    /// Message you want to encode
    pub message: String,
    /// File you want to write the png to
    pub output_file: Option<PathBuf>
}

#[derive(Debug, Args)]
pub struct DecodeCommand {
    /// File path of the png file
    pub file_path: PathBuf,
    /// Chunk type of the chunk that the message is in
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct RemoveCommand {
    /// File path of the png file
    pub file_path: PathBuf,
    /// Chunk type of chunk you want to remove
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintCommand {
    /// File path of the png file
    pub file_path: PathBuf,
}