// we need to specify the 'eh_personality' and 'panic_fmt' language items
#![feature(lang_items, core_intrinsics)]
// we want to specify a custom Solaris kernel allocator
#![feature(alloc, global_allocator, allocator_api, heap_api)]
#![feature(compiler_builtins_lib)]
#![feature(integer_atomics)]
// we don't have access to a standard library in the kernel
#![no_std]

extern crate alloc;
extern crate compiler_builtins;
extern crate cstr_core;
extern crate cty;
extern crate libc;

use alloc::allocator::{Alloc, AllocErr, Layout};
use core::sync::atomic::{ATOMIC_U32_INIT, AtomicU32, Ordering};
use cstr_core::{CStr, CString};
use libc::{c_char, c_int, c_long, c_longlong, c_void, int32_t, uint32_t};

type uint_t = uint32_t;
type ulong_t = c_long;

// constants from kmem.h
#[allow(unused)]
mod kmem_flags {
    use libc::c_int;
    pub const KM_SLEEP: c_int = 0x0000; /* can block for memory; success guaranteed */
    pub const KM_NOSLEEP: c_int = 0x0001; /* cannot block for memory; may fail */
    pub const KM_PANIC: c_int = 0x0002; /* if memory cannot be allocated, panic */
    pub const KM_PUSHPAGE: c_int = 0x0004; /* can block for memory; may use reserve */
    pub const KM_NORMALPRI: c_int = 0x0008; /* with KM_NOSLEEP, lower priority allocation */
    pub const KM_VMFLAGS: c_int = 0x00ff; /* flags that must match VM_* flags */

    pub const KM_FLAGS: c_int = 0xffff; /* all settable kmem flags */
}

// <allocator>
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
// </allocator>

#[allow(non_camel_case_types)]
pub enum modlinkage {}

#[allow(unused)]
mod cmn_err_flags {
    use libc::c_int;
    pub const CE_CONT: c_int = 0; /* continuation */
    pub const CE_NOTE: c_int = 1; /* notice */
    pub const CE_WARN: c_int = 2; /* warning */
    pub const CE_PANIC: c_int = 3; /* panic */
    pub const CE_IGNORE: c_int = 4; /* print nothing */
}

extern "C" {
    // cmn_err.h
    pub fn cmn_err(level: libc::c_int, message: *const c_char, ...);

    // modctl.h
    pub fn mod_install(module: *mut modlinkage) -> int32_t;
    pub fn mod_remove(module: *mut modlinkage) -> int32_t;

    // os/printf.h
    pub fn printf(fmt: *const c_char, ...);
}

// mount count
static LOREFS_MOUNT_COUNT: AtomicU32 = ATOMIC_U32_INIT;

#[no_mangle]
pub extern "C" fn lorefs_inc_mount_count() {
    LOREFS_MOUNT_COUNT.fetch_add(1, Ordering::AcqRel);
}

#[no_mangle]
pub extern "C" fn lorefs_dec_mount_count() {
    LOREFS_MOUNT_COUNT.fetch_sub(1, Ordering::AcqRel);
}

#[no_mangle]
pub extern "C" fn lorefs_mount_count() -> uint32_t {
    LOREFS_MOUNT_COUNT.load(Ordering::Acquire)
}

#[no_mangle]
pub extern "C" fn lorefs_reset_mount_count() {
    LOREFS_MOUNT_COUNT.store(0, Ordering::Release);
}

// vnops
/*
 * vnode types.  VNON means no type.  These values are unrelated to
 * values in on-disk inodes.
 */
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum vtype {
    VNON = 0,
    VREG = 1,
    VDIR = 2,
    VBLK = 3,
    VCHR = 4,
    VLNK = 5,
    VFIFO = 6,
    VDOOR = 7,
    VPROC = 8,
    VSOCK = 9,
    VPORT = 10,
    VBAD = 11,
}

#[allow(non_camel_case_types)]
pub enum vfs {}

#[allow(non_camel_case_types)]
pub enum stdata {}

