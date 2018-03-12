use winapi::shared::minwindef::DWORD;
use winapi::shared::ws2def::WSABUF;
use core::{slice, u32};

#[derive(Clone)]
pub struct IoVec {
    inner: WSABUF,
}

pub const MAX_LENGTH: usize = u32::MAX as usize;

impl IoVec {
    pub unsafe fn from_bytes(src: &[u8]) -> Self {
        IoVec {
            inner: WSABUF {
                buf: src.as_ptr() as *mut _,
                len: src.len() as DWORD,
            }
        }
    }

    pub unsafe fn from_bytes_mut(src: &mut [u8]) -> Self {
        IoVec {
            inner: WSABUF {
                buf: src.as_ptr() as *mut _,
                len: src.len() as DWORD,
            }
        }
    }

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

impl Default for IoVec {
    fn default() -> Self {
        unsafe { Self::from_bytes(<&[u8]>::default()) }
    }
}
