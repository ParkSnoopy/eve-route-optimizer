use futures_util::{ StreamExt as _ };

use reqwest::Client;
use crossbeam::{
    channel::{
        unbounded,
        Sender,
        Receiver,
        SendError,
    },
};

use std::sync::{ RwLock, RwLockReadGuard, RwLockWriteGuard };

use crate::{
    config,
    args::Args,
    system::{
        SystemHolder,
        CurrentShortest,
    },
    progress::{
        ProgressHolder,
    },
    channel::ManagerResponse,
};



pub struct RouteOptimizeManager {
    client: Client,
    cli_args: Args,
    system_holder: RwLock<SystemHolder>,
    progress_holder: RwLock<ProgressHolder>,

    sender: Sender<ManagerResponse>
}

impl RouteOptimizeManager {
    pub fn with_args(args: Args) -> (Receiver<ManagerResponse>, RouteOptimizeManager) {
        let (sender, receiver) = unbounded();

        (
            receiver,
            Self {
                client: Client::builder()
                    .user_agent(config::USER_AGENT)
                    .brotli(true)
                    .deflate(true)
                    .gzip(true)
                    .zstd(true)
                    .build()
                    .unwrap(),
                
                cli_args: args,
                system_holder: RwLock::new(SystemHolder::new()),
                progress_holder: RwLock::new(ProgressHolder::with_sender(sender.clone())),

                sender
            },
        )
    }
}

impl RouteOptimizeManager {
    pub fn report_msg(&self, msg: impl AsRef<str>) -> Result<(), SendError<ManagerResponse>> {
        self.sender.send( ManagerResponse::Msg(
            msg.as_ref().to_string()
        ))
    }
    pub fn report_err(&self, err: impl AsRef<str>) -> Result<(), SendError<ManagerResponse>> {
        self.sender.send( ManagerResponse::Err(
            err.as_ref().to_string()
        ))
    }

    fn client(&self) -> &Client {
        &self.client
    }
    pub fn args(&self) -> &Args {
        &self.cli_args
    }

    fn system_holder_wlock(&self) -> RwLockWriteGuard<SystemHolder> {
        self.system_holder.write().unwrap()
    }
    pub fn system_holder_rlock(&self) -> RwLockReadGuard<SystemHolder> {
        self.system_holder.read().unwrap()
    }
    pub fn progress_holder_wlock(&self) -> RwLockWriteGuard<ProgressHolder> {
        self.progress_holder.write().unwrap()
    }
    fn _progress_holder_rlock(&self) -> RwLockReadGuard<ProgressHolder> {
        self.progress_holder.read().unwrap()
    }

    pub async fn run(&self) -> eyre::Result<()> {
        self.system_holder_wlock().register_route(&self.args().route);
        self.system_holder_wlock().register_system(&self.args().start);

        if self.args().end.is_some() {
            self.system_holder_wlock().register_system(
                &self.args().end.clone().unwrap()
            );
        }

        let system_pairs = self.system_holder.write().unwrap().all_inter_systems_iter();

        self.report_msg("Fetching distances between system...")?;
        let responses = futures_util::stream::iter(system_pairs)
            .map(|system_pair| async {
                let url = self.make_url(&system_pair);
                let resp = self.client().get(url).send().await;
                (system_pair, resp)
            })
            .buffer_unordered(self.args().concurrent);
        responses
            .for_each(|(system_pair, resp_result)| 
                async move {
                    match resp_result {
                        Err(e) => {
                            self.report_err(format!("Fetch failed: {e}")).unwrap();
                        },
                        Ok(response) => {
                            let text = response.text().await.unwrap();
                            let distance = self.parse_text_into_length(&text);
                            if let Err(e) = system_pair.set_distance(distance) {
                                self.report_err(format!("Failed to set distance between {} : {e}", system_pair.try_to_string().unwrap())).unwrap();
                            };
                        },
                    };
                }
            )
            .await;

        self.report_msg("Fetch complete!")?;
        self.report_msg("Start to build Shortest Path...")?;

        let calculation_count: u128 = self.system_holder_rlock().permutation_size_hint().unwrap_or(u128::MAX);
        self.progress_holder_wlock().set_total(calculation_count);
        self.report_msg(format!("'{}' Calculation(s) to process", calculation_count))?;

        let feedback_step: usize = std::cmp::min(1_000_000, std::cmp::max(1, calculation_count/200) as usize);
        let current_shortest: CurrentShortest = self.system_holder_wlock().build_shortest_path( self, feedback_step )?;
        self.report_msg("Shortest path build success!")?;

        self.sender.send(ManagerResponse::Ok(
            current_shortest
        )).unwrap();

        Ok(())
    }
}
