use std::{
    ffi::CStr,
    ptr
};

use bitflags::bitflags;
use libc::c_int;

use crate::common::{CapErr, CapErrType, CapResult};

use super::CapChannel;

/// A handle to the Casper sysctl service
///
/// # Examples
/// ```
/// # use capsicum::{Casper, CapSysctlLimit, CapSysctlFlags};
///
/// let casper = Casper::new().unwrap();
/// let sysctl = casper.sysctl().unwrap();
/// ```
#[derive(Debug)]
pub struct CapSysctl(CapChannel);

impl CapSysctl {
    pub(super) fn new(chan: CapChannel) -> Self {
        CapSysctl(chan)
    }

    pub fn limit(&self) -> CapResult<CapSysctlLimit> {
        // Safe as long as the channel pointer is valid
        let lim = unsafe { libc::cap_sysctl_limit_init(self.0.0) };
        if lim == ptr::null_mut() {
            Err(CapErr::from(CapErrType::Invalid))
        } else {
            Ok(CapSysctlLimit(lim))
        }
    }
}

bitflags! {
    #[repr(transparent)]
    pub struct CapSysctlFlags: c_int {
        const READ = libc::CAP_SYSCTL_READ;
        const WRITE = libc::CAP_SYSCTL_WRITE;
        const RDWR = libc::CAP_SYSCTL_RDWR;
        const RECURSIVE = libc::CAP_SYSCTL_RECURSIVE;
    }
}

/// A limit handle used to store a list of permitted sysctls and access rights
// Must not be Clone or Copy!  The inner pointer is an opaque structure created
// by cap_sysctl_limit_init, and must be freed with cap_sysctl_limit or, on
// error, cap_sysctl_limit_name or cap_sysctl_limit_mib.
#[derive(Debug)]
pub struct CapSysctlLimit(*mut libc::cap_sysctl_limit_t);

impl CapSysctlLimit {
    // TODO: add_mib
    /// Add the named sysctl to the allowed limit list.
    pub fn add_name(self, name: &CStr, flags: CapSysctlFlags) -> CapResult<Self>
    {
        let lim = unsafe {
            libc::cap_sysctl_limit_name(self.0, name.as_ptr(), flags.bits())
        };
        if lim == ptr::null_mut() {
            Err(CapErr::from(CapErrType::Invalid))
        } else {
            Ok(CapSysctlLimit(lim))
        }
    }

    /// Apply the limit list
    pub fn limit(self) -> CapResult<()> {
        if unsafe { libc::cap_sysctl_limit(self.0) } < 0 {
            Err(CapErr::from(CapErrType::Invalid))
        } else {
            Ok(())
        }
    }
}
