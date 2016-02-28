// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use St;
use core;
use ralloc::boxed::Box as RBox;

pub struct Box<T: ?Sized>(RBox<T>);

impl<T> Box<T> {
    pub fn new(_: &St, x: T) -> Box<T> {
        Box(RBox::new(x))
    }

    /// Convert to the real Rust Box type
    pub fn into_rbox(self) -> RBox<T> {
        self.0
    }
}

impl<T: ?Sized> Box<T> {
    pub unsafe fn from_raw(raw: *mut T) -> Self {
        Box(RBox::from_raw(raw))
    }

    pub fn into_raw(b: Box<T>) -> *mut T {
        RBox::into_raw(b.0)
    }
}

impl<T: ?Sized + core::marker::Unsize<U>, U: ?Sized>
    core::ops::CoerceUnsized<Box<U>> for Box<T> {}

impl<T: ?Sized + core::hash::Hash> core::hash::Hash for Box<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: ?Sized> core::ops::Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<T: ?Sized> core::ops::DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.0
    }
}

impl<T: core::fmt::Display + ?Sized> core::fmt::Display for Box<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl<T: core::fmt::Debug + ?Sized> core::fmt::Debug for Box<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T> core::fmt::Pointer for Box<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Pointer::fmt(&self.0, f)
    }
}

impl<T: ?Sized + PartialEq> PartialEq for Box<T> {
    fn eq(&self, other: &Box<T>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: ?Sized + PartialOrd> PartialOrd for Box<T> {
    fn partial_cmp(&self, other: &Box<T>) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: ?Sized + Ord> Ord for Box<T> {
    fn cmp(&self, other: &Box<T>) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ?Sized + Eq> Eq for Box<T> {}
