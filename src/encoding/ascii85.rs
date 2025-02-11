use std::fs;
use std::io;
use std::io::Error;
use std::str;

const SIZE_BLOCK: usize = 4096;

pub fn encode(path_in: &str, path_out: &str) -> Result<bool, Error> {
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

    let mut buffer = [0u8; SIZE_BLOCK];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        let encoded = ascii85::encode(&buffer[..n]);
        writer.write_all(encoded.as_bytes())?;
    }

    Ok(true)
}

pub fn decode(path_in: &str, path_out: &str) -> Result<bool, Error> {
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

    let mut buffer = [0u8; SIZE_BLOCK];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        let decoded = ascii85::decode(str::from_utf8(&buffer[..n]).unwrap()).unwrap();

        let _ = writer.write_all(&decoded);
    }

    Ok(true)
}
