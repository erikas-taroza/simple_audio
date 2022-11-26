use crate::utils::playback_state::PlaybackState;

#[derive(Default)]
pub struct Metadata
{
    pub title:Option<String>,
    pub artist:Option<String>,
    pub album:Option<String>,
    pub art_uri:Option<String>
}

/// Callback events from the media notification.
#[derive(Clone, Copy, Debug)]
pub enum Event
{
    Next,
    Previous,
    Play,
    Pause,
    Stop,
    PlayPause,
    /// `u64`: Position.
    /// 
    /// `bool`: Is absolute.
    /// If `true`, the position is between `0-duration`.
    /// If false, the position can be negative to indicate going backwards.
    Seek(i64, bool)
}

/// Commands to be sent via the thread's channels.
pub enum Command
{
    SetMetadata(Metadata),
    SetPlaybackState(PlaybackState)
}

#[derive(PartialEq, Clone, Copy)]
pub enum Actions
{
    Rewind,
    SkipPrev,
    PlayPause,
    SkipNext,
    FastForward
}

impl From<i32> for Actions
{
    fn from(i:i32) -> Self
    {
        match i
        {
            0 => Self::Rewind,
            1 => Self::SkipPrev,
            2 => Self::PlayPause,
            3 => Self::SkipNext,
            4 => Self::FastForward,
            _ => panic!("ERR: This action is not supported.")
        } 
    }
}