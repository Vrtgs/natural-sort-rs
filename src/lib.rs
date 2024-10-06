#![no_std]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::cmp::Ordering;
use core::marker::PhantomData;

#[derive(Debug, Copy, Default)]
#[repr(transparent)]
pub struct Natural<T, Ref: ?Sized = str>(pub T, PhantomData<Ref>);

pub type NaturalAscii<T> = Natural<T, [u8]>;

impl<T, Ref: ?Sized> Natural<T, Ref> {
    pub fn new(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Natural<T> {
    pub fn str(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> NaturalAscii<T> {
    pub fn ascii(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: Clone, Ref: ?Sized> Clone for Natural<T, Ref> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0)
    }
}
impl<T: AsRef<Ref>, Ref: ?Sized + NaturalSortable> PartialEq for Natural<T, Ref> {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref().eq(other.0.as_ref())
    }
}
impl<T: AsRef<Ref>, Ref: ?Sized + NaturalSortable> PartialOrd for Natural<T, Ref> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: AsRef<Ref>, Ref: ?Sized + NaturalSortable> Eq for Natural<T, Ref> {}
impl<T: AsRef<Ref>, Ref: ?Sized + NaturalSortable> Ord for Natural<T, Ref> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        natural_cmp(&self.0, &other.0)
    }
}
impl<T: AsRef<Ref>, Ref: ?Sized + NaturalSortable> AsRef<Ref> for Natural<T, Ref> {
    fn as_ref(&self) -> &Ref {
        self.0.as_ref()
    }
}

mod sealed {
    pub trait NaturalSortable: AsRef<Self> + 'static {
        fn eq(&self, other: &Self) -> bool;
    }

    impl NaturalSortable for str {
        fn eq(&self, other: &Self) -> bool {
            self == other
        }
    }
    impl NaturalSortable for [u8] {
        fn eq(&self, other: &Self) -> bool {
            self == other
        }
    }

    pub trait NaturalSort {}

    impl<T> NaturalSort for [T] {}
}

pub trait NaturalSortable: sealed::NaturalSortable {
    /// returns the [natural sort order](https://en.wikipedia.org/wiki/Natural_sort_order)
    /// note the bytes are interpreted as ascii when using `[u8]`
    fn natural_cmp(&self, other: &Self) -> Ordering;
}

impl NaturalSortable for str {
    #[inline(always)]
    fn natural_cmp(&self, other: &Self) -> Ordering {
        cmp_ascii(self.as_bytes(), other.as_bytes())
    }
}

impl NaturalSortable for [u8] {
    #[inline(always)]
    fn natural_cmp(&self, other: &Self) -> Ordering {
        cmp_ascii(self, other)
    }
}

pub trait NaturalSort<T>: sealed::NaturalSort {
    fn natural_sort_unstable<Ref: ?Sized + NaturalSortable>(&mut self)
    where
        T: AsRef<Ref>;

    fn natural_sort_unstable_by_key<Ref: ?Sized + NaturalSortable, K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: AsRef<Ref>;

    #[cfg(feature = "alloc")]
    fn natural_sort<Ref: ?Sized + NaturalSortable>(&mut self)
    where
        T: AsRef<Ref>;

    #[cfg(feature = "alloc")]
    fn natural_sort_by_key<Ref: ?Sized + NaturalSortable, K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: AsRef<Ref>;

    #[cfg(feature = "alloc")]
    fn natural_sort_by_cached_key<Ref: ?Sized + NaturalSortable, K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: AsRef<Ref>;
}

impl<T> NaturalSort<T> for [T] {
    /// like `<[T]>::sort_unstable` but using natural sort order
    /// ## Example
    /// ```
    /// # use natural_sort_rs::NaturalSort;
    ///
    /// let mut files = ["file0002.txt", "file1.txt"];
    ///
    /// files.natural_sort_unstable::<str>();
    /// assert_eq!(files, ["file1.txt", "file0002.txt"])
    /// ```
    fn natural_sort_unstable<Ref: ?Sized + NaturalSortable>(&mut self)
    where
        T: AsRef<Ref>,
    {
        self.sort_unstable_by(natural_cmp)
    }

    /// like `<[T]>::sort_unstable_by_key` but using natural sort order
    /// ## Example
    /// ```
    /// # use natural_sort_rs::NaturalSort;
    ///
    /// let mut files = [4, 2, 3, 1];
    ///
    /// files.natural_sort_unstable_by_key::<str, _, _>(|x| x.to_string());
    /// assert_eq!(files, [1, 2, 3, 4])
    /// ```
    fn natural_sort_unstable_by_key<Ref: ?Sized + NaturalSortable, K, F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> K,
        K: AsRef<Ref>,
    {
        self.sort_unstable_by_key(|x| Natural::new(f(x)))
    }

