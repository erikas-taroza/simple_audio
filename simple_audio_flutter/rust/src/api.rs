use chrono::Duration;
use crossbeam::channel::{unbounded, Receiver, Sender};
use flutter_rust_bridge::{frb, RustOpaque, StreamSink};
pub use simple_audio::{
    error::Error,
    types::{PlaybackState, ProgressState},
};
use simple_audio::{types::PlayerEvent, Player};

use std::{sync::OnceLock, thread};

use crate::streams::*;

static EVENT_THREAD_KILLER: OnceLock<(Sender<bool>, Receiver<bool>)> = OnceLock::new();
static PLAYER_THREAD_KILLER: OnceLock<(Sender<bool>, Receiver<bool>)> = OnceLock::new();

pub struct PlayerWrapper
{
    pub(crate) internal: RustOpaque<Player>,
}

impl PlayerWrapper
{
    pub fn new() -> PlayerWrapper
    {
        let _ = EVENT_THREAD_KILLER.set(unbounded());
        let _ = PLAYER_THREAD_KILLER.set(unbounded());

        let internal = Player::new(PLAYER_THREAD_KILLER.get().unwrap().1.clone());

        thread::spawn({
            let event_receiver = internal.event_receiver.clone();
            let event_thread_killer = EVENT_THREAD_KILLER.get().unwrap().1.clone();
            move || loop {
                if let Ok(should_stop) = event_thread_killer.try_recv() {
                    if should_stop {
                        println!("Stopped");
                        break;
                    }
                }

                if let Ok(event) = event_receiver.recv() {
                    match event {
                        PlayerEvent::Playback(playback) => todo!(),
                        PlayerEvent::Progress(progress) => todo!(),
                        PlayerEvent::Error(error) => todo!(),
                    }
                }
            }
        });

        PlayerWrapper {
            internal: RustOpaque::new(internal),
        }
    }

    pub fn dispose()
    {
        if let Some(thread_killer) = EVENT_THREAD_KILLER.get() {
            thread_killer.0.send(true).unwrap();
        }

        if let Some(thread_killer) = PLAYER_THREAD_KILLER.get() {
            thread_killer.0.send(true).unwrap();
        }
    }

    pub fn playback_state_stream(stream: StreamSink<PlaybackState>)
    {
        playback_state_stream(stream);
    }

    pub fn progress_state_stream(stream: StreamSink<ProgressState>)
    {
        progress_state_stream(stream);
    }

    pub fn error_stream(stream: StreamSink<Error>)
    {
        error_stream(stream);
    }

    pub fn playback_state(&self) -> PlaybackState
    {
        self.internal.playback_state()
    }

    pub fn progress(&self) -> ProgressState
    {
        self.internal.progress()
    }

    /// Returns `true` if there is a file preloaded for playback.
    pub fn is_preloaded(&self) -> bool
    {
        self.internal.is_preloaded()
    }

    pub fn is_looping(&self) -> bool
    {
        self.internal.is_looping()
    }

    pub fn is_normalizing(&self) -> bool
    {
        self.internal.is_normalizing()
    }

    pub fn volume(&self) -> f32
    {
        self.internal.volume()
    }

    pub fn open(&self, path: String, autoplay: bool) -> Result<(), Error>
    {
        self.internal.open(path, autoplay)
    }

    pub fn preload(&self, path: String) -> Result<(), Error>
    {
        self.internal.preload(path)
    }

    pub fn play_preload(&self)
    {
        self.internal.play_preload()
    }

    pub fn clear_preload(&self)
    {
        self.internal.clear_preload()
    }

    pub fn play(&self)
    {
        self.internal.play()
    }

    pub fn pause(&self)
    {
        self.internal.pause()
    }

    pub fn stop(&self)
    {
        self.internal.stop()
    }

    pub fn loop_playback(&self, should_loop: bool)
    {
        self.internal.loop_playback(should_loop)
    }

    pub fn set_volume(&self, volume: f32)
    {
        self.internal.set_volume(volume)
    }

    pub fn seek(&self, position: Duration)
    {
        self.internal
            .seek(position.to_std().unwrap_or(std::time::Duration::ZERO))
    }

    pub fn normalize_volume(&self, should_normalize: bool)
    {
        self.internal.normalize_volume(should_normalize)
    }
}

#[frb(mirror(PlaybackState))]
pub enum _PlaybackState
{
    /// The player started playing the file.
    Started(Duration),
    /// The player is currently playing the file.
    Play,
    /// The player is currently paused and there is no output.
    Pause,
    /// The player has finished playing the file.
    Done,
    /// The player was stopped
    Stop,
    /// The player has automatically started playing the preloaded file.
    PreloadPlayed,
}

#[frb(mirror(ProgressState))]
pub struct _ProgressState
{
    /// The position of the player.
    pub position: Duration,
    /// The duration of the file that is being played.
    pub duration: Duration,
}

#[frb(mirror(Error))]
pub enum _Error
{
    /// An error occurred when trying to fetch more bytes for
    /// a network stream.
    NetworkStream(String),
    /// An error occurred when decoding the file.
    Decode(String),
    /// An error occurred when trying to open a file.
    Open(String),
    /// An error occurred when trying to preload a file.
    Preload(String),
}
