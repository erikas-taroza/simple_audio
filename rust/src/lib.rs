mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod utils;
mod audio;

use std::{fs::File, io::Cursor, thread};

use audio::{decoder::Decoder, controls::*};
use crossbeam::channel::bounded;
use flutter_rust_bridge::StreamSink;
use reqwest::blocking::Client;
use symphonia::core::io::MediaSource;

use crate::utils::{playback_state_stream::*, progress_state_stream::*};

// NOTE: Code gen fails with empty structs.
pub struct Player
{
    dummy:i32
}

impl Player
{
    pub fn new() -> Player
    {
        Player { dummy: 0 }
    }

    fn signal_to_stop()
    {
        // If there are any threads in existence that were spawned when calling open(),
        // they will read this value and break the decode loop if it is `true`.
        // This closes the thread and the cpal stream.
        if let Some(txrx) = &*TXRX.read().unwrap()
        { txrx.0.send(true).unwrap(); }

        // After all the threads have been stopped, a new tx and rx is created.
        // This will reset the `true` signal.
        let mut txrx = TXRX.write().unwrap();
        *txrx = Some(bounded(1));
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
    pub fn open(&self, path:String, autoplay:bool)
    {
        Self::internal_stop();

        let source:Box<dyn MediaSource> = if path.contains("http") {
            Box::new(Self::get_bytes_from_network(path))
        } else { Box::new(File::open(path).unwrap()) };

        if autoplay { Self::internal_play(); }
        else { Self::internal_pause(); }

        thread::spawn(move || {
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
        update_playback_state_stream(crate::utils::playback_state_stream::PLAY);
        IS_PLAYING.store(true, std::sync::atomic::Ordering::SeqCst);
    }
    
    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    fn internal_pause()
    {
        update_playback_state_stream(crate::utils::playback_state_stream::PAUSE);
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.)
    /// This stops all threads that are streaming.
    fn internal_stop()
    {
        Self::signal_to_stop();
        update_progress_state_stream(ProgressState { position: 0, duration: 0 });
        update_playback_state_stream(crate::utils::playback_state_stream::PAUSE);
        DURATION.store(0, std::sync::atomic::Ordering::SeqCst);
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
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

    pub fn seek(&self, seconds:u64)
    {
        *SEEK_TS.write().unwrap() = Some(seconds);
        update_progress_state_stream(ProgressState {
            position: seconds,
            duration: DURATION.load(std::sync::atomic::Ordering::SeqCst)
        });
    }
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
        player.open("/home/erikas/Music/test2.mp3".to_string(), true);
        player.seek(30);
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn open_network_and_play()
    {
        let player = crate::Player::new();
        player.open("https://github.com/anars/blank-audio/blob/master/1-minute-of-silence.mp3?raw=true".to_string(), true);
        thread::sleep(Duration::from_secs(10));
    }

    // The following tests are to check the responsiveness.
    #[test]
    fn play_pause()
    {
        let player = crate::Player::new();
        player.set_volume(0.5);

        player.open("/home/erikas/Music/test2.mp3".to_string(), true);
        thread::sleep(Duration::from_secs(1));
        println!("Pausing now");
        player.pause();
        thread::sleep(Duration::from_secs(5));
        println!("Playing now");
        player.play();
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn volume()
    {
        let player = crate::Player::new();
        player.open("/home/erikas/Music/test2.mp3".to_string(), true);
        thread::sleep(Duration::from_secs(1));
        println!("Changing volume now");
        player.set_volume(0.2);
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn seeking()
    {
        let player = crate::Player::new();
        player.set_volume(0.5);
        player.open("/home/erikas/Music/test2.mp3".to_string(), true);
        thread::sleep(Duration::from_secs(1));
        println!("Seeking now");
        player.seek(50);
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn stop()
    {
        let player = crate::Player::new();
        player.set_volume(0.5);

        player.open("/home/erikas/Music/test2.mp3".to_string(), true);
        player.seek(10);
        thread::sleep(Duration::from_secs(5));
        println!("Stopping now");
        player.stop();
        thread::sleep(Duration::from_secs(5));
        println!("Playing now");
        player.open("/home/erikas/Music/test.mp3".to_string(), true);
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn mpris()
    {
        // let _mpris = crate::utils::mpris::Mpris::new(
        //     "test".to_string(), "Test".to_string(), 
        //     |event| {
        //         println!("Event: {event:?}")
        //     }
        // );

        let player = crate::Player::new();
        player.set_volume(0.5);

        thread::sleep(Duration::from_secs(10));
    }
}