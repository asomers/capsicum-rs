use std::ptr;

use crate::common::{CapErr, CapErrType, CapResult, CapRights};

mod cap_sysctl;

pub use cap_sysctl::CapSysctl;

/// A channel to communicate with Casper or Casper services
// Must not be Clone or Copy!  The inner pointer is an opaque structure created
// by cap_init, and must be freed with cap_close.
#[derive(Debug)]
pub(self) struct CapChannel(*mut libc::cap_channel_t);

impl Drop for CapChannel {
    fn drop(&mut self) {
        // always safe
        unsafe{ libc::cap_close(self.0) }
    }
}

/// A channel to communicate with the Casper process
#[derive(Debug)]
pub struct Casper(CapChannel);

impl Casper {
    pub fn new() -> CapResult<Self> {
        // cap_init is always safe;
        let chan = unsafe { libc::cap_init() };
        if chan == ptr::null_mut() {
            Err(CapErr::from(CapErrType::Invalid))
        } else {
            Ok(Casper(CapChannel(chan)))
        }
    }

    fn service_open(&self, name: &cstr) -> CapResult<CapChannel> {
        let chan = unsafe { libc::cap_service_open(self.0.0, name.as_ptr()) };
        if chan == ptr::null_mut() {
            Err(CapErr::from(CapErrType::Invalid))
        } else {
            Ok(CapChannel(chan))
        }
    }

    pub fn sysctl(&self) -> CapResult<CapSysctl> {
        let chan = self.service_open(const_cstr!("system.sysctl"))?;
        Ok(CapSysctl::new(chan))
    }

    pub fn try_clone(&self) -> CapResult<Self> {
        // Safe as long as self.0 is a valid channel, which we ensure
        let chan2 = unsafe{ libc::cap_clone(self.0) };
        if chan2 == ptr::null_mut() {
            Err(CapErr::from(CapErrType::Invalid))
        } else {
            Ok(Casper(CapChannel(chan2)))
        }
    }
}

//#[derive(Clone, Copy, Debug, Eq, PartialEq)]
//pub enum CapService {
    ///// [`cap_sysctl`](https://www.freebsd.org/cgi/man.cgi?query=cap_sysctl)
    //Sysctl,
    //// Other services are as yet unimplemented
    //// Dns
    //// Grp
    //// Net
    //// Pwd
    //// Syslog
//}

//impl CapService {
    //fn new(self) -> 
