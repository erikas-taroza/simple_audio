use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_new__static_method__Player(port_: i64, mpris_name: *mut wire_uint_8_list) {
    wire_new__static_method__Player_impl(port_, mpris_name)
}

#[no_mangle]
pub extern "C" fn wire_playback_state_stream__static_method__Player(port_: i64) {
    wire_playback_state_stream__static_method__Player_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_progress_state_stream__static_method__Player(port_: i64) {
    wire_progress_state_stream__static_method__Player_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_metadata_callback_stream__static_method__Player(port_: i64) {
    wire_metadata_callback_stream__static_method__Player_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_playing__method__Player(port_: i64, that: *mut wire_Player) {
    wire_is_playing__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_open__method__Player(
    port_: i64,
    that: *mut wire_Player,
    path: *mut wire_uint_8_list,
    autoplay: bool,
) {
    wire_open__method__Player_impl(port_, that, path, autoplay)
}

#[no_mangle]
pub extern "C" fn wire_play__method__Player(port_: i64, that: *mut wire_Player) {
    wire_play__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_pause__method__Player(port_: i64, that: *mut wire_Player) {
    wire_pause__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_stop__method__Player(port_: i64, that: *mut wire_Player) {
    wire_stop__method__Player_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_set_volume__method__Player(port_: i64, that: *mut wire_Player, volume: f32) {
    wire_set_volume__method__Player_impl(port_, that, volume)
}

#[no_mangle]
pub extern "C" fn wire_seek__method__Player(port_: i64, that: *mut wire_Player, seconds: u64) {
    wire_seek__method__Player_impl(port_, that, seconds)
}

#[no_mangle]
pub extern "C" fn wire_set_metadata__method__Player(
    port_: i64,
    that: *mut wire_Player,
    metadata: *mut wire_Metadata,
) {
    wire_set_metadata__method__Player_impl(port_, that, metadata)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_metadata_0() -> *mut wire_Metadata {
    support::new_leak_box_ptr(wire_Metadata::new_with_null_ptr())
}

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

impl Wire2Api<Metadata> for *mut wire_Metadata {
    fn wire2api(self) -> Metadata {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Metadata>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Player> for *mut wire_Player {
    fn wire2api(self) -> Player {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Player>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Metadata> for wire_Metadata {
    fn wire2api(self) -> Metadata {
        Metadata {
            title: self.title.wire2api(),
            artist: self.artist.wire2api(),
            album: self.album.wire2api(),
            art_url: self.art_url.wire2api(),
        }
    }
}

impl Wire2Api<Player> for wire_Player {
    fn wire2api(self) -> Player {
        Player {
            dummy: self.dummy.wire2api(),
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
pub struct wire_Metadata {
    title: *mut wire_uint_8_list,
    artist: *mut wire_uint_8_list,
    album: *mut wire_uint_8_list,
    art_url: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Player {
    dummy: i32,
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

impl NewWithNullPtr for wire_Metadata {
    fn new_with_null_ptr() -> Self {
        Self {
            title: core::ptr::null_mut(),
            artist: core::ptr::null_mut(),
            album: core::ptr::null_mut(),
            art_url: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_Player {
    fn new_with_null_ptr() -> Self {
        Self {
            dummy: Default::default(),
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
