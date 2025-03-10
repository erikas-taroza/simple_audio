// Modified from https://github.com/serenity-rs/songbird/blob/next/src/input/codecs/opus.rs
//
// ISC License (ISC)
//
// Copyright (c) 2020, Songbird Contributors
//
// Permission to use, copy, modify, and/or distribute this software for any purpose
// with or without fee is hereby granted, provided that the above copyright notice
// and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
// OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
// THIS SOFTWARE.

use audiopus::{
    coder::{Decoder as AudiopusDecoder, GenericCtl},
    Channels, Error as OpusError, ErrorCode, SampleRate,
};
use symphonia_core::{
    audio::{AsAudioBufferRef, AudioBuffer, AudioBufferRef, Layout, Signal, SignalSpec},
    codecs::{
        CodecDescriptor, CodecParameters, Decoder, DecoderOptions, FinalizeResult, CODEC_TYPE_OPUS,
    },
    errors::{decode_error, Result as SymphResult},
    formats::Packet,
};

const SAMPLE_RATE: SampleRate = SampleRate::Hz48000;

const SAMPLE_RATE_RAW: usize = 48_000;

/// This is equally the number of stereo (joint) samples in an audio frame.
const MONO_FRAME_SIZE: usize = SAMPLE_RATE_RAW / 1000 * 60; // 60ms is the max frame size

/// Number of individual samples in one complete frame of stereo audio.
const STEREO_FRAME_SIZE: usize = 2 * MONO_FRAME_SIZE;

/// Opus decoder for symphonia, based on libopus v1.3 (via [`audiopus`]).
pub struct OpusDecoder
{
    inner: AudiopusDecoder,
    params: CodecParameters,
    buf: AudioBuffer<f32>,
    rawbuf: Vec<f32>,
}

/// # SAFETY
/// The underlying Opus decoder (currently) requires only a `&self` parameter
/// to decode given packets, which is likely a mistaken decision.
///
/// This struct makes stronger assumptions and only touches FFI decoder state with a
/// `&mut self`, preventing data races via `&OpusDecoder` as required by `impl Sync`.
/// No access to other internal state relies on unsafety or crosses FFI.
unsafe impl Sync for OpusDecoder {}

impl OpusDecoder
{
    fn decode_inner(&mut self, packet: &Packet) -> SymphResult<()>
    {
        let s_ct = loop {
            let pkt = if packet.buf().is_empty() {
                None
            }
            else if let Ok(checked_pkt) = packet.buf().try_into() {
                Some(checked_pkt)
            }
            else {
                return decode_error("Opus packet was too large (greater than i32::MAX bytes).");
            };
            let out_space = (&mut self.rawbuf[..]).try_into().expect("The following logic expands this buffer safely below i32::MAX, and we throw our own error.");

            match self.inner.decode_float(pkt, out_space, false) {
                Ok(v) => break v,
                Err(OpusError::Opus(ErrorCode::BufferTooSmall)) => {
                    // double the buffer size
                    // correct behav would be to mirror the decoder logic in the udp_rx set.
                    let new_size = (self.rawbuf.len() * 2).min(i32::MAX as usize);
                    if new_size == self.rawbuf.len() {
                        return decode_error("Opus frame too big: cannot expand opus frame decode buffer any further.");
                    }

                    self.rawbuf.resize(new_size, 0.0);
                    self.buf = AudioBuffer::new(
                        self.rawbuf.len() as u64 / 2,
                        SignalSpec::new_with_layout(SAMPLE_RATE_RAW as u32, Layout::Stereo),
                    );
                }
                Err(_) => {
                    return decode_error("Opus decode error");
                }
            }
        };

        self.buf.clear();
        self.buf.render_reserved(Some(s_ct));

        // Forcibly assuming stereo, for now.
        for ch in 0..2 {
            let iter = self.rawbuf.chunks_exact(2).map(|chunk| chunk[ch]);
            for (tgt, src) in self.buf.chan_mut(ch).iter_mut().zip(iter) {
                *tgt = src;
            }
        }

        Ok(())
    }
}

impl Decoder for OpusDecoder
{
    fn try_new(params: &CodecParameters, _options: &DecoderOptions) -> SymphResult<Self>
    {
        let inner = AudiopusDecoder::new(SAMPLE_RATE, Channels::Stereo).unwrap();

        let mut params = params.clone();
        params.with_sample_rate(SAMPLE_RATE_RAW as u32);

        Ok(Self {
            inner,
            params,
            buf: AudioBuffer::new(
                MONO_FRAME_SIZE as u64,
                SignalSpec::new_with_layout(SAMPLE_RATE_RAW as u32, Layout::Stereo),
            ),
            rawbuf: vec![0.0f32; STEREO_FRAME_SIZE],
        })
    }

    fn supported_codecs() -> &'static [CodecDescriptor]
    {
        &[symphonia_core::support_codec!(
            CODEC_TYPE_OPUS,
            "opus",
            "libopus (1.3+, audiopus)"
        )]
    }

    fn codec_params(&self) -> &CodecParameters
    {
        &self.params
    }

    fn decode(&mut self, packet: &Packet) -> SymphResult<AudioBufferRef<'_>>
    {
        if let Err(e) = self.decode_inner(packet) {
            self.buf.clear();
            Err(e)
        }
        else {
            Ok(self.buf.as_audio_buffer_ref())
        }
    }

    fn reset(&mut self)
    {
        _ = self.inner.reset_state();
    }

    fn finalize(&mut self) -> FinalizeResult
    {
        FinalizeResult::default()
    }

    fn last_decoded(&self) -> AudioBufferRef
    {
        self.buf.as_audio_buffer_ref()
    }
}
