#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libnv_sys::nvlist as nvlist_t;

/// For use with [`service_register`].
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct ServiceRegisterFlags(i32);
impl ServiceRegisterFlags {
    pub const STDIO: ServiceRegisterFlags = ServiceRegisterFlags(1);
    pub const FD: ServiceRegisterFlags = ServiceRegisterFlags(2);
    pub const NO_UNIQ_LIMITS: ServiceRegisterFlags = ServiceRegisterFlags(4);
}

include!("ffi.rs");
