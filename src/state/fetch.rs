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

    pub fn describe(&self) {
        let total = self.total;
        let done = self.success + self.failure;

        let width: f64 = max(16, utils::get_term_width()-20) as f64;

        let done_p: f64 = done as f64 / total as f64;
        let done_n = (width * done_p) as usize;
        let todo_n = (width as usize) - done_n;

        let done_s: String = repeat(config::DONE_BAR_CHAR).take(done_n).collect();
        let todo_s: String = repeat(config::TODO_BAR_CHAR).take(todo_n).collect();

        let heat_p = self.heatsink.temp_threshold_percentile() as f64;
        let heat_n = (width * heat_p / 100_f64) as usize;
        let cool_n = (width as usize) - heat_n;

        let heat_s: String = repeat(config::HEAT_BAR_CHAR).take(heat_n).collect();
        let cool_s: String = repeat(config::COOL_BAR_CHAR).take(cool_n).collect();

        print!("  {}\n",

            // Line1: - Done '590' out of '720' ( 59 fetching )
            format!("  - Done '{}' out of '{}' {}",
                Color::LightBlue.paint(done.to_string()),
                Color::LightBlue.paint(total.to_string()),
                format!("{}{}{}",
                    Color::LightGray.paint("( "),
                    Color::LightYellow.paint(self.fetching.to_string()),
                    Color::LightGray.paint(" fetching )"),
                )
            ),
        );

        print!("  {} {}\n",

            // Line2: ( 81.94 % )
            Color::Fixed(051).paint(format!(
                "( {:6.02} % )",
                done_p*100_f64 
            )),

            // Line2: [#################################################################################...................]
            format!(
                "[{}{}]",
                Color::Fixed(002).paint(done_s),
                Color::Fixed(015).paint(todo_s),
            ),
        );

        print!("  {} {}\n",

            // Line3: ( 81.94 C )
            Color::Fixed(160).paint(format!(
                "(    {:3} C )",
                self.heatsink.temp,
            )),

            // Line4: heatbar_todo
            format!(
                "[{}{}]",
                Color::Fixed(166).paint(heat_s),
                Color::Fixed(015).paint(cool_s),
            ),
        );

        print!("{}\r",
            ansi_escapes::CursorUp(3),
        );
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
