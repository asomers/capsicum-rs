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

impl CapChannel {
    pub fn as_mut_ptr(&mut self) -> *mut casper_sys::cap_channel_t {
        self.0
    }
    pub fn as_ptr(&self) -> *const casper_sys::cap_channel_t {
        self.0
    }
}

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

    /// Open a connection to the named Casper service.  Should not be used
    /// directly except by [`service!`].
    #[doc(hidden)]
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

mod macros {
    /// Declare a Casper service.
    ///
    /// # Arguments
    /// * `vis` - Visibility of the generated structure.
    /// * `_struct` - The name of the struct that accesses the service.
    /// * `cname` - The name that the service registers with Casper.
    /// * `meth` - The name of the accessor that will be added to `Casper`.
    ///
    /// # Examples
    /// ```
    /// use capsicum::casper;
    /// use const_cstr::const_cstr;
    ///
    /// casper::service!(CapGroup, const_cstr!("system.grp"), group);
    /// ```
    #[macro_export]
    macro_rules! service {
        ($(#[$attr:meta])* $vis:vis $_struct:ident, $cname:expr, $meth:ident) => {
            $(#[$attr])*
            $vis struct $_struct(::capsicum::casper::CapChannel);
            $vis trait CasperExt {
                fn $meth(&self) -> ::std::io::Result<$_struct>;
            }
            impl CasperExt for ::capsicum::casper::Casper {
                fn $meth(&self) -> ::std::io::Result<$_struct> {
                    self.service_open($cname.as_cstr())
                        .map($_struct)
                }
            }
        }
    }
    pub use service;

}

pub use macros::service;
