#![feature(allocator)]
#![feature(libc)]
#![no_std]
#![allocator]

extern crate libc;

use core::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};

static HEAP_POISONED: AtomicBool = ATOMIC_BOOL_INIT;

pub fn poison_heap() {
    HEAP_POISONED.store(true, Ordering::SeqCst);
}

// Copied from liballoc_system

// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values. In practice, the alignment is a
// constant at the call site and the branch will be optimized out.
#[cfg(all(any(target_arch = "x86",
              target_arch = "arm",
              target_arch = "mips",
              target_arch = "powerpc",
              target_arch = "powerpc64")))]
const MIN_ALIGN: usize = 8;
#[cfg(all(any(target_arch = "x86_64",
              target_arch = "aarch64")))]
const MIN_ALIGN: usize = 16;

#[no_mangle]
pub extern "C" fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    if HEAP_POISONED.load(Ordering::SeqCst) {
        core::ptr::null_mut()
    } else {
        unsafe { imp::allocate(size, align) }
    }
}

#[no_mangle]
pub extern "C" fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe { imp::deallocate(ptr, old_size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate(ptr: *mut u8,
                                    old_size: usize,
                                    size: usize,
                                    align: usize)
                                    -> *mut u8 {
    if HEAP_POISONED.load(Ordering::SeqCst) {
        core::ptr::null_mut()
    } else {
        unsafe { imp::reallocate(ptr, old_size, size, align) }
    }
}

#[no_mangle]
pub extern "C" fn __rust_reallocate_inplace(ptr: *mut u8,
                                            old_size: usize,
                                            size: usize,
                                            align: usize)
                                            -> usize {
    unsafe { imp::reallocate_inplace(ptr, old_size, size, align) }
}

#[no_mangle]
pub extern "C" fn __rust_usable_size(size: usize, align: usize) -> usize {
    imp::usable_size(size, align)
}

#[cfg(unix)]
mod imp {
    use core::cmp;
    use core::ptr;
    use libc;
    use super::MIN_ALIGN;

    pub unsafe fn allocate(size: usize, align: usize) -> *mut u8 {
        if align <= MIN_ALIGN {
            libc::malloc(size as libc::size_t) as *mut u8
        } else {
            let mut out = ptr::null_mut();
            let ret = libc::posix_memalign(&mut out, align as libc::size_t, size as libc::size_t);
            if ret != 0 {
                ptr::null_mut()
            } else {
                out as *mut u8
            }
        }
    }

    pub unsafe fn reallocate(ptr: *mut u8, old_size: usize, size: usize, align: usize) -> *mut u8 {
        if align <= MIN_ALIGN {
            libc::realloc(ptr as *mut libc::c_void, size as libc::size_t) as *mut u8
        } else {
            let new_ptr = allocate(size, align);
            ptr::copy(ptr, new_ptr, cmp::min(size, old_size));
            deallocate(ptr, old_size, align);
            new_ptr
        }
    }

    pub unsafe fn reallocate_inplace(_ptr: *mut u8,
                                     old_size: usize,
                                     _size: usize,
                                     _align: usize)
                                     -> usize {
        old_size
    }

    pub unsafe fn deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
        libc::free(ptr as *mut libc::c_void)
    }

    pub fn usable_size(size: usize, _align: usize) -> usize {
        size
    }
}

