use color_eyre::eyre;

use std::collections::{ HashMap };
use std::sync::{ Arc, RwLock };




#[derive(Clone)]
#[derive(Debug)]
pub struct System {
    name: String,
    distance_table: HashMap<String, u64>,
}

impl System {
    pub fn new<S: AsRef<str>>(name: S) -> System {
        System {
            name: name.as_ref().to_uppercase(),
            distance_table: HashMap::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn set_distance_to(&mut self, other: &Arc<RwLock< System >>, distance: u64) -> color_eyre::Result<()> {
        match other.read() {
            Ok(other_system) => {
                self.distance_table.insert(other_system.name().to_string(), distance);
                Ok(())
            },
            Err(_) => {
                Err(eyre::eyre!("Failed to obtain `RwLockReadGuard` for `Arc<RwLock<System>>`"))
            },
        }
    }

    pub fn get_distance_to(&self, other: &Arc<RwLock< System >>) -> Option<u64> {
        self.distance_table.get(other.read().unwrap().name()).copied()
    }
}

impl std::str::FromStr for System {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(System::new(s))
    }
}

impl PartialEq for System {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for System {}

impl std::hash::Hash for System {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
