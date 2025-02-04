use std::sync::{
    Arc, RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
};

use super::{
    manager::SystemManager,
};

use crate::{
    route::system::system::System,
};



pub struct SystemPair {
    system1: Arc<RwLock< System >>,
    system2: Arc<RwLock< System >>,
}

impl SystemPair {
    pub fn from_name<S: AsRef<str>>(system_holder: RwLockReadGuard<SystemManager>, system1_name: S, system2_name: S) -> Self {
        Self {
            system1: system_holder.get(system1_name).clone(),
            system2: system_holder.get(system2_name).clone(),
        }
    }

    pub fn from_vec(systems: Vec< Arc<RwLock< System >> >) -> Self {
        if systems.len() != 2 {
            panic!("`systems` argument must be length of 2, given {}", systems.len());
        }

        Self {
            system1: systems[0].clone(),
            system2: systems[1].clone(),
        }
    }

    pub fn left(&self) -> &Arc<RwLock< System >> {
        &self.system1
    }
    pub fn right(&self) -> &Arc<RwLock< System >> {
        &self.system2
    }

    fn left_wlock(&self) -> RwLockWriteGuard<System> {
        self.left().write().unwrap()
    }
    fn right_wlock(&self) -> RwLockWriteGuard<System> {
        self.right().write().unwrap()
    }

    pub fn left_rlock(&self) -> RwLockReadGuard<System> {
        self.left().read().unwrap()
    }
    pub fn right_rlock(&self) -> RwLockReadGuard<System> {
        self.right().read().unwrap()
    }

    pub fn set_distance(&self, distance: u64) -> color_eyre::Result<()> {
        self.left_wlock().set_distance_to(self.right(), distance)?;
        self.right_wlock().set_distance_to(self.left(), distance)?;
        Ok(())
    }

    pub fn try_to_string(&self) -> color_eyre::Result<String> {
        Ok(format!("( {} : {} )",
            self.left_rlock().name(),
            self.right_rlock().name(),
        ))
    }
}
