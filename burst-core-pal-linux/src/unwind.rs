#![allow(private_no_mangle_fns, unused_variables)]

use core;
use libunwind as uw;
use libc::c_int;

#[lang = "panic_fmt"]
#[unwind]
extern fn rust_begin_unwind(_msg: core::fmt::Arguments,
                                _file: &'static str, _line: u32) -> ! {
    unsafe { core::intrinsics::abort() }
}

#[lang = "eh_personality"]
#[no_mangle]
extern fn rust_eh_personality(
    version: c_int,
    actions: uw::_Unwind_Action,
    exception_class: uw::_Unwind_Exception_Class,
    ue_header: *mut uw::_Unwind_Exception,
    context: *mut uw::_Unwind_Context
        ) -> uw::_Unwind_Reason_Code
{
    unsafe { core::intrinsics::abort() }
}
