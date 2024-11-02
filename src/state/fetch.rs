use crate::{
    config,
    utils,
    trace,
};

use std::cmp::max;
use std::iter::repeat;
use std::thread::sleep;
use std::time::Duration;

use nu_ansi_term::Color;



struct Heatsink {
    pub temp: usize,
    overheated: bool,
}

fn cooldown() {
    sleep(Duration::from_millis(config::COOLDOWN_INTERVAL_MILLS));
}

impl Heatsink {
    fn new() -> Heatsink {
        Heatsink {
            temp: 0,
            overheated: false,
        }
    }

    fn wait(&mut self) {
        //trace::info(format!("Wait on heatsink ( heat: {} )", self.temp));

        if !self.overheated && self.temp >= config::REQUEST_OVERHEAT {
            self.overheated = true;
            //trace::warn("Heatsink overheated!");
        }

        if self.overheated {
            loop {
                trace::info(format!("Heatsink cooling... ( heat: {} )", self.temp));

                self.temp -= 1;
                cooldown();

                if self.temp <= 0 {
                    self.overheated = false;
                    trace::clear();
                    break
                }
            };
        }

        //trace::info("Passed heatsink");
        //cooldown();
        self.temp += 1;
        //trace::info(format!("Current temp: {}", self.temp));
        //cooldown();
    }

    fn temp_threshold_percentile(&self) -> usize {
        100 * self.temp / config::REQUEST_OVERHEAT
    }
}


pub struct FetchState {
    init: bool,
    heatsink: Heatsink,

    total: usize,
    fetching: usize,
    success: usize,
    failure: usize,
}

impl FetchState {
    pub fn new() -> FetchState {
        FetchState {
            init: false,
            heatsink: Heatsink::new(),

            total: 0,
            fetching: 0,
            success: 0,
            failure: 0,
        }
    }

    pub fn set_total(&mut self, total: usize) {
        self.init = true;
        self.total = total;
    }

    pub fn describe(&self) -> String {
        let total = self.total;
        let done = self.success + self.failure;

        let width: f64 = max(16, utils::get_term_width()-20) as f64;
        let done_p: f64 = done as f64 / total as f64;

        let done_n = (width * done_p) as usize;
        let todo_n = (width as usize) - done_n;

        let done_s: String = repeat(config::DONE_BAR_CHAR).take(done_n).collect();
        let todo_s: String = repeat(config::TODO_BAR_CHAR).take(todo_n).collect();

        format!("  {}\n  {} {}\n{}\r",
            format!("  - Done '{}' out of '{}' {}",
                Color::LightBlue.paint(done.to_string()),
                Color::LightBlue.paint(total.to_string()),
                format!("{}{}{}",
                    Color::LightGray.paint("( "),
                    Color::LightYellow.paint(self.fetching.to_string()),
                    Color::LightGray.paint(" fetching )"),
                )
            ),
            Color::Cyan.paint(format!(
                "( {:02.02} % )",
                done_p*100_f64 
            )),
            format!(
                "[{}{}]",
                Color::Green.paint(done_s),
                Color::LightGray.paint(todo_s),
            ),
            ansi_escapes::CursorUp(2),
        )
    }

    pub fn add_fetching(&mut self) {
        self.heatsink.wait();

        if self.init {
            self.fetching += 1;
        }
    }

    pub fn add_success(&mut self) {
        if self.init {
            self.fetching -= 1;
            self.success += 1;
        }
    }

    pub fn add_failure(&mut self) {
        if self.init {
            self.fetching -= 1;
            self.failure += 1;
        }
    }
}
