use std::io::Error;
use std::process::exit;

use clap::Parser;

// Local
mod args;
mod style;
mod encoding;
mod cipher;

use args::*;
use style::*;
use encoding::*;
use cipher::*;


fn _main(cli_args: VeilArgs) -> Result<i32, Error> {
    match cli_args.command {
        // Encoding
        VeilCommand::Hex(hex_args) => {
            let output: String = if hex_args.output.is_some() { hex_args.output.unwrap() } else { String::from("-") };
            let file: String = if hex_args.file.is_some() { hex_args.file.unwrap() } else { String::from("-") };

            cmd_hex(hex_args.decode, &file, &output)?;
        },

        VeilCommand::Base32(b32_args) => {
            let output: String = if b32_args.output.is_some() { b32_args.output.unwrap() } else { String::from("-") };
            let file: String = if b32_args.file.is_some() { b32_args.file.unwrap() } else { String::from("-") };

            cmd_base32(b32_args.decode, &file, &output)?;
        },
    
        VeilCommand::Base64(b64_args) => {
            let output: String = if b64_args.output.is_some() { b64_args.output.unwrap() } else { String::from("-") };
            let file: String = if b64_args.file.is_some() { b64_args.file.unwrap() } else { String::from("-") };

            cmd_base64(b64_args.decode, &file, &output)?;
        },

        VeilCommand::Base85(b85_args) => {
            let output: String = if b85_args.output.is_some() { b85_args.output.unwrap() } else { String::from("-") };
            let file: String = if b85_args.file.is_some() { b85_args.file.unwrap() } else { String::from("-") };

            cmd_base85(b85_args.decode, &file, &output)?;
        },

        VeilCommand::Ascii85(a85_args) => {
            let output: String = if a85_args.output.is_some() { a85_args.output.unwrap() } else { String::from("-") };
            let file: String = if a85_args.file.is_some() { a85_args.file.unwrap() } else { String::from("-") };

            cmd_ascii85(a85_args.decode, &file, &output)?;
        },

        // Encryption
        VeilCommand::Aes128(aes128_args) => {
            let iv = if aes128_args.iv.is_some() { aes128_args.iv.unwrap() } else { String::new() };
            let output: String = if aes128_args.output.is_some() { aes128_args.output.unwrap() } else { String::from("-") };
            let file: String = if aes128_args.file.is_some() { aes128_args.file.unwrap() } else { String::from("-") };
    
            cmd_aes128(aes128_args.decrypt, &aes128_args.key, &iv, &file, &output)?;
        },

        VeilCommand::Aes192(aes192_args) => {
            let iv = if aes192_args.iv.is_some() { aes192_args.iv.unwrap() } else { String::new() };
            let output: String = if aes192_args.output.is_some() { aes192_args.output.unwrap() } else { String::from("-") };
            let file: String = if aes192_args.file.is_some() { aes192_args.file.unwrap() } else { String::from("-") };
    
            cmd_aes192(aes192_args.decrypt, &aes192_args.key, &iv, &file, &output)?;
        },

        VeilCommand::Aes256(aes256_args) => {
            let iv = if aes256_args.iv.is_some() { aes256_args.iv.unwrap() } else { String::new() };
            let output: String = if aes256_args.output.is_some() { aes256_args.output.unwrap() } else { String::from("-") };
            let file: String = if aes256_args.file.is_some() { aes256_args.file.unwrap() } else { String::from("-") };
    
            cmd_aes256(aes256_args.decrypt, &aes256_args.key, &iv, &file, &output)?;
        },

        VeilCommand::Rc4(rc4_args) => {
            let skip: usize = if rc4_args.skip.is_some() { rc4_args.skip.unwrap() } else { 0 };
            let output: String = if rc4_args.output.is_some() { rc4_args.output.unwrap() } else { String::from("-") };
            let file: String = if rc4_args.file.is_some() { rc4_args.file.unwrap() } else { String::from("-") };

            cmd_rc4(&rc4_args.key, skip, &file, &output)?;
        }
    }

    Ok(0)
}

fn main() {
	let ret: i32;
    let cli_args = VeilArgs::parse();

	match _main(cli_args) {
		Err(e) => {
			print_error(&format!("{}", e));
			ret = 1;
		},

		Ok(r) => {
			ret = r;
		}
	}

	exit(ret);
}
