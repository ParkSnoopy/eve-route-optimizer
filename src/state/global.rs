// Trait for `<Vec>.permutations(k)`
use itertools::Itertools;

use super::fetch::FetchState;

use crate::{
    config,
    cli,
    types::Route,
};

use std::sync::{ Arc, RwLock };



pub struct CurrentShortest{
    pub routes: Vec<Route>,
    pub length: usize,
}

pub struct GlobalState {
    pub cli_args: cli::Args,
    pub req_client: reqwest::Client,
    pub fetch_state: Arc<RwLock<FetchState>>,
    pub curr_shortest: Arc<RwLock<CurrentShortest>>,

    pub all_routes_iter: Box<dyn Iterator<Item=Route>>,
}

impl GlobalState {
    pub fn with_init(args: cli::Args, client: reqwest::Client) -> GlobalState {
        let mut gs = GlobalState {
            cli_args: args,
            req_client: client,
            fetch_state: Arc::new(RwLock::new(FetchState::new())),
            curr_shortest: Arc::new(RwLock::new(CurrentShortest {
                routes: Vec::new(),
                length: usize::MAX,
            })),

            all_routes_iter: Box::new(Vec::new().into_iter()),
        };
        let mut all_routes_iter = Box::new(Vec::new());
        gs.init(&mut all_routes_iter);
        gs
    }

    fn init(&mut self, all_routes_iter: &mut Box<dyn Iterator<Item=Route>>) {
        let route_split_iter = self.cli_args.route.split(config::ROUTE_SPLIT_CHAR);
        let route_count = route_split_iter.size_hint().0;

        *all_routes_iter = Box::new(
            route_split_iter
                .map(String::from)
                .permutations(route_count)
        );
        self.fetch_state.write().unwrap().set_total(all_routes_iter.size_hint().0);
    }
}
