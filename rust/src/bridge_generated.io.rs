use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_new__static_method__Player(port_: i64) {
    wire_new__static_method__Player_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_playback_state_stream__static_method__Player(port_: i64) {
    wire_playback_state_stream__static_method__Player_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_is_playing__method__Player(port_: i64, that: *mut wire_Player) {
    wire_get_is_playing__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_open__method__Player(
    port_: i64,
    that: *mut wire_Player,
    path: *mut wire_uint_8_list,
) {
    wire_open__method__Player_impl(port_, that, path)
}

#[no_mangle]
pub extern "C" fn wire_play__method__Player(port_: i64, that: *mut wire_Player) {
    wire_play__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_pause__method__Player(port_: i64, that: *mut wire_Player) {
    wire_pause__method__Player_impl(port_, that)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_player_0() -> *mut wire_Player {
    support::new_leak_box_ptr(wire_Player::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Player> for *mut wire_Player {
    fn wire2api(self) -> Player {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Player>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Player> for wire_Player {
    fn wire2api(self) -> Player {
        Player {
            is_playing: self.is_playing.wire2api(),
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
    is_playing: bool,
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
            is_playing: Default::default(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
