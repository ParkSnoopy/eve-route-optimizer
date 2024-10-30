use itertools::Itertools;

use crate::{
    utils,
    state::GlobalState,
    types::Route,
};

use std::iter::repeat;

use nu_ansi_term::Color;



fn prettify_route(global_state: &GlobalState, route: &Route) -> String {
    format!("{}  {}  {}  {}  {}",
        prettify_route_prefix(global_state),
        Color::LightYellow.paint(">>>"),
        route.iter().map(|r| Color::LightCyan.paint(r)).join(&Color::DarkGray.paint(" -> ").to_string()),
        Color::LightYellow.paint(">>>"),
        prettify_route_postfix(global_state),
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
        println!( "  {}", prettify_route(global_state, route) );
    }
    println!();
    println!("  ------------------------------{postfix_bar}");
    println!();
}

fn prettify_route_prefix(global_state: &GlobalState) -> String {
    match global_state.cli_args.start.as_str() {
        "" => "".to_string(),
        _  => format!("{}  {}",
            Color::LightYellow.paint(">>>"),
            Color::LightBlue.paint(&global_state.cli_args.start),
        ),
    }
}

fn prettify_route_postfix(global_state: &GlobalState) -> String {
    match global_state.cli_args.start.as_str() {
        "" => "".to_string(),
        _  => format!("{}  {}",
            Color::LightBlue.paint(&global_state.cli_args.end),
            Color::LightYellow.paint(">>>"),
        ),
    }
}
