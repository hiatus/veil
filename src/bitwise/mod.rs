use std::io::Error;

mod operation;

pub fn cmd_and(mask: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
    operation::and(mask, path_in, path_out)
}

pub fn cmd_or(mask: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
    operation::or(mask, path_in, path_out)
}

pub fn cmd_xor(mask: &str, path_in: &str, path_out: &str) -> Result<bool, Error> {
    operation::xor(mask, path_in, path_out)
}
