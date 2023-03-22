// This file is a part of simple_audio
// Copyright (c) 2022-2023 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of 
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod utils;
mod audio;
mod metadata;

use std::{fs::File, thread};

use anyhow::Context;
use audio::{decoder::Decoder, controls::*, streaming::{http::HttpStream, hls::HlsStream}};
use crossbeam::channel::unbounded;
use flutter_rust_bridge::StreamSink;
use metadata::types::{Metadata, MediaControlAction};
use symphonia::core::io::MediaSource;
use utils::types::*;

use crate::utils::{playback_state_stream::*, progress_state_stream::*, callback_stream::*};

pub struct Player { }

impl Player
{
    pub fn new(
        actions: Vec<MediaControlAction>,
        dbus_name: String,
        hwnd: Option<i64>
    ) -> Player
    {
        crate::metadata::init(
            actions,
            dbus_name,
            hwnd,
            |e| {
                match e
                {
                    metadata::types::Event::Previous => update_callback_stream(Callback::MediaControlSkipPrev),
                    metadata::types::Event::Next => update_callback_stream(Callback::MediaControlSkipNext),
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
                            { Self::internal_seek(progress.position.saturating_sub(position.unsigned_abs())); }
                            else
                            { Self::internal_seek(progress.position + position as u64); }
                        }
                    }
                }
            }
        );

        let mut txrx = TXRX.write().unwrap();
        *txrx = unbounded();

        // Start the decoding thread.
        thread::spawn(|| {
            let decoder = Decoder::new();
            decoder.start();
        });

        Player { }
    }

    /// Stops any old players/threads and resets the
    /// static variables in `controls.rs`.
    pub fn dispose()
    {
        // Stop the working thread.
        TXRX.read().unwrap().0.send(ThreadMessage::Dispose).unwrap();
        // Reset the controls in `controls.rs` to default values.
        reset_controls_to_default();
        audio::streaming::streamable::IS_STREAM_BUFFERING.store(false, std::sync::atomic::Ordering::SeqCst);
        // Reset the Linux/Windows media controllers.
        metadata::dispose();
    }

    // ---------------------------------
    //          SETTERS/GETTERS
    // ---------------------------------

    pub fn playback_state_stream(stream: StreamSink<PlaybackState>) { playback_state_stream(stream); }
    pub fn progress_state_stream(stream: StreamSink<ProgressState>) { progress_state_stream(stream); }
    pub fn callback_stream(stream: StreamSink<Callback>) { callback_stream(stream); }

    pub fn is_playing(&self) -> bool
    { IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst) }

    /// Returns `true` if there is a file queued for playback.
    pub fn has_queue(&self) -> bool
    { IS_FILE_QUEUED.load(std::sync::atomic::Ordering::SeqCst) }

    pub fn get_progress(&self) -> ProgressState
    { *PROGRESS.read().unwrap() }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Returns a Symphonia `MediaSource` for playback.
    fn source_from_path(path: String) -> anyhow::Result<Box<dyn MediaSource>>
    {
        let path2 = path.clone();

        let source: Box<dyn MediaSource> = if path.contains("http") {
            if path.contains("m3u") {
                Box::new(HlsStream::new(path)
                    .context(format!("Could not open HLS stream at \"{path2}\""))?
                )
            }
            else {
                Box::new(HttpStream::new(path)
                    .context(format!("Could not open HTTP stream at \"{path2}\""))?
                )
            }
        } else {
            Box::new(File::open(path)
                .context(format!("Could not open file at \"{path2}\""))?
            )
        };

        Ok(source)
    }

    /// Opens a file or network resource for reading and playing.
    pub fn open(&self, path: String, autoplay: bool) -> anyhow::Result<()>
    {
        let source = Self::source_from_path(path)?;

        IS_STOPPED.store(false, std::sync::atomic::Ordering::SeqCst);
        TXRX.read().unwrap().0.send(ThreadMessage::Open(source))?;

        if autoplay { Self::internal_play(); }
        else { Self::internal_pause(); }

        Ok(())
    }

    /// Preloads a file or network resource for reading and playing.
    /// 
    /// Use this method if you want gapless playback. It reduces
    /// the time spent loading between tracks (especially important
    /// for streaming network files).
    pub fn queue(&self, path: String) -> anyhow::Result<()>
    {
        let source = Self::source_from_path(path)?;
        TXRX.read().unwrap().0.send(ThreadMessage::Queue(source))?;

        Ok(())
    }

    /// Plays the preloaded item from `queue`.
    pub fn play_queue(&self) -> anyhow::Result<()>
    {
        TXRX.read().unwrap().0.send(ThreadMessage::PlayQueue)?;
        Ok(())
    }

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    fn internal_play()
    {
        if IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst) { return; }

        TXRX.read().unwrap().0.send(ThreadMessage::Play).unwrap();

        update_playback_state_stream(PlaybackState::Play);
        IS_PLAYING.store(true, std::sync::atomic::Ordering::SeqCst);
        crate::metadata::set_playback_state(PlaybackState::Play);
    }
    
    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    fn internal_pause()
    {
        if !IS_PLAYING.load(std::sync::atomic::Ordering::SeqCst) { return; }

        TXRX.read().unwrap().0.send(ThreadMessage::Pause).unwrap();

        update_playback_state_stream(PlaybackState::Pause);
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
        crate::metadata::set_playback_state(PlaybackState::Pause);
    }

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    /// This stops all threads that are streaming.
    fn internal_stop()
    {
        if IS_STOPPED.load(std::sync::atomic::Ordering::SeqCst) { return; }

        TXRX.read().unwrap().0.send(ThreadMessage::Stop).unwrap();

        update_progress_state_stream(ProgressState { position: 0, duration: 0 });
        update_playback_state_stream(PlaybackState::Pause);
        *PROGRESS.write().unwrap() = ProgressState { position: 0, duration: 0 };
        IS_PLAYING.store(false, std::sync::atomic::Ordering::SeqCst);
        IS_STOPPED.store(true, std::sync::atomic::Ordering::SeqCst);
        crate::metadata::set_playback_state(PlaybackState::Pause);
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

    pub fn loop_playback(&self, should_loop: bool)
    { IS_LOOPING.store(should_loop, std::sync::atomic::Ordering::SeqCst); }

    pub fn set_volume(&self, volume: f32)
    { *VOLUME.write().unwrap() = volume; }

    pub fn seek(&self, seconds: u64)
    { Self::internal_seek(seconds); }

    pub fn set_metadata(&self, metadata: Metadata)
    { crate::metadata::set_metadata(metadata); }

    pub fn normalize_volume(&self, should_normalize: bool)
    { IS_NORMALIZING.store(should_normalize, std::sync::atomic::Ordering::SeqCst); }
}

