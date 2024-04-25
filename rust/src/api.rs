use std::{
    fs::File,
    sync::{atomic::AtomicBool, Arc, RwLock},
    thread,
};

use anyhow::Context;
use chrono::Duration;
use crossbeam::channel::unbounded;
use flutter_rust_bridge::{RustOpaque, StreamSink};
use symphonia::core::io::MediaSource;

use crate::{
    audio::{
        controls::{PlayerEvent, THREAD_KILLER},
        decoder::Decoder,
        sources::{hls::HlsStream, http::HttpStream},
    },
    utils::{
        callback_stream::*, error::Error, playback_state_stream::*, progress_state_stream::*,
        types::*,
    },
};

pub use crate::audio::controls::Controls;

pub struct Player
{
    pub(crate) controls: RustOpaque<Controls>,
}

impl Player
{
    pub fn new() -> Player
    {
        // Enable logging from Rust to Android logcat.
        // `android_logger::init_once` can safely be called multiple times
        // but will only initialize once.
        #[cfg(all(debug_assertions, target_os = "android"))]
        {
            use android_logger::Config;
            use log::LevelFilter;
            android_logger::init_once(Config::default().with_max_level(LevelFilter::Debug));
        }

        let player_controls = Controls::default();

        *THREAD_KILLER
            .get_or_init(|| RwLock::new(unbounded()))
            .write()
            .unwrap() = unbounded();

        // Start the decoding thread.
        thread::spawn({
            let controls = player_controls.clone();
            move || {
                let decoder = Decoder::new(controls);
                decoder.start();
            }
        });

        Player {
            controls: RustOpaque::new(player_controls),
        }
    }

    /// Stops the decoder thread.
    pub fn dispose()
    {
        if let Some(thread_killer) = THREAD_KILLER.get() {
            thread_killer.read().unwrap().0.send(true).unwrap();
        }
    }

    // ---------------------------------
    //          SETTERS/GETTERS
    // ---------------------------------

    pub fn playback_state_stream(stream: StreamSink<PlaybackState>)
    {
        playback_state_stream(stream);
    }

    pub fn progress_state_stream(stream: StreamSink<ProgressState>)
    {
        progress_state_stream(stream);
    }

    pub fn callback_stream(stream: StreamSink<Callback>)
    {
        callback_stream(stream);
    }

    pub fn is_playing(&self) -> bool
    {
        self.controls.is_playing()
    }

    /// Returns `true` if there is a file preloaded for playback.
    pub fn has_preloaded(&self) -> bool
    {
        self.controls.is_file_preloaded()
    }

    pub fn get_progress(&self) -> ProgressState
    {
        *self.controls.progress()
    }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Returns a Symphonia `MediaSource` for playback.
    fn source_from_path(
        path: String,
        buffer_signal: Arc<AtomicBool>,
    ) -> anyhow::Result<Box<dyn MediaSource>>
    {
        let path2 = path.clone();

        let source: Box<dyn MediaSource> = if path.contains("http") {
            if path.contains("m3u") {
                Box::new(
                    HlsStream::new(path, buffer_signal)
                        .context(format!("Could not open HLS stream at \"{path2}\""))?,
                )
            }
            else {
                Box::new(
                    HttpStream::new(path, buffer_signal)
                        .context(format!("Could not open HTTP stream at \"{path2}\""))?,
                )
            }
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
        let source = match Self::source_from_path(path, buffer_signal.clone()) {
            Ok(source) => source,
            Err(err) => {
                return Err(Error::Open {
                    message: err.to_string(),
                })
            }
        };

        let send_event = self
            .controls
            .event_handler()
            .send(PlayerEvent::Open(source, buffer_signal));

        if let Err(err) = send_event {
            return Err(Error::Open {
                message: err.to_string(),
            });
        }

        if autoplay {
            Self::internal_play(&self.controls);
        }
        else {
            Self::internal_pause(&self.controls);
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
        let source = match Self::source_from_path(path, buffer_signal.clone()) {
            Ok(source) => source,
            Err(err) => {
                return Err(Error::Preload {
                    message: err.to_string(),
                })
            }
        };

        let send_event = self
            .controls
            .event_handler()
            .send(PlayerEvent::Preload(source, buffer_signal));

        match send_event {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Preload {
                message: err.to_string(),
            }),
        }
    }

    /// Plays the preloaded file.
    pub fn play_preload(&self)
    {
        self.controls
            .event_handler()
            .send(PlayerEvent::PlayPreload)
            .unwrap();
    }

    /// Clears the preloaded file so that it doesn't play when the current file is finished.
    pub fn clear_preload(&self)
    {
        self.controls
            .event_handler()
            .send(PlayerEvent::ClearPreload)
            .unwrap();
    }

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    pub(crate) fn internal_play(controls: &Controls)
    {
        if controls.is_playing() {
            return;
        }

        controls.event_handler().send(PlayerEvent::Play).unwrap();

        update_playback_state_stream(PlaybackState::Play);
        controls.set_is_playing(true);
        controls.set_is_stopped(false);
    }

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    pub(crate) fn internal_pause(controls: &Controls)
    {
        if !controls.is_playing() {
            return;
        }

        controls.event_handler().send(PlayerEvent::Pause).unwrap();

        update_playback_state_stream(PlaybackState::Pause);
        controls.set_is_playing(false);
        controls.set_is_stopped(false);
    }

    /// Allows for access in other places
    /// where we would want to update the stream and
    /// the `IS_PLAYING` AtomicBool.
    /// This stops all threads that are streaming.
    fn internal_stop(controls: &Controls)
    {
        if controls.is_stopped() {
            return;
        }

        controls.event_handler().send(PlayerEvent::Stop).unwrap();

        let progress = ProgressState {
            position: Duration::zero(),
            duration: Duration::zero(),
        };

        update_progress_state_stream(progress);
        controls.set_progress(progress);
        update_playback_state_stream(PlaybackState::Stop);
        controls.set_is_playing(false);
        controls.set_is_stopped(true);
    }

    fn internal_seek(controls: &Controls, position: Duration)
    {
        controls.set_seek_ts(Some(position));
        update_progress_state_stream(ProgressState {
            position: position,
            duration: controls.progress().duration,
        });
    }

    // ---------------------------------
    //             CONTROLS
    // ---------------------------------

    pub fn play(&self)
    {
        Self::internal_play(&self.controls);
    }

    pub fn pause(&self)
    {
        Self::internal_pause(&self.controls);
    }

    pub fn stop(&self)
    {
        Self::internal_stop(&self.controls);
    }

    pub fn loop_playback(&self, should_loop: bool)
    {
        self.controls.set_is_looping(should_loop);
    }

    pub fn set_volume(&self, volume: f32)
    {
        self.controls.set_volume(volume);
    }

    pub fn seek(&self, position: Duration)
    {
        Self::internal_seek(&self.controls, position);
    }

    pub fn normalize_volume(&self, should_normalize: bool)
    {
        self.controls.set_is_normalizing(should_normalize);
    }
}
