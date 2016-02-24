//! Every collection is the same as in std, except that every function that
//! can allocate or deallocate is parameterized over Heap.
//!
//! * Replace the allocator with bump allocator
//! * Make panic not allocate
//! * Poison allocator after heap is dropped
//! * Provide collections parameterized over an allocator
//! * Provide lang start

#![feature(lang_items)]
#![feature(collections)]
#![feature(collections_range)]
#![feature(alloc)]
#![no_std]

// Rust runtime crates
extern crate alloc as ralloc;
extern crate collections as rcollections;

// The burst platform abstraction layer
extern crate burst_pal_linux as pal;

use core::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

pub fn open() -> Cap {
    static OPENED: AtomicBool = ATOMIC_BOOL_INIT;
    let old = OPENED.swap(true, Ordering::SeqCst);
    if !old {
        Cap(())
    } else {
        panic!("burst::open called twice")
    }
}

pub fn poison() {
    // Poison the heap so no more allocations can be made.
    pal::alloc::poison();
}

pub struct Cap(());

pub mod collections {
    pub mod vec;
}

pub mod thread;

mod rt;