impl Default for Player
{
    fn default() -> Self {
        crate::Player::new(
            vec![
                MediaControlAction::Rewind,
                MediaControlAction::SkipPrev,
                MediaControlAction::PlayPause,
                MediaControlAction::SkipNext,
                MediaControlAction::FastForward
            ],
            "com.erikas.SimpleAudio".to_string(),
            None
        )
    }
}

#[cfg(test)]
mod tests
{
    use std::{thread, time::Duration};

    use crate::metadata::types::{Metadata, MediaControlAction};

    #[test]
    fn open_and_play() -> anyhow::Result<()>
    {
        let player = crate::Player::default();
        player.set_volume(0.1);
        player.open("/home/erikas/Downloads/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(100));
        Ok(())
    }

    #[test]
    fn open_network_and_play() -> anyhow::Result<()>
    {
        let player = crate::Player::default();
        // You can change the file extension here for different formats ------v
        // https://docs.espressif.com/projects/esp-adf/en/latest/design-guide/audio-samples.html
        player.open("https://dl.espressif.com/dl/audio/ff-16b-2c-44100hz.mp3".to_string(), true)?;
        player.set_volume(0.1);
        thread::sleep(Duration::from_secs(10));
        player.seek(90);
        thread::sleep(Duration::from_secs(10));
        player.seek(60);
        thread::sleep(Duration::from_secs(187));
        Ok(())
    }

    #[test]
    fn open_hls_and_play() -> anyhow::Result<()>
    {
        let player = crate::Player::default();
        player.open("https://cf-hls-media.sndcdn.com/playlist/x7uSGJp4rku7.128.mp3/playlist.m3u8?Policy=eyJTdGF0ZW1lbnQiOlt7IlJlc291cmNlIjoiKjovL2NmLWhscy1tZWRpYS5zbmRjZG4uY29tL3BsYXlsaXN0L3g3dVNHSnA0cmt1Ny4xMjgubXAzL3BsYXlsaXN0Lm0zdTgqIiwiQ29uZGl0aW9uIjp7IkRhdGVMZXNzVGhhbiI6eyJBV1M6RXBvY2hUaW1lIjoxNjc1ODA1NTM2fX19XX0_&Signature=Cd6o8KT6AEoLaIHok~438sourFeoHywCDdG09MS38qxmWLsKyJU-eFHOdh8jccvfPaWfjYkEEqfnpp6EMINXP3f99GAwWFPGMrp43lqz2JAL5MBUAc1plLLm1KV~t5Vy5ON6M1X~Fj6nFV7vdD7mGR84lfeafFmXBP4U4oZATI9GoPrUkEgVtCViDg6kBMVKk77e144LFwzZtkiSHj-S7umU5Qf9r2lDCqYaHVVoWSMtJBWMXoKQZCjdR5e6pqINcRQA-348wX8C9bonQGeoCZ3xRQWPq0ZtznmDKdZ-p91YJL8o4LNSPOMreu-ELsXhoftd7iKpZoG7~YwX2Oxg5A__&Key-Pair-Id=APKAI6TU7MMXM5DG6EPQ".to_string(), true)?;
        player.set_volume(0.1);
        thread::sleep(Duration::from_secs(10));
        player.seek(90);
        thread::sleep(Duration::from_secs(10));
        player.seek(60);
        thread::sleep(Duration::from_secs(187));
        Ok(())
    }

    // The following tests are to check the responsiveness.
    #[test]
    fn play_pause() -> anyhow::Result<()>
    {
        let player = crate::Player::default();
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(1));
        println!("Pausing now");
        player.pause();
        thread::sleep(Duration::from_secs(5));
        println!("Playing now");
        player.play();
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn volume() -> anyhow::Result<()>
    {
        let player = crate::Player::default();
        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(1));
        println!("Changing volume now");
        player.set_volume(0.2);
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn seeking() -> anyhow::Result<()>
    {
        let player = crate::Player::default();
        player.set_volume(0.5);
        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(1));
        println!("Seeking now");
        player.seek(50);
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn stop() -> anyhow::Result<()>
    {
        let player = crate::Player::default();
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        player.seek(10);
        thread::sleep(Duration::from_secs(5));
        println!("Stopping now");
        player.stop();
        thread::sleep(Duration::from_millis(50));
        println!("Playing now");
        player.open("/home/erikas/Music/2.mp3".to_string(), true)?;
        player.stop();
        thread::sleep(Duration::from_millis(50));
        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn mpris() -> anyhow::Result<()>
    {
        let player = crate::Player::new(
            vec![MediaControlAction::PlayPause],
            "SimpleAudio".to_string(),
            None
        );
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
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
        Ok(())
    }
}