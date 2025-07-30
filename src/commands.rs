use crate::args;
use crate::args::{DecodeCommand, EncodeCommand, PngmeArgs, PrintCommand, RemoveCommand};
use crate::Result;
use std::fs;
use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

fn encode(args: EncodeCommand) -> Result<()>{
    let file_bytes = fs::read(&args.file_path)?;
    let output_file = args.output_file.unwrap_or(args.file_path);
    let mut png_file = Png::try_from(file_bytes.as_ref())?;

    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let message: Vec<u8> = args.message.as_bytes().iter().copied().collect();
    let message_chunk = Chunk::new(chunk_type, message);

    png_file.append_chunk(message_chunk);
    fs::write(output_file, png_file.as_bytes())?;

    Ok(())
}

fn decode(args: DecodeCommand) -> Result<()>{
    let file_bytes = fs::read(&args.file_path)?;
    let png = Png::try_from(file_bytes.as_ref())?;

    let message_chunk = png.chunk_by_type(&args.chunk_type);

    match message_chunk {
        Some(chunk) => println!("Message: {}", chunk.data_as_string()?),
        None => println!("No message found in PNG with given chunk type"),
    }

    Ok(())
}

fn remove(args: RemoveCommand) -> Result<()>{
    let file_bytes = fs::read(&args.file_path)?;
    let mut png = Png::try_from(file_bytes.as_ref())?;

    match png.remove_first_chunk(&args.chunk_type) {
        Ok(_) => {
            fs::write(&args.file_path, png.as_bytes())?;
            println!("Removed message from {:?}", &args.file_path)
        },
        Err(_) => println!("Failed to remove message from PNG, no message in chunk type"),
    }
    Ok(())
}

fn print(args: PrintCommand) -> Result<()>{
    let file_bytes = fs::read(&args.file_path)?;
    let png = Png::try_from(file_bytes.as_ref())?;

    for chunk in png.chunks() {
        println!("{}", chunk)
    }
    Ok(())
}

pub fn run(args: PngmeArgs) -> Result<()> {
    let feature = args.feature;

    match feature {
        args::Feature::Encode(sub_args) => encode(sub_args),
        args::Feature::Decode(sub_args) => decode(sub_args),
        args::Feature::Remove(sub_args) => remove(sub_args),
        args::Feature::Print(sub_args) => print(sub_args),
    }
}