use std::fs;
use std::io;
use std::io::{Error, ErrorKind};

use hex;
use crypto::rc4::Rc4;
use crypto::symmetriccipher::SynchronousStreamCipher;


const SIZE_BLOCK: usize = 4096;


pub fn encrypt(key: &str, skip: usize, path_in: &str, path_out: &str) -> Result<bool, Error> {
    let (mut stdin_read, mut file_read);
    let (mut stdout_write, mut file_write);

    let reader: &mut dyn io::Read = if path_in == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    }
    else {
        file_read = fs::File::open(path_in)?;
        &mut file_read
    };

    let writer: &mut dyn io::Write = if path_out == "-" {
        stdout_write = io::stdout();
        &mut stdout_write
    }
    else {
        file_write = fs::File::create(path_out)?;
        &mut file_write
    };

    if key.len() % 2 != 0 {
        return Err(Error::new(
            ErrorKind::UnexpectedEof, "Invalid key hex"
        ));
    }

    let key_bytes = hex::decode(key).unwrap();

    let mut buffer = [0u8; SIZE_BLOCK];
    let mut block = [0u8; SIZE_BLOCK];

    let mut cipher = Rc4::new(&key_bytes);

    for _ in 0..skip {
        let byte_in = [0u8; 1];
        let mut byte_out = [0u8; 1];

        cipher.process(&byte_in[..], &mut byte_out[..]);
    }

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        cipher.process(&buffer[..], &mut block[..]);
        writer.write_all(&block[..n])?;
    }

    Ok(true)
}
