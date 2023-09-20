/// Errors that can be thrown by `simple_audio`.
#[derive(Debug)]
pub enum Error
{
    /// An error occurred when trying to fetch more bytes for
    /// a network stream.
    NetworkStream
    {
        /// The error message.
        message: String
    },
    /// An error occurred when decoding the file.
    Decode
    {
        /// The error message.
        message: String
    },
    /// An error occurred when trying to open a file.
    Open
    {
        /// The error message.
        message: String
    },
    /// An error occurred when trying to preload a file.
    Preload
    {
        /// The error message.
        message: String
    },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}
