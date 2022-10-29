mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod src;
mod audio;

use std::{fs::File, io::Cursor, thread, sync::{RwLock, atomic::AtomicBool}};

use audio::{decoder::Decoder, controls::*};
use flutter_rust_bridge::StreamSink;
use reqwest::blocking::Client;
use symphonia::core::io::MediaSource;

use crate::src::{playback_state_stream::*, progress_state_stream::*};

static DECODER:RwLock<Decoder> = RwLock::new(Decoder::new());
static FIRST_TIME:AtomicBool = AtomicBool::new(true);

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
    pub fn progress_state_stream(stream:StreamSink<ProgressState>) { progress_state_stream(stream); }

    pub fn is_playing(&self) -> bool
    { IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst) }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Opens a file or network resource for reading and playing.
    pub fn open(&self, path:String)
    {
        let mut decoder = *DECODER.write()
            .expect(format!("ERR: Failed to open RwLock to READ decoder.").as_str());

        if !FIRST_TIME.load(std::sync::atomic::Ordering::SeqCst)
        {
            println!("Stop");
            decoder.stop();
        }

        FIRST_TIME.store(false, std::sync::atomic::Ordering::SeqCst);

        let source:Box<dyn MediaSource> = if path.contains("http") {
            Box::new(Self::get_bytes_from_network(path))
        } else { Box::new(File::open(path).unwrap()) };

        update_playback_state_stream(true);
        thread::spawn(move || {
            decoder.open_stream(source);
            update_playback_state_stream(false);
            println!("Done in thread");
        });
    }

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
        IS_PLAYING.store(true, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn pause(&self)
    {
        update_playback_state_stream(false);
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn set_volume(&self, volume:f32)
    { *VOLUME.write().unwrap() = volume; }

    pub fn seek(&self, seconds:f64)
    { *SEEK_TS.write().unwrap() = Some(seconds.floor()); }
}

#[cfg(test)]
mod tests
{
    use std::{thread, time::Duration};

    #[test]
    fn open_and_play()
    {
        let player = crate::Player::new();
        player.set_volume(0.2);
        player.open("/home/erikas/Music/test.mp3".to_string());
        player.seek(30.0);
        thread::sleep(Duration::from_secs(2));
        player.open("/home/erikas/Music/test.mp3".to_string());
        thread::sleep(Duration::from_secs(50));
    }

    #[test]
    fn open_network_and_play()
    {
        let player = crate::Player::new();
        player.open("".to_string());
    }
}