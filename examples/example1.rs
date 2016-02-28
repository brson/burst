#![no_std]
extern crate burst;
#[macro_use]
extern crate log;

use burst::collections::vec::Vec;
use burst::collections::hash::map::HashMap;

fn main() {
    // Acquire the singleton setup token
    let st = burst::begin_setup();

    // Do setup work

    // Create a vector
    let mut v = Vec::new();
    v.push(&st, 1);

    // Build a hash table
    let mut map: HashMap<u8, u8> = HashMap::new();
    map.insert(&st, 1, 2);
    map.insert(&st, 2, 1);
    map.insert(&st, 5, 2);

    // Reserve some space
    v.reserve_exact(&st, 100);
    map.reserve(&st, 100);

    // Spawn a thread
    burst::thread::spawn(&st, move || {
    });

    // Poison the heap, page in and lock memory.
    burst::end_setup(st);

    // Run realtime application logic.
    info!("hello, world");

    for i in 0..100 {
        assert!(!v.at_capacity());
        v.push_noalloc(i);
    }
    assert!(v.at_capacity());

    for i in 0..100 {
        assert!(!map.at_capacity());
        map.insert_noalloc(i, i);
    }
}
