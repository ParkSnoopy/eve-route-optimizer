pub fn clear() {
    print!("\r{}", ansi_escapes::EraseDown);
}

pub fn ok<S: AsRef<str>>(msg: S) {
    print!("{}{}  {}\n", ansi_escapes::CursorUp(1), ansi_escapes::EraseDown, ok!(msg));
}

pub fn info<S: AsRef<str>>(msg: S) {
    print!("{}{}  {}\n", ansi_escapes::CursorUp(1), ansi_escapes::EraseDown, info!(msg));
}

pub fn warn<S: AsRef<str>>(msg: S) {
    print!("{}{}  {}\n", ansi_escapes::CursorUp(1), ansi_escapes::EraseDown, warn!(msg));
}

pub fn debug<S: AsRef<str>>(msg: S) {
    print!("{}{}  {}\n", ansi_escapes::CursorUp(1), ansi_escapes::EraseDown, debug!(msg));
}

pub fn error<S: AsRef<str>>(msg: S) {
    print!("{}{}  {}\n", ansi_escapes::CursorUp(1), ansi_escapes::EraseDown, error!(msg));
}
