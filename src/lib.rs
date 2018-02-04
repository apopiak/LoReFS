#![feature(lang_items, core_intrinsics)]
#![no_std]
use core::intrinsics;

#[no_mangle]
pub extern fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    unsafe { intrinsics::abort() }
}
