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
    pub type MDB_txn;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn signal(
        _: std::ffi::c_int,
        _: Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>,
    ) -> Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
    fn exit(_: std::ffi::c_int) -> !;
    fn getopt(
        _: std::ffi::c_int,
        _: *const *mut std::ffi::c_char,
        _: *const std::ffi::c_char,
    ) -> std::ffi::c_int;
    static mut optarg: *mut std::ffi::c_char;
    static mut optind: std::ffi::c_int;
    fn mdb_strerror(err: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn mdb_env_create(env: *mut *mut MDB_env) -> std::ffi::c_int;
    fn mdb_env_open(
        env: *mut MDB_env,
        path: *const std::ffi::c_char,
        flags: std::ffi::c_uint,
        mode: mdb_mode_t,
    ) -> std::ffi::c_int;
    fn mdb_env_close(env: *mut MDB_env);
    fn mdb_env_set_maxdbs(env: *mut MDB_env, dbs: MDB_dbi) -> std::ffi::c_int;
    fn mdb_txn_begin(
        env: *mut MDB_env,
        parent: *mut MDB_txn,
        flags: std::ffi::c_uint,
        txn: *mut *mut MDB_txn,
    ) -> std::ffi::c_int;
    fn mdb_txn_commit(txn: *mut MDB_txn) -> std::ffi::c_int;
    fn mdb_txn_abort(txn: *mut MDB_txn);
    fn mdb_dbi_open(
        txn: *mut MDB_txn,
        name: *const std::ffi::c_char,
        flags: std::ffi::c_uint,
        dbi: *mut MDB_dbi,
    ) -> std::ffi::c_int;
    fn mdb_drop(txn: *mut MDB_txn, dbi: MDB_dbi, del: std::ffi::c_int) -> std::ffi::c_int;
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
pub type sig_atomic_t = std::ffi::c_int;
pub type mode_t = __darwin_mode_t;
pub type mdb_mode_t = mode_t;
pub type MDB_dbi = std::ffi::c_uint;
static mut gotsig: sig_atomic_t = 0;
unsafe extern "C" fn dumpsig(mut sig: std::ffi::c_int) {
    ::core::ptr::write_volatile(&mut gotsig as *mut sig_atomic_t, 1 as std::ffi::c_int);
}
unsafe extern "C" fn usage(mut prog: *mut std::ffi::c_char) {
    fprintf(
        __stderrp,
        b"usage: %s [-V] [-n] [-d] [-s subdb] dbpath\n\0" as *const u8 as *const std::ffi::c_char,
        prog,
    );
    exit(1 as std::ffi::c_int);
}
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0;
    let mut rc: std::ffi::c_int = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut dbi: MDB_dbi = 0;
    let mut prog: *mut std::ffi::c_char = *argv.offset(0 as std::ffi::c_int as isize);
    let mut envname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut subname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
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
        if !(i != -(1 as std::ffi::c_int)) {
            break;
        }
        match i {
            86 => {
                printf(
                    b"%s\n\0" as *const u8 as *const std::ffi::c_char,
                    b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8 as *const std::ffi::c_char,
                );
                exit(0 as std::ffi::c_int);
            }
            100 => {
                delete = 1 as std::ffi::c_int;
            }
            110 => {
                envflags |= 0x4000 as std::ffi::c_int;
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
    signal(13 as std::ffi::c_int, Some(dumpsig as unsafe extern "C" fn(std::ffi::c_int) -> ()));
    signal(1 as std::ffi::c_int, Some(dumpsig as unsafe extern "C" fn(std::ffi::c_int) -> ()));
    signal(2 as std::ffi::c_int, Some(dumpsig as unsafe extern "C" fn(std::ffi::c_int) -> ()));
    signal(15 as std::ffi::c_int, Some(dumpsig as unsafe extern "C" fn(std::ffi::c_int) -> ()));
    envname = *argv.offset(optind as isize);
    rc = mdb_env_create(&mut env);
    if rc != 0 {
        fprintf(
            __stderrp,
            b"mdb_env_create failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
            rc,
            mdb_strerror(rc),
        );
        return 1 as std::ffi::c_int;
    }
    mdb_env_set_maxdbs(env, 2 as std::ffi::c_int as MDB_dbi);
    rc = mdb_env_open(
        env,
        envname,
        envflags as std::ffi::c_uint,
        0o664 as std::ffi::c_int as mdb_mode_t,
    );
    if rc != 0 {
        fprintf(
            __stderrp,
            b"mdb_env_open failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
            rc,
            mdb_strerror(rc),
        );
    } else {
        rc = mdb_txn_begin(
            env,
            0 as *mut MDB_txn,
            0 as std::ffi::c_int as std::ffi::c_uint,
            &mut txn,
        );
        if rc != 0 {
            fprintf(
                __stderrp,
                b"mdb_txn_begin failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
                rc,
                mdb_strerror(rc),
            );
        } else {
            rc = mdb_dbi_open(txn, subname, 0 as std::ffi::c_int as std::ffi::c_uint, &mut dbi);
            if rc != 0 {
                fprintf(
                    __stderrp,
                    b"mdb_open failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
                    rc,
                    mdb_strerror(rc),
                );
            } else {
                rc = mdb_drop(txn, dbi, delete);
                if rc != 0 {
                    fprintf(
                        __stderrp,
                        b"mdb_drop failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
                        rc,
                        mdb_strerror(rc),
                    );
                } else {
                    rc = mdb_txn_commit(txn);
                    if rc != 0 {
                        fprintf(
                            __stderrp,
                            b"mdb_txn_commit failed, error %d %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            rc,
                            mdb_strerror(rc),
                        );
                    } else {
                        txn = 0 as *mut MDB_txn;
                    }
                }
            }
            if !txn.is_null() {
                mdb_txn_abort(txn);
            }
        }
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
