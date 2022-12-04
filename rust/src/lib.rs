mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod utils;
mod audio;
mod metadata;

use std::{fs::File, io::Cursor, thread};

use audio::{decoder::Decoder, controls::*};
use crossbeam::channel::bounded;
use flutter_rust_bridge::StreamSink;
use metadata::types::{Metadata, Actions};
use reqwest::blocking::Client;
use symphonia::core::io::MediaSource;

use crate::utils::{playback_state_stream::*, progress_state_stream::*, metadata_callback_stream::*};

// NOTE: Code gen fails with empty structs.
pub struct Player
{
    dummy:i32
}

impl Player
{
    pub fn new(
        actions:Vec<i32>,
        mpris_name:String,
        hwnd:Option<i64>
    ) -> Player
    {
        crate::metadata::init(
            actions.iter().map(|i| {
                Actions::from(*i)
            }).collect::<Vec<Actions>>(),
            mpris_name,
            hwnd,
            |e| {
                match e
                {
                    metadata::types::Event::Previous => update_metadata_callback_stream(false),
                    metadata::types::Event::Next => update_metadata_callback_stream(true),
                    metadata::types::Event::Play => Self::internal_play(),
                    metadata::types::Event::Pause => Self::internal_pause(),
                    metadata::types::Event::Stop => Self::internal_stop(),
                    metadata::types::Event::PlayPause => {
                        if IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst)
                        { Self::internal_pause(); }
                        else { Self::internal_play(); }
                    },
                    metadata::types::Event::Seek(position, is_absolute) => {
                        if is_absolute
                        { Self::internal_seek(position as u64); }
                        else
                        {
                            let progress = PROGRESS.read().unwrap();
                            if position.is_negative()
                            { Self::internal_seek(progress.position.saturating_sub(position.abs() as u64)); }
                            else
                            { Self::internal_seek(progress.position + position as u64); }
                        }
                    }
                }
            }
        );

        Player { dummy: 0 }
    }

    fn signal_to_stop()
    {
        // If there are any threads in existence that were spawned when calling open(),
        // they will read this value and break the decode loop if it is `true`.
        // This closes the thread and the cpal stream.
        if let Some(txrx) = &*TXRX.read().unwrap()
        { txrx.0.send(ThreadMessage::Quit).unwrap(); }

        // After all the threads have been stopped, a new tx and rx is created.
        // This will reset the `true` signal.
        let mut txrx = TXRX.write().unwrap();
        *txrx = Some(bounded(10));
    }

    // ---------------------------------
    //          SETTERS/GETTERS
    // ---------------------------------

    pub fn playback_state_stream(stream:StreamSink<i32>) { playback_state_stream(stream); }
    pub fn progress_state_stream(stream:StreamSink<ProgressState>) { progress_state_stream(stream); }
    pub fn metadata_callback_stream(stream:StreamSink<bool>) { metadata_callback_stream(stream); }

    pub fn is_playing(&self) -> bool
    { IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst) }

    pub fn get_progress(&self) -> ProgressState
    { PROGRESS.read().unwrap().clone() }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Opens a file or network resource for reading and playing.
    pub fn open(&self, path:String, autoplay:bool)
    {
        let source:Box<dyn MediaSource> = if path.contains("http") {
            if path.contains("m3u") { Box::new(Self::open_m3u(path)) }
            // Everything but m3u/m3u8
            else { Box::new(Cursor::new(Self::get_bytes_from_network(path))) }
        } else { Box::new(File::open(path).unwrap()) };

        Self::internal_stop();

        if autoplay { Self::internal_play(); }
        else { Self::internal_pause(); }

        thread::spawn(move || {
            Decoder::default().open_stream(source);
        });
    }

    fn get_bytes_from_network(url:String) -> Vec<u8>
    {
        let response = Client::new().get(url.clone())
            .header("Range", "bytes=0-")
            .send()
            .expect(format!("ERR: Could not open {url}").as_str());
            
        response.bytes().unwrap().to_vec()
    }

    /// This doesn't support all m3u files. It only supports files that have parts
    /// of a whole song in mp3 format. For example:
    /// - https://some-domain.com/part1.mp3
    /// - https://some-domain.com/part2.mp3
    /// - https://some-domain.com/part3.mp3
    /// 
    /// Which then gets combined into a single byte array.
    fn open_m3u(url:String) -> Cursor<Vec<u8>>
    {
        let mut total_data = Vec::new();

        let m3u_content = Client::new().get(url.clone())
            .header("Range", "bytes=0-")
            .send()
            .expect(format!("ERR: Could not open {url}").as_str())
            .text().expect("ERR: Could not read content of M3U file.");

        for line in m3u_content.split("\n").collect::<Vec<&str>>()
        {
            if !line.contains("http") { continue; }
            total_data.append(&mut Self::get_bytes_from_network(line.to_string()));
        }

        Cursor::new(total_data)
    }

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    fn internal_play()
    {
        if IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst) { return; }

        if let Some(txrx) = &*TXRX.read().unwrap()
        { txrx.0.send(ThreadMessage::Play).unwrap(); }

        update_playback_state_stream(utils::playback_state::PlaybackState::Play);
        IS_PLAYING.store(true, std::sync::atomic::Ordering::SeqCst);
        crate::metadata::set_playback_state(utils::playback_state::PlaybackState::Play);
    }
    
    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    fn internal_pause()
    {
        if !IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst) { return; }

        if let Some(txrx) = &*TXRX.read().unwrap()
        { txrx.0.send(ThreadMessage::Pause).unwrap(); }

        update_playback_state_stream(utils::playback_state::PlaybackState::Pause);
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
        crate::metadata::set_playback_state(utils::playback_state::PlaybackState::Pause);
    }

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.)
    /// This stops all threads that are streaming.
    fn internal_stop()
    {
        Self::signal_to_stop();
        update_progress_state_stream(ProgressState { position: 0, duration: 0 });
        update_playback_state_stream(utils::playback_state::PlaybackState::Pause);
        *PROGRESS.write().unwrap() = ProgressState { position: 0, duration: 0 };
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
        crate::metadata::set_playback_state(utils::playback_state::PlaybackState::Pause);
    }

    fn internal_seek(seconds:u64)
    {
        *SEEK_TS.write().unwrap() = Some(seconds);
        update_progress_state_stream(ProgressState {
            position: seconds,
            duration: PROGRESS.read().unwrap().duration
        });
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
    { Self::internal_seek(seconds); }

    pub fn set_metadata(&self, metadata:Metadata)
    { crate::metadata::set_metadata(metadata); }
}

