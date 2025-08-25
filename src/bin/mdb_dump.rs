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
    fn freopen(
        __filename: *const std::ffi::c_char,
        __modes: *const std::ffi::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn fprintf(__stream: *mut FILE, __format: *const std::ffi::c_char, ...) -> std::ffi::c_int;
    fn printf(__format: *const std::ffi::c_char, ...) -> std::ffi::c_int;
    fn putc(__c: std::ffi::c_int, __stream: *mut FILE) -> std::ffi::c_int;
    fn malloc(__size: size_t) -> *mut std::ffi::c_void;
    fn free(__ptr: *mut std::ffi::c_void);
    fn exit(__status: std::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut std::ffi::c_void,
        __src: *const std::ffi::c_void,
        __n: size_t,
    ) -> *mut std::ffi::c_void;
    fn memchr(
        __s: *const std::ffi::c_void,
        __c: std::ffi::c_int,
        __n: size_t,
    ) -> *mut std::ffi::c_void;
    fn strerror(__errnum: std::ffi::c_int) -> *mut std::ffi::c_char;
    #[cfg(all(target_endian = "little", target_os = "linux", target_pointer_width = "64"))]
    fn __ctype_b_loc() -> *mut *const std::ffi::c_ushort;
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

#[cfg(all(target_endian = "little", target_os = "linux", target_pointer_width = "64"))]
const unsafe fn get_stdout() -> *mut FILE {
    unsafe extern "C" {
        static mut stdout: *mut FILE;
    }
    unsafe { stdout }
}
#[cfg(all(target_endian = "little", target_os = "macos", target_pointer_width = "64"))]
const unsafe fn get_stdout() -> *mut FILE {
    unsafe extern "C" {
        static mut __stdoutp: *mut FILE;
    }
    unsafe { __stdoutp }
}

#[cfg(all(target_endian = "little", target_os = "linux", target_pointer_width = "64"))]
unsafe fn get_errno_location() -> *mut std::ffi::c_int {
    unsafe extern "C" {
        fn __errno_location() -> *mut std::ffi::c_int;
    }
    unsafe { __errno_location() }
}
#[cfg(all(target_endian = "little", target_os = "macos", target_pointer_width = "64"))]
unsafe fn get_errno_location() -> *mut std::ffi::c_int {
    unsafe extern "C" {
        fn __error() -> *mut std::ffi::c_int;
    }
    unsafe { __error() }
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
pub type C2RustUnnamed = std::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
pub type mdb_size_t = size_t;
pub type MDB_dbi = std::ffi::c_uint;
pub type MDB_cursor_op = std::ffi::c_uint;
pub const MDB_PREV_MULTIPLE: MDB_cursor_op = 18;
pub const MDB_SET_RANGE: MDB_cursor_op = 17;
pub const MDB_SET_KEY: MDB_cursor_op = 16;
pub const MDB_SET: MDB_cursor_op = 15;
pub const MDB_PREV_NODUP: MDB_cursor_op = 14;
pub const MDB_PREV_DUP: MDB_cursor_op = 13;
pub const MDB_PREV: MDB_cursor_op = 12;
pub const MDB_NEXT_NODUP: MDB_cursor_op = 11;
pub const MDB_NEXT_MULTIPLE: MDB_cursor_op = 10;
pub const MDB_NEXT_DUP: MDB_cursor_op = 9;
pub const MDB_NEXT: MDB_cursor_op = 8;
pub const MDB_LAST_DUP: MDB_cursor_op = 7;
pub const MDB_LAST: MDB_cursor_op = 6;
pub const MDB_GET_MULTIPLE: MDB_cursor_op = 5;
pub const MDB_GET_CURRENT: MDB_cursor_op = 4;
pub const MDB_GET_BOTH_RANGE: MDB_cursor_op = 3;
pub const MDB_GET_BOTH: MDB_cursor_op = 2;
pub const MDB_FIRST_DUP: MDB_cursor_op = 1;
pub const MDB_FIRST: MDB_cursor_op = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flagbit {
    pub bit: std::ffi::c_int,
    pub name: *mut std::ffi::c_char,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: std::ffi::c_int) -> std::ffi::c_int { unsafe {
    putc(__c, get_stdout())
}}
pub const EXIT_FAILURE: std::ffi::c_int = 1 as std::ffi::c_int;
pub const EXIT_SUCCESS: std::ffi::c_int = 0 as std::ffi::c_int;
pub const MDB_NOSUBDIR: std::ffi::c_int = 0x4000 as std::ffi::c_int;
pub const MDB_PREVSNAPSHOT: std::ffi::c_int = 0x2000000 as std::ffi::c_int;
pub const MDB_REVERSEKEY: std::ffi::c_int = 0x2 as std::ffi::c_int;
pub const MDB_INTEGERKEY: std::ffi::c_int = 0x8 as std::ffi::c_int;
pub const MDB_DUPFIXED: std::ffi::c_int = 0x10 as std::ffi::c_int;
pub const MDB_INTEGERDUP: std::ffi::c_int = 0x20 as std::ffi::c_int;
pub const MDB_REVERSEDUP: std::ffi::c_int = 0x40 as std::ffi::c_int;
pub const MDB_SUCCESS: std::ffi::c_int = 0 as std::ffi::c_int;
pub const EINTR: std::ffi::c_int = 4 as std::ffi::c_int;
pub const SIGINT: std::ffi::c_int = 2 as std::ffi::c_int;
pub const SIGTERM: std::ffi::c_int = 15 as std::ffi::c_int;
pub const SIGHUP: std::ffi::c_int = 1 as std::ffi::c_int;
pub const SIGPIPE: std::ffi::c_int = 13 as std::ffi::c_int;
pub const PRINT: std::ffi::c_int = 1 as std::ffi::c_int;
static mut mode: std::ffi::c_int = 0;
#[unsafe(no_mangle)]
pub static mut dbflags: [flagbit; 7] = [
    {
        
        flagbit {
            bit: MDB_REVERSEKEY,
            name: b"reversekey\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        }
    },
    {
        
        flagbit {
            bit: 0x4 as std::ffi::c_int,
            name: b"dupsort\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        }
    },
    {
        
        flagbit {
            bit: MDB_INTEGERKEY,
            name: b"integerkey\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        }
    },
    {
        
        flagbit {
            bit: MDB_DUPFIXED,
            name: b"dupfixed\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        }
    },
    {
        
        flagbit {
            bit: MDB_INTEGERDUP,
            name: b"integerdup\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        }
    },
    {
        
        flagbit {
            bit: MDB_REVERSEDUP,
            name: b"reversedup\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        }
    },
    {
        
        flagbit {
            bit: 0 as std::ffi::c_int,
            name: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
        }
    },
];
static mut gotsig: sig_atomic_t = 0;
unsafe extern "C" fn dumpsig(mut sig: std::ffi::c_int) { unsafe {
    ::core::ptr::write_volatile(&raw mut gotsig, 1 as std::ffi::c_int as sig_atomic_t);
}}
static mut hexc: [std::ffi::c_char; 17] =
    unsafe { *::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"0123456789abcdef\0") };
unsafe extern "C" fn hex(mut c: std::ffi::c_uchar) { unsafe {
    putchar(hexc[(c as std::ffi::c_int >> 4 as std::ffi::c_int) as usize] as std::ffi::c_int);
    putchar(hexc[(c as std::ffi::c_int & 0xf as std::ffi::c_int) as usize] as std::ffi::c_int);
}}
unsafe extern "C" fn text(mut v: *mut MDB_val) { unsafe {
    let mut c: *mut std::ffi::c_uchar = std::ptr::null_mut::<std::ffi::c_uchar>();
    let mut end: *mut std::ffi::c_uchar = std::ptr::null_mut::<std::ffi::c_uchar>();
    putchar(' ' as i32);
    c = (*v).mv_data as *mut std::ffi::c_uchar;
    end = c.offset((*v).mv_size as isize);
    while c < end {
        let myc = (*c as std::ffi::c_int as isize) as u8;
        if myc.is_ascii_graphic() || myc == b' ' {
            if *c as std::ffi::c_int == '\\' as i32 {
                putchar('\\' as i32);
            }
            putchar(*c as std::ffi::c_int);
        } else {
            putchar('\\' as i32);
            hex(*c);
        }
        c = c.offset(1);
        c;
    }
    putchar('\n' as i32);
}}
unsafe extern "C" fn byte(mut v: *mut MDB_val) { unsafe {
    let mut c: *mut std::ffi::c_uchar = std::ptr::null_mut::<std::ffi::c_uchar>();
    let mut end: *mut std::ffi::c_uchar = std::ptr::null_mut::<std::ffi::c_uchar>();
    putchar(' ' as i32);
    c = (*v).mv_data as *mut std::ffi::c_uchar;
    end = c.offset((*v).mv_size as isize);
    while c < end {
        let fresh0 = c;
        c = c.offset(1);
        hex(*fresh0);
    }
    putchar('\n' as i32);
}}
unsafe extern "C" fn dumpit(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut name: *mut std::ffi::c_char,
) -> std::ffi::c_int { unsafe {
    let mut mc: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
    let mut ms: MDB_stat = MDB_stat {
        ms_psize: 0,
        ms_depth: 0,
        ms_branch_pages: 0,
        ms_leaf_pages: 0,
        ms_overflow_pages: 0,
        ms_entries: 0,
    };
    let mut key: MDB_val = MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
    let mut data: MDB_val = MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
    let mut info: MDB_envinfo = MDB_envinfo {
        me_mapaddr: std::ptr::null_mut::<std::ffi::c_void>(),
        me_mapsize: 0,
        me_last_pgno: 0,
        me_last_txnid: 0,
        me_maxreaders: 0,
        me_numreaders: 0,
    };
    let mut flags: std::ffi::c_uint = 0;
    let mut rc: std::ffi::c_int = 0;
    let mut i: std::ffi::c_int = 0;
    rc = mdb_dbi_flags(txn, dbi, &mut flags);
    if rc != 0 {
        return rc;
    }
    rc = mdb_stat(txn, dbi, &mut ms);
    if rc != 0 {
        return rc;
    }
    rc = mdb_env_info(mdb_txn_env(txn), &mut info);
    if rc != 0 {
        return rc;
    }
    printf(b"VERSION=3\n\0" as *const u8 as *const std::ffi::c_char);
    printf(
        b"format=%s\n\0" as *const u8 as *const std::ffi::c_char,
        if mode & PRINT != 0 {
            b"print\0" as *const u8 as *const std::ffi::c_char
        } else {
            b"bytevalue\0" as *const u8 as *const std::ffi::c_char
        },
    );
    if !name.is_null() {
        printf(b"database=%s\n\0" as *const u8 as *const std::ffi::c_char, name);
    }
    printf(b"type=btree\n\0" as *const u8 as *const std::ffi::c_char);
    printf(b"mapsize=%zu\n\0" as *const u8 as *const std::ffi::c_char, info.me_mapsize);
    if !(info.me_mapaddr).is_null() {
        printf(b"mapaddr=%p\n\0" as *const u8 as *const std::ffi::c_char, info.me_mapaddr);
    }
    printf(b"maxreaders=%u\n\0" as *const u8 as *const std::ffi::c_char, info.me_maxreaders);
    if flags & 0x4 as std::ffi::c_uint != 0 {
        printf(b"duplicates=1\n\0" as *const u8 as *const std::ffi::c_char);
    }
    i = 0 as std::ffi::c_int;
    while dbflags[i as usize].bit != 0 {
        if flags & dbflags[i as usize].bit as std::ffi::c_uint != 0 {
            printf(b"%s=1\n\0" as *const u8 as *const std::ffi::c_char, dbflags[i as usize].name);
        }
        i += 1;
        i;
    }
    printf(b"db_pagesize=%d\n\0" as *const u8 as *const std::ffi::c_char, ms.ms_psize);
    printf(b"HEADER=END\n\0" as *const u8 as *const std::ffi::c_char);
    rc = mdb_cursor_open(txn, dbi, &mut mc);
    if rc != 0 {
        return rc;
    }
    loop {
        rc = (mdb_cursor_get(mc, &mut key, &mut data, MDB_NEXT) == MDB_SUCCESS) as std::ffi::c_int;
        if rc == 0 {
            break;
        }
        if gotsig != 0 {
            rc = EINTR;
            break;
        } else if mode & PRINT != 0 {
            text(&mut key);
            text(&mut data);
        } else {
            byte(&mut key);
            byte(&mut data);
        }
    }
    printf(b"DATA=END\n\0" as *const u8 as *const std::ffi::c_char);
    if rc == -(30798 as std::ffi::c_int) {
        rc = MDB_SUCCESS;
    }
    rc
}}
unsafe extern "C" fn usage(mut prog: *mut std::ffi::c_char) { unsafe {
    fprintf(
        get_stderr(),
        b"usage: %s [-V] [-f output] [-l] [-n] [-p] [-v] [-a|-s subdb] dbpath\n\0" as *const u8
            as *const std::ffi::c_char,
        prog,
    );
    exit(EXIT_FAILURE);
}}
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int { unsafe {
    let mut current_block: u64;
    let mut i: std::ffi::c_int = 0;
    let mut rc: std::ffi::c_int = 0;
    let mut env: *mut MDB_env = std::ptr::null_mut::<MDB_env>();
    let mut txn: *mut MDB_txn = std::ptr::null_mut::<MDB_txn>();
    let mut dbi: MDB_dbi = 0;
    let mut prog: *mut std::ffi::c_char = *argv.offset(0 as std::ffi::c_int as isize);
    let mut envname: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
    let mut subname: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
    let mut alldbs: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut envflags: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut list: std::ffi::c_int = 0 as std::ffi::c_int;
    if argc < 2 as std::ffi::c_int {
        usage(prog);
    }
    loop {
        i = getopt(
            argc,
            argv as *const *mut std::ffi::c_char,
            b"af:lnps:vV\0" as *const u8 as *const std::ffi::c_char,
        );
        if i == -(1 as std::ffi::c_int) {
            break;
        }
        let mut current_block_19: u64;
        match i {
            86 => {
                printf(
                    b"%s\n\0" as *const u8 as *const std::ffi::c_char,
                    b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8 as *const std::ffi::c_char,
                );
                exit(0 as std::ffi::c_int);
            }
            108 => {
                list = 1 as std::ffi::c_int;
                current_block_19 = 8780738868093770578;
            }
            97 => {
                current_block_19 = 8780738868093770578;
            }
            102 => {
                if (freopen(optarg, b"w\0" as *const u8 as *const std::ffi::c_char, get_stdout()))
                    .is_null()
                {
                    fprintf(
                        get_stderr(),
                        b"%s: %s: reopen: %s\n\0" as *const u8 as *const std::ffi::c_char,
                        prog,
                        optarg,
                        strerror(*get_errno_location()),
                    );
                    exit(EXIT_FAILURE);
                }
                current_block_19 = 15768484401365413375;
            }
            110 => {
                envflags |= MDB_NOSUBDIR;
                current_block_19 = 15768484401365413375;
            }
            118 => {
                envflags |= MDB_PREVSNAPSHOT;
                current_block_19 = 15768484401365413375;
            }
            112 => {
                mode |= PRINT;
                current_block_19 = 15768484401365413375;
            }
            115 => {
                if alldbs != 0 {
                    usage(prog);
                }
                subname = optarg;
                current_block_19 = 15768484401365413375;
            }
            _ => {
                usage(prog);
                current_block_19 = 15768484401365413375;
            }
        }
        if current_block_19 == 8780738868093770578 {
            if !subname.is_null() {
                usage(prog);
            }
            alldbs += 1;
            alldbs;
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
    if alldbs != 0 || !subname.is_null() {
        mdb_env_set_maxdbs(env, 2 as MDB_dbi);
    }
    rc = mdb_env_open(
        env,
        envname,
        (envflags | 0x20000 as std::ffi::c_int) as std::ffi::c_uint,
        0o664 as mdb_mode_t,
    );
    if rc != 0 {
        fprintf(
            get_stderr(),
            b"mdb_env_open failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
            rc,
            mdb_strerror(rc),
        );
    } else {
        rc = mdb_txn_begin(env, std::ptr::null_mut::<MDB_txn>(), 0x20000 as std::ffi::c_uint, &mut txn);
        if rc != 0 {
            fprintf(
                get_stderr(),
                b"mdb_txn_begin failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
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
                if alldbs != 0 {
                    let mut cursor: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                    let mut key: MDB_val =
                        MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
                    let mut count: std::ffi::c_int = 0 as std::ffi::c_int;
                    rc = mdb_cursor_open(txn, dbi, &mut cursor);
                    if rc != 0 {
                        fprintf(
                            get_stderr(),
                            b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            rc,
                            mdb_strerror(rc),
                        );
                        current_block = 3406114488404958958;
                    } else {
                        loop {
                            rc =
                                mdb_cursor_get(cursor, &mut key, std::ptr::null_mut::<MDB_val>(), MDB_NEXT_NODUP);
                            if rc != 0 as std::ffi::c_int {
                                break;
                            }
                            let mut str: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
                            let mut db2: MDB_dbi = 0;
                            if !(memchr(key.mv_data, '\0' as i32, key.mv_size)).is_null() {
                                continue;
                            }
                            count += 1;
                            count;
                            str = malloc((key.mv_size).wrapping_add(1 as size_t))
                                as *mut std::ffi::c_char;
                            memcpy(str as *mut std::ffi::c_void, key.mv_data, key.mv_size);
                            *str.offset(key.mv_size as isize) = '\0' as i32 as std::ffi::c_char;
                            rc = mdb_dbi_open(txn, str, 0 as std::ffi::c_uint, &mut db2);
                            if rc == MDB_SUCCESS {
                                if list != 0 {
                                    printf(b"%s\n\0" as *const u8 as *const std::ffi::c_char, str);
                                    list += 1;
                                    list;
                                } else {
                                    rc = dumpit(txn, db2, str);
                                    if rc != 0 {
                                        break;
                                    }
                                }
                                mdb_dbi_close(env, db2);
                            }
                            free(str as *mut std::ffi::c_void);
                            rc != 0;
                        }
                        mdb_cursor_close(cursor);
                        if count == 0 {
                            fprintf(
                                get_stderr(),
                                b"%s: %s does not contain multiple databases\n\0" as *const u8
                                    as *const std::ffi::c_char,
                                prog,
                                envname,
                            );
                            rc = -(30798 as std::ffi::c_int);
                        } else if rc == -(30798 as std::ffi::c_int) {
                            rc = MDB_SUCCESS;
                        }
                        current_block = 7158658067966855297;
                    }
                } else {
                    rc = dumpit(txn, dbi, subname);
                    current_block = 7158658067966855297;
                }
                match current_block {
                    3406114488404958958 => {}
                    _ => {
                        if rc != 0 && rc != -(30798 as std::ffi::c_int) {
                            fprintf(
                                get_stderr(),
                                b"%s: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                                prog,
                                envname,
                                mdb_strerror(rc),
                            );
                        }
                        mdb_dbi_close(env, dbi);
                    }
                }
            }
            mdb_txn_abort(txn);
        }
    }
    mdb_env_close(env);
    if rc != 0 { EXIT_FAILURE } else { EXIT_SUCCESS }
}}
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
