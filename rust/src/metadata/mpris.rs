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

// Reference: https://github.com/Sinono3/souvlaki

#![cfg(all(unix, not(target_os = "macos"), not(target_os = "android"), not(target_os = "ios")))]

use std::{thread::{self, JoinHandle}, sync::{Arc, Mutex, RwLock}, time::Duration, collections::HashMap};

use crossbeam::channel::{Receiver, unbounded};
use dbus::{blocking::{Connection, stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged}, channel::{MatchingReceiver, Sender}, message::{MatchRule, SignalArgs}, arg::{Variant, RefArg}, Path};
use dbus_crossroads::{Crossroads, IfaceBuilder};

use crate::{utils::types::PlaybackState, audio::controls::PROGRESS};

use super::types::{Metadata, Event, Command, Actions};

pub static HANDLER: RwLock<Option<Mpris>> = RwLock::new(None);

pub struct Mpris
{
    tx: crossbeam::channel::Sender<Command>,
    handle: JoinHandle<()>
}

impl Mpris
{
    pub fn new<C>(actions: Vec<Actions>, dbus_name: String, callback: C) -> Self
    where
        C: Fn(Event) + Send + 'static
    {
        let (tx, rx) = unbounded::<Command>();
        
        let handle = thread::spawn(move || {
            Self::run(actions, dbus_name, rx, callback).unwrap();
        });

        Mpris { tx, handle }
    }

    pub fn set_metadata(&self, metadata: Metadata)
    { self.tx.send(Command::SetMetadata(metadata)).unwrap(); }

    pub fn set_playback_state(&self, state: PlaybackState)
    { self.tx.send(Command::SetPlaybackState(state)).unwrap(); }

    pub fn stop(self)
    {
        self.tx.send(Command::Stop).unwrap();
        let _ = self.handle.join();
    }

    fn run<C>(actions: Vec<Actions>, dbus_name: String, rx: Receiver<Command>, callback: C) -> Result<(), dbus::Error>
    where
        C: Fn(Event) + Send + 'static
    {
        let bus_name = dbus_name.split('.').last().unwrap().to_string();

        let callback = Arc::new(Mutex::new(callback));

        let conn = Connection::new_session()?;

        conn.request_name(
            format!("org.mpris.MediaPlayer2.{bus_name}"), 
            false, 
            true, 
            false
        )?;

        let mut cr = Crossroads::new();

        let metadata = Arc::new(Mutex::new(Metadata::default()));
        let playback_state = Arc::new(Mutex::new(PlaybackState::Done));

        let bus_name_clone = bus_name.clone();
        let mp = cr.register("org.mpris.MediaPlayer2", move |e: &mut IfaceBuilder<()>| {
            e.property("Identity")
                .get(move |_, &mut _| Ok(bus_name_clone.clone()));
            e.property("CanQuit")
                .get(|_, &mut _| Ok(false))
                .emits_changed_true();
            e.property("CanRaise")
                .get(|_, &mut _| Ok(true))
                .emits_changed_true();
            e.property("HasTracklist")
                .get(|_, &mut _| Ok(false))
                .emits_changed_true();

            // Open the app when the MPRIS notification is clicked.
            e.method("Raise", (), (), move |_, _, _: ()| {
                let process = std::env::current_exe().unwrap();

                let _ = std::process::Command::new(process)
                    .spawn();

                Ok(())
            });
        });

        let mpp = cr.register("org.mpris.MediaPlayer2.Player", |e: &mut IfaceBuilder<()>|{
            let callback = callback.clone();

            // Defaults
            register_method(e, &callback, "Stop", Event::Stop);

            e.property("CanControl")
                .get(|_, _| Ok(true))
                .emits_changed_true();

            e.property("Metadata")
                .get({
                    let metadata = metadata.clone();
                    move |_, _| Ok(metadata_to_map(&metadata.lock().unwrap()))
                })
                .emits_changed_true();

            e.property("PlaybackStatus")
                .get({
                    let playback_state = playback_state.clone();
                    move |_, _| Ok(playback_state_to_string(&playback_state.lock().unwrap()))
                })
                .emits_changed_true();

            e.property("Position")
                .get(move |_, _|{
                    let position: i64 = PROGRESS.read().unwrap().position as i64;
                    Ok(position)
                });

                e.property("CanSeek")
                .get(|_, _| Ok(true))
                .emits_changed_true();

            e.method("Seek", ("Offset",), (), {
                let callback = callback.clone();
                let actions = actions.to_vec();

                move |ctx, _, (offset,): (i64,)| {
                    let offset = offset * 1_000_000;

                    //TODO: This needs testing.
                    if actions.contains(&Actions::Rewind) || actions.contains(&Actions::FastForward)
                    { callback.lock().unwrap()(Event::Seek(offset, false)); }

                    ctx.push_msg(ctx.make_signal("Seeked", ()));
                    Ok(())
                }
            });

            e.method("SetPosition", ("TrackId", "Position"), (), {
                let callback = callback.clone();
                move |_, _, (_track_id, position): (Path, i64)| {
                    if position > PROGRESS.read().unwrap().duration as i64 { return Ok(()); }
                    
                    if let Ok(position) = u64::try_from(position)
                    {
                        callback.lock().unwrap()(Event::Seek((position * 1_000_000) as i64, true));
                    }

                    Ok(())
                }
            });

            // The following actions can be tweaked by the user.

            for action in &actions
            {
                match action
                {
                    Actions::SkipPrev => {
                        register_method(e, &callback, "Previous", Event::Previous);

                        e.property("CanGoPrevious")
                            .get(|_, _| Ok(true))
                            .emits_changed_true();
                    },
                    Actions::PlayPause => {
                        register_method(e, &callback, "Play", Event::Play);
                        register_method(e, &callback, "Pause", Event::Pause);
                        register_method(e, &callback, "PlayPause", Event::PlayPause);

                        e.property("CanPlay")
                            .get(|_, _| Ok(true))
                            .emits_changed_true();
                        e.property("CanPause")
                            .get(|_, _| Ok(true))
                            .emits_changed_true();
                    },
                    Actions::SkipNext => {
                        register_method(e, &callback, "Next", Event::Next);

                        e.property("CanGoNext")
                            .get(|_, _| Ok(true))
                            .emits_changed_true();
                    },
                    _ => continue
                }
            }
        });

        cr.insert("/org/mpris/MediaPlayer2", &[mp, mpp], ());

        conn.start_receive(MatchRule::new_method_call(), Box::new(move |message, conn| {
            cr.handle_message(message, conn).unwrap();
            true
        }));

        loop
        {
            // Check for any new commands.
            match rx.try_recv()
            {
                Err(_) => (),
                Ok(message) => {
                    let mut changes: HashMap<String, Variant<Box<dyn RefArg>>> = HashMap::new();

                    match message
                    {
                        Command::SetMetadata(data) => {
                            let mut metadata = metadata.lock().unwrap();
                            *metadata = data;

                            changes.insert(
                                "Metadata".to_string(),
                                Variant(metadata_to_map(&metadata).box_clone())
                            );
                        },
                        Command::SetPlaybackState(state) => {
                            let mut playback_state = playback_state.lock().unwrap();
                            *playback_state = state;

                            changes.insert(
                                "PlaybackStatus".to_string(),
                                Variant(Box::new(playback_state_to_string(&playback_state)))
                            );
                        },
                        Command::Stop => break
                    }

                    let properties_changed = PropertiesPropertiesChanged
                    {
                        interface_name: "org.mpris.MediaPlayer2.Player".to_owned(),
                        changed_properties: changes,
                        invalidated_properties: Vec::new()
                    };

                    conn.send(properties_changed.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap()))
                        .unwrap();
                }
            }

            conn.process(Duration::from_millis(200))?;
        }

