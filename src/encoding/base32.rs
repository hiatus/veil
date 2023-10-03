use std::fs;
use std::io;
use std::io::Error;
use std::str;

use base32;


const SIZE_BLOCK: usize = 4096;


pub fn encode(path_in: &str, path_out: &str) -> Result<bool, Error> {
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

	let mut buffer = [0u8; SIZE_BLOCK];

	loop {
		let n = reader.read(&mut buffer[..])?;

		if n == 0 {
			break;
		}

		let encoded = base32::encode(
			base32::Alphabet::RFC4648 { padding: true }, &buffer[..n]
		);
  
		if writer.write(&encoded.as_bytes())? == 0 {
			break;
		}
	}

	Ok(true)
}

pub fn decode(path_in: &str, path_out: &str) -> Result<bool, Error> {
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

	let mut buffer = [0u8; SIZE_BLOCK];

	loop {
		let n = reader.read(&mut buffer[..])?;

		if n == 0 {
			break;
		}

		let decoded = base32::decode(
			base32::Alphabet::RFC4648 { padding: true },
			&str::from_utf8(&buffer[..n]).unwrap()
		).unwrap();

		if writer.write(&decoded)? == 0 {
			break;
		}
	}

	Ok(true)
}