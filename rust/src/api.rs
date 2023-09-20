use std::{
    fs::File,
    sync::{atomic::AtomicBool, Arc, RwLock},
    thread,
};

use anyhow::Context;
use crossbeam::channel::unbounded;
use flutter_rust_bridge::{RustOpaque, StreamSink};
use media_controllers::types::{Event, MediaControlAction, Metadata};
use symphonia::core::io::MediaSource;

use crate::{
    audio::{
        controls::{PlayerEvent, THREAD_KILLER},
        decoder::Decoder,
        sources::{hls::HlsStream, http::HttpStream},
    },
    media_controllers,
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
    pub fn new(actions: Vec<MediaControlAction>, dbus_name: String, hwnd: Option<i64>) -> Player
    {
        // Enable logging from Rust to Android logcat.
        // `android_logger::init_once` can safely be called multiple times
        // but will only initialize once.
        #[cfg(all(debug_assertions, target_os = "android"))]
        {
            use android_logger::Config;
            use log::LevelFilter;
            android_logger::init_once(Config::default().with_max_level(LevelFilter::Trace));
        }

        let player_controls = Controls::default();

        media_controllers::init(actions, dbus_name, hwnd, {
            let controls = player_controls.clone();
            move |e| match e {
                Event::Previous => update_callback_stream(Callback::MediaControlSkipPrev),
                Event::Next => update_callback_stream(Callback::MediaControlSkipNext),
                Event::Play => Self::internal_play(&controls),
                Event::Pause => Self::internal_pause(&controls),
                Event::Stop => Self::internal_stop(&controls),
                Event::PlayPause => {
                    if controls.is_playing() {
                        Self::internal_pause(&controls);
                    }
                    else {
                        Self::internal_play(&controls);
                    }
                }
                Event::Seek(position, is_absolute) => {
                    if is_absolute {
                        Self::internal_seek(&controls, position as u64);
                    }
                    else {
                        let progress = controls.progress();
                        if position.is_negative() {
                            Self::internal_seek(
                                &controls,
                                progress.position.saturating_sub(position.unsigned_abs()),
                            );
                        }
                        else {
                            Self::internal_seek(&controls, progress.position + position as u64);
                        }
                    }
                }
            }
        });

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

    /// Stops media controllers and decoder threads.
    pub fn dispose()
    {
        if let Some(thread_killer) = THREAD_KILLER.get() {
            thread_killer.read().unwrap().0.send(true).unwrap();
        }
        // Reset the Linux/Windows media controllers.
        media_controllers::dispose();
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
            Err(err) => return Err(Error::Open(err.to_string())),
        };

        let send_event = self
            .controls
            .event_handler()
            .send(PlayerEvent::Open(source, buffer_signal));

        if let Err(err) = send_event {
            return Err(Error::Open(err.to_string()));
        }

        if autoplay {
            Self::internal_play(&self.controls);
        }
        else {
            Self::internal_pause(&self.controls);
        }

        Ok(())
    }

    /// Preloads a file or network resource for reading and playing.
    ///
    /// Use this method if you want gapless playback. It reduces
    /// the time spent loading between tracks (especially important
    /// for streaming network files).
    pub fn preload(&self, path: String) -> anyhow::Result<()>
    {
        let buffer_signal = Arc::new(AtomicBool::new(false));
        let source = Self::source_from_path(path, buffer_signal.clone())?;

        self.controls
            .event_handler()
            .send(PlayerEvent::Preload(source, buffer_signal))?;

        Ok(())
    }

    /// Plays the preloaded item from `preload`. The file starts playing automatically.
    pub fn play_preload(&self) -> anyhow::Result<()>
    {
        self.controls
            .event_handler()
            .send(PlayerEvent::PlayPreload)?;
        Ok(())
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
        media_controllers::set_playback_state(PlaybackState::Play);
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
        media_controllers::set_playback_state(PlaybackState::Pause);
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
            position: 0,
            duration: 0,
        };

        update_progress_state_stream(progress);
        controls.set_progress(progress);
        update_playback_state_stream(PlaybackState::Pause);
        controls.set_is_playing(false);
        controls.set_is_stopped(true);
        media_controllers::set_playback_state(PlaybackState::Pause);
    }

    fn internal_seek(controls: &Controls, seconds: u64)
    {
        controls.set_seek_ts(Some(seconds));
        update_progress_state_stream(ProgressState {
            position: seconds,
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

    pub fn seek(&self, seconds: u64)
    {
        Self::internal_seek(&self.controls, seconds);
    }

    pub fn set_metadata(&self, metadata: Metadata)
    {
        media_controllers::set_metadata(metadata);
    }

    pub fn normalize_volume(&self, should_normalize: bool)
    {
        self.controls.set_is_normalizing(should_normalize);
    }
}

impl Default for Player
{
    fn default() -> Self
    {
        Player::new(
            vec![
                MediaControlAction::Rewind,
                MediaControlAction::SkipPrev,
                MediaControlAction::PlayPause,
                MediaControlAction::SkipNext,
                MediaControlAction::FastForward,
            ],
            "com.erikas.SimpleAudio".to_string(),
            None,
        )
    }
}
