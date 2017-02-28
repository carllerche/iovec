//! IoVec extensions for Unix platforms.
//!
//! These functions provide conversions to unix specific representations of the
//! vectored data.
//!
//! # Examples
//!
//! ```
//! use iovec::IoVec;
//! use iovec::unix;
//!
//! let a = b"hello".to_vec();
//! let b = b"world".to_vec();
//!
//! let bufs: &[&IoVec] = &[a[..].as_ref(), b[..].as_ref()];
//! let os_bufs = unix::as_os_slice(&bufs[..]);
//!
//! // Use the `os_bufs` slice with `writev`.
//! ```

use IoVec;
use libc;

use std::mem;

/// Convert a slice of `IoVec` refs to a slice of `libc::iovec`.
///
/// The return value can be passed to `writev` bindings.
///
/// # Examples
///
/// ```
/// use iovec::IoVec;
/// use iovec::unix;
///
/// let a = b"hello".to_vec();
/// let b = b"world".to_vec();
///
/// let bufs: &[&IoVec] = &[a[..].as_ref(), b[..].as_ref()];
/// let os_bufs = unix::as_os_slice(bufs);
///
/// // Use the `os_bufs` slice with `writev`.
/// ```
pub fn as_os_slice<'a>(iov: &'a [&IoVec]) -> &'a [libc::iovec] {
    unsafe { mem::transmute(iov) }
}

/// Convert a mutable slice of `IoVec` refs to a mutable slice of `libc::iovec`.
///
/// The return value can be passed to `readv` bindings.
///
/// # Examples
///
/// ```
/// use iovec::IoVec;
/// use iovec::unix;
///
/// let mut a = [0; 10];
/// let mut b = [0; 10];
///
/// let bufs: &mut [&mut IoVec] = &mut [a[..].as_mut(), b[..].as_mut()];
/// let os_bufs = unix::as_os_slice_mut(bufs);
///
/// // Use the `os_bufs` slice with `readv`.
/// ```
pub fn as_os_slice_mut<'a>(iov: &'a mut [&mut IoVec]) -> &'a mut [libc::iovec] {
    unsafe { mem::transmute(iov) }
}
