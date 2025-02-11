use colored::Colorize;

#[allow(dead_code)]
pub fn print_info(s: &str) {
    println!("{}{}{} {}", "[".bright_black(), "*".white(), "]".bright_black(), s);
}

#[allow(dead_code)]
pub fn print_message(s: &str) {
    println!("{}{}{} {}", "[".bright_black(), "+".bright_cyan(), "]".bright_black(), s);
}

#[allow(dead_code)]
pub fn print_error(s: &str) {
    println!("{}{}{} {}", "[".bright_black(), "!".bright_red(), "]".bright_black(), s);
}
