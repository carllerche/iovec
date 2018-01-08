use std::slice;
use std::usize;

#[derive(Clone)]
pub struct IoVec {
    ptr: *const u8,
    len: usize,
}

pub const MAX_LENGTH: usize = usize::MAX;

impl IoVec {
    pub unsafe fn from_bytes(src: &[u8]) -> Self {
        IoVec {
            ptr: src.as_ptr(),
            len: src.len(),
        }
    }

    pub unsafe fn from_bytes_mut(src: &mut [u8]) -> Self {
        IoVec {
            ptr: src.as_ptr(),
            len: src.len(),
        }
    }

    pub fn as_ref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr as *mut u8, self.len) }
    }
}

impl Default for IoVec {
    fn default() -> Self {
        unsafe { Self::from_bytes(<&[u8]>::default()) }
    }
}
