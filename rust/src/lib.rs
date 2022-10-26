mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod output;
mod src;

use std::{sync::RwLock, path::Path};

use flutter_rust_bridge::StreamSink;

use crate::src::playback_state_stream::*;
use crate::output::Output;

static IS_PLAYING:RwLock<bool> = RwLock::new(false);
static OUTPUT:RwLock<Option<Output>> = RwLock::new(None);

// NOTE: Code gen fails with empty structs.
pub struct Player
{
    dummy:i32
}

impl Player
{
    pub fn new() -> Player
    {
        let mut w = OUTPUT.write().unwrap();
        *w = Some(Output::new());

        Player { dummy: 0 }
    }

    // ---------------------------------
    //          SETTERS/GETTERS
    // ---------------------------------

    pub fn playback_state_stream(stream:StreamSink<bool>) { playback_state_stream(stream); }

    pub fn is_playing(&self) -> bool { *IS_PLAYING.read().unwrap() }
    fn set_is_playing(value:bool)
    {
        let mut w = IS_PLAYING.write().unwrap();
        *w = value;
    }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Opens a file for reading.
    pub fn open(&self, path:String)
    {
        if path.contains("http")
        {
            return;
        }
        else
        {
            let output = &*OUTPUT.read().unwrap();
            if let Some(output) = output
            {
                output.open_file(Path::new(&path));
                output.sink.sleep_until_end();
            }
        }
    }

    // ---------------------------------
    //             CONTROLS
    // ---------------------------------

    pub fn play(&self)
    {
        Self::set_is_playing(true);
        update_playback_state_stream(true);
        
        let output = &*OUTPUT.read().unwrap();
        if let Some(output) = output
        {
            output.sink.play();
        }
    }

    pub fn pause(&self)
    {
        Self::set_is_playing(false);
        update_playback_state_stream(false);

        let output = &*OUTPUT.read().unwrap();
        if let Some(output) = output
        {
            output.sink.pause();
        }
    }

    // pub fn seek(&self, seconds:i32)
    // {

    // }

    // pub fn set_volume(&self, volume:f32)
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