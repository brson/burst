#![no_std]
extern crate burst;

use burst::collections::vec::Vec;

fn main() {
    // Acquire the one-time setup capability
    let cap = burst::open();

    // Do setup work
    let mut v = Vec::new();
    v.push(&cap, 1);

    let mut v2 = Vec::new();
    v2.push(&cap, 1);

    // Spawn a thread
    burst::thread::spawn(&cap, move || {
        let _v2 = v2;
    });

    // Drop caps so we can do no more allocations.
    //
    // This will poison the heap, page in and lock memory.
    drop(cap);

    // Run realtime application logic.

}
