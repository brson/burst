// Problematic APIs
// * Clone
// * Extend
// * Cow

use St;
use core;
use collections::CAPACITY;
use rcollections::vec::Vec as RVec;
use rcollections::range::RangeArgument;
use ralloc::boxed::Box;

pub struct Vec<T>(RVec<T>);
pub use rcollections::vec::Drain;
pub use rcollections::vec::IntoIter;

// The allocating functions on Vec, augmented with
// the setup token, and their noalloc counterparts,
// which can be used after the setup phase, but panic
// if the capacity is exceeded.
impl<T> Vec<T> {
    pub fn with_capacity(_: &St, capacity: usize) -> Vec<T> {
        Vec(RVec::with_capacity(capacity))
    }
    pub fn reserve(&mut self, _: &St, additional: usize) {
        self.0.reserve(additional)
    }
    pub fn reserve_exact(&mut self, _: &St, additional: usize) {
        self.0.reserve_exact(additional)
    }
    pub fn shrink_to_fit(&mut self, _: &St) {
        self.0.shrink_to_fit()
    }
    pub fn split_off(&mut self, _: &St, at: usize) -> Self {
        Vec(self.0.split_off(at))
    }

    pub fn push(&mut self, _: &St, value: T) {
        self.0.push(value)
    }
    pub fn insert(&mut self, _: &St, index: usize, element: T) {
        self.0.insert(index, element)
    }
    pub fn append(&mut self, _: &St, other: &mut Self) {
        self.0.append(&mut other.0)
    }

    pub fn push_noalloc(&mut self, value: T) {
        if self.len() < self.capacity() {
            self.0.push(value)
        } else {
            panic!("{}", CAPACITY)
        }
    }
    pub fn insert_noalloc(&mut self, index: usize, element: T) {
        if self.len() < self.capacity() {
            self.0.insert(index, element)
        } else {
            panic!("{}", CAPACITY)
        }
    }
    pub fn append_noalloc(&mut self, _: &St, other: &mut Self) {
        // FIXME: overflow
        if self.len() + other.len() <= self.capacity() {
            self.0.append(&mut other.0)
        } else {
            panic!("{}", CAPACITY)
        }
    }
    pub fn at_capacity(&self) -> bool {
        self.len() == self.capacity()
    }
}

impl<T> Vec<T> {
    pub fn new() -> Vec<T> { Vec(RVec::new()) }
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Vec<T> {
        Vec(RVec::from_raw_parts(ptr, length, capacity))
    }
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    pub fn into_boxed_slice(self) -> Box<[T]> {
        self.0.into_boxed_slice()
    }
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len)
    }
    pub unsafe fn set_len(&mut self, len: usize) {
        self.0.set_len(len)
    }
    pub fn swap_remove(&mut self, index: usize) -> T {
        self.0.swap_remove(index)
    }
    pub fn remove(&mut self, index: usize) -> T {
        self.0.remove(index)
    }
    pub fn retain<F>(&mut self, f: F) where F: FnMut(&T) -> bool {
        self.0.retain(f)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
    pub fn drain<R>(&mut self, range: R) -> Drain<T> where R: RangeArgument<usize> {
        self.0.drain(range)
    }
    pub fn clear(&mut self) {
        self.0.clear()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> core::default::Default for Vec<T> {
    fn default() -> Self { Vec(RVec::default()) }
}

impl<T: Eq> Eq for Vec<T> { }

impl<T: Ord> Ord for Vec<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, k: usize) -> &T {
        self.0.index(k)
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, k: usize) -> &mut T {
        self.0.index_mut(k)
    }
}


impl<T> core::ops::Index<core::ops::Range<usize>> for Vec<T> {
    type Output = [T];
    fn index(&self, k: core::ops::Range<usize>) -> &[T] {
        self.0.index(k)
    }
}


impl<T> core::ops::IndexMut<core::ops::Range<usize>> for Vec<T> {
    fn index_mut(&mut self, k: core::ops::Range<usize>) -> &mut [T] {
        self.0.index_mut(k)
    }
}


impl<T> core::ops::Index<core::ops::RangeTo<usize>> for Vec<T> {
    type Output = [T];
    fn index(&self, k: core::ops::RangeTo<usize>) -> &[T] {
        self.0.index(k)
    }
}

