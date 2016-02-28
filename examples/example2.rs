#![no_std]
extern crate burst;
#[macro_use]
extern crate log;

use burst::boxed::Box;
use burst::rc::Rc;
use burst::arc::Arc;

fn main() {
    // Acquire the singleton setup token
    let st = burst::begin_setup();

    let b = Box::new(&st, "hi");

    info!("{}", b);

    let r = Rc::new(&st, "what");
    let r2 = r.clone();

    info!("{} {}", r, r2);

    let r = Arc::new(&st, "the");
    let r2 = r.clone();

    info!("{} {}", r, r2);

    burst::end_setup(st);

    info!("wooo!");
}
