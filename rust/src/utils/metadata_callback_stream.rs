use std::sync::RwLock;

use flutter_rust_bridge::{StreamSink, support::lazy_static};

lazy_static! { static ref METADATA_CALLBACK_STREAM:RwLock<Option<StreamSink<bool>>> = RwLock::new(None); }

/// Creates a new stream for responding to callbacks from the metadata handler.
pub fn metadata_callback_stream(stream:StreamSink<bool>)
{
    let mut state_stream = METADATA_CALLBACK_STREAM.write().unwrap();
    *state_stream = Some(stream);
}

/// Updates the stream with the given value.
/// `True` means that a signal for "Next" was received.
/// `False` means that a signal for "Previous" was received.
pub fn update_metadata_callback_stream(value:bool)
{
    if let Some(stream) = &*METADATA_CALLBACK_STREAM.read().unwrap()
    { stream.add(value); }
}