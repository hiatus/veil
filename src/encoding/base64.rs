use std::fs::File;
use std::io;
use std::str;

use base64::{engine::general_purpose::STANDARD, read::DecoderReader, write::EncoderWriter};

pub fn encode(path_in: &str, path_out: &str) -> Result<bool, io::Error> {
    let mut reader: Box<dyn io::Read> = if path_in == "-" {
        Box::new(io::stdin().lock())
    } else {
        Box::new(File::open(path_in)?)
    };

    let writer: Box<dyn io::Write> = if path_out == "-" {
        Box::new(io::stdout().lock())
    } else {
        Box::new(File::create(path_out)?)
    };

    let mut encoder = EncoderWriter::new(writer, &STANDARD);

    io::copy(&mut reader, &mut encoder)?;
    encoder.finish()?;

    Ok(true)
}

pub fn decode(path_in: &str, path_out: &str) -> Result<bool, io::Error> {
    let reader: Box<dyn io::Read> = if path_in == "-" {
        Box::new(io::stdin().lock())
    } else {
        Box::new(File::open(path_in)?)
    };

    let mut writer: Box<dyn io::Write> = if path_out == "-" {
        Box::new(io::stdout().lock())
    } else {
        Box::new(File::create(path_out)?)
    };

    let mut decoder = DecoderReader::new(reader, &STANDARD);

    io::copy(&mut decoder, &mut writer)?;

    Ok(true)
}
