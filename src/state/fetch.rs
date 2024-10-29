use crate::{
    config,
    utils,
};

use std::cmp::max;
use std::iter::repeat;

use nu_ansi_term::Color;



pub struct FetchState {
    init: bool,
    total: usize,
    fetching: usize,
    success: usize,
    failure: usize,
}

impl FetchState {
    pub fn new() -> FetchState {
        FetchState {
            init: false,
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
