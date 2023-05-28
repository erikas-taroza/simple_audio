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

// MPRIS docs: https://buildmedia.readthedocs.org/media/pdf/mpris2/latest/mpris2.pdf
// Reference: https://github.com/Sinono3/souvlaki

#![cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "android"),
    not(target_os = "ios")
))]

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crossbeam::channel::{unbounded, Receiver};
use zbus::{
    blocking::ConnectionBuilder,
    dbus_interface,
    fdo::RequestNameFlags,
    zvariant::{ObjectPath, Value},
};

use crate::utils::types::PlaybackState;

use super::types::{Command, Event, MediaControlAction, MediaController, Metadata};

pub struct Mpris
{
    tx: crossbeam::channel::Sender<Command>,
}

impl MediaController for Mpris
{
    fn set_metadata(&self, metadata: Metadata)
    {
        self.tx.send(Command::SetMetadata(metadata)).unwrap();
    }

    fn set_position(&self, position: u64)
    {
        self.tx.send(Command::SetPosition(position)).unwrap();
    }

    fn set_duration(&self, duration: u64)
    {
        self.tx.send(Command::SetDuration(duration)).unwrap();
    }

    fn set_playback_state(&self, state: PlaybackState)
    {
        self.tx.send(Command::SetPlaybackState(state)).unwrap();
    }

    fn stop(&self)
    {
        self.tx.send(Command::Stop).unwrap();
    }
}

impl Mpris
{
    pub fn new<C>(actions: Vec<MediaControlAction>, dbus_name: String, callback: C) -> Self
    where
        C: Fn(Event) + Send + 'static,
    {
        let (tx, rx) = unbounded::<Command>();

        thread::spawn(move || {
            pollster::block_on(Self::run(actions, dbus_name, rx, callback)).unwrap();
        });

        Mpris { tx }
    }

    async fn run<C>(
        actions: Vec<MediaControlAction>,
        dbus_name: String,
        rx: Receiver<Command>,
        callback: C,
    ) -> Result<(), zbus::Error>
    where
        C: Fn(Event) + Send + 'static,
    {
        let bus_name = dbus_name.split('.').last().unwrap().to_string();

        let app_interface = AppInterface {
            name: bus_name.clone(),
        };

        let player_interface = PlayerInterface {
            actions,
            callback: Arc::new(Mutex::new(callback)),
            metadata: Default::default(),
            playback_state: PlaybackState::Done,
            position: 0,
            duration: 0,
        };

        let full_bus_name = format!("org.mpris.MediaPlayer2.{bus_name}");
        let bus_path = ObjectPath::try_from("/org/mpris/MediaPlayer2")?;

        let conn = ConnectionBuilder::session()?
            .serve_at(&bus_path, app_interface)?
            .serve_at(&bus_path, player_interface)?
            .build()?;

        conn.request_name_with_flags(
            full_bus_name.as_str(),
            RequestNameFlags::ReplaceExisting.into(),
        )?;

        loop {
            // Check for any new commands.
            match rx.try_recv() {
                Err(_) => (),
                Ok(message) => {
                    let player_iface_ref = conn
                        .object_server()
                        .interface::<_, PlayerInterface>(&bus_path)?;

                    let mut player_iface = player_iface_ref.get_mut();

                    let context = player_iface_ref.signal_context();

                    match message {
                        Command::SetMetadata(data) => {
                            player_iface.metadata = data;
                            player_iface.metadata_changed(context).await?;
                        }
                        Command::SetPosition(position) => {
                            player_iface.position = position;
                            player_iface.position_changed(context).await?;
                        }
                        Command::SetDuration(duration) => {
                            player_iface.duration = duration;
                            player_iface.metadata_changed(context).await?;
                        }
                        Command::SetPlaybackState(state) => {
                            player_iface.playback_state = state;
                            player_iface.playback_status_changed(context).await?;
                        }
                        Command::Stop => break,
                    }
                }
            }

            std::thread::sleep(Duration::from_millis(200));
        }

        // Clean up.
        conn.release_name(full_bus_name)?;

        Ok(())
    }
}

/// D-Bus interface that describes the app and its MPRIS capabilities
/// (ex. can raise).
struct AppInterface
{
    name: String,
}

#[dbus_interface(name = "org.mpris.MediaPlayer2")]
impl AppInterface
{
    #[dbus_interface(property)]
    fn identity(&self) -> &str
    {
        &self.name
    }

    #[dbus_interface(property)]
    fn can_quit(&self) -> bool
    {
        false
    }

