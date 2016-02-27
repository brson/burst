#![feature(alloc)]
#![feature(libc)]
#![feature(fnbox)]
#![feature(zero_one)]
#![allow(dead_code)]
#![no_std]

extern crate alloc as ralloc;
extern crate libc;

pub mod thread;
pub mod io;

#[link(name = "pthread")]
extern { }

mod fd;

mod utils {
    use core::num::One;
    use core::cmp::PartialEq;
    use core::ops::Neg;
    use io;

    pub fn cvt<T: One + PartialEq + Neg<Output=T>>(t: T) -> io::Result<T> {
        let one: T = T::one();
        if t == -one {
            Err(io::Error::last_os_error())
        } else {
            Ok(t)
        }
    }
}
