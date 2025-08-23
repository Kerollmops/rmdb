#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type __sFILEX;
    pub type MDB_env;
    pub type MDB_txn;
    pub type MDB_cursor;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static mut __stdoutp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn freopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char, _: *mut FILE) -> *mut FILE;
    fn putchar(_: std::ffi::c_int) -> std::ffi::c_int;
    fn __error() -> *mut std::ffi::c_int;
    fn signal(
        _: std::ffi::c_int,
        _: Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>,
    ) -> Option<unsafe extern "C" fn(std::ffi::c_int) -> ()>;
    fn exit(_: std::ffi::c_int) -> !;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn memchr(
        _: *const std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn strerror(_: std::ffi::c_int) -> *mut std::ffi::c_char;
    static mut _DefaultRuneLocale: _RuneLocale;
    fn __maskrune(_: __darwin_ct_rune_t, _: std::ffi::c_ulong) -> std::ffi::c_int;
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
        mode_0: mdb_mode_t,
    ) -> std::ffi::c_int;
    fn mdb_env_info(env: *mut MDB_env, stat: *mut MDB_envinfo) -> std::ffi::c_int;
    fn mdb_env_close(env: *mut MDB_env);
    fn mdb_env_set_maxdbs(env: *mut MDB_env, dbs: MDB_dbi) -> std::ffi::c_int;
    fn mdb_txn_begin(
        env: *mut MDB_env,
        parent: *mut MDB_txn,
        flags: std::ffi::c_uint,
        txn: *mut *mut MDB_txn,
    ) -> std::ffi::c_int;
    fn mdb_txn_env(txn: *mut MDB_txn) -> *mut MDB_env;
    fn mdb_txn_abort(txn: *mut MDB_txn);
    fn mdb_dbi_open(
        txn: *mut MDB_txn,
        name: *const std::ffi::c_char,
        flags: std::ffi::c_uint,
        dbi: *mut MDB_dbi,
    ) -> std::ffi::c_int;
    fn mdb_stat(txn: *mut MDB_txn, dbi: MDB_dbi, stat: *mut MDB_stat) -> std::ffi::c_int;
    fn mdb_dbi_flags(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        flags: *mut std::ffi::c_uint,
    ) -> std::ffi::c_int;
    fn mdb_dbi_close(env: *mut MDB_env, dbi: MDB_dbi);
    fn mdb_cursor_open(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        cursor: *mut *mut MDB_cursor,
    ) -> std::ffi::c_int;
    fn mdb_cursor_close(cursor: *mut MDB_cursor);
    fn mdb_cursor_get(
        cursor: *mut MDB_cursor,
        key: *mut MDB_val,
        data: *mut MDB_val,
        op: MDB_cursor_op,
    ) -> std::ffi::c_int;
}
pub type __uint16_t = std::ffi::c_ushort;
pub type __uint32_t = std::ffi::c_uint;
pub type __int64_t = std::ffi::c_longlong;
pub type __darwin_ct_rune_t = std::ffi::c_int;
pub type __darwin_size_t = std::ffi::c_ulong;
pub type __darwin_wchar_t = std::ffi::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneEntry {
    pub __min: __darwin_rune_t,
    pub __max: __darwin_rune_t,
    pub __map: __darwin_rune_t,
    pub __types: *mut __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneRange {
    pub __nranges: std::ffi::c_int,
    pub __ranges: *mut _RuneEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneCharClass {
    pub __name: [std::ffi::c_char; 14],
    pub __mask: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _RuneLocale {
    pub __magic: [std::ffi::c_char; 8],
    pub __encoding: [std::ffi::c_char; 32],
    pub __sgetrune: Option<
        unsafe extern "C" fn(
            *const std::ffi::c_char,
            __darwin_size_t,
            *mut *const std::ffi::c_char,
        ) -> __darwin_rune_t,
    >,
    pub __sputrune: Option<
        unsafe extern "C" fn(
            __darwin_rune_t,
            *mut std::ffi::c_char,
            __darwin_size_t,
            *mut *mut std::ffi::c_char,
        ) -> std::ffi::c_int,
    >,
    pub __invalid_rune: __darwin_rune_t,
    pub __runetype: [__uint32_t; 256],
    pub __maplower: [__darwin_rune_t; 256],
    pub __mapupper: [__darwin_rune_t; 256],
    pub __runetype_ext: _RuneRange,
    pub __maplower_ext: _RuneRange,
    pub __mapupper_ext: _RuneRange,
    pub __variable: *mut std::ffi::c_void,
    pub __variable_len: std::ffi::c_int,
    pub __ncharclasses: std::ffi::c_int,
    pub __charclasses: *mut _RuneCharClass,
}
pub type mdb_mode_t = mode_t;
pub type mdb_size_t = size_t;
pub type MDB_dbi = std::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_val {
    pub mv_size: size_t,
    pub mv_data: *mut std::ffi::c_void,
}
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
pub struct MDB_stat {
    pub ms_psize: std::ffi::c_uint,
    pub ms_depth: std::ffi::c_uint,
    pub ms_branch_pages: mdb_size_t,
    pub ms_leaf_pages: mdb_size_t,
    pub ms_overflow_pages: mdb_size_t,
    pub ms_entries: mdb_size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_envinfo {
    pub me_mapaddr: *mut std::ffi::c_void,
    pub me_mapsize: mdb_size_t,
    pub me_last_pgno: mdb_size_t,
    pub me_last_txnid: mdb_size_t,
    pub me_maxreaders: std::ffi::c_uint,
    pub me_numreaders: std::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flagbit {
    pub bit: std::ffi::c_int,
    pub name: *mut std::ffi::c_char,
}
#[inline]
unsafe extern "C" fn isascii(mut _c: std::ffi::c_int) -> std::ffi::c_int {
    return (_c & !(0x7f as std::ffi::c_int) == 0 as std::ffi::c_int) as std::ffi::c_int;
}
#[inline]
unsafe extern "C" fn __istype(
    mut _c: __darwin_ct_rune_t,
    mut _f: std::ffi::c_ulong,
) -> std::ffi::c_int {
    return if isascii(_c) != 0 {
        (_DefaultRuneLocale.__runetype[_c as usize] as std::ffi::c_ulong & _f != 0)
            as std::ffi::c_int
    } else {
        (__maskrune(_c, _f) != 0) as std::ffi::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isprint(mut _c: std::ffi::c_int) -> std::ffi::c_int {
    return __istype(_c, 0x40000 as std::ffi::c_long as std::ffi::c_ulong);
}
static mut mode: std::ffi::c_int = 0;
#[no_mangle]
pub static mut dbflags: [flagbit; 7] = [
    {
        let mut init = flagbit {
            bit: 0x2 as std::ffi::c_int,
            name: b"reversekey\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x4 as std::ffi::c_int,
            name: b"dupsort\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x8 as std::ffi::c_int,
            name: b"integerkey\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x10 as std::ffi::c_int,
            name: b"dupfixed\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x20 as std::ffi::c_int,
            name: b"integerdup\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0x40 as std::ffi::c_int,
            name: b"reversedup\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
        };
        init
    },
    {
        let mut init = flagbit {
            bit: 0 as std::ffi::c_int,
            name: 0 as *const std::ffi::c_char as *mut std::ffi::c_char,
        };
        init
    },
];
static mut gotsig: sig_atomic_t = 0;
unsafe extern "C" fn dumpsig(mut sig: std::ffi::c_int) {
    ::core::ptr::write_volatile(&mut gotsig as *mut sig_atomic_t, 1 as std::ffi::c_int);
}
static mut hexc: [std::ffi::c_char; 17] =
    unsafe { *::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(b"0123456789abcdef\0") };
unsafe extern "C" fn hex(mut c: std::ffi::c_uchar) {
    putchar(hexc[(c as std::ffi::c_int >> 4 as std::ffi::c_int) as usize] as std::ffi::c_int);
    putchar(hexc[(c as std::ffi::c_int & 0xf as std::ffi::c_int) as usize] as std::ffi::c_int);
}
unsafe extern "C" fn text(mut v: *mut MDB_val) {
    let mut c: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut end: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    putchar(' ' as i32);
    c = (*v).mv_data as *mut std::ffi::c_uchar;
    end = c.offset((*v).mv_size as isize);
    while c < end {
        if isprint(*c as std::ffi::c_int) != 0 {
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
}
unsafe extern "C" fn byte(mut v: *mut MDB_val) {
    let mut c: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut end: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    putchar(' ' as i32);
    c = (*v).mv_data as *mut std::ffi::c_uchar;
    end = c.offset((*v).mv_size as isize);
    while c < end {
        let fresh0 = c;
        c = c.offset(1);
        hex(*fresh0);
    }
    putchar('\n' as i32);
}
unsafe extern "C" fn dumpit(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut name: *mut std::ffi::c_char,
) -> std::ffi::c_int {
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut ms: MDB_stat = MDB_stat {
        ms_psize: 0,
        ms_depth: 0,
        ms_branch_pages: 0,
        ms_leaf_pages: 0,
        ms_overflow_pages: 0,
        ms_entries: 0,
    };
    let mut key: MDB_val = MDB_val { mv_size: 0, mv_data: 0 as *mut std::ffi::c_void };
    let mut data: MDB_val = MDB_val { mv_size: 0, mv_data: 0 as *mut std::ffi::c_void };
    let mut info: MDB_envinfo = MDB_envinfo {
        me_mapaddr: 0 as *mut std::ffi::c_void,
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
        if mode & 1 as std::ffi::c_int != 0 {
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
    if flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
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
        rc = (mdb_cursor_get(mc, &mut key, &mut data, MDB_NEXT) == 0 as std::ffi::c_int)
            as std::ffi::c_int;
        if !(rc != 0) {
            break;
        }
        if gotsig != 0 {
            rc = 4 as std::ffi::c_int;
            break;
        } else if mode & 1 as std::ffi::c_int != 0 {
            text(&mut key);
            text(&mut data);
        } else {
            byte(&mut key);
            byte(&mut data);
        }
    }
    printf(b"DATA=END\n\0" as *const u8 as *const std::ffi::c_char);
    if rc == -(30798 as std::ffi::c_int) {
        rc = 0 as std::ffi::c_int;
    }
    return rc;
}
unsafe extern "C" fn usage(mut prog: *mut std::ffi::c_char) {
    fprintf(
        __stderrp,
        b"usage: %s [-V] [-f output] [-l] [-n] [-p] [-v] [-a|-s subdb] dbpath\n\0" as *const u8
            as *const std::ffi::c_char,
        prog,
    );
    exit(1 as std::ffi::c_int);
}
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    let mut current_block: u64;
    let mut i: std::ffi::c_int = 0;
    let mut rc: std::ffi::c_int = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut dbi: MDB_dbi = 0;
    let mut prog: *mut std::ffi::c_char = *argv.offset(0 as std::ffi::c_int as isize);
    let mut envname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut subname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
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
        if !(i != -(1 as std::ffi::c_int)) {
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
                current_block_19 = 15433940732114890699;
            }
            97 => {
                current_block_19 = 15433940732114890699;
            }
            102 => {
                if (freopen(optarg, b"w\0" as *const u8 as *const std::ffi::c_char, __stdoutp))
                    .is_null()
                {
                    fprintf(
                        __stderrp,
                        b"%s: %s: reopen: %s\n\0" as *const u8 as *const std::ffi::c_char,
                        prog,
                        optarg,
                        strerror(*__error()),
                    );
                    exit(1 as std::ffi::c_int);
                }
                current_block_19 = 15768484401365413375;
            }
            110 => {
                envflags |= 0x4000 as std::ffi::c_int;
                current_block_19 = 15768484401365413375;
            }
            118 => {
                envflags |= 0x2000000 as std::ffi::c_int;
                current_block_19 = 15768484401365413375;
            }
            112 => {
                mode |= 1 as std::ffi::c_int;
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
        match current_block_19 {
            15433940732114890699 => {
                if !subname.is_null() {
                    usage(prog);
                }
                alldbs += 1;
                alldbs;
            }
            _ => {}
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
    if alldbs != 0 || !subname.is_null() {
        mdb_env_set_maxdbs(env, 2 as std::ffi::c_int as MDB_dbi);
    }
    rc = mdb_env_open(
        env,
        envname,
        (envflags | 0x20000 as std::ffi::c_int) as std::ffi::c_uint,
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
            0x20000 as std::ffi::c_int as std::ffi::c_uint,
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
                if alldbs != 0 {
                    let mut cursor: *mut MDB_cursor = 0 as *mut MDB_cursor;
                    let mut key: MDB_val =
                        MDB_val { mv_size: 0, mv_data: 0 as *mut std::ffi::c_void };
                    let mut count: std::ffi::c_int = 0 as std::ffi::c_int;
                    rc = mdb_cursor_open(txn, dbi, &mut cursor);
                    if rc != 0 {
                        fprintf(
                            __stderrp,
                            b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            rc,
                            mdb_strerror(rc),
                        );
                        current_block = 13407604293425256640;
                    } else {
                        loop {
                            rc =
                                mdb_cursor_get(cursor, &mut key, 0 as *mut MDB_val, MDB_NEXT_NODUP);
                            if !(rc == 0 as std::ffi::c_int) {
                                break;
                            }
                            let mut str: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
                            let mut db2: MDB_dbi = 0;
                            if !(memchr(key.mv_data, '\0' as i32, key.mv_size)).is_null() {
                                continue;
                            }
                            count += 1;
                            count;
                            str = malloc((key.mv_size).wrapping_add(1 as std::ffi::c_int as size_t))
                                as *mut std::ffi::c_char;
                            memcpy(str as *mut std::ffi::c_void, key.mv_data, key.mv_size);
                            *str.offset(key.mv_size as isize) = '\0' as i32 as std::ffi::c_char;
                            rc = mdb_dbi_open(
                                txn,
                                str,
                                0 as std::ffi::c_int as std::ffi::c_uint,
                                &mut db2,
                            );
                            if rc == 0 as std::ffi::c_int {
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
                                __stderrp,
                                b"%s: %s does not contain multiple databases\n\0" as *const u8
                                    as *const std::ffi::c_char,
                                prog,
                                envname,
                            );
                            rc = -(30798 as std::ffi::c_int);
                        } else if rc == -(30798 as std::ffi::c_int) {
                            rc = 0 as std::ffi::c_int;
                        }
                        current_block = 7158658067966855297;
                    }
                } else {
                    rc = dumpit(txn, dbi, subname);
                    current_block = 7158658067966855297;
                }
                match current_block {
                    13407604293425256640 => {}
                    _ => {
                        if rc != 0 && rc != -(30798 as std::ffi::c_int) {
                            fprintf(
                                __stderrp,
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
