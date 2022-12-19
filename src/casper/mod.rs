use std::{
    ffi::CStr,
    io,
    ptr
};

/// A channel to communicate with Casper or Casper services
// Must not be Clone or Copy!  The inner pointer is an opaque structure created
// by cap_init, and must be freed with cap_close.
#[derive(Debug)]
pub struct CapChannel(*mut casper_sys::cap_channel_t);

impl Drop for CapChannel {
    fn drop(&mut self) {
        // always safe
        unsafe{ casper_sys::cap_close(self.0) }
    }
}

/// A channel to communicate with the Casper process
#[derive(Debug)]
pub struct Casper(CapChannel);

impl Casper {
    pub fn new() -> io::Result<Self> {
        // cap_init is always safe;
        let chan = unsafe { casper_sys::cap_init() };
        if chan == ptr::null_mut() {
            Err(io::Error::last_os_error())
        } else {
            Ok(Casper(CapChannel(chan)))
        }
    }

    /// Open a connection to the named Casper service.
    pub fn service_open(&self, name: &CStr) -> io::Result<CapChannel> {
        let chan = unsafe {
            casper_sys::cap_service_open(self.0.0, name.as_ptr()) 
        };
        if chan == ptr::null_mut() {
            Err(io::Error::last_os_error())
        } else {
            Ok(CapChannel(chan))
        }
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        // Safe as long as self.0 is a valid channel, which we ensure
        let chan2 = unsafe{ casper_sys::cap_clone(self.0.0) };
        if chan2 == ptr::null_mut() {
            Err(io::Error::last_os_error())
        } else {
            Ok(Casper(CapChannel(chan2)))
        }
    }
}