impl Default for Player
{
    fn default() -> Self {
        crate::Player::new(
            vec![0, 1, 2, 3, 4],
            "SimpleAudio".to_string(),
            None
        )
    }
}

#[cfg(test)]
mod tests
{
    use std::{thread, time::Duration};

    use crate::metadata::types::Metadata;

    #[test]
    fn open_and_play()
    {
        let player = crate::Player::default();
        player.set_volume(0.5);
        player.open("/home/erikas/Music/1.mp3".to_string(), true);
        player.seek(30);
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn open_network_and_play()
    {
        let player = crate::Player::default();
        player.open("https://github.com/anars/blank-audio/blob/master/1-minute-of-silence.mp3?raw=true".to_string(), true);
        thread::sleep(Duration::from_secs(10));
    }

    // The following tests are to check the responsiveness.
    #[test]
    fn play_pause()
    {
        let player = crate::Player::default();
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true);
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
        let player = crate::Player::default();
        player.open("/home/erikas/Music/1.mp3".to_string(), true);
        thread::sleep(Duration::from_secs(1));
        println!("Changing volume now");
        player.set_volume(0.2);
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn seeking()
    {
        let player = crate::Player::default();
        player.set_volume(0.5);
        player.open("/home/erikas/Music/1.mp3".to_string(), true);
        thread::sleep(Duration::from_secs(1));
        println!("Seeking now");
        player.seek(50);
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn stop()
    {
        let player = crate::Player::default();
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true);
        player.seek(10);
        thread::sleep(Duration::from_secs(5));
        println!("Stopping now");
        player.stop();
        thread::sleep(Duration::from_secs(5));
        println!("Playing now");
        player.open("/home/erikas/Music/2.mp3".to_string(), true);
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn mpris()
    {
        let player = crate::Player::new(
            vec![2],
            "SimpleAudio".to_string(),
            None
        );
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true);
        player.set_metadata(Metadata {
            title: Some("My Title".to_string()),
            artist: Some("My Artist".to_string()),
            album: Some("My Album".to_string()),
            ..Default::default()
        });
        
        thread::sleep(Duration::from_secs(2));

        player.set_metadata(Metadata {
            title: Some("My Title2".to_string()),
            artist: Some("My Artist2".to_string()),
            album: Some("My Album2".to_string()),
            ..Default::default()
        });

        thread::sleep(Duration::from_secs(10));
    }
}