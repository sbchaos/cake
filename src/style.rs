pub fn bold(text: &str) -> String {
    format!("\x1B[00;1m{}\x1B[0m", text)
}

pub fn red(text: &str) -> String {
    format!("\x1B[00;31m{}\x1B[0m", text)
}

pub fn yellow(text: &str) -> String {
    format!("\x1B[00;33m{}\x1B[0m", text)
}

pub fn green(text: &str) -> String {
    format!("\x1B[00;32m{}\x1B[0m", text)
}
