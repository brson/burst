#![no_std]
extern crate burst;
#[macro_use]
extern crate log;

use burst::boxed::Box;
use burst::rc::Rc;

fn main() {
    // Acquire the singleton setup token
    let st = burst::begin_setup();

    let b = Box::new(&st, "hi");

    info!("box: {}", b);

    let r = Rc::new(&st, "what");

    info!("rc: {}", r);

    burst::end_setup(st);

    info!("wooo!");
}
