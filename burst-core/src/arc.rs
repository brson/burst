use St;
use core;
use ralloc::arc::Arc as RArc;

pub use ralloc::arc::Weak;

pub struct Arc<T: ?Sized>(RArc<T>);

impl<T: ?Sized + core::marker::Unsize<U>, U: ?Sized>
    core::ops::CoerceUnsized<Arc<U>> for Arc<T> { }


impl<T> Arc<T> {
    pub fn new(_: &St, data: T) -> Arc<T> {
        Arc(RArc::new(data))
    }

    pub fn try_unwrap(this: Self) -> Result<T, Self> {
        RArc::try_unwrap(this.0).map_err(Arc)
    }
}

impl<T: ?Sized> Arc<T> {
    pub fn downgrade(this: &Self) -> Weak<T> {
        RArc::downgrade(&this.0)
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        RArc::get_mut(&mut this.0)
    }
}

impl<T: ?Sized> Clone for Arc<T> {
    fn clone(&self) -> Arc<T> {
        Arc(self.0.clone())
    }
}

impl<T: ?Sized> core::ops::Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

impl<T: ?Sized + PartialEq> PartialEq for Arc<T> {
    fn eq(&self, other: &Arc<T>) -> bool {
        *(*self) == *(*other)
    }
}

impl<T: ?Sized + PartialOrd> PartialOrd for Arc<T> {
    fn partial_cmp(&self, other: &Arc<T>) -> Option<core::cmp::Ordering> {
        (**self).partial_cmp(&**other)
    }
}

impl<T: ?Sized + Ord> Ord for Arc<T> {
    fn cmp(&self, other: &Arc<T>) -> core::cmp::Ordering {
        (**self).cmp(&**other)
    }
}

impl<T: ?Sized + Eq> Eq for Arc<T> {}

impl<T: ?Sized + core::fmt::Display> core::fmt::Display for Arc<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

impl<T: ?Sized + core::fmt::Debug> core::fmt::Debug for Arc<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.0, f)
    }
}

impl<T> core::fmt::Pointer for Arc<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::fmt::Pointer::fmt(&self.0, f)
    }
}
