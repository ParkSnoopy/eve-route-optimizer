use crossbeam::channel::Sender;

use std::fmt::{
    self,
    Display,
};

use crate::{
    channel::ManagerResponse,
};



pub struct ProgressHolder {
    total: u128,
    done : u128,

    sender: Sender<ManagerResponse>,
}

impl ProgressHolder {
    pub fn with_sender(sender: Sender<ManagerResponse>) -> ProgressHolder {
        ProgressHolder {
            total: 0,
            done: 0,
            sender,
        }
    }

    pub fn set_total(&mut self, total: u128) {
        self.total = total;
    }

    pub fn feedback(&mut self, current_step: u128, msg: impl AsRef<str>) {
        self.done = current_step;

        self.sender.send( ManagerResponse::Progress(
            Progress::with_msg(self, msg)
        )).unwrap();
    }
}
