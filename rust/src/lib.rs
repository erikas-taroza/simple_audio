mod bridge_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
pub mod output;
mod src;

use std::sync::RwLock;

use flutter_rust_bridge::StreamSink;
use symphonia::{
    core::{
        io::MediaSourceStream,
        formats::{
            FormatOptions,
            FormatReader
        },
        audio::AudioBufferRef
    }, default
};

use crate::src::playback_state_stream::*;

static IS_PLAYING:RwLock<bool> = RwLock::new(false);

// NOTE: Code gen fails with empty structs.
pub struct Player
{
    dummy:i32
}

impl Player
{
    pub fn new() -> Player
    {
        Player { dummy: 0 }
    }

    // ---------------------------------
    //          SETTERS/GETTERS
    // ---------------------------------

    pub fn playback_state_stream(stream:StreamSink<bool>) { playback_state_stream(stream); }

    pub fn is_playing(&self) -> bool { *IS_PLAYING.read().unwrap() }
    fn set_is_playing(value:bool)
    {
        let mut w = IS_PLAYING.write().unwrap();
        *w = value;
    }

    // ---------------------------------
    //            PLAYBACK
    // ---------------------------------

    /// Opens a file for reading.
    pub fn open(&self, path:String) -> Result<(), std::io::Error>
    {
        //TODO: Handle web requests.
        if path.contains("http") { return Ok(()); }

        let file = std::fs::File::open(path)?;
        let source = Box::new(file);
        let source_stream = MediaSourceStream::new(source, Default::default());
        let hint = symphonia::core::probe::Hint::new();

        // Probe the source.
        let probe_result = default::get_probe().format(
            &hint,
            source_stream,
            &FormatOptions { enable_gapless: true, ..Default::default() },
            &Default::default()
        );

        // If the source was successfully probed, start the playback.
        match probe_result
        {
            Err(err) => {
                panic!("Probe Error: {}", err)
            },
            Ok(mut probed) => {
                self.start_playback(&mut probed.format);
            }
        }

        Ok(())
    }

    /// Plays the probed file at the default track.
    /// Creates a new thread where the packets are being read.
    fn start_playback(&self, reader:&mut Box<dyn FormatReader>)
    {
        let track = reader.default_track();
        if let None = track { return; }

        let track = track.unwrap();
        let track_id = track.id;
        let mut decoder = default::get_codecs().make(&track.codec_params, &Default::default())
            .expect("Unsupported codec.");

        let mut output:Option<Box<dyn output::AudioOutput>> = None;

        update_playback_state_stream(true);
        Self::set_is_playing(true);

        // Decode loop.
        loop
        {
            if !self.is_playing() { continue; }

            // Get the next packet.
            let packet = match reader.next_packet()
            {
                Ok(packet) => packet,
                Err(err) => {
                    println!("Packet Error: {}", err);
                    break;
                }
            };

            // Make sure that the packet is of the track we want.
            if packet.track_id() != track_id { continue; }

            // Decode the packet and produce audio output.
            match decoder.decode(&packet)
            {
                Ok(decoded) => {
                    self.handle_output(&mut output, &decoded);
                },
                Err(err) => {
                    println!("Decoder Error: {}", err);
                    break;
                }
            }
        }
        
        if let Some(output) = output.as_mut()
        { output.flush(); }

        update_playback_state_stream(false);
        Self::set_is_playing(false);
    }

    /// Handles outputting the decoded output to an audio device.
    fn handle_output(&self, output:&mut Option<Box<dyn output::AudioOutput>>, decoded:&AudioBufferRef)
    {
        if output.is_none()
        {
            let spec = *decoded.spec();
            let duration = decoded.capacity() as u64;
            output.replace(output::try_open(spec, duration).unwrap());
        }

        if let Some(output) = output
        {
            output.write(decoded.clone()).unwrap();
        }
    }

    // ---------------------------------
    //             CONTROLS
    // ---------------------------------

    pub fn play(&self)
    {
        Self::set_is_playing(true);
        update_playback_state_stream(true);
    }

    pub fn pause(&self)
    {
        Self::set_is_playing(false);
        update_playback_state_stream(false);
    }

    // pub fn seek(&self, seconds:i32)
    // {

    // }

    // pub fn set_volume(&self, volume:f32)
    // {

    // }
}