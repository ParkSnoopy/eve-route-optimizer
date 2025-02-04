use color_eyre::{
    Result,
};

use super::{
    SyncSystem,
};



pub struct SystemPair {
    system1: SyncSystem,
    system2: SyncSystem,
}

impl SystemPair {
    pub fn new(systems: Vec<SyncSystem>) -> SystemPair {
        if systems.len() != 2 {
            panic!("`systems` argument must be length of 2, given {}", systems.len());
        }

        SystemPair {
            system1: systems[0].clone(),
            system2: systems[1].clone(),
        }
    }

    pub fn left(&self) -> SyncSystem {
        self.system1.clone()
    }
    pub fn right(&self) -> SyncSystem {
        self.system2.clone()
    }

    pub fn set_distance(&self, distance: u64) -> Result<()> {
        self.system1.write().unwrap().set_distance_to(&self.system2, distance)?;
        self.system2.write().unwrap().set_distance_to(&self.system1, distance)?;
        Ok(())
    }

    pub fn to_string(&self) -> String {
        format!("( {} <-> {} )",
            self.system1.read().unwrap().name(),
            self.system2.read().unwrap().name(),
        )
    }
}

impl std::fmt::Display for SystemPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "( {} <-> {} )", self.system1.read().unwrap().name(), self.system2.read().unwrap().name())
    }
}
