use std::{path::Path, fs::File, io::{BufReader, Cursor}};

// use reqwest::IntoUrl;
use rodio::{OutputStream, OutputStreamHandle, Sink, Decoder};

pub struct Output
{
    _stream:OutputStream,
    _handle:OutputStreamHandle,
    pub sink:Sink
}

impl Output
{
    pub fn new() -> Self
    {
        let (stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        
        Output
        {
            _stream: stream,
            _handle: handle,
            sink
        }
    }

    pub fn open_file(&self, path:&Path)
    {
        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        self.sink.append(source);
    }
}

unsafe impl Sync for Output {}

unsafe impl Send for Output {}