#[repr(C)]
pub struct kmutex_t {
    _opaque: *mut c_void,
}

type dev_t = ulong_t;

#[repr(C)]
pub struct vnode {
    pub v_lock: kmutex_t,      /* protects vnode fields */
    pub v_flag: uint_t,        /* vnode flags (see below) */
    pub v_count: uint_t,       /* reference count */
    pub v_data: *mut c_void,   /* private data for fs */
    pub v_vfsp: *mut vfs,      /* ptr to containing VFS */
    pub v_stream: *mut stdata, /* associated stream */
    pub v_type: vtype,         /* vnode type */
    pub v_rdev: dev_t,         /* device (VCHR, VBLK) */

     /* PRIVATE FIELDS BELOW
        LEFT OUT BECAUSE THEY SHOULD NOT BE USED */
}

#[repr(C)]
struct lnode {
    lo_next: *mut lnode,  /* link for hash chain */
    lo_vp: *mut vnode,    /* pointer to real vnode */
    lo_looping: uint32_t, /* looping flags */
    lo_vnode: *mut vnode, /* place holder vnode for file */
}

// return the vnode pointer of the underlying fs
#[inline]
unsafe fn realvp(vp: *mut vnode) -> *mut vnode {
    (*((*vp).v_data as *mut lnode)).lo_vp
}

#[allow(non_camel_case_types)]
pub enum caller_context_t {}

#[allow(non_camel_case_types)]
pub enum cred {}

extern "C" {
    // vnode.h
    pub fn fop_close(
        vp: *mut vnode,
        f: c_int,
        c: c_int,
        o: offset_t,
        cr: *mut cred,
        ct: *mut caller_context_t,
    ) -> c_int;
}

#[inline]
unsafe fn vop_close(
    vp: *mut vnode,
    flag: c_int,
    count: c_int,
    offset: offset_t,
    cr: *mut cred,
    ct: *mut caller_context_t,
) -> c_int {
    fop_close(vp, flag, count, offset, cr, ct)
}

#[allow(unused)]
mod looping {
    use libc::c_int;
    pub const LO_LOOPING: c_int = 0x01; /* Looping detected */
    pub const LO_AUTOLOOP: c_int = 0x02; /* Autonode looping detected */
}

type offset_t = c_longlong;

#[no_mangle]
pub extern "C" fn lo_close(
    vp: *mut vnode,
    flag: c_int,
    count: c_int,
    offset: offset_t,
    cr: *mut cred,
    ct: *mut caller_context_t,
) -> c_int {
    unsafe {
        printf(
            CStr::from_bytes_with_nul_unchecked(b"lo_close vp %p realvp %p\n\0").as_ptr(),
            vp,
            realvp(vp),
        );

        let rvp = realvp(vp);
        return vop_close(rvp, flag, count, offset, cr, ct);
    }
}

// module
#[no_mangle]
pub extern "C" fn lorefs_mod_remove(module: *mut modlinkage) -> int32_t {
    unsafe { mod_remove(module) }
}

// just a test to see if we can call into Rust with allocations; TODO: remove
#[no_mangle]
pub extern "C" fn lorefs_print_notice() {
    unsafe {
        let msg = CString::new("a message from rust");
        match msg {
            Ok(ref str) => cmn_err(cmn_err_flags::CE_NOTE, str.as_c_str().as_ptr()),
            Err(_) => (),
        }
    }
}

// just a test to see if we can call into Rust; TODO: remove
#[no_mangle]
pub extern "C" fn lorefs_add(a: int32_t, b: int32_t) -> int32_t {
    a + b
}

// use compiler intrinsics so they get compiled in -.-
#[allow(unused)]
fn dummy() {
    #[allow(unused)]
    let a = compiler_builtins::int::udiv::__udivti3(1, 2);
    #[allow(unused)]
    let b = compiler_builtins::int::udiv::__umodti3(3, 4);
}

// ----- language items that need to be defined -----
use core::intrinsics;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
) -> ! {
    unsafe { intrinsics::abort() }
}
