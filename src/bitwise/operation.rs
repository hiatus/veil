use std::fs;
use std::io;
use std::io::{Error, ErrorKind};

const SIZE_BLOCK: usize = 4096;

pub fn and(mask: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
    let (mut stdin_read, mut file_read);
    let (mut stdout_write, mut file_write);

    let reader: &mut dyn io::Read = if path_in == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(path_in)?;
        &mut file_read
    };

    let writer: &mut dyn io::Write = if path_out == "-" {
        stdout_write = io::stdout();
        &mut stdout_write
    } else {
        file_write = fs::File::create(path_out)?;
        &mut file_write
    };

    if mask.len() % 2 != 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Invalid mask length"));
    }

    let mask_bytes = hex::decode(mask).unwrap();
    let mut buffer = [0u8; SIZE_BLOCK];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        for (byte, mask_byte) in buffer[..n].iter_mut().zip(mask_bytes.iter().cycle()) {
            *byte &= *mask_byte;
        }

        writer.write_all(&buffer[..n])?;
    }

    Ok(true)
}

pub fn or(mask: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
    let (mut stdin_read, mut file_read);
    let (mut stdout_write, mut file_write);

    let reader: &mut dyn io::Read = if path_in == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(path_in)?;
        &mut file_read
    };

    let writer: &mut dyn io::Write = if path_out == "-" {
        stdout_write = io::stdout();
        &mut stdout_write
    } else {
        file_write = fs::File::create(path_out)?;
        &mut file_write
    };

    if mask.len() % 2 != 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Invalid mask length"));
    }

    let mask_bytes = hex::decode(mask).unwrap();
    let mut buffer = [0u8; SIZE_BLOCK];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        for (byte, mask_byte) in buffer[..n].iter_mut().zip(mask_bytes.iter().cycle()) {
            *byte |= *mask_byte;
        }

        writer.write_all(&buffer[..n])?;
    }

    Ok(true)
}

pub fn xor(mask: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
    let (mut stdin_read, mut file_read);
    let (mut stdout_write, mut file_write);

    let reader: &mut dyn io::Read = if path_in == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(path_in)?;
        &mut file_read
    };

    let writer: &mut dyn io::Write = if path_out == "-" {
        stdout_write = io::stdout();
        &mut stdout_write
    } else {
        file_write = fs::File::create(path_out)?;
        &mut file_write
    };

    if mask.len() % 2 != 0 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Invalid mask length"));
    }

    let mask_bytes = hex::decode(mask).unwrap();
    let mut buffer = [0u8; SIZE_BLOCK];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        for (byte, mask_byte) in buffer[..n].iter_mut().zip(mask_bytes.iter().cycle()) {
            *byte ^= *mask_byte;
        }

        writer.write_all(&buffer[..n])?;
    }

    Ok(true)
}
