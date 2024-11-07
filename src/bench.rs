use chrono::{ DateTime, Local, TimeDelta };



pub struct Bencher {
    t0: DateTime<Local>,
}

impl Bencher {
    pub fn start_new() -> Self {
        Self {
            t0: Local::now(),
        }
    }

    pub fn done(&self) -> TimeDelta {
        Local::now() - self.t0
    }
}
