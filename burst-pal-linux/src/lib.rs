#![feature(libc)]
#![feature(lang_items)]
#![feature(unwind_attributes)]
#![feature(core_intrinsics)]
#![feature(alloc)]
#![feature(fnbox)]
#![no_std]

extern crate burst_alloc;
extern crate alloc as ralloc;
extern crate libc;

pub mod alloc {
    pub use ::burst_alloc::poison;
}
pub mod thread;

mod unwind;
mod libunwind;

#[link(name = "pthread")]
extern { }
