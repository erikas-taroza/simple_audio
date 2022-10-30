use std::sync::RwLock;

use flutter_rust_bridge::{StreamSink, support::lazy_static};

lazy_static! { static ref PLAYBACK_STATE_STREAM:RwLock<Option<StreamSink<i32>>> = RwLock::new(None); }

pub const PLAY:i32 = 0;
pub const PAUSE:i32 = 1;
pub const DONE:i32 = 2;

/// Creates a new playback stream.
pub fn playback_state_stream(stream:StreamSink<i32>)
{
    let mut state_stream = PLAYBACK_STATE_STREAM.write().unwrap();
    *state_stream = Some(stream);
}

/// Updates the playback stream with the given value.
pub fn update_playback_state_stream(value:i32)
{
    if let Some(stream) = &*PLAYBACK_STATE_STREAM.read().unwrap()
    { stream.add(value); }
}