    #[dbus_interface(property)]
    fn can_raise(&self) -> bool
    {
        true
    }

    #[dbus_interface(property)]
    fn has_tracklist(&self) -> bool
    {
        false
    }

    fn raise(&self)
    {
        let process = std::env::current_exe().unwrap();
        let _ = std::process::Command::new(process).spawn();
    }
}

/// D-Bus interface that describes the app's player and its MPRIS capabilities
/// (ex. play/pause).
struct PlayerInterface
{
    actions: Vec<MediaControlAction>,
    callback: Arc<Mutex<dyn Fn(Event) + Send + 'static>>,
    position: u64,
    duration: u64,
    metadata: Metadata,
    playback_state: PlaybackState,
}

#[dbus_interface(name = "org.mpris.MediaPlayer2.Player")]
impl PlayerInterface
{
    #[dbus_interface(property)]
    fn can_control(&self) -> bool
    {
        true
    }

    #[dbus_interface(property)]
    fn can_seek(&self) -> bool
    {
        true
    }

    #[dbus_interface(property)]
    fn can_play(&self) -> bool
    {
        true
    }

    #[dbus_interface(property)]
    fn can_pause(&self) -> bool
    {
        true
    }

    #[dbus_interface(property)]
    fn can_go_previous(&self) -> bool
    {
        self.actions.contains(&MediaControlAction::SkipPrev)
    }

    #[dbus_interface(property)]
    fn can_go_next(&self) -> bool
    {
        self.actions.contains(&MediaControlAction::SkipNext)
    }

    #[dbus_interface(property)]
    fn position(&self) -> i64
    {
        let in_micros = Duration::from_secs(self.position).as_micros();
        in_micros.try_into().unwrap_or_default()
    }

    #[dbus_interface(property)]
    fn playback_status(&self) -> &'static str
    {
        match self.playback_state {
            PlaybackState::Play => "Playing",
            PlaybackState::Pause => "Paused",
            _ => "Paused",
        }
    }

    #[dbus_interface(property)]
    fn metadata(&self) -> HashMap<&str, Value>
    {
        let mut map = HashMap::<&str, Value>::new();

        let path = ObjectPath::try_from("/").unwrap();
        map.insert("mpris:trackid", Value::new(path));

        if let Some(title) = self.metadata.title.clone() {
            map.insert("xesam:title", Value::new(title));
        }

        if let Some(artist) = self.metadata.artist.clone() {
            map.insert("xesam:artist", Value::new(vec![artist]));
        }

        if let Some(album) = self.metadata.album.clone() {
            map.insert("xesam:album", Value::new(album));
        }

        if let Some(art_uri) = self.metadata.art_uri.clone() {
            map.insert("mpris:artUrl", Value::new(art_uri));
        }

        let in_micros: i64 = Duration::from_secs(self.duration)
            .as_micros()
            .try_into()
            .unwrap_or_default();

        map.insert("mpris:length", Value::new(in_micros));

        map
    }

    fn seek(&self, offset: i64)
    {
        let in_seconds = Duration::from_micros(offset.unsigned_abs())
            .as_secs()
            .try_into()
            .unwrap_or_default();

        // Seeking forward
        if offset.is_positive() && self.actions.contains(&MediaControlAction::FastForward) {
            self.callback.lock().unwrap()(Event::Seek(in_seconds, false));
        }
        // Seeking backwards
        else if offset.is_negative() && self.actions.contains(&MediaControlAction::Rewind) {
            self.callback.lock().unwrap()(Event::Seek(-in_seconds, false));
        }
    }

    fn set_position(&self, _track_id: ObjectPath, position: i64)
    {
        let in_seconds = Duration::from_micros(position.unsigned_abs())
            .as_secs()
            .try_into()
            .unwrap_or_default();

        self.callback.lock().unwrap()(Event::Seek(in_seconds, true));
    }

    fn stop(&self)
    {
        self.callback.lock().unwrap()(Event::Stop);
    }

    fn previous(&self)
    {
        self.callback.lock().unwrap()(Event::Previous);
    }

    fn play(&self)
    {
        self.callback.lock().unwrap()(Event::Play);
    }

    fn pause(&self)
    {
        self.callback.lock().unwrap()(Event::Pause);
    }

    fn play_pause(&self)
    {
        self.callback.lock().unwrap()(Event::PlayPause);
    }

    fn next(&self)
    {
        self.callback.lock().unwrap()(Event::Next);
    }
}
