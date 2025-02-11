use std::fs;
use std::io;
use std::io::{Error, ErrorKind};

use chacha20::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};
use chacha20::ChaCha20;
use rand::Rng;

const SIZE_BLOCK: usize = 4096;

pub fn encrypt_20(key: &str, nonce: &str, skip: usize, path_in: &str, path_out: &str) -> Result<bool, Error> {
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

    if key.len() != 32 * 2 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Invalid key length"));
    }

    if ! nonce.is_empty() && nonce.len() != 12 * 2 {
        return Err(Error::new(ErrorKind::UnexpectedEof, "Invalid nonce length"));
    }

    let key_bytes: [u8; 32] = hex::decode(key).unwrap().try_into().unwrap();
    let mut nonce_bytes = rand::rng().random::<[u8; 12]>();
    let mut buffer = [0u8; SIZE_BLOCK];

    if ! nonce.is_empty() {
        nonce_bytes = hex::decode(nonce).unwrap().try_into().unwrap();
    } else {
        writer.write_all(&nonce_bytes)?;
    }

    let mut cipher = ChaCha20::new(&key_bytes.into(), &nonce_bytes.into());

    if skip > 0 {
        cipher.seek(skip);
    }

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }

        cipher.apply_keystream(&mut buffer[..n]);
        writer.write_all(&buffer[..n])?;
    }

    Ok(true)
}
