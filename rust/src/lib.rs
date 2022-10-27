mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod output;
mod src;

use std::{sync::RwLock, path::Path};

use flutter_rust_bridge::StreamSink;

use crate::src::playback_state_stream::*;
use crate::output::Output;

// NOTE: This is used to prevent flutter_rust_bridge
// from generating a field in the struct.
// It also allows the item to be mutable.
// This prevents the use of mutable methods which
// flutter_rust_bridge does not support.
static OUTPUT:RwLock<Option<Output>> = RwLock::new(None);

// NOTE: Code gen fails with empty structs.
pub struct Player
{
    dummy:i32
}

impl Player
{
    pub fn new() -> Player { Player { dummy: 0 } }

    /// Insures that `OUTPUT` is initialized.
    /// At first, it is set to `None` because `Output::new()`
    /// is a non-static function.
    fn insure_output_initialized()
    {
        let mut w = OUTPUT.write()
            .expect(format!("ERR: Failed to open RwLock to WRITE new output.").as_str());
        if w.is_none()
        { *w = Some(Output::new()); }
    }

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

    /// Opens a file or network resource for reading and playing.
    pub fn open(&self, path:String)
    {
        Self::insure_output_initialized();

        let output = &*OUTPUT.read()
            .expect(format!("ERR: Failed to open RwLock to READ output.").as_str());
        let output = output.as_ref().unwrap();

        if path.contains("http")
        { output.append_network(&path); }
        else
        { output.append_file(Path::new(&path)); }

        update_playback_state_stream(true);
        output.sink.sleep_until_end();
        update_playback_state_stream(false);
    }

    /// Similar to open, except it opens multiple
    /// sources at once.
    pub fn open_list(&self, paths:Vec<String>)
    {
        Self::insure_output_initialized();

        let output = &*OUTPUT.read()
            .expect(format!("ERR: Failed to open RwLock to READ output.").as_str());
        let output = output.as_ref().unwrap();

        for path in paths
        {
            if path.contains("http")
            { output.append_network(&path); }
            else
            { output.append_file(Path::new(&path)); }
        }

        update_playback_state_stream(true);
        output.sink.sleep_until_end();
        update_playback_state_stream(false);
    }

    // ---------------------------------
    //             CONTROLS
    // ---------------------------------

    pub fn play(&self)
    {
        update_playback_state_stream(true);
        
        let output = &*OUTPUT.read()
            .expect(format!("ERR: Failed to open RwLock to READ output.").as_str());
        if let Some(output) = output
        {
            output.sink.play();
        }
    }

    pub fn pause(&self)
    {
        update_playback_state_stream(false);

        let output = &*OUTPUT.read()
            .expect(format!("ERR: Failed to open RwLock to READ output.").as_str());
        if let Some(output) = output
        {
            output.sink.pause();
        }
    }

    pub fn set_volume(&self, volume:f32)
    {
        let output = &*OUTPUT.read()
            .expect(format!("ERR: Failed to open RwLock to READ output.").as_str());
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

    #[test]
    fn open_network_and_play()
    {
        let player = crate::Player::new();
        player.open("".to_string());
    }

    #[test]
    fn open_list_and_play()
    {
        let player = crate::Player::new();
        player.open_list(vec!["/home/erikas/Music/test.mp3".to_string(), "/home/erikas/Music/test2.mp3".to_string()]);
    }
}