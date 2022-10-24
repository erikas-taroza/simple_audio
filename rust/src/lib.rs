use symphonia::{
    core::{
        codecs::CodecRegistry, 
        probe::Probe, 
        io::{
            MediaSource, MediaSourceStream
        },
        formats::FormatOptions,
        meta::MetadataOptions
    }, default
};

struct Player
{
    codec_registry:&'static CodecRegistry,
    probe:&'static Probe,
    format_options:FormatOptions,
    metadata_options:MetadataOptions
    // media_source:Box<dyn MediaSource>
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
            metadata_options: Default::default()
        }
    }

    /// Points to either a local file or an online file.
    pub fn open(&mut self, path:&str) -> Result<(), std::io::Error>
    {
        if path.contains("http") { return Ok(()); }

        let file = std::fs::File::open(path)?;
        let source = Box::new(file);
        let source_stream = MediaSourceStream::new(source, Default::default());

        let probe_result = self.probe.format(
            &mut symphonia::core::probe::Hint::new(),
            source_stream,
            &self.format_options,
            &self.metadata_options
        );

        match probe_result
        {
            Err(err) => return Ok(()),
            Ok(mut probed) => {
                
            }
        }

        Ok(())
    }

    pub fn play(&self)
    {

    }

    pub fn pause(&self)
    {

    }

    pub fn seek(&self, seconds:i32)
    {

    }

    pub fn set_volume(&self, volume:f32)
    {

    }
}