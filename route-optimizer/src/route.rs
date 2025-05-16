use derive_more::IntoIterator;

use color_eyre::eyre;

use std::collections::HashSet;

use crate::{
    config,
    system::System,
};



#[derive(Clone, IntoIterator)]
pub struct Route {
    #[into_iterator(owned, ref, ref_mut)]
    inner: HashSet<System>,
}

#[derive(clap::ValueEnum, Clone)]
pub enum RouteOption {
    Fastest,
    Highsec,
    LowNull,
}

impl Route {
    fn new(s: &str) -> Route {
        let mut b = HashSet::new();
        for system in s.split(config::ROUTE_SPLIT_CHAR).map(System::new) {
            b.insert(system);
        }
        
        Route {
            inner: b
        }
    }
}

impl std::str::FromStr for Route {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Route::new(s))
    }
}
