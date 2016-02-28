use St;
use core;
use ralloc::rc::Rc as RRc;

pub use ralloc::rc::Weak;

pub struct Rc<T: ?Sized>(RRc<T>);

impl<T: ?Sized + core::marker::Unsize<U>, U: ?Sized>
    core::ops::CoerceUnsized<Rc<U>> for Rc<T> {}

impl<T> Rc<T> {
    pub fn new(_: &St, value: T) -> Rc<T> {
        Rc(RRc::new(value))
    }

    pub fn try_unwrap(this: Self) -> Result<T, Self> {
        RRc::try_unwrap(this.0).map_err(Rc)
    }
}

impl<T: ?Sized> Rc<T> {
    pub fn downgrade(this: &Self) -> Weak<T> {
        RRc::downgrade(&this.0)
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        RRc::get_mut(&mut this.0)
    }
}


impl<T: Clone> Rc<T> {
    pub fn clone(&self) -> Rc<T> {
        Rc(self.0.clone())
    }
}

impl<T: ?Sized> core::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<T: ?Sized + core::fmt::Display> core::fmt::Display for Rc<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl<T: ?Sized + core::fmt::Debug> core::fmt::Debug for Rc<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T> core::fmt::Pointer for Rc<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Pointer::fmt(&self.0, f)
    }
}

impl<T: ?Sized + PartialEq> PartialEq for Rc<T> {
    fn eq(&self, other: &Rc<T>) -> bool {
        **self == **other
    }
}

impl<T: ?Sized + Eq> Eq for Rc<T> {}

impl<T: ?Sized + PartialOrd> PartialOrd for Rc<T> {
    fn partial_cmp(&self, other: &Rc<T>) -> Option<core::cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: ?Sized + Ord> Ord for Rc<T> {
    fn cmp(&self, other: &Rc<T>) -> core::cmp::Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: ?Sized + core::hash::Hash> core::hash::Hash for Rc<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}
