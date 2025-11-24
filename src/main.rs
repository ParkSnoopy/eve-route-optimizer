mod config;

mod cli;
mod request;

mod route;
mod system;

mod progress;
#[allow(unused)]
mod trace;

use std::sync::{
    LazyLock,
    RwLock,
};

use system::{
    SystemHolder,
    SystemPair,
};
use request::{
    make_url,
    parse_text_into_length,
};
use futures::{
    stream,
    StreamExt,
};
use reqwest::Client;
use clap::Parser;
use nu_ansi_term::Color;



pub static CLI_ARGS: LazyLock<RwLock<cli::Args>> =
    LazyLock::new(|| RwLock::new(cli::Args::parse()));

pub static SYSTEM_HOLDER: LazyLock<RwLock<SystemHolder>> =
    LazyLock::new(|| RwLock::new(SystemHolder::from_cli_args(CLI_ARGS.read().unwrap())));



pub static PROGRESS_HOLDER: LazyLock<RwLock<progress::ProgressHolder>> =
    LazyLock::new(|| RwLock::new(progress::ProgressHolder::new()));

pub static REQUEST_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::builder()
        .cookie_store(true)
        .user_agent(config::USER_AGENT)
        .build()
        .unwrap()
});



#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    #[cfg(all(target_family = "windows"))]
    enable_ansi_support::enable_ansi_support()?;
    color_eyre::install()?;

    let system_pairs: Vec<SystemPair> = SYSTEM_HOLDER
        .read()
        .unwrap()
        .all_inter_systems_iter()
        .collect(); //.map(|system_pair| make_url(&system_pair)).collect();

    println!();
    trace::info("Fetching distances between system...");
    println!();

    let bodies = stream::iter(system_pairs)
        .map(|system_pair| {
            let client = &REQUEST_CLIENT;
            async move {
                let url = make_url(
                    &system_pair,
                    route::avoid_system_as_url_string(&CLI_ARGS.read().unwrap().avoid),
                );
                let resp = client.get(url).send().await.unwrap();
                (system_pair, resp.text().await)
            }
        })
        .buffer_unordered(CLI_ARGS.read().unwrap().concurrent);

    bodies
        .for_each(|(system_pair, resp_text_result)| {
            async move {
                match resp_text_result {
                    Ok(resp_text) => {
                        let distance = parse_text_into_length(&resp_text);
                        system_pair.set_distance(distance).unwrap();

                        trace::ok(format!(
                            "Setting distance '{}' between system {}",
                            Color::LightCyan.paint(distance.to_string()),
                            Color::Fixed(118).paint(system_pair.to_string()),
                        ));
                    }
                    Err(e) => {
                        trace::error(format!("Error while processing request"));
                        trace::debug(e.to_string());
                    }
                }
            }
        })
        .await;

    println!();

    trace::ok("Information fetch complete!");

    println!();

    trace::info("Start to build Shortest Path...");

    let calculation_count: u128 = SYSTEM_HOLDER
        .read()
        .unwrap()
        .permutation_size_hint()
        .unwrap_or(u128::MAX);
    PROGRESS_HOLDER
        .write()
        .unwrap()
        .set_total(calculation_count);
    trace::info(format!("'{}' Calculation(s) to process", calculation_count));

    println!();
    println!();
    println!();

    let feedback_step: usize = std::cmp::min(
        1_000_000,
        std::cmp::max(1, calculation_count / 200) as usize,
    );
    let current_shortest = SYSTEM_HOLDER
        .read()
        .unwrap()
        .build_shortest_path(feedback_step);

    current_shortest.read().unwrap().report_stdout();

    Ok(())
}
