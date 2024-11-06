use clap::Parser;

use crate::{
    config,
    route::{ Route, RouteOption },
    system::System,
};



#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    // specific separator (,:) separated system names 
    // ex: Jita,Amarr,Hek,BKG-Q2,SI-I89
    #[arg(short, long, value_parser = clap::value_parser!(Route))]
    pub route: Route,

    // system to start route
    #[arg(short, long, value_parser = clap::value_parser!(System))]
    pub start: System,

    // system to end route
    #[arg(short, long, value_parser = clap::value_parser!(System))]
    pub end: Option<System>,

    // route option (one of `fastest` `highsec` `low-null`)
    #[arg(short = 'o', long, value_enum, default_value_t=RouteOption::Fastest)]
    pub route_option: RouteOption,

    // concurrent fetches (too high may blocked by DOTLAN)
    #[arg(short, long, default_value_t=config::DEFAULT_PARAREL_REQUEST)]
    pub concurrent: usize,
}
