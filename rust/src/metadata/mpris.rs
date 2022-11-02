use std::{thread::{self, JoinHandle}, sync::{Arc, Mutex}, time::Duration, collections::HashMap};

use crossbeam::channel::{Receiver, unbounded};
use dbus::{blocking::{Connection, stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged}, channel::{MatchingReceiver, Sender}, message::{MatchRule, SignalArgs}, arg::{Variant, RefArg}, Path};
use dbus_crossroads::{Crossroads, IfaceBuilder};

use crate::utils::playback_state::PlaybackState;

use super::metadata::Metadata;

pub struct Mpris
{
    dbus_name:String,
    display_name:String,
    thread:JoinHandle<()>,
    tx:crossbeam::channel::Sender<MprisCommand>
}

impl Mpris
{
    pub fn new<C>(dbus_name:String, display_name:String, callback:C) -> Self
    where
        C: Fn(MprisEvent) + Send + 'static
    {
        let (tx, rx) = unbounded::<MprisCommand>();

        let dbus = dbus_name.clone();
        let display = display_name.clone();
        
        let thread = thread::spawn(move || {
            Self::run(dbus, display, rx, callback).unwrap();
        });

        Mpris { dbus_name, display_name, thread, tx }
    }

    pub fn set_metadata(&self, metadata:Metadata)
    { self.tx.send(MprisCommand::SetMetadata(metadata)).unwrap(); }

    fn run<C>(dbus_name:String, display_name:String, rx:Receiver<MprisCommand>, callback:C) -> Result<(), dbus::Error>
    where
        C: Fn(MprisEvent) + Send + 'static
    {
        let callback = Arc::new(Mutex::new(callback));

        let conn = Connection::new_session()?;

        conn.request_name(
            format!("org.mpris.MediaPlayer2.{}", dbus_name), 
            false, 
            true, 
            false
        )?;

        let mut cr = Crossroads::new();

        let metadata = Arc::new(Mutex::new(Metadata::default()));

        let mp = cr.register("org.mpris.MediaPlayer2", move |e:&mut IfaceBuilder<()>| {
            e.property("Identity")
                .get(move |_, &mut _| Ok(display_name.clone()));
            e.property("CanQuit")
                .get(|_, &mut _| Ok(false))
                .emits_changed_true();
            e.property("CanRaise")
                .get(|_, &mut _| Ok(false))
                .emits_changed_true();
            e.property("HasTracklist")
                .get(|_, &mut _| Ok(false))
                .emits_changed_true();
        });

        let mpp = cr.register("org.mpris.MediaPlayer2.Player", |e:&mut IfaceBuilder<()>|{
            let callback = callback.clone();
            
            register_method(e, &callback, "Next", MprisEvent::Next);
            register_method(e, &callback, "Previous", MprisEvent::Previous);
            register_method(e, &callback, "Play", MprisEvent::Play);
            register_method(e, &callback, "Pause", MprisEvent::Pause);
            register_method(e, &callback, "PlayPause", MprisEvent::PlayPause);
            register_method(e, &callback, "Stop", MprisEvent::Stop);

            e.property("CanGoNext")
                .get(|_, _| Ok(true))
                .emits_changed_true();
            e.property("CanGoPrevious")
                .get(|_, _| Ok(true))
                .emits_changed_true();
            e.property("CanPlay")
                .get(|_, _| Ok(true))
                .emits_changed_true();
            e.property("CanPause")
                .get(|_, _| Ok(true))
                .emits_changed_true();
            e.property("CanSeek")
                .get(|_, _| Ok(true))
                .emits_changed_true();
            e.property("CanControl")
                .get(|_, _| Ok(true))
                .emits_changed_true();

            e.property("Metadata")
                .get({
                    let metadata = metadata.clone();
                    move |_, _| Ok(metadata_to_map(&metadata.lock().unwrap()))
                })
                .emits_changed_true();
        });

        cr.insert("/org/mpris/MediaPlayer2", &[mp, mpp], ());

        conn.start_receive(MatchRule::new_method_call(), Box::new(move |message, conn| {
            cr.handle_message(message, conn).unwrap();
            true
        }));

        loop
        {
            match rx.try_recv()
            {
                Err(_) => (),
                Ok(message) => {
                    let mut changes:HashMap<String, Variant<Box<dyn RefArg>>> = HashMap::new();

                    match message
                    {
                        MprisCommand::SetMetadata(data) => {
                            let mut metadata = metadata.lock().unwrap();
                            *metadata = data;

                            changes.insert(
                                "Metadata".to_string(),
                                Variant(metadata_to_map(&metadata).box_clone())
                            );
                        },
                        MprisCommand::SetPlaybackState(state) => {
                            
                        }
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

            conn.process(Duration::from_secs(1))?;
        }
    }
}

/// Helper method to register callbacks to a MPRIS callback.
fn register_method<C>(e:&mut IfaceBuilder<()>, callback:&Arc<Mutex<C>>, name:&'static str, event:MprisEvent)
where
    C: Fn(MprisEvent) + Send + 'static
{
    let callback = callback.clone();
    e.method(name, (), (), move |_, _, _:()| {
        callback.lock().unwrap()(event);
        Ok(())
    });
}

/// Converts the metadata object to a map that the MPRIS interface uses.
fn metadata_to_map(metadata:&Metadata) -> HashMap<String, Variant<Box<dyn RefArg>>>
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

    if let Some(duration) = metadata.duration
    { map.insert("mpris:length".to_string(), Variant(Box::new(duration))); }

    if let Some(art_url) = metadata.art_url.clone()
    { map.insert("mpris:artUrl".to_string(), Variant(Box::new(art_url))); }

    map
}

/// Callback events from the MPRIS player.
#[derive(Clone, Copy, Debug)]
pub enum MprisEvent
{
    Next,
    Previous,
    Play,
    Pause,
    Stop,
    PlayPause
}

/// Commands to be sent via the thread's channels.
enum MprisCommand
{
    SetMetadata(Metadata),
    SetPlaybackState(PlaybackState)
}