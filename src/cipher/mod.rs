use std::io::Error;

// Local
mod aes;
mod rc4;
mod chacha;


pub fn cmd_aes128(decrypt: bool, key: &str, iv: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return if decrypt {
        aes::decrypt_128(key, iv, path_in, path_out)
    }
    else {
        aes::encrypt_128(key, iv, path_in, path_out)
    };
}

pub fn cmd_aes192(decrypt: bool, key: &str, iv: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return if decrypt {
        aes::decrypt_192(key, iv, path_in, path_out)
    }
    else {
        aes::encrypt_192(key, iv, path_in, path_out)
    };
}

pub fn cmd_aes256(decrypt: bool, key: &str, iv: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return if decrypt {
        aes::decrypt_256(key, iv, path_in, path_out)
    }
    else {
        aes::encrypt_256(key, iv, path_in, path_out)
    };
}

pub fn cmd_rc4(key: &str, skip: usize, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return rc4::encrypt(key, skip, path_in, path_out);
}

pub fn cmd_chacha20(key: &str, nonce: &str, skip: usize, path_in: &str, path_out: &str) -> Result<bool, Error> {
    return chacha::encrypt_20(key, nonce, skip, path_in, path_out);
}
