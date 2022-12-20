// This file is a part of simple_audio
// Copyright (c) 2022 Erikas Taroza <erikastaroza@gmail.com>
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

use std::sync::RwLock;

use flutter_rust_bridge::{StreamSink, support::lazy_static};

lazy_static! { static ref METADATA_CALLBACK_STREAM:RwLock<Option<StreamSink<bool>>> = RwLock::new(None); }

/// Creates a new stream for responding to callbacks from the metadata handler.
pub fn metadata_callback_stream(stream:StreamSink<bool>)
{
    let mut state_stream = METADATA_CALLBACK_STREAM.write().unwrap();
    *state_stream = Some(stream);
}

/// Updates the stream with the given value.
/// `True` means that a signal for "Next" was received.
/// `False` means that a signal for "Previous" was received.
pub fn update_metadata_callback_stream(value:bool)
{
    if let Some(stream) = &*METADATA_CALLBACK_STREAM.read().unwrap()
    { stream.add(value); }
}