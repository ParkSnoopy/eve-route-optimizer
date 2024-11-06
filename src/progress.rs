use crate::{
    trace,
};



pub struct ProgressHolder {
    total: u128,
    done: u128,
}

impl ProgressHolder {
    pub fn new() -> ProgressHolder {
        ProgressHolder {
            total: 0,
            done: 0,
        }
    }

    pub fn set_total(&mut self, total: u128) {
        self.total = total;
    }

    pub fn feedback(&mut self, current_step: u128) {
        self.done = current_step;

        trace::inline::info(format!("In Progress ( {} / {} )", self.done, self.total));
    }
}
