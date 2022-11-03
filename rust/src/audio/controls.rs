// A file that defines controls globally.

use std::sync::{RwLock, atomic::AtomicBool};

use crossbeam::channel::{Sender, Receiver};

use crate::utils::progress_state_stream::ProgressState;

pub static TXRX:RwLock<Option<(Sender<bool>, Receiver<bool>)>> = RwLock::new(None);
pub static IS_PLAYING:AtomicBool = AtomicBool::new(true);
pub static VOLUME:RwLock<f32> = RwLock::new(1.0);
pub static SEEK_TS:RwLock<Option<u64>> = RwLock::new(None);
pub static PROGRESS:RwLock<Option<ProgressState>> = RwLock::new(None);