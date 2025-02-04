//use std::sync::Arc;

/*use crate::{
    system::System,
};*/

use std::sync::{ Arc, RwLock };

use super::route::Route;



#[derive(Clone)]
pub struct CurrentShortest {
    routes: Vec< Arc<RwLock< Route >> >,
    length: u64,
}

#[derive(Debug)]
pub struct StringCurrentShortest {
    pub routes: Vec<Vec<String>>,
    pub length: u64,
}

impl CurrentShortest {
    pub fn new() -> CurrentShortest {
        CurrentShortest {
            routes: Vec::new(),
            length: u64::MAX,
        }
    }

    pub fn len(&self) -> u64 {
        self.length
    }

    pub fn inner(&self) -> (u64, Vec< Arc<RwLock< Route >> >) {
        (self.length, self.routes.clone())
    }

    pub fn weak_check_registerable(&self, length: u64) -> bool {
        length <= self.length
    }

    pub fn register(&mut self, sync_route: &Arc<RwLock< Route >>, length: u64) {
        if length < self.length {
            self.routes.clear();
            self.routes.push(sync_route.clone());
            self.length = length;
        } else if length == self.length {
            self.routes.push(sync_route.clone());
        }
    }

    pub fn to_named(&self) -> StringCurrentShortest {
        StringCurrentShortest {
            length: self.length,
            routes: self.routes
                .iter()
                .map(|sync_route| sync_route
                    .into_iter()
                    .map(|sync_system|
                        sync_system
                            .read()
                            .unwrap()
                            .name()
                            .clone()
                    )
                    .collect()
                )
                .collect()
        }
    }
}
