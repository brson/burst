//! Every collection is the same as in std, except that every function that
//! can allocate or deallocate is parameterized over Heap.
//!
//! * Make panic not allocate
//! * Poison allocator after heap is dropped
//! * Provide lang start
//! * Run mlockall
//! * Touch all stacks

#![feature(lang_items)]
#![no_std]

// Burst realtime data structures
extern crate burst_core as bcore;

// Burst platform abstraction layer
extern crate burst_pal_linux as bpal;

#[macro_use]
extern crate log;

pub use bcore::Cap;
pub use bcore::collections;

pub mod thread;
pub mod io;

pub fn open() -> Cap {
    let cap = bcore::open();

    logger::init(&cap);

    info!("");
    info!(r"* \(^.^)/ ** get ready to burst ** \(^.^)/ *");
    info!("");

    cap
}

mod rt;
mod logger;