    /// like `<[T]>::sort` but using natural sort order
    /// ## Example
    /// ```
    /// # use natural_sort_rs::NaturalSort;
    ///
    /// let mut files = ["file0002.txt", "file1.txt"];
    ///
    /// files.natural_sort::<str>();
    /// assert_eq!(files, ["file1.txt", "file0002.txt"])
    /// ```
    #[cfg(feature = "alloc")]
    fn natural_sort<Ref: ?Sized + NaturalSortable>(&mut self)
    where
        T: AsRef<Ref>,
    {
        self.sort_by(natural_cmp);
    }

    /// like `<[T]>::sort_by_key` but using natural sort order
    /// ## Example
    /// ```
    /// # use natural_sort_rs::NaturalSort;
    ///
    /// let mut files = [4, 2, 3, 1];
    ///
    /// files.natural_sort_by_key::<str, _, _>(|x| x.to_string());
    /// assert_eq!(files, [1, 2, 3, 4])
    /// ```
    #[cfg(feature = "alloc")]
    fn natural_sort_by_key<Ref: ?Sized + NaturalSortable, K, F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> K,
        K: AsRef<Ref>,
    {
        self.sort_by_key(|x| Natural::new(f(x)))
    }

    /// like sort but using natural sort order
    /// ## Example
    /// ```
    /// # use std::path::{Path, PathBuf};
    /// # use natural_sort_rs::NaturalSort;
    ///
    /// let mut files = ["file1", "file2", "file4", "file3"].map(PathBuf::from);
    ///
    /// files.natural_sort_by_key::<[u8], _, _>(|x| x.as_os_str().as_encoded_bytes().to_owned());
    /// assert_eq!(files, ["file1", "file2", "file3", "file4"].map(Path::new))
    /// ```
    #[cfg(feature = "alloc")]
    fn natural_sort_by_cached_key<Ref: ?Sized + NaturalSortable, K, F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> K,
        K: AsRef<Ref>,
    {
        self.sort_by_cached_key(|x| Natural::new(f(x)))
    }
}

pub fn natural_cmp<Ref: ?Sized + NaturalSortable, T: ?Sized + AsRef<Ref>>(x: &T, y: &T) -> Ordering {
    NaturalSortable::natural_cmp(x.as_ref(), y.as_ref())
}

fn cmp_ascii(mut a: &[u8], mut b: &[u8]) -> Ordering {
    while let ([a_c, a_tail @ ..], [b_c, b_tail @ ..]) = (a, b) {
        let ord = match a_c.is_ascii_digit() && b_c.is_ascii_digit() {
            true => cmp_digits(&mut a, &mut b),
            false => {
                a = a_tail;
                b = b_tail;
                a_c.cmp(b_c)
            }
        };

        if ord != Ordering::Equal {
            return ord;
        }
    }

    usize::cmp(&a.len(), &b.len())
}

#[inline]
fn cmp_digits(a: &mut &[u8], b: &mut &[u8]) -> Ordering {
    fn trim_zeros(slice: &mut &[u8]) {
        while let [b'0', rest @ ..] = *slice {
            *slice = rest
        }
    }

    fn read_digits<'a>(slice: &mut &'a [u8]) -> &'a [u8] {
        trim_zeros(slice);

        let slice_start = slice.as_ptr();
        let mut i = 0;
        while let [b'0'..=b'9', rest @ ..] = *slice {
            i += 1;
            *slice = rest;
        }

        // Safety:
        // i < slice_start.len()
        // as i only increases up to slice_start.len() (when all of the slice is digits)
        unsafe { core::slice::from_raw_parts(slice_start, i) }
    }

    let a = read_digits(a);
    let b = read_digits(b);

    match a.len().cmp(&b.len()) {
        Ordering::Equal => {
            // a.len() == b.len()
            // if this isn't true, usize has a bad Ord impl,
            // if that's the case, anything can be UB :>
            // we just do this to help the optimizer
            unsafe { core::hint::assert_unchecked(a.len() == b.len()) };
            a.cmp(b)
        }
        ord => ord,
    }
}

#[cfg(test)]
mod tests {
    use crate::{Natural, NaturalSort};

    #[test]
    fn it_works() {
        let mut files = ["file2.txt", "file11.txt", "file1.txt"];
        files.sort();
        assert_eq!(files, ["file1.txt", "file11.txt", "file2.txt"]);

        assert!(Natural::str("file0002.txt") > Natural::str("file1B.txt"));
        assert!(Natural::str("file0002.txt") < Natural::str("file11.txt"));

        let mut files = [
            "file1.txt",
            "file1B.txt",
            "file00.txt",
            "file11.txt",
            "file0002.txt",
        ];

        files.natural_sort::<str>();

        // Here, "file11.txt" comes last because `natural_sort` saw that there was a
        // number inside the string, and did a numerical, rather than lexical,
        // comparison.
        assert_eq!(
            files,
            [
                "file00.txt",
                "file1.txt",
                "file1B.txt",
                "file0002.txt",
                "file11.txt"
            ]
        );
    }
}
