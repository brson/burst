#![feature(lang_items)]
#![feature(collections)]
#![feature(collections_range)]
#![feature(alloc)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(unsafe_no_drop_flag)]
#![feature(dropck_parametricity)]
#![feature(heap_api)]
#![feature(unique)]
#![feature(oom)]
#![feature(filling_drop)]
#![no_std]

// Rust runtime crates
extern crate alloc as ralloc;
extern crate collections as rcollections;

// The burst platform abstraction layer
extern crate burst_core_pal_linux as bcpal;

#[macro_use]
extern crate log;

use core::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

pub fn begin_setup() -> St {
    static OPENED: AtomicBool = ATOMIC_BOOL_INIT;
    let old = OPENED.swap(true, Ordering::SeqCst);
    if !old {
        bcpal::begin_setup();

        St(())
    } else {
        panic!("burst::open called twice")
    }
}

pub fn end_setup(_: St) { }

/// Setup token
pub struct St(());

impl Drop for St {
    fn drop(&mut self) {
        bcpal::end_setup();

        debug!("startup completed");
    }
}

pub mod boxed;

pub mod collections {
    pub mod vec;
    pub mod hash;

    // copied from stdhash
    mod stdhash;

    static CAPACITY: &'static str = "capacity exceeded";
}
