use std::{ffi::CStr, fmt, io, mem, ptr};

use capsicum::casper;
use const_cstr::{const_cstr};
use libc::gid_t;

casper::service!(
    /// A connection to the Casper `cap_group` helper.
    pub CapGroup, const_cstr!("system.grp"), group
);


impl CapGroup {
    // Not robust; just for demonstration purposes
    /// # Example
    /// ```
    /// use capsicum::casper::Casper;
    /// use cap_grp::CasperExt as _;
    /// let casper = Casper::new().expect("casper failed");
    /// capsicum::enter();
    /// let mut cap_grp = casper.group().expect("cap_grp failed");
    /// let wheel = cap_grp.getgrgid(0).expect("cap_getgrgid_r failed")
    ///     .expect("group wheel not found");
    /// assert_eq!(wheel.gid(), 0);
    /// assert_eq!(wheel.name().to_str().unwrap(), "wheel");
    /// assert!(wheel.members()
    ///     .find(|&m| m.to_str().unwrap() == "root")
    ///     .is_some());
    /// ```
    pub fn getgrgid(&mut self, gid: gid_t) -> io::Result<Option<Group>> {
        // TODO: handle entries that need even larger buffers.
        const BUFSIZE: usize = 1 << 20;

        let mut grp = mem::MaybeUninit::<libc::group>::uninit();
        let mut res = ptr::null_mut();
        let mut buf = Vec::with_capacity(BUFSIZE);

        let error = unsafe {
            cap_grp_sys::cap_getgrgid_r(
                self.0.as_mut_ptr(),
                gid,
                grp.as_mut_ptr(),
                buf.as_mut_ptr(),
                BUFSIZE,
                &mut res
            )
        };
        if error == 0 {
            if res.is_null() {
                Ok(None)
            } else {
                assert_eq!(grp.as_ptr(), res);
                Ok(Some(Group{_buf: buf, grp: unsafe{ grp.assume_init() }}))
            }
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

/// A wrapper around [`struct grent`](https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpwuid_r.html)
pub struct Group{
    // Points to offsets within _buf
    grp: libc::group,
    _buf: Vec<i8>
}

impl Group {
    /// Group ID
    pub fn gid(&self) -> gid_t {
        self.grp.gr_gid
    }

    /// Group name
    pub fn name(&self) -> &CStr {
        // Safe because cap_getgrgid completed successfully, and we limit the
        // lifetime of the return value, and we have no mutators.
        unsafe { CStr::from_ptr(self.grp.gr_name) }
    }

    /// Group members
    pub fn members(&self) -> Members {
        Members::new(self)
    }

    /// Group password
    pub fn passwd(&self) -> &CStr {
        // Safe because cap_getgrgid completed successfully, and we limit the
        // lifetime of the return value, and we have no mutators.
        unsafe { CStr::from_ptr(self.grp.gr_passwd) }
    }
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Group")
            .field("gr_gid", &self.gid())
            .field("gr_name", &self.name())
            .field("gr_passwd", &self.passwd())
            .field("gr_mem", &DebugMembers(self))
            .finish()
    }
}

/// A special struct that only exists to debug Group
struct DebugMembers<'a>(&'a Group);

impl<'a> fmt::Debug for DebugMembers<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list()
            .entries(Members::new(self.0))
            .finish()
    }
}

/// The membership list of a [`Group`].
pub struct Members<'a> {
    grp: &'a Group,
    ofs: isize
}

impl<'a> Members<'a> {
    fn new(grp: &'a Group) -> Self {
        Self{grp, ofs: 0}
    }
}

impl<'a> Iterator for Members<'a> {
    type Item = &'a CStr;

    fn next(&mut self) -> Option<Self::Item> {
        // Safe because cap_getgrgid_r succeeded, we never mutate _buf, and we
        // terminate iteration when we find a null pointer.
        unsafe {
            let u = self.grp.grp.gr_mem.offset(self.ofs);
            if (*u).is_null() {
                None
            } else {
                let s = CStr::from_ptr(*u);
                self.ofs += 1;
                Some(s)
            }
        }
    }
}
