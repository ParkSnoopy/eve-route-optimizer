use color_eyre::eyre::{ OptionExt as _ };
use factorial::{ Factorial as _ };
use itertools::{ Itertools as _ };
use derive_more::IntoIterator;
use rayon::prelude::*;

use std::collections::{ HashMap };
use std::sync::{ Arc, RwLock };

use super::{
    system::System,
    pair::SystemPair,
};

use crate::{
    args::args::Args,
    route::{
        route::Route,
        shortest::CurrentShortest,
    },
};



#[derive(IntoIterator)]
pub struct SystemManager {
    #[into_iterator(owned, ref, ref_mut)]
    inner: HashMap<String, Arc<RwLock< System >>>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn get<S: AsRef<str>>(&self, system_name: S) -> &Arc<RwLock< System >> {
        self.inner.get(system_name.as_ref()).unwrap()
    }

    pub fn all_inter_systems_iter(&self) -> impl Iterator<Item=SystemPair> {
        self.inner.clone().into_values().combinations(2).map(SystemPair::from_vec)
    }

    pub fn register_system(&mut self, system: &System) {
        self.inner.insert(
            system.name().to_string(), 
            Arc::new(RwLock::new(system.clone()))
        );
    }

    pub fn register_route(&mut self, route: &Route) {
        for system in route {
            self.register_system(system);
        }
    }
}

impl SystemManager {
    pub fn permutation_size_hint(&self) -> Option<u128> {
        ((self.inner.len()-1) as u128).checked_factorial()
    }

    pub fn build_shortest_path(&self, args: &Args, feedback_step: usize) -> color_eyre::Result<CurrentShortest> {
        let system_from = self.get(
            args.start.name()
        );
        let system_to = match args.end {
            Some(system) => Some(self.get(system.name())),
            None => None,
        };

        let mut systems: Vec< Arc<RwLock<System>> > = self.inner.clone().into_values().collect();

        let system_from_index = systems
            .iter()
            .position(|ss| ss.read().unwrap().name() == system_from.read().unwrap().name())
            .unwrap();
        systems.remove(system_from_index);

        match system_to {
            Some(system_to) => {
                if system_to.read().unwrap().name() != system_from.read().unwrap().name() {
                    let system_to_index = systems
                        .iter()
                        .position(|ss| ss.read().unwrap().name() == system_to.read().unwrap().name())
                        .unwrap();
                    systems.remove(system_to_index);
                }
            },
            None => {},
        };

        let current_shortest: Arc<RwLock<CurrentShortest>> = Arc::new(RwLock::new(CurrentShortest::new()));

        systems.clone().into_iter().permutations(systems.len()).enumerate().par_bridge().for_each(|(idx, sync_route)| {
            if idx.wrapping_rem(feedback_step) == 0 {
                manager.progress_holder_wlock().feedback(
                    idx as u128,
                    format!("Building shortest path ( {} )", idx)
                );
            }

            let system_from_rlock = match system_from.read() {
                Err(e) => {
                    manager.progress_holder_wlock().feedback(
                        idx as u128,
                        format!("While building shortest path: {e}")
                    );
                    return;
                },
                Ok(system_from_rlock) => system_from_rlock,
            };

            let mut route_length: u64 = match system_from_rlock.get_distance_to(&sync_route[0]) {
                None => {
                    manager.progress_holder_wlock().feedback(
                        idx as u128,
                        format!("Distance from '{}' to '{}' not set. Skipping",
                            system_from_rlock.name(),
                            sync_route[0].read().unwrap().name(),
                        )
                    );
                    return;
                },
                Some(route_length) => route_length,
            };

            match system_to {
                Some(system_to) => {
                    route_length += match sync_route[sync_route.len()-1].read() {
                        Err(_) => return,
                        Ok(last_route) => match last_route.get_distance_to(system_to) {
                            Some(distance) => distance,
                            None => {
                                manager.progress_holder_wlock().feedback(
                                    idx as u128,
                                     format!("Distance from '{}' to '{}' not set. Skipping",
                                        last_route.name(),
                                        system_to.read().unwrap().name(),
                                    )
                                );
                                return;
                            },
                        },
                    };
                },
                None => {},
            }

            sync_route.windows(2).for_each(
                |window| {
                    let prev_rlock = window[0].read().unwrap();
                    let next_rlock = window[1].read().unwrap();

                    route_length += match prev_rlock.get_distance_to(self.get(next_rlock.name())) {
                        Some(distance) => distance,
                        None => {
                            manager.progress_holder_wlock().feedback(
                                idx as u128,
                                format!("Distance from '{}' to '{}' not set. Skipping",
                                    prev_rlock.name(),
                                    next_rlock.name(),
                                )
                            );
                            return;
                        },
                    };
                }
            );

            if current_shortest.read().unwrap().weak_check_registerable(route_length) {
                current_shortest.write().unwrap().register(&sync_route, route_length);
            };
        });

        // last report on 100%
        manager.progress_holder_wlock().feedback(
            self.permutation_size_hint().unwrap_or(u128::MAX),
            "Shortest path built"
        );

        manager.report_msg("Unwraping `Arc<RwLock<_>>` of `CurrentShortest`")?;
        let current_shortest = Arc::into_inner(current_shortest).ok_or_eyre("Failed to unwrap `Arc`")?.into_inner()?;

        Ok(current_shortest)
    }
}
