#![feature(alloc, lang_items, core_intrinsics, integer_atomics)]
#![feature(global_allocator, allocator_api, heap_api)]
#![no_std]

extern crate alloc;
extern crate cstr_core;
extern crate cty;
extern crate libc;

use alloc::allocator::{Alloc, Layout, AllocErr};
use core::sync::atomic::{AtomicU32, Ordering, ATOMIC_U32_INIT};
use cstr_core::{CStr, CString};
use cty::{c_char, int32_t, uint32_t};

// constants from kmem.h
#[allow(unused)]
mod kmem_flags {
    pub const KM_SLEEP: ::libc::c_int     = 0x0000;    /* can block for memory; success guaranteed */
    pub const KM_NOSLEEP: ::libc::c_int   = 0x0001;    /* cannot block for memory; may fail */
    pub const KM_PANIC: ::libc::c_int     = 0x0002;    /* if memory cannot be allocated, panic */
    pub const KM_PUSHPAGE: ::libc::c_int  = 0x0004;    /* can block for memory; may use reserve */
    pub const KM_NORMALPRI: ::libc::c_int = 0x0008;    /* with KM_NOSLEEP, lower priority allocation */
    pub const KM_VMFLAGS: ::libc::c_int   = 0x00ff;    /* flags that must match VM_* flags */

    pub const KM_FLAGS: ::libc::c_int     = 0xffff;    /* all settable kmem flags */
}

extern "C" {
    // kmem.h
    pub fn kmem_alloc(size: libc::size_t, kmflags: libc::c_int) -> *mut libc::c_void;
    pub fn kmem_zalloc(size: libc::size_t, kmflags: libc::c_int) -> *mut libc::c_void;
    pub fn kmem_free(buf: *mut libc::c_void, size: libc::size_t);
}

struct SolarisKernelAllocator;

unsafe impl<'a> Alloc for &'a SolarisKernelAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        Ok(kmem_alloc(layout.size() as libc::size_t, kmem_flags::KM_SLEEP) as *mut u8)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        kmem_free(ptr as *mut libc::c_void, layout.size() as libc::size_t)
    }
}

#[global_allocator]
static GLOBAL: SolarisKernelAllocator = SolarisKernelAllocator;

#[allow(non_camel_case_types)]
pub enum modlinkage {}

#[allow(unused)]
mod cmn_err_flags {
    pub const CE_CONT:      ::libc::c_int = 0;    /* continuation */
    pub const CE_NOTE:      ::libc::c_int = 1;    /* notice */
    pub const CE_WARN:      ::libc::c_int = 2;    /* warning */
    pub const CE_PANIC:     ::libc::c_int = 3;    /* panic */
    pub const CE_IGNORE:    ::libc::c_int = 4;    /* print nothing */
}

extern "C" {
    // cmn_err.h
    pub fn cmn_err(level: libc::c_int, message: *const c_char, ...);

    // modctl.h
    pub fn mod_install(module: *mut modlinkage) -> int32_t;
    pub fn mod_remove(module: *mut modlinkage) -> int32_t;
}

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
pub extern fn lorfs_mount_count() -> uint32_t {
    LORFS_MOUNT_COUNT.load(Ordering::Acquire)
}

#[no_mangle]
pub extern fn lorfs_reset_mount_count() {
    LORFS_MOUNT_COUNT.store(0, Ordering::Release);
}

#[no_mangle]
pub extern fn lorfs_mod_remove(module: *mut modlinkage) -> int32_t {
    unsafe { mod_remove(module) }
}

#[no_mangle]
pub extern fn lorfs_print_notice() {
    unsafe {
        //let str = CStr::from_bytes_with_nul_unchecked(b"a notice from rust\0");
        //cmn_err(cmn_err_flags::CE_NOTE, str.as_ptr());
        let msg = CString::new("a message from rust");
        match msg {
            Ok(ref str) => cmn_err(cmn_err_flags::CE_NOTE, str.as_c_str().as_ptr()),
            Err(_) => ()
        }
    }
}

// just a test to see if we can call into Rust; TODO: remove
#[no_mangle]
pub extern fn lorfs_add(a: int32_t, b: int32_t) -> int32_t {
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
