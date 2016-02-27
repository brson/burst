#![feature(libc)]
#![feature(lang_items)]
#![feature(unwind_attributes)]
#![feature(core_intrinsics)]
#![feature(alloc)]
#![no_std]

extern crate burst_alloc;
extern crate alloc as ralloc;
extern crate libc;
#[macro_use]
extern crate log;

pub fn begin_setup() {
}

pub fn end_setup() {
    // Lock memory at the end of setup so that failures
    // will be logged.
    alloc::lock_memory();
    burst_alloc::poison_heap()
}

mod alloc {
    use libc;

    pub fn lock_memory() {
        unsafe {
            let res = libc::mlockall(libc::MCL_CURRENT | libc::MCL_FUTURE);
            if res != 0 {
                error!("{}", "mlockall failed. process not realtime");
            }
        }
    }
}

mod unwind;
mod libunwind;