        // Clean up.
        conn.release_name(format!("org.mpris.MediaPlayer2.{bus_name}"))?;

        Ok(())
    }
}

/// Helper method to register callbacks to a MPRIS callback.
fn register_method<C>(e: &mut IfaceBuilder<()>, callback: &Arc<Mutex<C>>, name: &'static str, event: Event)
where
    C: Fn(Event) + Send + 'static
{
    let callback = callback.clone();
    e.method(name, (), (), move |_, _, _:()| {
        callback.lock().unwrap()(event);
        Ok(())
    });
}

/// Converts the metadata object to a map that the MPRIS interface uses.
fn metadata_to_map(metadata: &Metadata) -> HashMap<String, Variant<Box<dyn RefArg>>>
{
    let mut map = HashMap::<String, Variant<Box<dyn RefArg>>>::new();

    let path = Path::new("/").unwrap();

    map.insert("mpris:trackid".to_string(), Variant(Box::new(path)));
    
    if let Some(title) = metadata.title.clone()
    { map.insert("xesam:title".to_string(), Variant(Box::new(title))); }

    if let Some(artist) = metadata.artist.clone()
    { map.insert("xesam:artist".to_string(), Variant(Box::new(vec![artist]))); }

    if let Some(album) = metadata.album.clone()
    { map.insert("xesam:album".to_string(), Variant(Box::new(album))); }

    if let Some(art_uri) = metadata.art_uri.clone()
    { map.insert("mpris:artUrl".to_string(), Variant(Box::new(art_uri))); }

    // Wait for a valid value (duration != 0).
    loop
    {
        let progress = PROGRESS.read().unwrap();
        if progress.duration == 0 { continue; }

        map.insert("mpris:length".to_string(), Variant(Box::new(progress.duration)));
        break;
    }

    map
}

/// Converts the playback status to a string that MPRIS can use.
fn playback_state_to_string(state: &PlaybackState) -> String
{
    match state
    {
        PlaybackState::Play => "Playing".to_string(),
        PlaybackState::Pause => "Paused".to_string(),
        _ => "Paused".to_string()
    }
}