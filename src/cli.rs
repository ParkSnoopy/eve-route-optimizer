use crate::{
    config,
    types::RouteOption,
};

use clap::Parser;



#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    // `:` separated DOTLAN-formatted route. e.g.) `Jita:Amarr:Hek:BKG-Q2:SI-I89`
    #[arg(short, long)]
    pub route: String,

    // system to start route
    #[arg(short, long, default_value_t=String::from(""))]
    pub start: String,

    // system to end route
    #[arg(short, long, default_value_t=String::from(""))]
    pub end: String,

    // route option (one of `fastest` `highsec` `low-null`)
    #[arg(value_enum, long, default_value_t=RouteOption::Fastest)]
    pub route_option: RouteOption,

    // concurrent fetches (too high may blocked by DOTLAN)
    #[arg(short, long, default_value_t=config::DEFAULT_PARAREL_REQUEST)]
    pub concurrent: usize,
}
