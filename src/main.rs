mod config;

mod cli;
mod request;

mod route;
mod system;

#[allow(unused)]
mod trace;
mod progress;
//mod bench;

use system::{ SystemPair, SystemHolder };
use request::{ make_url, parse_text_into_length };

use futures::{ stream, StreamExt };
use reqwest::Client;
use std::sync::{ LazyLock, RwLock };

use clap::Parser;
use nu_ansi_term::Color;



pub static PROGRESS_HOLDER: LazyLock<RwLock<progress::ProgressHolder>> = LazyLock::new(|| RwLock::new(
    progress::ProgressHolder::new()
));
pub static CLI_ARGS: LazyLock<RwLock<cli::Args>> = LazyLock::new(|| RwLock::new(
    cli::Args::parse()
));
pub static REQUEST_CLIENT: LazyLock<Client> = LazyLock::new(|| 
    Client::builder()
        .cookie_store(true)
        .user_agent(config::USER_AGENT)
        .build()
        .unwrap()
);
pub static SYSTEM_HOLDER: LazyLock<RwLock<SystemHolder>> = LazyLock::new(|| RwLock::new(
    SystemHolder::new()
));



#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    enable_ansi_support::enable_ansi_support()?;
    color_eyre::install()?;

    SYSTEM_HOLDER.write().unwrap().register_route(&CLI_ARGS.read().unwrap().route);
    SYSTEM_HOLDER.write().unwrap().register_system(&CLI_ARGS.read().unwrap().start);
    match &CLI_ARGS.read().unwrap().end {
        Some(system) => {
            SYSTEM_HOLDER.write().unwrap().register_system(&system);
        },
        _ => (),
    }

    let system_pairs: Vec<SystemPair> = SYSTEM_HOLDER.read().unwrap().all_inter_systems_iter().collect();//.map(|system_pair| make_url(&system_pair)).collect();

    let bodies = stream::iter(system_pairs)
        .map(|system_pair| {
            let client = &REQUEST_CLIENT;
            async move {
                trace::info(
                    format!("Sending request for: {}",
                        Color::Fixed(118).paint( system_pair.to_string() )
                    )
                );

                let url = make_url(&system_pair);
                let resp = client.get(url).send().await.unwrap();
                ( system_pair, resp.text().await )
            }
        })
        .buffer_unordered(CLI_ARGS.read().unwrap().concurrent);

    bodies
        .for_each(|(system_pair, resp_text_result)| async move {
            match resp_text_result {
                Ok(resp_text) => {
                    let distance = parse_text_into_length(&resp_text);
                    system_pair.set_distance(distance).unwrap();

                    trace::ok(
                        format!("Setting distance '{}' between system {}",
                            Color::LightCyan.paint( distance.to_string() ),
                            Color::Fixed(118).paint( system_pair.to_string() ),
                        )
                    );
                },
                Err(e) => {
                    trace::error(format!("Error while processing request"));
                    trace::debug(e.to_string());
                },
            }
        })
        .await;

    println!();
    trace::ok("Information fetch complete!");
    trace::info("Start to build Shortest Path...");

    let calculation_count: u128 = SYSTEM_HOLDER.read().unwrap().permutation_size_hint().unwrap_or(u128::MAX);
    PROGRESS_HOLDER.write().unwrap().set_total(calculation_count);
    trace::info(format!("'{}' Calculation(s) to process", calculation_count));
    println!();

    let feedback_step: usize = std::cmp::max(1, calculation_count/200) as usize;
    let current_shortest = SYSTEM_HOLDER.read().unwrap().build_shortest_path( feedback_step );

    current_shortest.report_stdout();

    Ok(())
}
