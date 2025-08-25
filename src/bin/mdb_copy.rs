#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
unsafe extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(__stream: *mut FILE, __format: *const std::ffi::c_char, ...) -> std::ffi::c_int;
    fn printf(__format: *const std::ffi::c_char, ...) -> std::ffi::c_int;
    fn exit(__status: std::ffi::c_int) -> !;
    fn signal(__sig: std::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
}

#[cfg(all(target_endian = "little", target_os = "linux", target_pointer_width = "64"))]
const unsafe fn get_stderr() -> *mut FILE {
    unsafe extern "C" {
        static mut stderr: *mut FILE;
    }
    unsafe { stderr }
}
#[cfg(all(target_endian = "little", target_os = "macos", target_pointer_width = "64"))]
const unsafe fn get_stderr() -> *mut FILE {
    unsafe extern "C" {
        static mut __stderrp: *mut FILE;
    }
    unsafe { __stderrp }
}

use rmdb::*;

use crate::mdb_mode_t;
pub type __mode_t = std::ffi::c_uint;
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: std::ffi::c_int,
    pub _IO_read_ptr: *mut std::ffi::c_char,
    pub _IO_read_end: *mut std::ffi::c_char,
    pub _IO_read_base: *mut std::ffi::c_char,
    pub _IO_write_base: *mut std::ffi::c_char,
    pub _IO_write_ptr: *mut std::ffi::c_char,
    pub _IO_write_end: *mut std::ffi::c_char,
    pub _IO_buf_base: *mut std::ffi::c_char,
    pub _IO_buf_end: *mut std::ffi::c_char,
    pub _IO_save_base: *mut std::ffi::c_char,
    pub _IO_backup_base: *mut std::ffi::c_char,
    pub _IO_save_end: *mut std::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: std::ffi::c_int,
    pub _flags2: std::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: std::ffi::c_ushort,
    pub _vtable_offset: std::ffi::c_schar,
    pub _shortbuf: [std::ffi::c_char; 1],
    pub _lock: *mut std::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut std::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: std::ffi::c_int,
    pub _unused2: [std::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type mode_t = __mode_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
pub type mdb_filehandle_t = std::ffi::c_int;
pub const MDB_STDOUT: std::ffi::c_int = 1 as std::ffi::c_int;
pub const EXIT_FAILURE: std::ffi::c_int = 1 as std::ffi::c_int;
pub const EXIT_SUCCESS: std::ffi::c_int = 0 as std::ffi::c_int;
pub const MDB_SUCCESS: std::ffi::c_int = 0 as std::ffi::c_int;
pub const SIGINT: std::ffi::c_int = 2 as std::ffi::c_int;
pub const SIGTERM: std::ffi::c_int = 15 as std::ffi::c_int;
pub const SIGHUP: std::ffi::c_int = 1 as std::ffi::c_int;
pub const SIGPIPE: std::ffi::c_int = 13 as std::ffi::c_int;
unsafe extern "C" fn sighandle(mut sig: std::ffi::c_int) {}
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0;
        let mut env: *mut MDB_env = std::ptr::null_mut::<MDB_env>();
        let mut progname: *const std::ffi::c_char = *argv.offset(0 as std::ffi::c_int as isize);
        let mut act: *const std::ffi::c_char = std::ptr::null::<std::ffi::c_char>();
        let mut flags: std::ffi::c_uint = 0x20000 as std::ffi::c_uint;
        let mut cpflags: std::ffi::c_uint = 0 as std::ffi::c_uint;
        while argc > 1 as std::ffi::c_int
            && *(*argv.offset(1 as std::ffi::c_int as isize)).offset(0 as std::ffi::c_int as isize)
                as std::ffi::c_int
                == '-' as i32
        {
            if *(*argv.offset(1 as std::ffi::c_int as isize)).offset(1 as std::ffi::c_int as isize)
                as std::ffi::c_int
                == 'n' as i32
                && *(*argv.offset(1 as std::ffi::c_int as isize))
                    .offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                    == '\0' as i32
            {
                flags |= 0x4000 as std::ffi::c_uint;
            } else if *(*argv.offset(1 as std::ffi::c_int as isize))
                .offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                == 'v' as i32
                && *(*argv.offset(1 as std::ffi::c_int as isize))
                    .offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                    == '\0' as i32
            {
                flags |= 0x2000000 as std::ffi::c_uint;
            } else if *(*argv.offset(1 as std::ffi::c_int as isize))
                .offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                == 'c' as i32
                && *(*argv.offset(1 as std::ffi::c_int as isize))
                    .offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                    == '\0' as i32
            {
                cpflags |= 0x1 as std::ffi::c_uint;
            } else if *(*argv.offset(1 as std::ffi::c_int as isize))
                .offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
                == 'V' as i32
                && *(*argv.offset(1 as std::ffi::c_int as isize))
                    .offset(2 as std::ffi::c_int as isize) as std::ffi::c_int
                    == '\0' as i32
            {
                printf(
                    b"%s\n\0" as *const u8 as *const std::ffi::c_char,
                    b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8 as *const std::ffi::c_char,
                );
                exit(0 as std::ffi::c_int);
            } else {
                argc = 0 as std::ffi::c_int;
            }
            argc -= 1;
            argc;
            argv = argv.offset(1);
            argv;
        }
        if argc < 2 as std::ffi::c_int || argc > 3 as std::ffi::c_int {
            fprintf(
                get_stderr(),
                b"usage: %s [-V] [-c] [-n] [-v] srcpath [dstpath]\n\0" as *const u8
                    as *const std::ffi::c_char,
                progname,
            );
            exit(EXIT_FAILURE);
        }
        signal(SIGPIPE, Some(sighandle as unsafe extern "C" fn(std::ffi::c_int) -> ()));
        signal(SIGHUP, Some(sighandle as unsafe extern "C" fn(std::ffi::c_int) -> ()));
        signal(SIGINT, Some(sighandle as unsafe extern "C" fn(std::ffi::c_int) -> ()));
        signal(SIGTERM, Some(sighandle as unsafe extern "C" fn(std::ffi::c_int) -> ()));
        act = b"opening environment\0" as *const u8 as *const std::ffi::c_char;
        rc = mdb_env_create(&mut env);
        if rc == MDB_SUCCESS {
            rc = mdb_env_open(
                env,
                *argv.offset(1 as std::ffi::c_int as isize),
                flags,
                0o600 as mdb_mode_t,
            );
        }
        if rc == MDB_SUCCESS {
            act = b"copying\0" as *const u8 as *const std::ffi::c_char;
            if argc == 2 as std::ffi::c_int {
                rc = mdb_env_copyfd2(env, MDB_STDOUT, cpflags);
            } else {
                rc = mdb_env_copy2(env, *argv.offset(2 as std::ffi::c_int as isize), cpflags);
            }
        }
        if rc != 0 {
            fprintf(
                get_stderr(),
                b"%s: %s failed, error %d (%s)\n\0" as *const u8 as *const std::ffi::c_char,
                progname,
                act,
                rc,
                mdb_strerror(rc),
            );
        }
        mdb_env_close(env);
        if rc != 0 { EXIT_FAILURE } else { EXIT_SUCCESS }
    }
}
pub fn main() {
    let mut args: Vec<*mut std::ffi::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as std::ffi::c_int,
            args.as_mut_ptr() as *mut *mut std::ffi::c_char,
        ) as i32)
    }
}
