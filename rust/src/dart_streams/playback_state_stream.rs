use std::sync::RwLock;

use flutter_rust_bridge::{StreamSink, support::lazy_static};

lazy_static! { static ref PLAYBACK_STATE_STREAM:RwLock<Option<StreamSink<bool>>> = RwLock::new(None); }

/// Creates a new playback stream.
pub fn playback_state_stream(stream:StreamSink<bool>)
{
    let mut state_stream = PLAYBACK_STATE_STREAM.write().unwrap();
    *state_stream = Some(stream);
}

/// Updates the playback stream with the given value.
pub fn update_playback_state_stream(value:bool)
{
    if let Some(stream) = &*PLAYBACK_STATE_STREAM.read().unwrap()
    { stream.add(value); }
}