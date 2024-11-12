pub fn ok<S: AsRef<str>>(msg: S) {
    println!("  {}", ok!(msg));
}

pub fn info<S: AsRef<str>>(msg: S) {
    println!("  {}", info!(msg));
}

pub fn warn<S: AsRef<str>>(msg: S) {
    println!("  {}", warn!(msg));
}

pub fn debug<S: AsRef<str>>(msg: S) {
    println!("  {}", debug!(msg));
}

pub fn error<S: AsRef<str>>(msg: S) {
    println!("  {}", error!(msg));
}
