use std::sync::LazyLock;

use terminal_size::{
    terminal_size,
    Width,
};
use nu_ansi_term::Color;

use crate::{
    config,
    trace,
};



static TRACE_USABLE_TERM_WIDTH: LazyLock<usize> = LazyLock::new(|| {
    let (Width(w), _) = terminal_size().expect(&trace::string::error("Unable to detect terminal"));
    let w = (w - 16) as usize;
    assert!(
        w > 0,
        "{}",
        trace::string::error("Usable Terminal Width is less or equal then Zero")
    );
    w
});

pub struct ProgressHolder {
    total: u128,
    done: u128,
}

impl ProgressHolder {
    pub fn new() -> ProgressHolder {
        ProgressHolder { total: 0, done: 0 }
    }

    pub fn set_total(&mut self, total: u128) {
        self.total = total;
    }

    pub fn feedback(&mut self, current_step: u128) {
        self.done = current_step;

        let progress_bar = ProgressBar::from_total_done(self.total, self.done);

        print!(
            "{}  {}\n  {}\n",
            ansi_escapes::CursorUp(2),
            trace::string::info(format!("In Progress ( {} / {} )", self.done, self.total)),
            trace::string::info(progress_bar.build()),
        );
    }
}

struct ProgressBar {
    done_n: usize,
    todo_n: usize,
}

impl ProgressBar {
    fn from_total_done(total: u128, done: u128) -> Self {
        // downcast to float, to perform percentage calculations
        let total = total as f64;
        let done = done as f64;

        let done_p = done / total; // range=[0, 1)

        let done_n: usize = (*TRACE_USABLE_TERM_WIDTH as f64 * done_p).ceil() as usize;
        let todo_n: usize = *TRACE_USABLE_TERM_WIDTH - done_n;

        Self { done_n, todo_n }
    }

    fn build(&self) -> String {
        let done_s: String = std::iter::repeat(config::DONE_BAR_CHAR)
            .take(self.done_n)
            .collect();
        let todo_s: String = std::iter::repeat(config::TODO_BAR_CHAR)
            .take(self.todo_n)
            .collect();

        format!(
            "[{}{}]",
            Color::Fixed(082).paint(done_s),
            Color::Fixed(064).paint(todo_s),
        )
    }
}
