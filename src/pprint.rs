use itertools::Itertools;

use crate::{
    utils,
    state::GlobalState,
    types::Route,
};

use std::iter::repeat;

use nu_ansi_term::Color;



fn prettify_route(route: &Route) -> String {
    format!("{}  {}  {}",
        nu_ansi_term::Color::LightYellow.paint(">>>"),
        route.iter().map(|r| Color::LightCyan.paint(r)).join(&Color::DarkGray.paint(" -> ").to_string()),
        nu_ansi_term::Color::LightYellow.paint(">>>"),
    )
}

pub fn route_summary(global_state: &GlobalState) {
    let postfix_bar: String = repeat('-').take( utils::get_term_width() - 40 ).collect();
    let curr_shortest_lock = global_state.curr_shortest.read().unwrap();

    println!();
    println!();
    println!();
    println!("  Shortest Route Length is '{}'", Color::LightCyan.paint(curr_shortest_lock.length.to_string()));
    println!();
    println!("  ----------[ ROUTES ]----------{postfix_bar}");
    println!();
    for route in curr_shortest_lock.routes.iter() {
        println!( "    {}", prettify_route(route) );
    }
    println!();
    println!("  ------------------------------{postfix_bar}");
    println!();
}
