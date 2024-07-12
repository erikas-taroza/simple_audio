use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_new__static_method__PlayerWrapper(port_: i64) {
    wire_new__static_method__PlayerWrapper_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_dispose__static_method__PlayerWrapper(port_: i64) {
    wire_dispose__static_method__PlayerWrapper_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_playback_started_stream__static_method__PlayerWrapper(port_: i64) {
    wire_playback_started_stream__static_method__PlayerWrapper_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_playback_state_stream__static_method__PlayerWrapper(port_: i64) {
    wire_playback_state_stream__static_method__PlayerWrapper_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_progress_state_stream__static_method__PlayerWrapper(port_: i64) {
    wire_progress_state_stream__static_method__PlayerWrapper_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_error_stream__static_method__PlayerWrapper(port_: i64) {
    wire_error_stream__static_method__PlayerWrapper_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_playback_state__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
) {
    wire_playback_state__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_progress__method__PlayerWrapper(port_: i64, that: *mut wire_PlayerWrapper) {
    wire_progress__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_is_preloaded__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
) {
    wire_is_preloaded__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_is_looping__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
) {
    wire_is_looping__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_is_normalizing__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
) {
    wire_is_normalizing__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_volume__method__PlayerWrapper(port_: i64, that: *mut wire_PlayerWrapper) {
    wire_volume__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_open__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
    path: *mut wire_uint_8_list,
    autoplay: bool,
) {
    wire_open__method__PlayerWrapper_impl(port_, that, path, autoplay)
}

#[no_mangle]
pub extern "C" fn wire_preload__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
    path: *mut wire_uint_8_list,
) {
    wire_preload__method__PlayerWrapper_impl(port_, that, path)
}

#[no_mangle]
pub extern "C" fn wire_play_preload__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
) {
    wire_play_preload__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_clear_preload__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
) {
    wire_clear_preload__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_play__method__PlayerWrapper(port_: i64, that: *mut wire_PlayerWrapper) {
    wire_play__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_pause__method__PlayerWrapper(port_: i64, that: *mut wire_PlayerWrapper) {
    wire_pause__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_stop__method__PlayerWrapper(port_: i64, that: *mut wire_PlayerWrapper) {
    wire_stop__method__PlayerWrapper_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_loop_playback__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
    should_loop: bool,
) {
    wire_loop_playback__method__PlayerWrapper_impl(port_, that, should_loop)
}

#[no_mangle]
pub extern "C" fn wire_set_volume__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
    volume: f32,
) {
    wire_set_volume__method__PlayerWrapper_impl(port_, that, volume)
}

#[no_mangle]
pub extern "C" fn wire_seek__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
    position: i64,
) {
    wire_seek__method__PlayerWrapper_impl(port_, that, position)
}

#[no_mangle]
pub extern "C" fn wire_normalize_volume__method__PlayerWrapper(
    port_: i64,
    that: *mut wire_PlayerWrapper,
    should_normalize: bool,
) {
    wire_normalize_volume__method__PlayerWrapper_impl(port_, that, should_normalize)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_Player() -> wire_Player {
    wire_Player::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_player_wrapper_0() -> *mut wire_PlayerWrapper {
    support::new_leak_box_ptr(wire_PlayerWrapper::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_Player(ptr: *const c_void) {
    unsafe {
        Arc::<Player>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_Player(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Player>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::microseconds(self)
    }
}
impl Wire2Api<RustOpaque<Player>> for wire_Player {
    fn wire2api(self) -> RustOpaque<Player> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<PlayerWrapper> for *mut wire_PlayerWrapper {
    fn wire2api(self) -> PlayerWrapper {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<PlayerWrapper>::wire2api(*wrap).into()
    }
}

impl Wire2Api<PlayerWrapper> for wire_PlayerWrapper {
    fn wire2api(self) -> PlayerWrapper {
        PlayerWrapper {
            internal: self.internal.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_Player {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_PlayerWrapper {
    internal: wire_Player,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Player {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_PlayerWrapper {
    fn new_with_null_ptr() -> Self {
        Self {
            internal: wire_Player::new_with_null_ptr(),
        }
    }
}

impl Default for wire_PlayerWrapper {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
