// This code is took from https://github.com/twmb/slice-copy licenced on MIT licence
// Credits goes to Travis Bischel Copyright (c) 2018

use std::cmp::min;

#[cfg(feature = "nightly")]
trait Cpy<T = Self>
where
    T: ?Sized,
{
    fn copy(&mut self, src: &T) -> usize;
}

#[cfg(feature = "nightly")]
default impl<T> Cpy<[T]> for [T]
where
    T: Copy,
{
    #[inline]
    fn copy(&mut self, src: &Self) -> usize {
        let len = min(src.len(), self.len());
        (&mut self[..len]).copy_from_slice(&src[..len]);
        len
    }
}

#[cfg(feature = "nightly")]
impl Cpy<[u8]> for [u8] {
    #[inline]
    fn copy(&mut self, src: &Self) -> usize {
        use std::io::Read;
        let len = min(src.len(), self.len());
        (&src[..len])
            .read(&mut self[..len])
            .expect("&[u8] reads never error")
    }
}

/// Copies as many `T` as possible from `src` into `dst`, returning the number of `T` copied. This
/// function is short form for `dst.copy_from_slice(src)`, but accounts for if their lengths are
/// unequal to avoid panics.
///
/// With the `nightly` feature, `[u8]` is specialized to use [`Read`], which is implemented
/// specially for small slices.
///
/// [`Read`]: https://doc.rust-lang.org/std/primitive.slice.html#impl-Read
///
/// # Examples
///
/// ```
/// use slice_copy::copy;
///
/// let mut l = vec![1 as u8, 2, 3, 4, 5];
/// let r = vec![10, 11, 12];
///
/// let n = copy(&mut l, &r);
///
/// assert_eq!(n, 3);
/// assert_eq!(l, vec![10, 11, 12, 4, 5]);
/// ```
#[inline]
pub fn copy<T>(dst: &mut [T], src: &[T]) -> usize
where
    T: Copy,
{
    #[cfg(feature = "nightly")]
    {
        dst.copy(src)
    }
    #[cfg(not(feature = "nightly"))]
    {
        let len = min(src.len(), dst.len());
        (&mut dst[..len]).copy_from_slice(&src[..len]);
        len
    }
}

/// Clones as many `T` as possible from `src` into `dst`, returning the number of `T` cloned. This
/// function is short form for `dst.clone_from_slice(src)`, but accounts for if their lengths are
/// unequal to avoid panics.
///
/// Examples
///
/// ```
/// use slice_copy::clone;
///
/// let mut l = b"left".to_vec();
/// let r = b"right".to_vec();
///
/// let n = clone(&mut l, &r);
///
/// assert_eq!(n, 4);
/// assert_eq!(l, b"righ");
/// ```
#[inline]
pub fn clone<T>(dst: &mut [T], src: &[T]) -> usize
where
    T: Clone,
{
    let len = min(src.len(), dst.len());
    (&mut dst[..len]).clone_from_slice(&src[..len]);
    len
}

#[test]
fn test_copy() {
    fn lr() -> (Vec<u8>, Vec<u8>) {
        (b"hello".to_vec(), b"goodbye".to_vec())
    }

    // longer to shorter
    let (mut l, r) = lr();
    assert_eq!(copy(&mut l, &r), 5);
    assert_eq!(l, b"goodb");
    assert_eq!(r, b"goodbye");

    // shorter to longer
    let (l, mut r) = lr();
    assert_eq!(copy(&mut r, &l[..4]), 4);
    assert_eq!(l, b"hello");
    assert_eq!(r, b"hellbye");

    // dst length 0
    let (mut l, r) = lr();
    assert_eq!(copy(&mut l[..0], &r), 0);
    assert_eq!(l, b"hello");
    assert_eq!(r, b"goodbye");

    // src length 0
    assert_eq!(copy(&mut l, &r[..0]), 0);
    assert_eq!(l, b"hello");
    assert_eq!(r, b"goodbye");
}

/// Encode an object as bytes into a byte array
///
/// # Arguments
/// * `bytes` - The bytes buffer that will be written
/// * `offset` - An offset
/// * `size` - Ths size of the obj that will be written
/// * `obj` - The object that will be written
///
pub fn encode_into_bytes<T>(bytes: &mut [u8], offset: usize, size: usize, obj: T) {
    let buffer = &mut bytes[offset..(offset + size)];

    let encoded = unsafe {
        std::slice::from_raw_parts((&obj as *const T) as *const u8, std::mem::size_of::<T>())
    };

    copy(buffer, encoded);
}
