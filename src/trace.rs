use nu_ansi_term::Color;



fn before() {
    print!("{}{}", "\n\n\n\n", ansi_escapes::EraseDown);
}
fn after() {
    print!("{}", ansi_escapes::CursorUp(5));
}

pub fn clear() {
    before();
    println!();
    after();
}

pub fn info<S: AsRef<str>>(msg: S) {
    before();
    println!("{} {}",
        Color::Fixed(051).paint("[ INFO ]"),
        Color::Fixed(255).paint(msg.as_ref()),
    );
    after();
}

pub fn warn<S: AsRef<str>>(msg: S) {
    before();
    println!("{} {}",
        Color::Fixed(172).paint("[ WARN ]"),
        Color::Fixed(255).paint(msg.as_ref()),
    );
    after();
}

pub fn debug<S: AsRef<str>>(msg: S) {
    before();
    println!("{} {}",
        Color::Fixed(226).paint("[ DBUG ]"),
        Color::Fixed(255).paint(msg.as_ref()),
    );
    after();
}
