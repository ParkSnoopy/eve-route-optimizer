// Trait for async `<futures::stream::Iter>.for_each(...)`
use futures::StreamExt;

// Trait for direct `cli::Args::parse()` instead `cli::Args::try_parse().unwrap()`
use clap::Parser;

// attach modules
mod cli;
mod config;
mod pprint;
mod types;
mod utils;
mod state;

// debuging
#[allow(unused)]
mod trace;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init();
    use std::sync::{ Arc };
    use state::global::GlobalState;

    let global_state: Arc<GlobalState> = Arc::new(GlobalState::with_init(
        cli::Args::parse(),
        reqwest::Client::builder()
            .cookie_store(true)
            .user_agent(config::USER_AGENT)
            .build()?
    ));

    // route_length_tuples: impl Iterator<(route_length:usize, route:Route)>
    let route_length_tuples = futures::stream::iter(
        global_state.all_routes_iter().map(|route| {
            {
                let global_state = global_state.clone();

                async move {
                    global_state.fetch_state.write().unwrap().add_fetching();

                    match utils::get_route_length(&global_state, &route).await {
                        Ok(route_length_tuple) => {
                            global_state.fetch_state.write().unwrap().add_success();
                            route_length_tuple
                        },
                        Err(_error) => {
                            global_state.fetch_state.write().unwrap().add_failure();
                            (usize::MAX, Vec::new())
                        },
                    }
                }
            }
        })
    ).buffer_unordered(global_state.cli_args.concurrent);

    print!("\n\n{}", global_state.fetch_state.read().unwrap().describe());

    let _ = route_length_tuples
        .for_each(|length_tuple| {
            let global_state = global_state.clone();
            let fetch_state   = global_state.fetch_state.clone();
            let curr_shortest = global_state.curr_shortest.clone();

            async move {
                let (route_length, route) = length_tuple;

                let mut curr_shortest_lock = curr_shortest.write().unwrap();

                if route_length < curr_shortest_lock.length {

                    curr_shortest_lock.routes.clear();
                    curr_shortest_lock.routes.push(route.to_owned());
                    curr_shortest_lock.length = route_length;

                } else if route_length == curr_shortest_lock.length {

                    curr_shortest_lock.routes.push(route.to_owned());

                } else {

                }

                print!("{}", fetch_state.read().unwrap().describe());
            }
        }).await;

    pprint::route_summary(&global_state);

    Ok(())
}

fn init() {
    print!("\n\n\n");
    #[cfg(target_os = "windows")]
    {
        enable_ansi_support::enable_ansi_support().unwrap();
    }
}
