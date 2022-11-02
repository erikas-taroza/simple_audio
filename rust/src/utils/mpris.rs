use std::{thread::{self, JoinHandle}, sync::{Arc, Mutex}, time::Duration};

use crossbeam::channel::{bounded, Receiver};
use dbus::{blocking::Connection, channel::MatchingReceiver, message::MatchRule};
use dbus_crossroads::{Crossroads, IfaceBuilder};

pub struct Mpris
{
    dbus_name:String,
    display_name:String,
    thread:JoinHandle<()>,
    tx:crossbeam::channel::Sender<bool>
}

impl Mpris
{
    pub fn new<C>(dbus_name:String, display_name:String, callback:C) -> Self
    where
        C: Fn(MprisEvent) + Send + 'static
    {
        let (tx, rx) = bounded::<bool>(1);

        let dbus = dbus_name.clone();
        let display = display_name.clone();
        
        let thread = thread::spawn(move || {
            Self::run(dbus, display, rx, callback).unwrap();
        });

        Mpris { dbus_name, display_name, thread, tx }
    }

    fn run<C>(dbus_name:String, display_name:String, rx:Receiver<bool>, callback:C) -> Result<(), dbus::Error>
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

        let mp = cr.register("org.mpris.MediaPlayer2", {
            let callback = callback.clone();

            move |e:&mut IfaceBuilder<()>| {
                register_method(e, &callback, "Raise", MprisEvent::Raise);
                register_method(e, &callback, "Quit", MprisEvent::Quit);

                e.property("Identity")
                    .get(move |_, &mut _| Ok(display_name.clone()));
                e.property("CanQuit")
                    .get(|_, &mut _| Ok(true))
                    .emits_changed_true();
                e.property("CanRaise")
                    .get(|_, &mut _| Ok(true))
                    .emits_changed_true();
                e.property("HasTracklist")
                    .get(|_, &mut _| Ok(false))
                    .emits_changed_true();
                e.property("SupportedUriSchemes")
                    .get(move |_, &mut _| Ok(&[] as &[String]))
                    .emits_changed_true();
                e.property("SupportedMimeTypes")
                    .get(move |_, &mut _| Ok(&[] as &[String]))
                    .emits_changed_true();
            }
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
                Ok(message) => if message { break; }
            }

            conn.process(Duration::from_secs(1))?;
        }

        Ok(())
    }
}

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

#[derive(Clone, Copy, Debug)]
pub enum MprisEvent
{
    Raise,
    Quit,
    
    Next,
    Previous,
    Play,
    Pause,
    Stop,
    PlayPause
}