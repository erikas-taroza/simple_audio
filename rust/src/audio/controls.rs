// A file that defines controls globally.

use std::sync::{RwLock, atomic::AtomicBool};

use crossbeam::channel::{Sender, Receiver};

pub static TXRX:RwLock<Option<(Sender<bool>, Receiver<bool>)>> = RwLock::new(None);
pub static IS_PLAYING:AtomicBool = AtomicBool::new(true);
pub static IS_DONE:AtomicBool = AtomicBool::new(false);
pub static VOLUME:RwLock<f32> = RwLock::new(1.0);
pub static SEEK_TS:RwLock<Option<f64>> = RwLock::new(None);