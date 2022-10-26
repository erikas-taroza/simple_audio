mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod output;
mod src;

use std::{sync::RwLock, path::Path};

use flutter_rust_bridge::StreamSink;

use crate::src::playback_state_stream::*;
use crate::output::Output;

static OUTPUT:RwLock<Option<Output>> = RwLock::new(None);

// NOTE: Code gen fails with empty structs.
pub struct Player
{
    dummy:i32
}

impl Player
{
    pub fn new() -> Player { Player { dummy: 0 } }

    // ---------------------------------
    //          SETTERS/GETTERS
    // ---------------------------------

    pub fn playback_state_stream(stream:StreamSink<bool>) { playback_state_stream(stream); }

    pub fn is_playing(&self) -> bool
    {
        let output = &*OUTPUT.read().unwrap();
        if let Some(output) = output
        {
            return !output.sink.is_paused();
        }

        false
    }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Opens a file for reading.
    pub fn open(&self, path:String)
    {
        let mut w = OUTPUT.write().unwrap();
        if w.is_none()
        { *w = Some(Output::new()); }
        drop(w);

        if path.contains("http")
        {
            return;
        }
        else
        {
            let output = &*OUTPUT.read().unwrap();
            if let Some(output) = output
            {
                update_playback_state_stream(true);

                output.open_file(Path::new(&path));
                output.sink.sleep_until_end();

                update_playback_state_stream(false);
            }
        }
    }

    // ---------------------------------
    //             CONTROLS
    // ---------------------------------

    pub fn play(&self)
    {
        update_playback_state_stream(true);
        
        let output = &*OUTPUT.read().unwrap();
        if let Some(output) = output
        {
            output.sink.play();
        }
    }

    pub fn pause(&self)
    {
        update_playback_state_stream(false);

        let output = &*OUTPUT.read().unwrap();
        if let Some(output) = output
        {
            output.sink.pause();
        }
    }

    pub fn set_volume(&self, volume:f32)
    {
        let output = &*OUTPUT.read().unwrap();
        if let Some(output) = output
        {
            output.sink.set_volume(volume.clamp(0.0, 1.0));
        }
    }

    // pub fn seek(&self, seconds:i32)
    // {

    // }
}

mod tests
{
    #[test]
    fn open_and_play()
    {
        let player = crate::Player::new();
        player.open("/home/erikas/Music/test.mp3".to_string());
    }
}