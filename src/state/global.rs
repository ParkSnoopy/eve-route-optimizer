// Trait for `<Vec>.permutations(k)` and calculate its length
use itertools::Itertools;
use factorial::Factorial;

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
    pub parsed_route: Route,

    pub req_client: reqwest::Client,

    pub fetch_state: Arc<RwLock<FetchState>>,
    pub curr_shortest: Arc<RwLock<CurrentShortest>>,

    //pub all_routes_iter: Box<dyn Iterator<Item=Route>>,
}

impl GlobalState {
    pub fn with_init(args: cli::Args, client: reqwest::Client) -> GlobalState {
        GlobalState {
            parsed_route: args.route.split(config::ROUTE_SPLIT_CHAR).map(String::from).collect(),
            cli_args: args,

            req_client: client,

            fetch_state: Arc::new(RwLock::new(FetchState::new())),
            curr_shortest: Arc::new(RwLock::new(CurrentShortest {
                routes: Vec::new(),
                length: usize::MAX,
            })),
        }
    }

    pub fn all_routes_iter(&self) -> impl Iterator<Item=Route> {
        self.fetch_state.write().unwrap().set_total(
            match self.parsed_route.len().checked_factorial() /* which is length of permutation iterator */ {
                Some(total) => total,
                None        => usize::MAX,
            }
        );

        //trace::debug(format!("{:?}", self.parsed_route));

        //let temp: Vec<Route> = self.parsed_route.clone().into_iter().permutations(self.parsed_route.len()).collect();
        //trace::debug(format!("{:?}", temp));

        self.parsed_route.clone().into_iter().permutations(self.parsed_route.len())
    }
}
