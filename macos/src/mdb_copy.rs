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
extern "C" {
    pub type __sFILEX;
    pub type MDB_env;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn signal(
        _: std::ffi::c_int,
        _: Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>,
    ) -> Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
    fn exit(_: std::ffi::c_int) -> !;
    fn mdb_strerror(err: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn mdb_env_create(env: *mut *mut MDB_env) -> std::ffi::c_int;
    fn mdb_env_open(
        env: *mut MDB_env,
        path: *const std::ffi::c_char,
        flags: std::ffi::c_uint,
        mode: mdb_mode_t,
    ) -> std::ffi::c_int;
    fn mdb_env_copy2(
        env: *mut MDB_env,
        path: *const std::ffi::c_char,
        flags: std::ffi::c_uint,
    ) -> std::ffi::c_int;
    fn mdb_env_copyfd2(
        env: *mut MDB_env,
        fd: mdb_filehandle_t,
        flags: std::ffi::c_uint,
    ) -> std::ffi::c_int;
    fn mdb_env_close(env: *mut MDB_env);
}
pub type __uint16_t = std::ffi::c_ushort;
pub type __int64_t = std::ffi::c_longlong;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut std::ffi::c_uchar,
    pub _size: std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut std::ffi::c_uchar,
    pub _r: std::ffi::c_int,
    pub _w: std::ffi::c_int,
    pub _flags: std::ffi::c_short,
    pub _file: std::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::ffi::c_int,
    pub _cookie: *mut std::ffi::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> std::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(
            *mut std::ffi::c_void,
            *mut std::ffi::c_char,
            std::ffi::c_int,
        ) -> std::ffi::c_int,
    >,
    pub _seek:
        Option<unsafe extern "C" fn(*mut std::ffi::c_void, fpos_t, std::ffi::c_int) -> fpos_t>,
    pub _write: Option<
        unsafe extern "C" fn(
            *mut std::ffi::c_void,
            *const std::ffi::c_char,
            std::ffi::c_int,
        ) -> std::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: std::ffi::c_int,
    pub _ubuf: [std::ffi::c_uchar; 3],
    pub _nbuf: [std::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type mode_t = __darwin_mode_t;
pub type mdb_mode_t = mode_t;
pub type mdb_filehandle_t = std::ffi::c_int;
unsafe extern "C" fn sighandle(mut sig: std::ffi::c_int) {}
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    let mut rc: std::ffi::c_int = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut progname: *const std::ffi::c_char = *argv.offset(0 as std::ffi::c_int as isize);
    let mut act: *const std::ffi::c_char = 0 as *const std::ffi::c_char;
    let mut flags: std::ffi::c_uint = 0x20000 as std::ffi::c_int as std::ffi::c_uint;
    let mut cpflags: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
    while argc > 1 as std::ffi::c_int
        && *(*argv.offset(1 as std::ffi::c_int as isize)).offset(0 as std::ffi::c_int as isize)
            as std::ffi::c_int
            == '-' as i32
    {
        if *(*argv.offset(1 as std::ffi::c_int as isize)).offset(1 as std::ffi::c_int as isize)
            as std::ffi::c_int
            == 'n' as i32
            && *(*argv.offset(1 as std::ffi::c_int as isize)).offset(2 as std::ffi::c_int as isize)
                as std::ffi::c_int
                == '\0' as i32
        {
            flags |= 0x4000 as std::ffi::c_int as std::ffi::c_uint;
        } else if *(*argv.offset(1 as std::ffi::c_int as isize))
            .offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
            == 'v' as i32
            && *(*argv.offset(1 as std::ffi::c_int as isize)).offset(2 as std::ffi::c_int as isize)
                as std::ffi::c_int
                == '\0' as i32
        {
            flags |= 0x2000000 as std::ffi::c_int as std::ffi::c_uint;
        } else if *(*argv.offset(1 as std::ffi::c_int as isize))
            .offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
            == 'c' as i32
            && *(*argv.offset(1 as std::ffi::c_int as isize)).offset(2 as std::ffi::c_int as isize)
                as std::ffi::c_int
                == '\0' as i32
        {
            cpflags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
        } else if *(*argv.offset(1 as std::ffi::c_int as isize))
            .offset(1 as std::ffi::c_int as isize) as std::ffi::c_int
            == 'V' as i32
            && *(*argv.offset(1 as std::ffi::c_int as isize)).offset(2 as std::ffi::c_int as isize)
                as std::ffi::c_int
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
            __stderrp,
            b"usage: %s [-V] [-c] [-n] [-v] srcpath [dstpath]\n\0" as *const u8
                as *const std::ffi::c_char,
            progname,
        );
        exit(1 as std::ffi::c_int);
    }
    signal(13 as std::ffi::c_int, Some(sighandle as unsafe extern "C" fn(std::ffi::c_int) -> ()));
    signal(1 as std::ffi::c_int, Some(sighandle as unsafe extern "C" fn(std::ffi::c_int) -> ()));
    signal(2 as std::ffi::c_int, Some(sighandle as unsafe extern "C" fn(std::ffi::c_int) -> ()));
    signal(15 as std::ffi::c_int, Some(sighandle as unsafe extern "C" fn(std::ffi::c_int) -> ()));
    act = b"opening environment\0" as *const u8 as *const std::ffi::c_char;
    rc = mdb_env_create(&mut env);
    if rc == 0 as std::ffi::c_int {
        rc = mdb_env_open(
            env,
            *argv.offset(1 as std::ffi::c_int as isize),
            flags,
            0o600 as std::ffi::c_int as mdb_mode_t,
        );
    }
    if rc == 0 as std::ffi::c_int {
        act = b"copying\0" as *const u8 as *const std::ffi::c_char;
        if argc == 2 as std::ffi::c_int {
            rc = mdb_env_copyfd2(env, 1 as std::ffi::c_int, cpflags);
        } else {
            rc = mdb_env_copy2(env, *argv.offset(2 as std::ffi::c_int as isize), cpflags);
        }
    }
    if rc != 0 {
        fprintf(
            __stderrp,
            b"%s: %s failed, error %d (%s)\n\0" as *const u8 as *const std::ffi::c_char,
            progname,
            act,
            rc,
            mdb_strerror(rc),
        );
    }
    mdb_env_close(env);
    return if rc != 0 { 1 as std::ffi::c_int } else { 0 as std::ffi::c_int };
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
