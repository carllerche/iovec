use std::mem;
use std::usize;

pub struct IoVec {
    inner: [u8],
}

pub const MAX_LENGTH: usize = usize::MAX;

impl IoVec {
    pub fn as_ref(&self) -> &[u8] {
        &self.inner
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl<'a> From<&'a [u8]> for &'a IoVec {
    fn from(src: &'a [u8]) -> Self {
        assert!(src.len() > 0);
        unsafe { mem::transmute(src) }
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut IoVec {
    fn from(src: &'a mut [u8]) -> Self {
        assert!(src.len() > 0);
        unsafe { mem::transmute(src) }
    }
}
