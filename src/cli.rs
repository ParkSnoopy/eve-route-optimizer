use crate::{
    config,
    types::RouteOption,
};

use clap::Parser;



#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub route: String,

    #[arg(value_enum, long, default_value_t=RouteOption::Fastest)]
    pub route_option: RouteOption,

    #[arg(short, long, default_value_t=config::DEFAULT_PARAREL_REQUEST)]
    pub concurrent: usize,
}
