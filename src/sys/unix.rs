use libc;
use std::{slice, usize};

#[derive(Clone)]
pub struct IoVec {
    inner: libc::iovec,
}

pub const MAX_LENGTH: usize = usize::MAX;

impl IoVec {
    pub fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.inner.iov_base as *const u8,
                self.inner.iov_len)
        }
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.inner.iov_base as *mut u8,
                self.inner.iov_len)
        }
    }
}

impl<'a> From<&'a [u8]> for IoVec {
    fn from(src: &'a [u8]) -> Self {
        IoVec {
            inner: libc::iovec {
                iov_base: src.as_ptr() as *mut _,
                iov_len: src.len(),
            },
        }
    }
}

impl<'a> From<&'a mut [u8]> for IoVec {
    fn from(src: &'a mut [u8]) -> Self {
        IoVec {
            inner: libc::iovec {
                iov_base: src.as_ptr() as *mut _,
                iov_len: src.len(),
            },
        }
    }
}

impl Default for IoVec {
    fn default() -> Self {
        Self::from(<&[u8]>::default())
    }
}
