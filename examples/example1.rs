#![no_std]
extern crate burst;

use burst::collections::vec::Vec;

fn main() {
    // Acquire the one-time setup capability
    let cap = burst::open();

    // Do setup work
    let mut v = Vec::new();
    v.push(&cap, 1);

    // Spawn a thread
    burst::thread::spawn(&cap, |cap| {
        let mut v = Vec::new();
        v.push(&cap, 1);
        drop(cap);
    });

    // Drop caps so we can do no more allocations
    drop(cap);

    // Poison the allocator to ensure nobody circumvents our static
    // protection by directly importing the allocator crate.
    burst::poison();

    // Run the application
}