impl<T> core::ops::IndexMut<core::ops::RangeTo<usize>> for Vec<T> {
    fn index_mut(&mut self, k: core::ops::RangeTo<usize>) -> &mut [T] {
        self.0.index_mut(k)
    }
}


impl<T> core::ops::Index<core::ops::RangeFrom<usize>> for Vec<T> {
    type Output = [T];
    fn index(&self, k: core::ops::RangeFrom<usize>) -> &[T] {
        self.0.index(k)
    }
}


impl<T> core::ops::IndexMut<core::ops::RangeFrom<usize>> for Vec<T> {
    fn index_mut(&mut self, k: core::ops::RangeFrom<usize>) -> &mut [T] {
        self.0.index_mut(k)
    }
}

impl<T> core::ops::Index<core::ops::RangeFull> for Vec<T> {
    type Output = [T];
    fn index(&self, k: core::ops::RangeFull) -> &[T] {
        self.0.index(k)
    }
}


impl<T> core::ops::IndexMut<core::ops::RangeFull> for Vec<T> {
    fn index_mut(&mut self, k: core::ops::RangeFull) -> &mut [T] {
        self.0.index_mut(k)
    }
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        self.0.deref()
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.0.deref_mut()
    }
}

impl<T: core::hash::Hash> core::hash::Hash for Vec<T> {
    fn hash<H: core::hash::Hasher>(&self, h: &mut H) {
        self.0.hash(h)
    }
}

impl<T: core::fmt::Debug> core::fmt::Debug for Vec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}


impl<T> core::iter::FromIterator<T> for Vec<T> {
    fn from_iter<I: core::iter::IntoIterator<Item=T>>(iterable: I) -> Vec<T> {
        Vec(core::iter::FromIterator::from_iter(iterable))
    }
}

impl<T> core::iter::IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> {
        self.0.into_iter()
    }
}


impl<'a, T> core::iter::IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;
    fn into_iter(self) -> core::slice::Iter<'a, T> {
        (*self.0).into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;
    fn into_iter(self) -> core::slice::IterMut<'a, T> {
        (&mut *self.0).into_iter()
    }
}


impl<T: PartialOrd> core::cmp::PartialOrd for Vec<T> {
    fn partial_cmp(&self, other: &Vec<T>) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> core::convert::AsRef<Vec<T>> for Vec<T> {
    fn as_ref(&self) -> &Vec<T> {
        self
    }
}

impl<T> core::convert::AsMut<Vec<T>> for Vec<T> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        self
    }
}

impl<T> core::convert::AsRef<[T]> for Vec<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T> core::convert::AsMut<[T]> for Vec<T> {
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.0
    }
}

// Copied from libcollections

macro_rules! __impl_slice_eq1 {
    ($Lhs: ty, $Rhs: ty) => {
        __impl_slice_eq1! { $Lhs, $Rhs, Sized }
    };
    ($Lhs: ty, $Rhs: ty, $Bound: ident) => {
        impl<'a, 'b, A: $Bound, B> PartialEq<$Rhs> for $Lhs where A: PartialEq<B> {
            #[inline]
            fn eq(&self, other: &$Rhs) -> bool { self[..] == other[..] }
            #[inline]
            fn ne(&self, other: &$Rhs) -> bool { self[..] != other[..] }
        }
    }
}

__impl_slice_eq1! { Vec<A>, Vec<B> }
__impl_slice_eq1! { Vec<A>, &'b [B] }
__impl_slice_eq1! { Vec<A>, &'b mut [B] }

macro_rules! array_impls {
    ($($N: expr)+) => {
        $(
            // NOTE: some less important impls are omitted to reduce code bloat
            __impl_slice_eq1! { Vec<A>, [B; $N] }
            __impl_slice_eq1! { Vec<A>, &'b [B; $N] }
            // __impl_slice_eq1! { Vec<A>, &'b mut [B; $N] }
            // __impl_slice_eq1! { Cow<'a, [A]>, [B; $N], Clone }
            // __impl_slice_eq1! { Cow<'a, [A]>, &'b [B; $N], Clone }
            // __impl_slice_eq1! { Cow<'a, [A]>, &'b mut [B; $N], Clone }
        )+
    }
}

array_impls! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}
