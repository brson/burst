#![no_std]
extern crate burst;
#[macro_use]
extern crate log;

use burst::collections::vec::Vec;
use burst::collections::hash::map::HashMap;

fn main() {
    // Acquire the singleton setup token
    let st = burst::begin_setup();

    // Do non-realtime setup and allocation

    // Build a Vec and HashMap
    let mut v = Vec::new();
    v.push(&st, 1);

    let mut map: HashMap<u8, u8> = HashMap::new();
    map.insert(&st, 1, 2);

    // Reserve some space
    v.reserve_exact(&st, 100);
    map.reserve(&st, 100);

    // Spawn a thread
    burst::thread::spawn(&st, move || {
    });

    // Drop setup token, poison the heap, page in and lock memory.
    burst::end_setup(st);

    // Run realtime application logic.

    // We can still modify our basic collection types, as long as they
    // don't exceed their capacity.
    for i in 0..100 {
        assert!(!v.at_capacity());
        v.push_noalloc(i);
    }
    assert!(v.at_capacity());

    for i in 0..100 {
        assert!(!map.at_capacity());
        map.insert_noalloc(i, i);
    }

    // Built-in logger. Not yet realtime friendly.
    info!("hello, world");
}
