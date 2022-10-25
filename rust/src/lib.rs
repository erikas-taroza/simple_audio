mod output;

use symphonia::{
    core::{
        codecs::{
            CodecRegistry,
            DecoderOptions
        }, 
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
    codec_registry:&'static CodecRegistry,
    probe:&'static Probe,
    format_options:FormatOptions,
    metadata_options:MetadataOptions,
    decoder_options:DecoderOptions
}

impl Player
{
    pub fn new() -> Self
    {
        Player
        {
            codec_registry: default::get_codecs(),
            probe: default::get_probe(),
            format_options: FormatOptions { enable_gapless: true, ..Default::default() },
            metadata_options: Default::default(),
            decoder_options: Default::default()
        }
    }

    /// Opens a file for reading.
    pub fn open(&mut self, path:&str) -> Result<(), std::io::Error>
    {
        //TODO: Handle web requests.
        if path.contains("http") { return Ok(()); }

        let file = std::fs::File::open(path)?;
        let source = Box::new(file);
        let source_stream = MediaSourceStream::new(source, Default::default());
        let hint = symphonia::core::probe::Hint::new();

        // Probe the source.
        let probe_result = self.probe.format(
            &hint,
            source_stream,
            &self.format_options,
            &self.metadata_options
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
    fn start_playback(&self, reader:&mut Box<dyn FormatReader>)
    {
        let track = reader.default_track();
        if let None = track { return; }

        let track = track.unwrap();
        let track_id = track.id;
        let mut decoder = self.codec_registry.make(&track.codec_params, &self.decoder_options)
            .expect("Unsupported codec.");

        let mut output:Option<Box<dyn output::AudioOutput>> = None;

        // Decode loop.
        loop
        {
            // Get the next packet.
            let packet = match reader.next_packet()
            {
                Ok(packet) => packet,
                Err(err) => panic!("Packet Error: {}", err)
            };

            // Make sure that the packet is of the track we want.
            if packet.track_id() != track_id { continue; }

            // Decode the packet and produce audio output.
            match decoder.decode(&packet)
            {
                Ok(decoded) => {
                    self.handle_output(&mut output, &decoded)
                },
                Err(err) => panic!("Decoder Error: {}", err)
            }
        }
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

    // Controls
    // pub fn play(&self)
    // {

    // }

    // pub fn pause(&self)
    // {

    // }

    // pub fn seek(&self, seconds:i32)
    // {

    // }

    // pub fn set_volume(&self, volume:f32)
    // {

    // }
}

mod tests
{
    #[test]
    fn open_and_play()
    {
        let mut player = crate::Player::new();
        player.open("/home/erikas/Music/test.mp3").expect("Error");
    }
}