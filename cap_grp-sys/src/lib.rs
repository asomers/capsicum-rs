#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use casper_sys::cap_channel_t as cap_channel_t;
use libc::gid_t;
use libc::group;

include!("ffi.rs");
