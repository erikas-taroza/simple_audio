mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod output;
mod src;
mod audio;

use std::{fs::File, io::Cursor};

use audio::{decoder::Decoder, controls::*};
use flutter_rust_bridge::StreamSink;
use reqwest::blocking::Client;
use symphonia::core::io::MediaSource;

use crate::src::playback_state_stream::*;

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
    { IS_PLAYING.load(std::sync::atomic::Ordering::Relaxed) }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Opens a file or network resource for reading and playing.
    pub fn open(&self, path:String)
    {
        let source:Box<dyn MediaSource> = if path.contains("http") {
            Box::new(Self::get_bytes_from_network(path))
        } else { Box::new(File::open(path).unwrap()) };

        let decoder = Decoder::new();

        update_playback_state_stream(true);
        decoder.open_stream(source);
        update_playback_state_stream(false);
    }

    /// Similar to open, except it opens multiple
    /// sources at once.
    // pub fn open_list(&self, paths:Vec<String>)
    // {
    //     Self::insure_output_initialized();

    //     let output = &*OUTPUT.read()
    //         .expect(format!("ERR: Failed to open RwLock to READ output.").as_str());
    //     let output = output.as_ref().unwrap();

    //     for path in paths
    //     {
    //         if path.contains("http")
    //         { output.append_network(&path); }
    //         else
    //         { output.append_file(Path::new(&path)); }
    //     }

    //     update_playback_state_stream(true);
    //     output.sink.sleep_until_end();
    //     update_playback_state_stream(false);
    // }

    fn get_bytes_from_network(url:String) -> Cursor<Vec<u8>>
    {
        let response = Client::new().get(url.clone())
            .header("Range", "bytes=0-")
            .send()
            .expect(format!("ERR: Could not open {url}").as_str());
            
        let bytes = response.bytes().unwrap().to_vec();
        Cursor::new(bytes)
    }

    // ---------------------------------
    //             CONTROLS
    // ---------------------------------

    pub fn play(&self)
    {
        update_playback_state_stream(true);
        IS_PLAYING.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn pause(&self)
    {
        update_playback_state_stream(false);
        IS_PLAYING.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn set_volume(&self, volume:f32)
    { *VOLUME.write().unwrap() = volume; }

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
        player.open("/home/erikas/Music/BitBeat/HURT LXCKER [Prod. MCTR] by SCARLXRD.mp3".to_string());
    }

    #[test]
    fn open_network_and_play()
    {
        let player = crate::Player::new();
        player.open("".to_string());
    }

    // #[test]
    // fn open_list_and_play()
    // {
    //     let player = crate::Player::new();
    //     player.open_list(vec!["/home/erikas/Music/test.mp3".to_string(), "/home/erikas/Music/test2.mp3".to_string()]);
    // }
}