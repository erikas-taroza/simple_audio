use self::types::{Event, Metadata, Actions};

use crate::utils::playback_state::PlaybackState;

pub mod mpris;
pub mod smtc;
pub mod types;

/// Initialize a platform specific metadata handler.
pub fn init<C>(actions:Vec<Actions>, use_progress_bar:bool, mpris_name:String, hwnd:Option<i64>, callback:C)
where
    C: Fn(Event) + Send + 'static
{   
    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android"), not(target_os = "ios")))]
    {
        let mut mpris = mpris::HANDLER.write().unwrap();
        *mpris = Some(mpris::Mpris::new(actions, use_progress_bar, mpris_name, callback));
    }

    #[cfg(target_os = "windows")]
    {
        let mut smtc = smtc::HANDLER.write().unwrap();
        let new = smtc::Smtc::new(actions, hwnd.unwrap() as isize, callback);
        if new.is_none() { return; }

        *smtc = Some(new.unwrap());
    }
}


pub fn set_metadata(metadata:Metadata)
{
    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android"), not(target_os = "ios")))]
    {
        let mpris = mpris::HANDLER.read().unwrap();
        mpris.as_ref().unwrap().set_metadata(metadata);
    }

    #[cfg(target_os = "windows")]
    {
        let smtc = smtc::HANDLER.read().unwrap();
        smtc.as_ref().unwrap().set_metadata(metadata);
    }
}


pub fn set_playback_state(state:PlaybackState)
{
    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "android"), not(target_os = "ios")))]
    {
        let mpris = mpris::HANDLER.read().unwrap();
        mpris.as_ref().unwrap().set_playback_state(state);
    }

    #[cfg(target_os = "windows")]
    {
        let smtc = smtc::HANDLER.read().unwrap();
        smtc.as_ref().unwrap().set_playback_state(state);
    }
}