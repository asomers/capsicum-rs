//! A toy Casper service that provides `getuid()`.

use std::{io, ptr};

use libc::uid_t;
use libnv::libnv::{NvList, NvFlag};
//use casper_sys::nvlist_t;
use libnv_sys::nvlist as nvlist_t;
use capsicum::casper::{self, Casper};
use const_cstr::{ConstCStr, const_cstr};
use ctor::ctor;

const SERVICE_NAME: ConstCStr = const_cstr!("getuid");

extern fn getuid_cmd(
    cmd: *const ::std::os::raw::c_char,
    arg1: *const nvlist_t,
    arg2: *mut nvlist_t,
    arg3: *mut nvlist_t
) -> i32 {
    todo!()
}

// TODO: make a macro for this part
#[ctor]
unsafe fn init_casper_service() {
    casper_sys::service_register(
        SERVICE_NAME.as_cstr().as_ptr(),
        None,
        Some(getuid_cmd),
        0
    );
}

casper::service!(
    /// A connection to the Casper `uid` helper.
    pub CapUid, const_cstr!("getuid"), uid
);

impl CapUid {
    pub fn uid(&mut self) -> io::Result<uid_t> {
        let nvl = NvList::new(NvFlag::None).unwrap();
        let r = unsafe {
            casper_sys::cap_xfer_nvlist(self.0.as_ptr(), nvl.into())
        };
        if r.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(42)
        }
    }
}

fn main() {
    let casper = Casper::new().unwrap();
    capsicum::enter().unwrap();
    let mut cap_uid = casper.uid().unwrap();
    println!("{:?}", cap_uid.uid());
}
