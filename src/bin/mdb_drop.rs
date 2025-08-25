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
    static mut optarg: *mut std::ffi::c_char;
    static mut optind: std::ffi::c_int;
    fn getopt(
        ___argc: std::ffi::c_int,
        ___argv: *const *mut std::ffi::c_char,
        __shortopts: *const std::ffi::c_char,
    ) -> std::ffi::c_int;
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
pub type __sig_atomic_t = std::ffi::c_int;
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
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
pub type MDB_dbi = std::ffi::c_uint;
pub const EXIT_FAILURE: std::ffi::c_int = 1 as std::ffi::c_int;
pub const EXIT_SUCCESS: std::ffi::c_int = 0 as std::ffi::c_int;
pub const MDB_NOSUBDIR: std::ffi::c_int = 0x4000 as std::ffi::c_int;
pub const SIGINT: std::ffi::c_int = 2 as std::ffi::c_int;
pub const SIGTERM: std::ffi::c_int = 15 as std::ffi::c_int;
pub const SIGHUP: std::ffi::c_int = 1 as std::ffi::c_int;
pub const SIGPIPE: std::ffi::c_int = 13 as std::ffi::c_int;
static mut gotsig: sig_atomic_t = 0;
unsafe extern "C" fn dumpsig(mut sig: std::ffi::c_int) {
    unsafe {
        ::core::ptr::write_volatile(&raw mut gotsig, 1 as std::ffi::c_int as sig_atomic_t);
    }
}
unsafe extern "C" fn usage(mut prog: *mut std::ffi::c_char) {
    unsafe {
        fprintf(
            get_stderr(),
            b"usage: %s [-V] [-n] [-d] [-s subdb] dbpath\n\0" as *const u8
                as *const std::ffi::c_char,
            prog,
        );
        exit(EXIT_FAILURE);
    }
}
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_int = 0;
        let mut rc: std::ffi::c_int = 0;
        let mut env: *mut MDB_env = std::ptr::null_mut::<MDB_env>();
        let mut txn: *mut MDB_txn = std::ptr::null_mut::<MDB_txn>();
        let mut dbi: MDB_dbi = 0;
        let mut prog: *mut std::ffi::c_char = *argv.offset(0 as std::ffi::c_int as isize);
        let mut envname: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut subname: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut envflags: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut delete: std::ffi::c_int = 0 as std::ffi::c_int;
        if argc < 2 as std::ffi::c_int {
            usage(prog);
        }
        loop {
            i = getopt(
                argc,
                argv as *const *mut std::ffi::c_char,
                b"dns:V\0" as *const u8 as *const std::ffi::c_char,
            );
            if i == -(1 as std::ffi::c_int) {
                break;
            }
            match i {
                86 => {
                    printf(
                        b"%s\n\0" as *const u8 as *const std::ffi::c_char,
                        b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8
                            as *const std::ffi::c_char,
                    );
                    exit(0 as std::ffi::c_int);
                }
                100 => {
                    delete = 1 as std::ffi::c_int;
                }
                110 => {
                    envflags |= MDB_NOSUBDIR;
                }
                115 => {
                    subname = optarg;
                }
                _ => {
                    usage(prog);
                }
            }
        }
        if optind != argc - 1 as std::ffi::c_int {
            usage(prog);
        }
        signal(SIGPIPE, Some(dumpsig as unsafe extern "C" fn(std::ffi::c_int) -> ()));
        signal(SIGHUP, Some(dumpsig as unsafe extern "C" fn(std::ffi::c_int) -> ()));
        signal(SIGINT, Some(dumpsig as unsafe extern "C" fn(std::ffi::c_int) -> ()));
        signal(SIGTERM, Some(dumpsig as unsafe extern "C" fn(std::ffi::c_int) -> ()));
        envname = *argv.offset(optind as isize);
        rc = mdb_env_create(&mut env);
        if rc != 0 {
            fprintf(
                get_stderr(),
                b"mdb_env_create failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
                rc,
                mdb_strerror(rc),
            );
            return EXIT_FAILURE;
        }
        mdb_env_set_maxdbs(env, 2 as MDB_dbi);
        rc = mdb_env_open(env, envname, envflags as std::ffi::c_uint, 0o664 as mdb_mode_t);
        if rc != 0 {
            fprintf(
                get_stderr(),
                b"mdb_env_open failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
                rc,
                mdb_strerror(rc),
            );
        } else {
            rc = mdb_txn_begin(
                env,
                std::ptr::null_mut::<MDB_txn>(),
                0 as std::ffi::c_uint,
                &mut txn,
            );
            if rc != 0 {
                fprintf(
                    get_stderr(),
                    b"mdb_txn_begin failed, error %d %s\n\0" as *const u8
                        as *const std::ffi::c_char,
                    rc,
                    mdb_strerror(rc),
                );
            } else {
                rc = mdb_dbi_open(txn, subname, 0 as std::ffi::c_uint, &mut dbi);
                if rc != 0 {
                    fprintf(
                        get_stderr(),
                        b"mdb_open failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
                        rc,
                        mdb_strerror(rc),
                    );
                } else {
                    rc = mdb_drop(txn, dbi, delete);
                    if rc != 0 {
                        fprintf(
                            get_stderr(),
                            b"mdb_drop failed, error %d %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            rc,
                            mdb_strerror(rc),
                        );
                    } else {
                        rc = mdb_txn_commit(txn);
                        if rc != 0 {
                            fprintf(
                                get_stderr(),
                                b"mdb_txn_commit failed, error %d %s\n\0" as *const u8
                                    as *const std::ffi::c_char,
                                rc,
                                mdb_strerror(rc),
                            );
                        } else {
                            txn = std::ptr::null_mut::<MDB_txn>();
                        }
                    }
                }
                if !txn.is_null() {
                    mdb_txn_abort(txn);
                }
            }
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
