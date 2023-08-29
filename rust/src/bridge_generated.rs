#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.74.0.

use crate::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::media_controllers::types::MediaControlAction;
use crate::media_controllers::types::Metadata;
use crate::utils::types::Callback;
use crate::utils::types::PlaybackState;
use crate::utils::types::ProgressState;
use crate::Player;

// Section: wire functions

fn SimpleAudio_wire_new__static_method__Player_impl(
    port_: MessagePort,
    actions: impl Wire2Api<Vec<MediaControlAction>> + UnwindSafe,
    dbus_name: impl Wire2Api<String> + UnwindSafe,
    hwnd: impl Wire2Api<Option<i64>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new__static_method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_actions = actions.wire2api();
            let api_dbus_name = dbus_name.wire2api();
            let api_hwnd = hwnd.wire2api();
            move |task_callback| Ok(Player::new(api_actions, api_dbus_name, api_hwnd))
        },
    )
}
fn SimpleAudio_wire_dispose__static_method__Player_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "dispose__static_method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(Player::dispose()),
    )
}
fn SimpleAudio_wire_playback_state_stream__static_method__Player_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "playback_state_stream__static_method__Player",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| Ok(Player::playback_state_stream(task_callback.stream_sink())),
    )
}
fn SimpleAudio_wire_progress_state_stream__static_method__Player_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "progress_state_stream__static_method__Player",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| Ok(Player::progress_state_stream(task_callback.stream_sink())),
    )
}
fn SimpleAudio_wire_callback_stream__static_method__Player_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "callback_stream__static_method__Player",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| Ok(Player::callback_stream(task_callback.stream_sink())),
    )
}
fn SimpleAudio_wire_is_playing__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "is_playing__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(Player::is_playing(&api_that))
        },
    )
}
fn SimpleAudio_wire_has_preloaded__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "has_preloaded__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(Player::has_preloaded(&api_that))
        },
    )
}
fn SimpleAudio_wire_get_progress__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_progress__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(Player::get_progress(&api_that))
        },
    )
}
fn SimpleAudio_wire_open__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
    path: impl Wire2Api<String> + UnwindSafe,
    autoplay: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "open__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_path = path.wire2api();
            let api_autoplay = autoplay.wire2api();
            move |task_callback| Player::open(&api_that, api_path, api_autoplay)
        },
    )
}
fn SimpleAudio_wire_preload__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
    path: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "preload__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_path = path.wire2api();
            move |task_callback| Player::preload(&api_that, api_path)
        },
    )
}
fn SimpleAudio_wire_play_preload__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "play_preload__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Player::play_preload(&api_that)
        },
    )
}
fn SimpleAudio_wire_play__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "play__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(Player::play(&api_that))
        },
    )
}
fn SimpleAudio_wire_pause__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "pause__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(Player::pause(&api_that))
        },
    )
}
fn SimpleAudio_wire_stop__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "stop__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(Player::stop(&api_that))
        },
    )
}
fn SimpleAudio_wire_loop_playback__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
    should_loop: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "loop_playback__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_should_loop = should_loop.wire2api();
            move |task_callback| Ok(Player::loop_playback(&api_that, api_should_loop))
        },
    )
}
fn SimpleAudio_wire_set_volume__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
    volume: impl Wire2Api<f32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_volume__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_volume = volume.wire2api();
            move |task_callback| Ok(Player::set_volume(&api_that, api_volume))
        },
    )
}
fn SimpleAudio_wire_seek__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
    seconds: impl Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "seek__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_seconds = seconds.wire2api();
            move |task_callback| Ok(Player::seek(&api_that, api_seconds))
        },
    )
}
fn SimpleAudio_wire_set_metadata__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
    metadata: impl Wire2Api<Metadata> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_metadata__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_metadata = metadata.wire2api();
            move |task_callback| Ok(Player::set_metadata(&api_that, api_metadata))
        },
    )
}
fn SimpleAudio_wire_normalize_volume__method__Player_impl(
    port_: MessagePort,
    that: impl Wire2Api<Player> + UnwindSafe,
    should_normalize: impl Wire2Api<bool> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "normalize_volume__method__Player",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_should_normalize = should_normalize.wire2api();
            move |task_callback| Ok(Player::normalize_volume(&api_that, api_should_normalize))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<i64> for i64 {
    fn wire2api(self) -> i64 {
        self
    }
}

impl Wire2Api<MediaControlAction> for i32 {
    fn wire2api(self) -> MediaControlAction {
        match self {
            0 => MediaControlAction::Rewind,
            1 => MediaControlAction::SkipPrev,
            2 => MediaControlAction::PlayPause,
            3 => MediaControlAction::SkipNext,
            4 => MediaControlAction::FastForward,
            _ => unreachable!("Invalid variant for MediaControlAction: {}", self),
        }
    }
}

impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for Callback {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::MediaControlSkipPrev => 0,
            Self::MediaControlSkipNext => 1,
            Self::NetworkStreamError => 2,
            Self::DecodeError => 3,
            Self::PlaybackLooped => 4,
            Self::DurationCalculated => 5,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Callback {}

impl support::IntoDart for PlaybackState {
    fn into_dart(self) -> support::DartAbi {
        match self {
            Self::Play => 0,
            Self::Pause => 1,
            Self::Done => 2,
        }
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for PlaybackState {}
impl support::IntoDart for Player {
    fn into_dart(self) -> support::DartAbi {
        vec![self.controls.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Player {}

impl support::IntoDart for ProgressState {
    fn into_dart(self) -> support::DartAbi {
        vec![self.position.into_dart(), self.duration.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ProgressState {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
