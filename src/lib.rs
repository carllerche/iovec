//! A specialized byte slice type for performing vectored I/O operations.
//!
//! For more detail, see [`IoVec`] documentation.
//!
//! [`IoVec`]: struct.IoVec.html

#![no_std]

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate winapi;

mod sys;

use core::{mem, ops, slice};
use core::marker::PhantomData;

#[cfg(unix)]
pub mod unix;

/// Max length of an `IoVec` slice.
///
/// Attempts to convert slices longer than this value will result in a panic.
pub const MAX_LENGTH: usize = sys::MAX_LENGTH;

/// A specialized byte slice type for performing vectored I/O operations.
///
/// On all systems, the types needed to perform vectored I/O systems have the
/// same size as Rust's [`slice`]. However, the layout is not necessarily the
/// same. `IoVec` provides a portable compatibility layer.
///
/// The `IoVec` behaves like a Rust [`slice`], providing the same functions.
/// It also provides conversion functions to and from the OS specific vectored
/// types.
///
/// [`slice`]: https://doc.rust-lang.org/std/primitive.slice.html
///
/// # Examples
///
/// ```
/// use iovec::IoVec;
///
/// let mut data = vec![];
/// data.extend_from_slice(b"hello");
///
/// let iovec: IoVec = data.as_slice().into();
///
/// assert_eq!(&iovec[..], &b"hello"[..]);
/// ```
///
/// # Panics
///
/// Attempting to convert a zero-length slice or a slice longer than
/// [`MAX_LENGTH`] to an `IoVec` will result in a panic.
///
/// [`MAX_LENGTH`]: constant.MAX_LENGTH.html
pub struct IoVec<'a> {
    sys: sys::IoVec,
    _p: PhantomData<&'a [u8]>,
}

/// Mutable byte slice type for performing vectored I/O operations.
pub struct IoVecMut<'a> {
    sys: sys::IoVec,
    _p: PhantomData<&'a mut [u8]>,
}

// ===== impl IoVec =====

impl<'a> IoVec<'a> {
    /// Convert an `IoVec` from a byte slice
    pub fn from_bytes(slice: &'a [u8]) -> Self {
        IoVec {
            sys: unsafe { sys::IoVec::from_bytes(slice) },
            _p: PhantomData,
        }
    }

    /// Convert a slice of mutable iovecs to immutable iovecs
    pub fn from_mut_slice<'b>(slice: &'b [IoVecMut<'a>]) -> &'b [Self] {
        unsafe { transmute_slice(slice) }
    }

    /// Immutable borrow of the iovec.
    pub fn borrow(&self) -> IoVec {
        IoVec {
            sys: self.sys.clone(),
            _p: PhantomData,
        }
    }
}

impl<'a> ops::Deref for IoVec<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.sys.as_ref()
    }
}

impl<'a> From<&'a [u8]> for IoVec<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        IoVec::from_bytes(bytes)
    }
}

impl<'a> Default for IoVec<'a> {
    fn default() -> Self {
        IoVec {
            sys: sys::IoVec::default(),
            _p: PhantomData,
        }
    }
}

// ===== impl IoVecMut =====

impl<'a> IoVecMut<'a> {
    /// Convert an `IoVecMut` from a mutable byte slice.
    pub fn from_bytes(slice: &'a mut [u8]) -> Self {
        IoVecMut {
            sys: unsafe { sys::IoVec::from_bytes_mut(slice) },
            _p: PhantomData,
        }
    }

    /// Immutable borrow of the iovec.
    pub fn borrow(&self) -> IoVec {
        IoVec {
            sys: self.sys.clone(),
            _p: PhantomData,
        }
    }

    /// Mutable borrow of the iovec.
    pub fn borrow_mut(&mut self) -> IoVecMut {
        IoVecMut {
            sys: self.sys.clone(),
            _p: PhantomData,
        }
    }
}

impl<'a> ops::Deref for IoVecMut<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.sys.as_ref()
    }
}

impl<'a> ops::DerefMut for IoVecMut<'a> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.sys.as_mut()
    }
}

impl<'a> From<&'a mut [u8]> for IoVecMut<'a> {
    fn from(bytes: &'a mut [u8]) -> Self {
        IoVecMut::from_bytes(bytes)
    }
}

impl<'a> Default for IoVecMut<'a> {
    fn default() -> Self {
        IoVecMut {
            sys: sys::IoVec::default(),
            _p: PhantomData,
        }
    }
}

/// Transmutes a &[T] into a &[U]
unsafe fn transmute_slice<T, U>(src: &[T]) -> &[U] {
    debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<U>());
    debug_assert_eq!(mem::align_of::<T>(), mem::align_of::<U>());
    let ptr = src.as_ptr();
    debug_assert_eq!(ptr as usize % mem::align_of::<U>(), 0);
    slice::from_raw_parts(ptr as *const U, src.len())
}

/// Transmutes a &[T] into a &[U]
unsafe fn transmute_mut_slice<T, U>(src: &mut [T]) -> &mut [U] {
    debug_assert_eq!(mem::size_of::<T>(), mem::size_of::<U>());
    debug_assert_eq!(mem::align_of::<T>(), mem::align_of::<U>());
    let ptr = src.as_mut_ptr();
    debug_assert_eq!(ptr as usize % mem::align_of::<U>(), 0);
    slice::from_raw_parts_mut(ptr as *mut U, src.len())
}

#[cfg(test)]
mod test {
    extern crate std;

    use self::std::vec::Vec;
    use super::{IoVec, IoVecMut};

    #[test]
    fn convert_ref() {
        let buf: IoVec = (&b"hello world"[..]).into();
        assert_eq!(buf[..], b"hello world"[..]);
    }

    #[test]
    fn convert_mut() {
        let mut buf: Vec<u8> = b"hello world".to_vec();
        let buf: IoVecMut = (&mut buf[..]).into();
        assert_eq!(buf[..], b"hello world"[..]);
    }

    #[test]
    fn default() {
        let buf: IoVec = IoVec::default();
        assert_eq!(buf[..], b""[..]);
    }
}
