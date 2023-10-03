use std::fs;
use std::io;
use std::io::{Error, ErrorKind};

use hex;
use libaes::Cipher;
use rand::Rng;

const SIZE_IV: usize = 16;
const SIZE_BLOCK: usize = 4096;


fn _gen_iv() -> [u8; SIZE_IV] {
	rand::thread_rng().gen::<[u8; SIZE_IV]>()
}

pub fn encrypt_128(key: &str, iv: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
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

	if key.len() != 16 * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES key length"
		));
	}

	if iv != "" && iv.len() != SIZE_IV * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES IV length"
		));
	}

	let key_bytes: [u8; 16] = hex::decode(key).unwrap().try_into().unwrap();
	let mut iv_bytes: [u8; SIZE_IV] = _gen_iv();
	let mut buffer = [0u8; SIZE_BLOCK];

	if iv != "" {
		iv_bytes = hex::decode(iv).unwrap().try_into().unwrap();
	}
	else {
		writer.write(&iv_bytes)?;
	}

	let cipher: Cipher = Cipher::new_128(&key_bytes);

	loop {
		let n = reader.read(&mut buffer[..])?;

		if n == 0 {
			break;
		}

		let cipher_block = cipher.cbc_encrypt(&iv_bytes, &buffer[..n]);
    
		if writer.write(&cipher_block)? == 0 {
			break;
		}
	}
    
	Ok(true)
}

pub fn encrypt_192(key: &str, iv: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
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

	if key.len() != 24 * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES key length"
		));
	}

	if iv != "" && iv.len() != SIZE_IV * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES IV length"
		));
	}

	let key_bytes: [u8; 24] = hex::decode(key).unwrap().try_into().unwrap();
	let mut iv_bytes: [u8; SIZE_IV] = _gen_iv();
	let mut buffer = [0u8; SIZE_BLOCK];

	if iv != "" {
		iv_bytes = hex::decode(iv).unwrap().try_into().unwrap();
	}
	else {
		writer.write(&iv_bytes)?;
	}

	let cipher: Cipher = Cipher::new_192(&key_bytes);

	loop {
		let n = reader.read(&mut buffer[..])?;

		if n == 0 {
			break;
		}

		let cipher_block = cipher.cbc_encrypt(&iv_bytes, &buffer[..n]);
    
		if writer.write(&cipher_block)? == 0 {
			break;
		}
	}
    
	Ok(true)
}

pub fn encrypt_256(key: &str, iv: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
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

	if key.len() != 32 * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES key length"
		));
	}

	if iv != "" && iv.len() != SIZE_IV * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES IV length"
		));
	}

	let key_bytes: [u8; 32] = hex::decode(key).unwrap().try_into().unwrap();
	let mut iv_bytes: [u8; SIZE_IV] = _gen_iv();
	let mut buffer = [0u8; SIZE_BLOCK];

	if iv != "" {
		iv_bytes = hex::decode(iv).unwrap().try_into().unwrap();
	}
	else {
		writer.write(&iv_bytes)?;
	}

	let cipher: Cipher = Cipher::new_256(&key_bytes);

	loop {
		let n = reader.read(&mut buffer[..])?;

		if n == 0 {
			break;
		}

		let cipher_block = cipher.cbc_encrypt(&iv_bytes, &buffer[..n]);
    
		if writer.write(&cipher_block)? == 0 {
			break;
		}
	}
    
	Ok(true)
}

pub fn decrypt_128(key: &str, iv: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
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

	if key.len() != 16 * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES key length"
		));
	}

	if iv != "" && iv.len() != SIZE_IV * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES IV length"
		));
	}

	let key_bytes: [u8; 16] = hex::decode(key).unwrap().try_into().unwrap();
	let mut iv_bytes: [u8; SIZE_IV] = [0u8; SIZE_IV];
	let mut buffer = [0u8; SIZE_BLOCK];

	if iv == "" {
		if reader.read(&mut iv_bytes[..])? != SIZE_IV {
			return Err(Error::new(
				ErrorKind::UnexpectedEof, "Invalid AES IV length"
			));
		}
	}
	else {
		// let _ = reader.read(&mut iv_bytes[..])?;
		iv_bytes = hex::decode(iv).unwrap().try_into().unwrap();

		if iv_bytes.len() != SIZE_IV {
			return Err(Error::new(
				ErrorKind::UnexpectedEof, "Invalid AES IV length"
			));
		}
	}

	let cipher: Cipher = Cipher::new_128(&key_bytes);

	loop {
		let n = reader.read(&mut buffer[..])?;

		if n == 0 {
			break;
		}

		let plain_block = cipher.cbc_decrypt(&iv_bytes, &buffer[..n]);
    
		if writer.write(&plain_block)? == 0 {
			break;
		}
	}
    
	Ok(true)
}

pub fn decrypt_192(key: &str, iv: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
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

	if key.len() != 24 * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES key length"
		));
	}

	if iv != "" && iv.len() != SIZE_IV * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES IV length"
		));
	}

	let key_bytes: [u8; 24] = hex::decode(key).unwrap().try_into().unwrap();
	let mut iv_bytes: [u8; SIZE_IV] = [0u8; SIZE_IV];
	let mut buffer = [0u8; SIZE_BLOCK];

	if iv == "" {
		if reader.read(&mut iv_bytes[..])? != SIZE_IV {
			return Err(Error::new(
				ErrorKind::UnexpectedEof, "Invalid AES IV length"
			));
		}
	}
	else {
		// let _ = reader.read(&mut iv_bytes[..])?;
		iv_bytes = hex::decode(iv).unwrap().try_into().unwrap();

		if iv_bytes.len() != SIZE_IV {
			return Err(Error::new(
				ErrorKind::UnexpectedEof, "Invalid AES IV length"
			));
		}
	}

	let cipher: Cipher = Cipher::new_192(&key_bytes);

	loop {
		let n = reader.read(&mut buffer[..])?;

		if n == 0 {
			break;
		}

		let plain_block = cipher.cbc_decrypt(&iv_bytes, &buffer[..n]);
    
		if writer.write(&plain_block)? == 0 {
			break;
		}
	}
    
	Ok(true)
}

pub fn decrypt_256(key: &str, iv: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
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

	if key.len() != 32 * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES key length"
		));
	}

	if iv != "" && iv.len() != SIZE_IV * 2 {
		return Err(Error::new(
			ErrorKind::UnexpectedEof, "Invalid AES IV length"
		));
	}

	let key_bytes: [u8; 32] = hex::decode(key).unwrap().try_into().unwrap();
	let mut iv_bytes: [u8; SIZE_IV] = [0u8; SIZE_IV];
	let mut buffer = [0u8; SIZE_BLOCK];

	if iv == "" {
		if reader.read(&mut iv_bytes[..])? != SIZE_IV {
			return Err(Error::new(
				ErrorKind::UnexpectedEof, "Invalid AES IV length"
			));
		}
	}
	else {
		// let _ = reader.read(&mut iv_bytes[..])?;
		iv_bytes = hex::decode(iv).unwrap().try_into().unwrap();

		if iv_bytes.len() != SIZE_IV {
			return Err(Error::new(
				ErrorKind::UnexpectedEof, "Invalid AES IV length"
			));
		}
	}

	let cipher: Cipher = Cipher::new_256(&key_bytes);

	loop {
		let n = reader.read(&mut buffer[..])?;

		if n == 0 {
			break;
		}

		let plain_block = cipher.cbc_decrypt(&iv_bytes, &buffer[..n]);
    
		if writer.write(&plain_block)? == 0 {
			break;
		}
	}
    
	Ok(true)
}