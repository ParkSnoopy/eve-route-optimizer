macro_rules! ok {
    ($msg:expr) => {
        format!(
            "{} {}",
            nu_ansi_term::Color::Fixed(082).paint("[  O K  ]"),
            $msg.as_ref() //nu_ansi_term::Color::Fixed(255).paint($msg.as_ref()),
        )
    };
}

macro_rules! info {
    ($msg:expr) => {
        format!(
            "{} {}",
            nu_ansi_term::Color::Fixed(051).paint("[  INF  ]"),
            $msg.as_ref() //nu_ansi_term::Color::Fixed(255).paint($msg.as_ref()),
        )
    };
}

macro_rules! warn {
    ($msg:expr) => {
        format!(
            "{} {}",
            nu_ansi_term::Color::Fixed(172).paint("[  WRN  ]"),
            $msg.as_ref() //nu_ansi_term::Color::Fixed(255).paint($msg.as_ref()),
        )
    };
}

macro_rules! debug {
    ($msg:expr) => {
        format!(
            "{} {}",
            nu_ansi_term::Color::Fixed(226).paint("[  DBG  ]"),
            $msg.as_ref() //nu_ansi_term::Color::Fixed(255).paint($msg.as_ref()),
        )
    };
}

macro_rules! error {
    ($msg:expr) => {
        format!(
            "{} {}",
            nu_ansi_term::Color::Fixed(196).paint("[  ERR  ]"),
            nu_ansi_term::Color::Fixed(226).paint($msg.as_ref()),
        )
    };
}
