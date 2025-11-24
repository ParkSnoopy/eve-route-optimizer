use std::sync::{
    Arc,
    RwLock,
};

use nu_ansi_term::Color;

use crate::trace;
use super::SyncRoute;



pub struct CurrentShortest {
    routes: Vec<SyncRoute>,
    length: u64,
}

impl CurrentShortest {
    pub fn new() -> CurrentShortest {
        CurrentShortest {
            routes: Vec::new(),
            length: u64::MAX,
        }
    }

    pub fn register(&mut self, sync_route: &SyncRoute, length: u64) {
        if length < self.length {
            self.routes.clear();
            self.routes.push(sync_route.clone());
            self.length = length;
        } else if length == self.length {
            self.routes.push(sync_route.clone());
        }
    }

    pub fn report_stdout(&self) {
        println!();
        self.route_summary();
        println!();
    }

    fn route_summary(&self) {
        trace::ok(format!(
            "  Shortest Route Length is '{}'",
            colored(039, self.length.to_string())
        ));
        println!();
        println!("{}", colored(048, "  [ ROUTE ]"));
        println!();
        for (idx, route) in self.routes.iter().enumerate() {
            println!(
                "  - {} : {}",
                colored(048, format!("{:03}", idx + 1)),
                prettify_route(route),
            );
        }
    }
}

fn colored<S: AsRef<str>>(code: u8, msg: S) -> String {
    Color::Fixed(code).paint(msg.as_ref()).to_string()
}

fn prettify_route(route: &SyncRoute) -> String {
    let yellow_arrow = colored(220, ">>>");
    format!(
        "{yellow_arrow}  {} {} {} {}",
        colored(087, crate::CLI_ARGS.read().unwrap().start.name()),
        // distance from start to first system
        // start system has no distance table.
        // so, distance is calculated from first system to start system
        // and wrap start system into Arc<RwLock<T>>,
        // maybe there is better way but currently settle down on this way
        arrow_with_distance(
            route[0]
                .read()
                .unwrap()
                .get_distance_to(&Arc::new(RwLock::new(
                    crate::CLI_ARGS.read().unwrap().start.clone()
                )))
                .unwrap()
        ),
        route.to_vec().windows(2).fold(
            colored(082, route[0].read().unwrap().name()),
            |acc, systems| {
                let curr_system_rlock = systems[0].read().unwrap();
                let next_system_rlock = systems[1].read().unwrap();
                let distance = curr_system_rlock.get_distance_to(&systems[1]).unwrap();

                format!(
                    "{} {} {}",
                    acc,
                    arrow_with_distance(distance),
                    colored(082, next_system_rlock.name())
                )
            }
        ),
        match &crate::CLI_ARGS.read().unwrap().end {
            Some(system) => {
                format!(
                    "{} {}  {}",
                    // distance from last system to end
                    // same as start to first system
                    arrow_with_distance(
                        route[route.len() - 1]
                            .read()
                            .unwrap()
                            .get_distance_to(&Arc::new(RwLock::new(
                                crate::CLI_ARGS.read().unwrap().end.clone().unwrap()
                            )))
                            .unwrap()
                    ),
                    colored(087, system.name()),
                    yellow_arrow,
                )
            }
            None => yellow_arrow.clone(),
        },
    )
}

fn arrow_with_distance(distance: u64) -> String {
    format!(
        " {}{}{} ",
        colored(238, "-"),
        colored(250, format!("({})", distance)),
        colored(238, ">")
    )
}
