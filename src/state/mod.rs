// Trait for `<Vec>.permutations(k)`
use itertools::Itertools;

mod fetch;
use fetch::FetchState;

use crate::{
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

    pub all_routes: Vec<Route>,
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

            all_routes: Vec::new(),
        };
        gs.init();
        gs
    }

    fn init(&mut self) {
        let routes: Vec<&str> = self.cli_args.route.split(':').collect();

        self.all_routes = routes
            .iter()
            .permutations(routes.len())
            .map(|route| {
                route
                    .into_iter()
                    .map(|s| { s.to_string() })
                    .collect()
            }).collect();

        self.fetch_state.write().unwrap().set_total(self.all_routes.len());
    }
}
