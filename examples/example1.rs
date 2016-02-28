#![no_std]
extern crate burst;
#[macro_use]
extern crate log;

use burst::collections::vec::Vec;
use burst::collections::hash::map::HashMap;

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

    let mut map: HashMap<u8, u8> = HashMap::new();
    map.insert(&cap, 1, 2);
    map.insert(&cap, 2, 1);
    map.insert(&cap, 5, 2);

    // Drop caps so we can do no more allocations.
    //
    // This will poison the heap, page in and lock memory.
    drop(cap);

    // Run realtime application logic.
    info!("hello, world");
}
