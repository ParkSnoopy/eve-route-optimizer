use nu_ansi_term::Color;



pub fn ok<S: AsRef<str>>(msg: S) {
    println!("  {} {}",
        Color::Fixed(082).paint("[  O K  ]"),
        Color::Fixed(255).paint(msg.as_ref()),
    );
}

pub fn info<S: AsRef<str>>(msg: S) {
    println!("  {} {}",
        Color::Fixed(051).paint("[  INF  ]"),
        Color::Fixed(255).paint(msg.as_ref()),
    );
}

pub fn warn<S: AsRef<str>>(msg: S) {
    println!("  {} {}",
        Color::Fixed(172).paint("[  WRN  ]"),
        Color::Fixed(255).paint(msg.as_ref()),
    );
}

pub fn debug<S: AsRef<str>>(msg: S) {
    println!("  {} {}",
        Color::Fixed(226).paint("[  DBG  ]"),
        Color::Fixed(255).paint(msg.as_ref()),
    );
}

pub fn error<S: AsRef<str>>(msg: S) {
    println!("  {} {}",
        Color::Fixed(009).paint("[  ERR  ]"),
        Color::Fixed(226).paint(msg.as_ref()),
    );
}
