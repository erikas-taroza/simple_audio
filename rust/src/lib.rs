mod output;

use std::{thread, sync::{Arc, Mutex}};

use symphonia::{
    core::{
        probe::Probe, 
        io::MediaSourceStream,
        formats::{
            FormatOptions,
            FormatReader
        },
        meta::MetadataOptions, audio::AudioBufferRef
    }, default
};

pub struct Player
{
    //probe:Probe,
    //format_options:FormatOptions,
    //metadata_options:MetadataOptions,
    is_playing:bool
    //is_playing:Arc<Mutex<bool>>,
}

impl Player
{
    pub fn new() -> Player
    {
        Player
        {
            //probe: default::get_probe(),
            //format_options: FormatOptions { enable_gapless: true, ..Default::default() },
            //metadata_options: Default::default(),
            //is_playing: Arc::new(Mutex::new(false))
            is_playing: false,
        }
    }

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
                // let is_playing = Arc::clone(&self.is_playing);
                
                // thread::spawn(move || {
                //     Self::start_playback(is_playing, &mut probed.format);
                // });
                self.start_playback(&mut probed.format);
            }
        }

        Ok(())
    }

    /// Plays the probed file at the default track.
    /// Creates a new thread where the packets are being read.
    fn start_playback(&self,/*is_playing:Arc<Mutex<bool>>,*/ reader:&mut Box<dyn FormatReader>)
    {
        let track = reader.default_track();
        if let None = track { return; }

        let track = track.unwrap();
        let track_id = track.id;
        let mut decoder = default::get_codecs().make(&track.codec_params, &Default::default())
            .expect("Unsupported codec.");

        let mut output:Option<Box<dyn output::AudioOutput>> = None;

        // Decode loop.
        loop
        {
            // let is_playing = *is_playing.lock().unwrap();
            if !self.is_playing { continue; }

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
                    Player::handle_output(&mut output, &decoded);
                },
                Err(err) => {
                    println!("Decoder Error: {}", err);
                    break;
                }
            }
        }
        
        if let Some(output) = output.as_mut()
        {
            output.flush();
        }
    }

    /// Handles outputting the decoded output to an audio device.
    fn handle_output(output:&mut Option<Box<dyn output::AudioOutput>>, decoded:&AudioBufferRef)
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

    // Controls
    pub fn play(&self)
    {
        // let mut value = self.is_playing.lock().unwrap();
        // *value = true;
        //self.is_playing = true;
    }

    pub fn pause(&self)
    {
        // let mut value = self.is_playing.lock().unwrap();
        // *value = false;
        //self.is_playing = false;
    }

    // pub fn seek(&self, seconds:i32)
    // {

    // }

    // pub fn set_volume(&self, volume:f32)
    // {

    // }
}

// mod tests
// {
//     #[test]
//     fn open_and_play()
//     {
//         let mut player = crate::Player::new();
//         player.open("/home/erikas/Music/test.mp3".to_string()).expect("Error");
//         loop
//         {
//             player.play();
//             std::thread::sleep(std::time::Duration::from_secs(2));
//             player.pause();
//             std::thread::sleep(std::time::Duration::from_secs(2));
//         }
//     }
// }