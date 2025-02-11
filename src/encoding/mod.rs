use std::io::Error;

// Local
mod ascii85;
mod base32;
mod base64;
mod base85;
mod hex;

pub fn cmd_hex(decode: bool, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return if decode {
        hex::decode(path_in, path_out)
    } else {
        hex::encode(path_in, path_out)
    };
}

pub fn cmd_base32(decode: bool, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return if decode {
        base32::decode(path_in, path_out)
    } else {
        base32::encode(path_in, path_out)
    };
}

pub fn cmd_base64(decode: bool, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return if decode {
        base64::decode(path_in, path_out)
    } else {
        base64::encode(path_in, path_out)
    };
}

pub fn cmd_base85(decode: bool, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return if decode {
        base85::decode(path_in, path_out)
    } else {
        base85::encode(path_in, path_out)
    };
}

pub fn cmd_ascii85(decode: bool, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return if decode {
        ascii85::decode(path_in, path_out)
    } else {
        ascii85::encode(path_in, path_out)
    };
}
