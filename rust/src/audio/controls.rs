// A file that defines controls globally.

use std::sync::{RwLock, atomic::AtomicBool};

pub static IS_PLAYING:AtomicBool = AtomicBool::new(true);
pub static VOLUME:RwLock<f32> = RwLock::new(0.5);
pub static SEEK_TS:RwLock<Option<f64>> = RwLock::new(None);