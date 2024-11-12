use super::base::*;



pub fn ok<S: AsRef<str>>(msg: S) -> String {
    ok!(msg)
}

pub fn info<S: AsRef<str>>(msg: S) -> String {
    info!(msg)
}

pub fn warn<S: AsRef<str>>(msg: S) -> String {
    warn!(msg)
}

pub fn debug<S: AsRef<str>>(msg: S) -> String {
    debug!(msg)
}

pub fn error<S: AsRef<str>>(msg: S) -> String {
    error!(msg)
}
