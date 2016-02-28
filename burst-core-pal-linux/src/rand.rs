// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;

pub fn fill_bytes(v: &mut [u8]) {
    let mut read = 0;
    while read < v.len() {
        let result = getrandom(&mut v[read..]);
        if result == -1 {
            error!("getrandom not supported. generating zeros");
            // FIXME: This is very bad.
            for byte in &mut v[read..] {
                *byte = 0;
            }
            return;
        } else {
            read += result as usize;
        }
    }
}

fn getrandom(buf: &mut [u8]) -> libc::c_long {
    #[cfg(target_arch = "x86_64")]
    const NR_GETRANDOM: libc::c_long = 318;
    #[cfg(target_arch = "x86")]
    const NR_GETRANDOM: libc::c_long = 355;
    #[cfg(target_arch = "arm")]
    const NR_GETRANDOM: libc::c_long = 384;
    #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64",
              target_arch = "powerpc64le"))]
    const NR_GETRANDOM: libc::c_long = 359;
    #[cfg(target_arch = "aarch64")]
    const NR_GETRANDOM: libc::c_long = 278;

    unsafe {
        libc::syscall(NR_GETRANDOM, buf.as_mut_ptr(), buf.len(), 0)
    }
}

