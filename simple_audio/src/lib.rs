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

mod audio;
pub mod error;
pub mod types;
mod utils;

// https://github.com/RustAudio/cpal/issues/720#issuecomment-1311813294
#[cfg(target_os = "android")]
#[no_mangle]
extern "C" fn JNI_OnLoad(vm: jni::JavaVM, res: *mut std::os::raw::c_void) -> jni::sys::jint
{
    use std::ffi::c_void;

    let vm = vm.get_java_vm_pointer() as *mut c_void;
    unsafe {
        ndk_context::initialize_android_context(vm, res);
    }
    jni::JNIVersion::V6.into()
}

use std::{
    fs::File,
    sync::{atomic::AtomicBool, Arc},
    thread,
};

use anyhow::Context;
use crossbeam::channel::Receiver;
use error::Error;
use std::time::Duration;
use symphonia::core::io::MediaSource;
use types::*;

use audio::{
    controls::{Controls, DecoderEvent},
    decoder::Decoder,
};

pub struct Player
{
    controls: Controls,
    pub event_receiver: Receiver<PlayerEvent>,
}

impl Player
{
    /// Creates a new player with a thread for decoding.
    ///
    /// * `thread_killer` - A Receiver that the decoding thread listens to in order to close.
    pub fn new(thread_killer: Receiver<bool>) -> Player
    {
        let player_controls = Controls::default();

        // Start the decoding thread.
        thread::spawn({
            let controls = player_controls.clone();
            move || {
                let decoder = Decoder::new(controls, thread_killer);
                decoder.start();
            }
        });

        let event_receiver = player_controls.player_event_handler().1.clone();

        Player {
            controls: player_controls,
            event_receiver,
        }
    }

    // ---------------------------------
    //          GETTERS
    // ---------------------------------

    pub fn playback_state(&self) -> PlaybackState
    {
        *self.controls.playback_state()
    }

    pub fn progress(&self) -> ProgressState
    {
        *self.controls.progress()
    }

    /// Returns `true` if there is a file preloaded for playback.
    pub fn is_preloaded(&self) -> bool
    {
        self.controls.is_preloaded()
    }

    pub fn is_looping(&self) -> bool
    {
        self.controls.is_looping()
    }

    pub fn is_normalizing(&self) -> bool
    {
        self.controls.is_normalizing()
    }

    pub fn volume(&self) -> f32
    {
        *self.controls.volume()
    }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Returns a Symphonia `MediaSource` for playback.
    fn source_from_path(
        &self,
        path: String,
        #[allow(unused_variables)] buffer_signal: Arc<AtomicBool>,
    ) -> anyhow::Result<Box<dyn MediaSource>>
    {
        let path2 = path.clone();

        let source: Box<dyn MediaSource> = if path.contains("http") {
            let _file: Box<dyn MediaSource> = if path.contains("m3u") {
                #[cfg(not(feature = "hls_streaming"))]
                panic!("HLS streaming feature not enabled.");

                #[cfg(feature = "hls_streaming")]
                Box::new(
                    audio::sources::hls::HlsStream::new(
                        path,
                        buffer_signal,
                        self.controls.player_event_handler().0.clone(),
                    )
                    .context(format!("Could not open HLS stream at \"{path2}\""))?,
                )
            }
            else {
                #[cfg(not(feature = "http_streaming"))]
                panic!("HTTP streaming feature not enabled.");

                #[cfg(feature = "http_streaming")]
                Box::new(
                    audio::sources::http::HttpStream::new(
                        path,
                        buffer_signal,
                        self.controls.player_event_handler().0.clone(),
                    )
                    .context(format!("Could not open HTTP stream at \"{path2}\""))?,
                )
            };

            #[allow(unreachable_code)]
            _file
        }
        else {
            let file = File::open(path).context(format!("Could not open file at \"{path2}\""))?;
            Box::new(file)
        };

        Ok(source)
    }

    /// Opens a file or network resource for reading and playing.
    pub fn open(&self, path: String, autoplay: bool) -> Result<(), Error>
    {
        let buffer_signal = Arc::new(AtomicBool::new(false));
        let source = match self.source_from_path(path, buffer_signal.clone()) {
            Ok(source) => source,
            Err(err) => return Err(Error::Open(err.to_string())),
        };

        let send_event = self
            .controls
            .decoder_event_handler()
            .0
            .send(DecoderEvent::Open(source, buffer_signal));

        if let Err(err) = send_event {
            return Err(Error::Open(err.to_string()));
        }

        if autoplay {
            self.play();
        }
        else {
            self.pause();
        }

        Ok(())
    }

