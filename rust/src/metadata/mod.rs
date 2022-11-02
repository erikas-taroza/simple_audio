use std::sync::RwLock;

use self::{mpris::Mpris, types::{Event, Metadata}};
use crate::utils::playback_state::PlaybackState;

pub mod mpris;
pub mod types;

static HANDLER:RwLock<Option<Mpris>> = RwLock::new(None);

/// Initialize a platform specific metadata handler.
pub fn init<C>(callback:C, dbus_name:String, display_name:String)
where
    C: Fn(Event) + Send + 'static
{
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let mut metadata = HANDLER.write().unwrap();
        *metadata = Some(Mpris::new(dbus_name, display_name, callback));
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn set_metadata(metadata:Metadata)
{
    let mpris = HANDLER.read().unwrap();
    mpris.as_ref().unwrap().set_metadata(metadata);
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn set_playback_state(state:PlaybackState)
{
    let mpris = HANDLER.read().unwrap();
    mpris.as_ref().unwrap().set_playback_state(state);
}