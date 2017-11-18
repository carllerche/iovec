use winapi::{WSABUF, DWORD};
use std::{mem, slice, u32};

#[derive(Clone)]
pub struct IoVec {
    inner: WSABUF,
}

pub const MAX_LENGTH: usize = u32::MAX as usize;

impl IoVec {
    pub fn as_ref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self.inner.buf as *const u8,
                self.inner.len as usize)
        }
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self.inner.buf as *mut u8,
                self.inner.len as usize)
        }
    }
}

impl<'a> From<&'a [u8]> for IoVec {
    fn from(src: &'a [u8]) -> Self {
        IoVec {
            inner: WSABUF {
                buf: src.as_ptr() as *mut _,
                len: src.len() as DWORD,
            }
        }
    }
}

impl<'a> From<&'a mut [u8]> for IoVec {
    fn from(src: &'a mut [u8]) -> Self {
        IoVec {
            inner: WSABUF {
                buf: src.as_ptr() as *mut _,
                len: src.len() as DWORD,
            }
        }
    }
}

impl Default for IoVec {
    fn default() -> Self {
        Self::from(<&[u8]>::default())
    }
}
