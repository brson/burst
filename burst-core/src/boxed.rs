use Cap;
use core;
use ralloc::boxed::Box as RBox;

pub struct Box<T: ?Sized>(RBox<T>);

impl<T> Box<T> {
    pub fn new(_: &Cap, x: T) -> Box<T> {
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

