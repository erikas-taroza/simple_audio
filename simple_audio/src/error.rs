// This file is a part of simple_audio
// Copyright (c) 2022-2023 Erikas Taroza <erikastaroza@gmail.com>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this program.
// If not, see <https://www.gnu.org/licenses/>.

/// Errors that can be thrown by `simple_audio`.
#[derive(Clone, Debug)]
pub enum Error
{
    /// An error occurred when trying to fetch more bytes for
    /// a network stream.
    NetworkStream(String),
    /// An error occurred when decoding the file.
    Decode(String),
    /// An error occurred when trying to open a file.
    Open(String),
    /// An error occurred when trying to preload a file.
    Preload(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:?}", self)
    }
}
