use std::{path::Path, fs::File, io::{BufReader, Cursor}};

use reqwest::blocking::Client;
use rodio::{OutputStream, OutputStreamHandle, Sink, Decoder};

pub struct Output
{
    _stream:OutputStream,
    _handle:OutputStreamHandle,
    pub sink:Sink,
    client:Client
}

impl Output
{
    /// Creates a new OutputStream and Sink.
    pub fn new() -> Self
    {
        let (stream, handle) = OutputStream::try_default()
            .expect(format!("ERR: Failed to create default OutputStream.").as_str());
        let sink = Sink::try_new(&handle)
            .expect(format!("ERR: Failed to create new sink.").as_str());
        
        Output
        {
            _stream: stream,
            _handle: handle,
            sink,
            client: Client::new()
        }
    }

    /// Appends a file to be read in the sink.
    pub fn append_file(&self, path:&Path)
    {
        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file))
            .expect(format!("ERR: Failed to create a new source for {:?}", path).as_str());
        self.sink.append(source);
    }

    /// Appends a network resource to be read in the sink.
    pub fn append_network(&self, url:&String)
    {
        let response = self.client.get(url)
            .header("Range", "bytes=0-")
            .send()
            .expect(format!("ERR: Could not open {url}").as_str());
            
        let bytes = response.bytes().unwrap().to_vec();
        let cursor = Cursor::new(bytes);
        let source = Decoder::new(BufReader::new(cursor))
            .expect(format!("ERR: Failed to create a new source for {url}").as_str());
        self.sink.append(source);
    }
}

unsafe impl Sync for Output {}

unsafe impl Send for Output {}