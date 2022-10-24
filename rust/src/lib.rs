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
        meta::MetadataOptions
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

        // Probe the source.
        let probe_result = self.probe.format(
            &mut symphonia::core::probe::Hint::new(),
            source_stream,
            &self.format_options,
            &self.metadata_options
        );

        // If the source was successfully probed, start the playback.
        match probe_result
        {
            Err(err) => panic!("{}", err),
            Ok(probed) => {
                self.start_playback(probed.format);
            }
        }

        Ok(())
    }

    /// Plays the probed file at the default track.
    fn start_playback(&self, mut reader:Box<dyn FormatReader>)
    {
        let track = reader.default_track();
        if let None = track { return; }

        let track = track.unwrap();
        let track_id = track.id;
        let mut decoder = self.codec_registry.make(&track.codec_params, &self.decoder_options)
            .expect("Unsupported codec.");

        // Decode loop.
        loop
        {
            // Get the next packet.
            let packet = match reader.next_packet()
            {
                Ok(packet) => packet,
                Err(err) => panic!("{}", err)
            };

            // Make sure that the packet is of the track we want.
            if packet.track_id() != track_id { continue; }

            // Decode the packet and produce audio output.
            match decoder.decode(&packet)
            {
                Ok(decoded) => {
                    
                },
                Err(err) => panic!("{}", err)
            }
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