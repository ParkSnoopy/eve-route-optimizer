use std::sync::{
    Arc,
    RwLock,
};

type ArcRwLock<T> = Arc<RwLock<T>>;
type SyncSystem = ArcRwLock<System>;
type SyncRoute = Vec<SyncSystem>;

mod holder;
mod pair;
mod system;

mod shortest;
use shortest::CurrentShortest;
pub use system::System;
pub use pair::SystemPair;
pub use holder::SystemHolder;
