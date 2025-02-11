use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct VeilArgs {
    #[clap(subcommand)]
    pub command: VeilCommand,
}

#[derive(Debug, Subcommand)]
#[command(subcommand_help_heading = "Bitwise Operations")]
pub enum VeilCommandBitWise {
    /// Apply a bitwise AND mask on a file (or stdin)
    And(VeilCommandAnd),
    /// Apply a bitwise OR mask on a file (or stdin)
    Or(VeilCommandOr),
    /// Apply a bitwise XOR mask on a file (or stdin)
    Xor(VeilCommandXor),
}

#[derive(Debug, Subcommand)]
#[command(subcommand_help_heading = "Encoding Schemes")]
pub enum VeilCommandEncoding {
    /// Encode or decode a file (or stdin) in hex
    Hex(VeilCommandHex),
    /// Encode or decode a file (or stdin) in Base32
    Base32(VeilCommandBase32),
    /// Encode or decode a file (or stdin) in Base64
    Base64(VeilCommandBase64),
    /// Encode or decode a file (or stdin) in Base85
    Base85(VeilCommandBase85),
    /// Encode or decode a file (or stdin) in ASCII85
    Ascii85(VeilCommandAscii85),
}

#[derive(Debug, Subcommand)]
#[command(subcommand_help_heading = "Cryptographic Ciphers")]
pub enum VeilCommandCipher {
    /// Encrypt or decrypt a file (or stdin) using AES128 in CBC mode
    Aes128(VeilCommandAes128),
    /// Encrypt or decrypt a file (or stdin) using AES192 in CBC mode
    Aes192(VeilCommandAes192),
    /// Encrypt or decrypt a file (or stdin) using AES256 in CBC mode
    Aes256(VeilCommandAes256),
    /// Encrypt or decrypt a file (or stdin) using RC4
    Rc4(VeilCommandRc4),
    /// Encrypt or decrypt a file (or stdin) using ChaCha20
    Chacha20(VeilCommandChacha20),
}

#[derive(Debug, Subcommand)]
pub enum VeilCommand {
    /// Apply a bitwise AND mask on a file (or stdin)
    And(VeilCommandAnd),
    /// Apply a bitwise OR mask on a file (or stdin)
    Or(VeilCommandOr),
    /// Apply a bitwise XOR mask on a file (or stdin)
    Xor(VeilCommandXor),

    /// Encode or decode a file (or stdin) in hex
    Hex(VeilCommandHex),
    /// Encode or decode a file (or stdin) in Base32
    Base32(VeilCommandBase32),
    /// Encode or decode a file (or stdin) in Base64
    Base64(VeilCommandBase64),
    /// Encode or decode a file (or stdin) in Base85
    Base85(VeilCommandBase85),
    /// Encode or decode a file (or stdin) in ASCII85
    Ascii85(VeilCommandAscii85),

    /// Encrypt or decrypt a file (or stdin) using AES128 in CBC mode
    Aes128(VeilCommandAes128),
    /// Encrypt or decrypt a file (or stdin) using AES192 in CBC mode
    Aes192(VeilCommandAes192),
    /// Encrypt or decrypt a file (or stdin) using AES256 in CBC mode
    Aes256(VeilCommandAes256),
    /// Encrypt or decrypt a file (or stdin) using RC4
    Rc4(VeilCommandRc4),
    /// Encrypt or decrypt a file (or stdin) using ChaCha20
    Chacha20(VeilCommandChacha20),
}

#[derive(Debug, Args)]
pub struct VeilCommandAnd {
    #[clap(short, long, required = true, help = "Byte mask (hex)")]
    pub mask: String,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandOr {
    #[clap(short, long, required = true, help = "Byte mask (hex)")]
    pub mask: String,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandXor {
    #[clap(short, long, required = true, help = "Byte mask (hex)")]
    pub mask: String,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandHex {
    #[clap(short, long, required = false, help = "Decode instead of encoding")]
    pub decode: bool,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandBase32 {
    #[clap(short, long, required = false, help = "Decode instead of encoding")]
    pub decode: bool,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandBase64 {
    #[clap(short, long, required = false, help = "Decode instead of encoding")]
    pub decode: bool,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandBase85 {
    #[clap(short, long, required = false, help = "Decode instead of encoding")]
    pub decode: bool,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandAscii85 {
    #[clap(short, long, required = false, help = "Decode instead of encoding")]
    pub decode: bool,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandAes128 {
    #[clap(short, long, required = false, help = "Decrypt instead of encrypting")]
    pub decrypt: bool,
    #[clap(short, long, required = true, help = "Encryption key (hex, 16 bytes)")]
    pub key: String,
    #[clap(
        short,
        long,
        required = false,
        help = "Specify initialization vector instead of reading or generating it (hex, 16 bytes)"
    )]
    pub iv: Option<String>,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandAes192 {
    #[clap(short, long, required = false, help = "Decrypt instead of encrypting")]
    pub decrypt: bool,
    #[clap(short, long, required = true, help = "Encryption key (hex, 24 bytes)")]
    pub key: String,
    #[clap(
        short,
        long,
        required = false,
        help = "Specify initialization vector instead of reading or generating it (hex, 16 bytes)"
    )]
    pub iv: Option<String>,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandAes256 {
    #[clap(short, long, required = false, help = "Decrypt instead of encrypting")]
    pub decrypt: bool,
    #[clap(short, long, required = true, help = "Encryption key (hex, 24 bytes)")]
    pub key: String,
    #[clap(
        short,
        long,
        required = false,
        help = "Specify initialization vector instead of reading or generating it (hex, 16 bytes)"
    )]
    pub iv: Option<String>,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandRc4 {
    #[clap(short, long, required = true, help = "Encryption key (hex)")]
    pub key: String,
    #[clap(
        short,
        long,
        required = false,
        help = "Skip the first [skip] stream bytes"
    )]
    pub skip: Option<usize>,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}

#[derive(Debug, Args)]
pub struct VeilCommandChacha20 {
    #[clap(short, long, required = true, help = "Encryption key (32 bytes, hex)")]
    pub key: String,
    #[clap(
        short,
        long,
        required = false,
        help = "Encryption nonce (12 bytes, hex)"
    )]
    pub nonce: Option<String>,
    #[clap(
        short,
        long,
        required = false,
        help = "Skip the first [skip] stream bytes"
    )]
    pub skip: Option<usize>,
    #[clap(short, long, required = false, help = "Write output to a file")]
    pub output: Option<String>,

    #[clap(required = false, help = "Read input from a file")]
    pub file: Option<String>,
}
