#![feature(lang_items, core_intrinsics, integer_atomics)]
#![no_std]
use core::sync::atomic::{AtomicU32, Ordering, ATOMIC_U32_INIT};

static LORFS_MOUNT_COUNT: AtomicU32 = ATOMIC_U32_INIT;

#[no_mangle]
pub extern fn lorfs_inc_mount_count() {
    LORFS_MOUNT_COUNT.fetch_add(1, Ordering::AcqRel);
}

#[no_mangle]
pub extern fn lorfs_dec_mount_count() {
    LORFS_MOUNT_COUNT.fetch_sub(1, Ordering::AcqRel);
}

#[no_mangle]
pub extern fn lorfs_mount_count() -> u32 {
    LORFS_MOUNT_COUNT.load(Ordering::Acquire)
}

#[no_mangle]
pub extern fn lorfs_reset_mount_count() {
    LORFS_MOUNT_COUNT.store(0, Ordering::Release);
}

// just a test to see if we can call into Rust; TODO: remove
#[no_mangle]
pub extern fn lorfs_add(a: i32, b: i32) -> i32 {
    a + b
}

// ----- language items that need to be defined -----
use core::intrinsics;

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
