use nu_ansi_term::Color;

use crate::trace;
use super::{
    SyncRoute,
};



#[derive(Debug)]
pub struct CurrentShortest{
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
        }/* else if length > self.length {

        }*/
    }

    pub fn report_stdout(&self) {
        self.route_summary();
    }

    fn route_summary(&self) {
        println!();
        println!();
        println!();
        trace::ok(format!("  Shortest Route Length is '{}'", colored(039, self.length.to_string())));
        println!();
        println!("{}", colored(048, "  [ ROUTES ]"));
        println!();
        for (idx, route) in self.routes.iter().enumerate() {
            println!("  - {} : {}",
                colored(048, format!("{:03}", idx+1)),
                prettify_route(route),
            );
        }
        println!();
    }
}

fn colored<S: AsRef<str>>(code: u8, msg: S) -> String {
    Color::Fixed(code).paint(msg.as_ref()).to_string()
}

fn prettify_route(route: &SyncRoute) -> String {
    format!("{}  {}  {}  {}  {}",
        colored(220, ">>>"),
        colored(087, crate::CLI_ARGS.read().unwrap().start.name() ),
        colored(220, ">>>"),
        route.iter().map(|r| colored(051, r.read().unwrap().name())).collect::<Vec<_>>().join(&colored(244, " -> ")),
        colored(220, ">>>"),
    )
}
