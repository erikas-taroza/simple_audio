mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod dart_streams;
mod audio;

use std::{fs::File, io::Cursor, thread};

use audio::{decoder::Decoder, controls::*};
use crossbeam::channel::unbounded;
use flutter_rust_bridge::StreamSink;
use reqwest::blocking::Client;
use symphonia::core::io::MediaSource;

use crate::dart_streams::{playback_state_stream::*, progress_state_stream::*};

// NOTE: Code gen fails with empty structs.
pub struct Player
{
    dummy:i32
}

impl Player
{
    pub fn new() -> Player { Player { dummy: 0 } }

    fn signal_to_stop()
    {
        // This should only happen once on new run.
        if TXRX.read().unwrap().is_none()
        {
            let mut txrx = TXRX.write().unwrap();
            *txrx = Some(unbounded());
        }
        
        // If there are any threads in existence that were spawned when calling open(),
        // they will read this value and break the decode loop if it is `true`.
        // This closes the thread and the cpal stream.
        { // Use a new scope here so that the lock is dropped.
            let txrx = TXRX.read().unwrap();
            let tx = &txrx.as_ref().unwrap().0;
            tx.send(true).unwrap();
        }

        // After all the threads have been stopped, a new tx and rx is created.
        // This will reset the `true` signal.
        let mut txrx = TXRX.write().unwrap();
        *txrx = Some(unbounded());
    }

    // ---------------------------------
    //          SETTERS/GETTERS
    // ---------------------------------

    pub fn playback_state_stream(stream:StreamSink<i32>) { playback_state_stream(stream); }
    pub fn progress_state_stream(stream:StreamSink<ProgressState>) { progress_state_stream(stream); }

    pub fn is_playing(&self) -> bool
    { IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst) }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Opens a file or network resource for reading and playing.
    pub fn open(&self, path:String)
    {
        Self::signal_to_stop();

        let source:Box<dyn MediaSource> = if path.contains("http") {
            Box::new(Self::get_bytes_from_network(path))
        } else { Box::new(File::open(path).unwrap()) };

        Self::internal_play();
        thread::spawn(|| {
            Decoder::default().open_stream(source);
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

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    fn internal_play()
    {
        update_playback_state_stream(crate::dart_streams::playback_state_stream::PLAY);
        IS_PLAYING.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    
    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    fn internal_pause()
    {
        update_playback_state_stream(crate::dart_streams::playback_state_stream::PAUSE);
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    /// This stops all threads that are streaming.
    fn internal_stop()
    {
        update_playback_state_stream(crate::dart_streams::playback_state_stream::DONE);
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
        Self::signal_to_stop();
    }

    // ---------------------------------
    //             CONTROLS
    // ---------------------------------

    pub fn play(&self)
    { Self::internal_play(); }

    pub fn pause(&self)
    { Self::internal_pause(); }

    pub fn stop(&self)
    { Self::internal_stop(); }

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
        player.set_volume(0.5);
        //player.open("/home/erikas/Music/test.mp3".to_string());
        // player.seek(30.0);
        // thread::sleep(Duration::from_secs(2));
        // player.open("/home/erikas/Music/test.mp3".to_string());
        // thread::sleep(Duration::from_secs(10));
        player.open("/home/erikas/Music/wavy.mp3".to_string());
        thread::sleep(Duration::from_secs(1));
        println!("now");
        player.seek(150.0);
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn open_network_and_play()
    {
        let player = crate::Player::new();
        player.open("".to_string());
    }
}