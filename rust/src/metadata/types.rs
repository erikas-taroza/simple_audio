use crate::utils::playback_state::PlaybackState;

#[derive(Default)]
pub struct Metadata
{
    pub title:Option<String>,
    pub artist:Option<String>,
    pub album:Option<String>,
    pub art_url:Option<String>
}

/// Callback events from the MPRIS player.
#[derive(Clone, Copy, Debug)]
pub enum Event
{
    Next,
    Previous,
    Play,
    Pause,
    Stop,
    PlayPause
}

/// Commands to be sent via the thread's channels.
pub enum Command
{
    SetMetadata(Metadata),
    SetPlaybackState(PlaybackState)
}