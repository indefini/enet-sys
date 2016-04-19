extern crate libc;

use libc::{c_void, c_int, c_char, c_float, c_ushort};//, c_ulong, c_long, c_uint, c_uchar, size_t};

pub type RustCb = extern fn(data : *mut c_void);

#[repr(C)]
pub struct ENetHost;
#[repr(C)]
pub struct ENetPeer;

extern "C" {
    pub fn enet_initialize() -> i32;
    pub fn enet_deinitialize();
    pub fn enet_host_destroy(host : *const ENetHost);

    pub fn server_new() -> *const ENetHost;
    pub fn client_new() -> *const ENetHost;
    pub fn handle_event(host : *const ENetHost);
    pub fn client_connect(
        client : *const ENetHost,
        address : *const c_char,
        port : c_ushort
        ) -> *const ENetPeer;
    pub fn host_send(host : *const ENetHost, peer : *const ENetPeer);
}

