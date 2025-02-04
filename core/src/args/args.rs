use crate::{
    config,
    route::{ Route, RouteOption },
    system::System,
};



#[derive(Debug)]
pub struct Args {
    // specific separator (,:) separated system names 
    // ex: Jita,Amarr,Hek,BKG-Q2,SI-I89
    pub route: Route,

    // system to start route
    pub start: System,

    // system to end route
    pub end: Option<System>,

    // route option (one of `fastest` `highsec` `low-null`)
    pub route_option: RouteOption,

    // concurrent fetches (too high may blocked by DOTLAN)
    pub concurrent: usize,
}

impl Args {
    pub fn builder() -> ArgBuilder {
        ArgBuilder::new()
    }
}
