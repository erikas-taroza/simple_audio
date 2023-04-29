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

#![cfg(target_os = "windows")]

use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use windows::{
    core::HSTRING,
    Foundation::{EventRegistrationToken, TypedEventHandler, Uri},
    Media::{
        MediaPlaybackStatus, MediaPlaybackType, PlaybackPositionChangeRequestedEventArgs,
        SystemMediaTransportControls, SystemMediaTransportControlsButton,
        SystemMediaTransportControlsButtonPressedEventArgs,
        SystemMediaTransportControlsDisplayUpdater, SystemMediaTransportControlsTimelineProperties,
    },
    Storage::Streams::{DataWriter, InMemoryRandomAccessStream, RandomAccessStreamReference},
    Win32::{Foundation::HWND, System::WinRT::ISystemMediaTransportControlsInterop},
};

use crate::{audio::controls::PROGRESS, utils::types::PlaybackState};

use super::types::{Event, MediaControlAction, Metadata};

pub static HANDLER: RwLock<Option<Smtc>> = RwLock::new(None);

pub struct Smtc
{
    controls: SystemMediaTransportControls,
    display: SystemMediaTransportControlsDisplayUpdater,
    timeline: SystemMediaTransportControlsTimelineProperties,
    button_press_token: EventRegistrationToken,
    playback_pos_token: EventRegistrationToken,
}

impl Smtc
{
    pub fn new<C>(actions: Vec<MediaControlAction>, hwnd: isize, callback: C) -> Self
    where
        C: Fn(Event) + Send + 'static,
    {
        let interop: ISystemMediaTransportControlsInterop = windows::core::factory::<
            SystemMediaTransportControls,
            ISystemMediaTransportControlsInterop,
        >()
        .unwrap();

        let controls: SystemMediaTransportControls =
            unsafe { interop.GetForWindow(HWND(hwnd)) }.unwrap();
        let display = controls.DisplayUpdater().unwrap();
        let timeline = SystemMediaTransportControlsTimelineProperties::new().unwrap();

        display.SetType(MediaPlaybackType::Music).unwrap();

        controls.SetIsEnabled(true).unwrap();
        controls.SetIsStopEnabled(true).unwrap();

        for action in actions {
            match action {
                MediaControlAction::Rewind => controls.SetIsRewindEnabled(true).unwrap(),
                MediaControlAction::SkipPrev => controls.SetIsPreviousEnabled(true).unwrap(),
                MediaControlAction::PlayPause => {
                    controls.SetIsPlayEnabled(true).unwrap();
                    controls.SetIsPauseEnabled(true).unwrap();
                }
                MediaControlAction::SkipNext => controls.SetIsNextEnabled(true).unwrap(),
                MediaControlAction::FastForward => controls.SetIsFastForwardEnabled(true).unwrap(),
            }
        }

        let callback = Arc::new(Mutex::new(callback));

        let button_callback = TypedEventHandler::new({
            let callback = callback.clone();

            move |_, args: &Option<_>| {
                let args: &SystemMediaTransportControlsButtonPressedEventArgs =
                    args.as_ref().unwrap();
                let button = args.Button().unwrap();

                let event = match button {
                    SystemMediaTransportControlsButton::Play => Event::Play,
                    SystemMediaTransportControlsButton::Pause => Event::Pause,
                    SystemMediaTransportControlsButton::Stop => Event::Stop,
                    SystemMediaTransportControlsButton::Previous => Event::Previous,
                    SystemMediaTransportControlsButton::Next => Event::Next,
                    SystemMediaTransportControlsButton::Rewind => Event::Seek(-10, false),
                    SystemMediaTransportControlsButton::FastForward => Event::Seek(10, false),
                    _ => return Ok(()),
                };

                callback.lock().unwrap()(event);
                Ok(())
            }
        });

        let position_callback = TypedEventHandler::new({
            let callback = callback.clone();

            move |_, args: &Option<_>| {
                let args: &PlaybackPositionChangeRequestedEventArgs = args.as_ref().unwrap();
                let position = Duration::from(args.RequestedPlaybackPosition().unwrap());
                callback.lock().unwrap()(Event::Seek(position.as_secs() as i64, true));

                Ok(())
            }
        });

        let button_press_token = controls.ButtonPressed(&button_callback).unwrap();
        let playback_pos_token = controls
            .PlaybackPositionChangeRequested(&position_callback)
            .unwrap();

        Smtc {
            controls,
            display,
            timeline,
            button_press_token,
            playback_pos_token,
        }
    }

    pub fn stop(self)
    {
        self.controls
            .RemoveButtonPressed(self.button_press_token)
            .unwrap();
        self.controls
            .RemovePlaybackPositionChangeRequested(self.playback_pos_token)
            .unwrap();
        self.display.ClearAll().unwrap();
        self.display.Update().unwrap();
    }

    pub fn set_metadata(&self, metadata: Metadata)
    {
        let properties = self.display.MusicProperties().unwrap();

        if let Some(title) = metadata.title {
            properties.SetTitle(&HSTRING::from(title)).unwrap();
        }

        if let Some(artist) = metadata.artist {
            properties.SetArtist(&HSTRING::from(artist)).unwrap();
        }

        if let Some(album) = metadata.album {
            properties.SetAlbumTitle(&HSTRING::from(album)).unwrap();
        }

        if let Some(art_uri) = metadata.art_uri {
            let uri = Uri::CreateUri(&HSTRING::from(art_uri)).unwrap();
            let stream = RandomAccessStreamReference::CreateFromUri(&uri).unwrap();
            self.display.SetThumbnail(&stream).unwrap();
        }

        if let Some(art_bytes) = metadata.art_bytes {
            // SEE: https://learn.microsoft.com/en-us/uwp/api/windows.storage.streams.datareader?view=winrt-22621
            let mem_stream = InMemoryRandomAccessStream::new().unwrap();
            let writer = DataWriter::CreateDataWriter(&mem_stream).unwrap();
            writer.WriteBytes(art_bytes.as_slice()).unwrap();
            writer.StoreAsync().unwrap();
            writer.FlushAsync().unwrap();
            writer.DetachStream().unwrap();

            let stream = RandomAccessStreamReference::CreateFromStream(&mem_stream).unwrap();
            self.display.SetThumbnail(&stream).unwrap();
        }

        self.timeline
            .SetStartTime(Duration::default().into())
            .unwrap();
        self.timeline
            .SetMinSeekTime(Duration::default().into())
            .unwrap();

        self.controls
            .UpdateTimelineProperties(&self.timeline)
            .unwrap();
        self.display.Update().unwrap();
    }

    pub fn set_duration(&self, duration: u64)
    {
        self.timeline
            .SetEndTime(Duration::from_secs(duration).into())
            .unwrap();
        self.timeline
            .SetMaxSeekTime(Duration::from_secs(duration).into())
            .unwrap();

        self.controls
            .UpdateTimelineProperties(&self.timeline)
            .unwrap();
    }

    pub fn set_playback_state(&self, state: PlaybackState)
    {
        let status: MediaPlaybackStatus = match state {
            PlaybackState::Play => MediaPlaybackStatus::Playing,
            PlaybackState::Pause => MediaPlaybackStatus::Paused,
            _ => MediaPlaybackStatus::Paused,
        };

        self.controls.SetPlaybackStatus(status).unwrap();

        self.controls
            .UpdateTimelineProperties(&self.timeline)
            .unwrap();
    }
}
