use std::ptr;

use const_cstr::const_cstr;

use crate::common::{CapErr, CapErrType, CapResult, CapRights};

use super::CapChannel;

/// A limit handle used to store a list of permitted sysctls and access rights
// Must not be Clone or Copy!  The inner pointer is an opaque structure created
// by cap_sysctl_limit_init, and must be freed with cap_sysctl_limit or, on
// error, cap_sysctl_limit_name or cap_sysctl_limit_mib.
#[derive(Debug)]
struct CapSysctlLimit(*mut libc::cap_sysctl_limit_t);

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
