use std::sync::RwLock;

use flutter_rust_bridge::{StreamSink, support::lazy_static};

lazy_static! { static ref PROGRESS_STATE_STREAM:RwLock<Option<StreamSink<ProgressState>>> = RwLock::new(None); }

/// Creates a new progress stream.
pub fn progress_state_stream(stream:StreamSink<ProgressState>)
{
    let mut state_stream = PROGRESS_STATE_STREAM.write().unwrap();
    *state_stream = Some(stream);
}

/// Updates the progress stream with the given value.
pub fn update_progress_state_stream(value:ProgressState)
{
    if let Some(stream) = &*PROGRESS_STATE_STREAM.read().unwrap()
    { stream.add(value); }
}

#[derive(Clone, Copy)]
pub struct ProgressState
{
    pub position:u64,
    pub duration:u64
}