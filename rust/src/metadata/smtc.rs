#![cfg(target_os = "windows")]

use std::{sync::{Arc, Mutex, RwLock}, time::Duration};

use windows::{Win32::{System::WinRT::ISystemMediaTransportControlsInterop, Foundation::HWND}, Media::{SystemMediaTransportControls, SystemMediaTransportControlsTimelineProperties, SystemMediaTransportControlsDisplayUpdater, MediaPlaybackType, SystemMediaTransportControlsButtonPressedEventArgs, SystemMediaTransportControlsButton, MediaPlaybackStatus, PlaybackPositionChangeRequestedEventArgs}, Foundation::{TypedEventHandler, Uri}, core::HSTRING, Storage::Streams::RandomAccessStreamReference};

use crate::{audio::controls::PROGRESS, utils::playback_state::PlaybackState};

use super::types::{Event, Metadata};

pub static HANDLER:RwLock<Option<Smtc>> = RwLock::new(None);

pub struct Smtc
{
    controls:SystemMediaTransportControls,
    display:SystemMediaTransportControlsDisplayUpdater,
    timeline:SystemMediaTransportControlsTimelineProperties
}

impl Smtc
{
    pub fn new<C>(hwnd:isize, callback:C) -> Self
    where
        C: Fn(Event) + Send + 'static
    {
        let interop:ISystemMediaTransportControlsInterop = windows::core::factory::<
            SystemMediaTransportControls,
            ISystemMediaTransportControlsInterop
        >().unwrap();

        let controls:SystemMediaTransportControls = unsafe { interop.GetForWindow(HWND(hwnd)) }.unwrap();
        let display = controls.DisplayUpdater().unwrap();
        let timeline = SystemMediaTransportControlsTimelineProperties::new().unwrap();

        display.SetType(MediaPlaybackType::Music).unwrap();

        controls.SetIsEnabled(true).unwrap();
        controls.SetIsPlayEnabled(true).unwrap();
        controls.SetIsPauseEnabled(true).unwrap();
        controls.SetIsStopEnabled(true).unwrap();
        controls.SetIsPreviousEnabled(true).unwrap();
        controls.SetIsNextEnabled(true).unwrap();
        controls.SetIsRewindEnabled(true).unwrap();
        controls.SetIsFastForwardEnabled(true).unwrap();

        let callback = Arc::new(Mutex::new(callback));
        
        let button_callback = TypedEventHandler::new({
            let callback = callback.clone();

            move |_, args:&Option<_>| {
                let args:&SystemMediaTransportControlsButtonPressedEventArgs = args.as_ref().unwrap();
                let button = args.Button().unwrap();

                let event = match button
                {
                    SystemMediaTransportControlsButton::Play => Event::Play,
                    SystemMediaTransportControlsButton::Pause => Event::Pause,
                    SystemMediaTransportControlsButton::Stop => Event::Stop,
                    SystemMediaTransportControlsButton::Previous => Event::Previous,
                    SystemMediaTransportControlsButton::Next => Event::Next,
                    SystemMediaTransportControlsButton::Rewind => Event::Seek(-10, false),
                    SystemMediaTransportControlsButton::FastForward => Event::Seek(10, false),
                    _ => return Ok(())
                };

                callback.lock().unwrap()(event);
                Ok(())
            }
        });

        let position_callback = TypedEventHandler::new({
            let callback = callback.clone();

            move |_, args:&Option<_>| {
                let args:&PlaybackPositionChangeRequestedEventArgs = args.as_ref().unwrap();
                let position = Duration::from(args.RequestedPlaybackPosition().unwrap());
                callback.lock().unwrap()(Event::Seek(position.as_secs() as i64, true));

                Ok(())
            }
        });
        
        controls.ButtonPressed(&button_callback).unwrap();
        controls.PlaybackPositionChangeRequested(&position_callback).unwrap();

        Smtc { controls, display, timeline }
    }

    pub fn set_metadata(&self, metadata:Metadata)
    {
        let properties = self.display.MusicProperties().unwrap();

        if let Some(title) = metadata.title
        { properties.SetTitle(&HSTRING::from(title)).unwrap(); }

        if let Some(artist) = metadata.artist
        { properties.SetArtist(&HSTRING::from(artist)).unwrap(); }

        if let Some(album) = metadata.album
        { properties.SetAlbumTitle(&HSTRING::from(album)).unwrap(); }

        if let Some(art_uri) = metadata.art_uri
        {
            let uri = Uri::CreateUri(&HSTRING::from(art_uri)).unwrap();
            let stream = RandomAccessStreamReference::CreateFromUri(&uri).unwrap();
            self.display.SetThumbnail(&stream).unwrap();
        }
        
        self.timeline.SetStartTime(Duration::default().into()).unwrap();
        self.timeline.SetMinSeekTime(Duration::default().into()).unwrap();

        loop
        {
            let progress = PROGRESS.read().unwrap();
            if progress.duration == 0 { continue; }

            self.timeline.SetEndTime(Duration::from_secs(progress.duration).into()).unwrap();
            self.timeline.SetMaxSeekTime(Duration::from_secs(progress.duration).into()).unwrap();
            break;
        }

        self.controls.UpdateTimelineProperties(&self.timeline).unwrap();
        self.display.Update().unwrap();
    }

    pub fn set_playback_state(&self, state:PlaybackState)
    {
        let status:MediaPlaybackStatus = match state
        {
            PlaybackState::Play => MediaPlaybackStatus::Playing,
            PlaybackState::Pause => MediaPlaybackStatus::Paused,
            _ => MediaPlaybackStatus::Paused,
        };

        self.controls.SetPlaybackStatus(status).unwrap();

        self.controls.UpdateTimelineProperties(&self.timeline).unwrap();
    }
}