    /// Preloads a file or network resource for playback.
    /// The preloaded file is automatically played when the current file is finished playing.
    ///
    /// Use this method if you want gapless playback. It reduces
    /// the time spent loading between tracks (especially important
    /// for streaming network files).
    pub fn preload(&self, path: String) -> Result<(), Error>
    {
        let buffer_signal = Arc::new(AtomicBool::new(false));
        let source = match self.source_from_path(path, buffer_signal.clone()) {
            Ok(source) => source,
            Err(err) => return Err(Error::Preload(err.to_string())),
        };

        let send_event = self
            .controls
            .decoder_event_handler()
            .0
            .send(DecoderEvent::Preload(source, buffer_signal));

        match send_event {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Preload(err.to_string())),
        }
    }

    /// Plays the preloaded file.
    pub fn play_preload(&self)
    {
        self.controls
            .decoder_event_handler()
            .0
            .send(DecoderEvent::PlayPreload)
            .unwrap();
    }

    /// Clears the preloaded file so that it doesn't play when the current file is finished.
    pub fn clear_preload(&self)
    {
        self.controls
            .decoder_event_handler()
            .0
            .send(DecoderEvent::ClearPreload)
            .unwrap();
    }

    pub fn play(&self)
    {
        if matches!(*(self.controls.playback_state()), PlaybackState::Play) {
            return;
        }

        self.controls
            .decoder_event_handler()
            .0
            .send(DecoderEvent::Play)
            .unwrap();
        self.controls.set_playback_state(PlaybackState::Play);
    }

    pub fn pause(&self)
    {
        if matches!(*(self.controls.playback_state()), PlaybackState::Pause) {
            return;
        }

        self.controls
            .decoder_event_handler()
            .0
            .send(DecoderEvent::Pause)
            .unwrap();
        self.controls.set_playback_state(PlaybackState::Pause);
    }

    pub fn stop(&self)
    {
        if matches!(*(self.controls.playback_state()), PlaybackState::Stop) {
            return;
        }

        self.controls
            .decoder_event_handler()
            .0
            .send(DecoderEvent::Stop)
            .unwrap();

        let progress = ProgressState {
            position: Duration::ZERO,
            duration: Duration::ZERO,
        };

        self.controls.set_progress(progress);
        self.controls.set_playback_state(PlaybackState::Stop);
    }

    pub fn seek(&self, position: Duration)
    {
        self.controls.set_seek_ts(Some(position));
    }

    pub fn loop_playback(&self, should_loop: bool)
    {
        self.controls.set_is_looping(should_loop);
    }

    pub fn set_volume(&self, volume: f32)
    {
        self.controls.set_volume(volume);
    }

    pub fn normalize_volume(&self, should_normalize: bool)
    {
        self.controls.set_is_normalizing(should_normalize);
    }
}

#[cfg(test)]
mod tests
{
    use std::{thread, time::Duration};

    use crossbeam::channel::unbounded;

    use crate::*;

