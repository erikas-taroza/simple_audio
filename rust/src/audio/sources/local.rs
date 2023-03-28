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

use std::{
    fs::File,
    io::{Read, Seek},
    sync::MutexGuard,
};

use symphonia::core::io::MediaSource;

/// A wrapper around `std::fs::File`.
///
/// Allows for better integration with decoding/outputting.
/// For example, locking `super::ACTIVE_LOCK`.
pub struct Local
{
    inner: File,
    active_lock: Option<MutexGuard<'static, ()>>,
}

impl Local
{
    pub fn new(file: File) -> Local
    {
        Local {
            inner: file,
            active_lock: super::try_get_active_lock(),
        }
    }
}

impl Read for Local
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
    {
        if self.active_lock.is_none() {
            self.active_lock = super::try_get_active_lock();
        }

        self.inner.read(buf)
    }
}

impl Seek for Local
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>
    {
        self.inner.seek(pos)
    }
}

impl MediaSource for Local
{
    fn is_seekable(&self) -> bool
    {
        self.inner.is_seekable()
    }

    fn byte_len(&self) -> Option<u64>
    {
        self.inner.byte_len()
    }
}

unsafe impl Send for Local {}
unsafe impl Sync for Local {}
