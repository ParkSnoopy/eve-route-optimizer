use derive_more::IntoIterator;

use color_eyre::eyre;

use std::collections::HashSet;

use crate::{
    config,
    system::System,
};



#[derive(Clone, IntoIterator)]
#[derive(Debug)]
pub struct UnorderedRoute {
    #[into_iterator(owned, ref, ref_mut)]
    inner: HashSet<System>,
}

#[derive(clap::ValueEnum, Clone)]
#[derive(Debug)]
pub enum RouteOption {
    Fastest,
    Highsec,
    LowNull,
}

impl UnorderedRoute {
    fn new(s: &str) -> UnorderedRoute {
        let mut b = HashSet::new();
        for system in s.split(config::ROUTE_SPLIT_CHAR).map(System::new) {
            b.insert(system);
        }
        
        UnorderedRoute {
            inner: b
        }
    }
}

impl std::str::FromStr for UnorderedRoute {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UnorderedRoute::new(s))
    }
}

pub fn avoid_system_as_url_string(avoid: &Option<UnorderedRoute>) -> String {
    let mut s = String::new();

    if let Some(inner) = avoid {
        for system in inner {
            s = s + &format!(":-{}", system.name())
        }
    };

    s
}