    #[test]
    fn open_and_play() -> anyhow::Result<()>
    {
        let thread_killer = unbounded();
        let player = Player::new(thread_killer.1);
        player.set_volume(0.1);
        player.open("/home/erikas/Downloads/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(100));
        Ok(())
    }

    #[test]
    fn open_network_and_play() -> anyhow::Result<()>
    {
        let thread_killer = unbounded();
        let player = Player::new(thread_killer.1);
        // You can change the file extension here for different formats ------v
        // https://docs.espressif.com/projects/esp-adf/en/latest/design-guide/audio-samples.html
        player.open(
            "https://dl.espressif.com/dl/audio/ff-16b-2c-44100hz.mp3".to_string(),
            true,
        )?;
        player.set_volume(0.1);
        thread::sleep(Duration::from_secs(10));
        player.seek(std::time::Duration::from_secs(90));
        thread::sleep(Duration::from_secs(10));
        player.seek(std::time::Duration::from_secs(60));
        thread::sleep(Duration::from_secs(187));
        Ok(())
    }

    #[test]
    fn open_hls_and_play() -> anyhow::Result<()>
    {
        let thread_killer = unbounded();
        let player = Player::new(thread_killer.1);
        player.open("https://cf-hls-media.sndcdn.com/playlist/x7uSGJp4rku7.128.mp3/playlist.m3u8?Policy=eyJTdGF0ZW1lbnQiOlt7IlJlc291cmNlIjoiKjovL2NmLWhscy1tZWRpYS5zbmRjZG4uY29tL3BsYXlsaXN0L3g3dVNHSnA0cmt1Ny4xMjgubXAzL3BsYXlsaXN0Lm0zdTgqIiwiQ29uZGl0aW9uIjp7IkRhdGVMZXNzVGhhbiI6eyJBV1M6RXBvY2hUaW1lIjoxNjc1ODA1NTM2fX19XX0_&Signature=Cd6o8KT6AEoLaIHok~438sourFeoHywCDdG09MS38qxmWLsKyJU-eFHOdh8jccvfPaWfjYkEEqfnpp6EMINXP3f99GAwWFPGMrp43lqz2JAL5MBUAc1plLLm1KV~t5Vy5ON6M1X~Fj6nFV7vdD7mGR84lfeafFmXBP4U4oZATI9GoPrUkEgVtCViDg6kBMVKk77e144LFwzZtkiSHj-S7umU5Qf9r2lDCqYaHVVoWSMtJBWMXoKQZCjdR5e6pqINcRQA-348wX8C9bonQGeoCZ3xRQWPq0ZtznmDKdZ-p91YJL8o4LNSPOMreu-ELsXhoftd7iKpZoG7~YwX2Oxg5A__&Key-Pair-Id=APKAI6TU7MMXM5DG6EPQ".to_string(), true)?;
        player.set_volume(0.1);
        thread::sleep(Duration::from_secs(10));
        player.seek(std::time::Duration::from_secs(90));
        thread::sleep(Duration::from_secs(10));
        player.seek(std::time::Duration::from_secs(60));
        thread::sleep(Duration::from_secs(187));
        Ok(())
    }

    // The following tests are to check the responsiveness.
    #[test]
    fn play_pause() -> anyhow::Result<()>
    {
        let thread_killer = unbounded();
        let player = Player::new(thread_killer.1);
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
        let thread_killer = unbounded();
        let player = Player::new(thread_killer.1);
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
        let thread_killer = unbounded();
        let player = Player::new(thread_killer.1);
        player.set_volume(0.5);
        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        thread::sleep(Duration::from_secs(1));
        println!("Seeking now");
        player.seek(std::time::Duration::from_secs(50));
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn stop() -> anyhow::Result<()>
    {
        let thread_killer = unbounded();
        let player = Player::new(thread_killer.1);
        player.set_volume(0.5);

        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        player.seek(std::time::Duration::from_secs(10));
        thread::sleep(Duration::from_secs(5));
        println!("Stopping now");
        player.stop();
        println!("Playing now");
        player.open("/home/erikas/Music/2.mp3".to_string(), true)?;
        player.stop();
        thread::sleep(Duration::from_millis(50));
        player.open("/home/erikas/Music/1.mp3".to_string(), true)?;
        player.play();
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    #[test]
    fn preload() -> anyhow::Result<()>
    {
        let path = "https://dl.espressif.com/dl/audio/ff-16b-2c-44100hz.mp3".to_string();

        let thread_killer = unbounded();
        let player = Player::new(thread_killer.1);
        player.open(path.clone(), true)?;
        player.set_volume(0.5);
        println!("Preloading");
        player.preload(path.clone())?;
        thread::sleep(Duration::from_secs(5));
        println!("Playing preloaded file.");
        player.stop();
        player.play_preload();
        thread::sleep(Duration::from_secs(10));
        player.stop();
        player.open(path, true)?;
        thread::sleep(Duration::from_secs(10));
        Ok(())
    }

    /// SEE: https://github.com/erikas-taroza/simple_audio/issues/19
    #[test]
    fn silent_adts_file() -> anyhow::Result<()>
    {
        let thread_killer = unbounded();
        let player = Player::new(thread_killer.1);
        player.set_volume(0.1);
        player.open("/home/erikas/Downloads/silent.aac".to_string(), true)?;
        thread::sleep(Duration::from_secs(3));
        Ok(())
    }
}
