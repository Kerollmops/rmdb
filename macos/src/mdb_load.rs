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
    static mut __stdinp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn fgetc(_: *mut FILE) -> std::ffi::c_int;
    fn fgets(_: *mut std::ffi::c_char, _: std::ffi::c_int, _: *mut FILE) -> *mut std::ffi::c_char;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn freopen(_: *const std::ffi::c_char, _: *const std::ffi::c_char, _: *mut FILE) -> *mut FILE;
    fn sscanf(_: *const std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn atoi(_: *const std::ffi::c_char) -> std::ffi::c_int;
    fn exit(_: std::ffi::c_int) -> !;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn realloc(_: *mut std::ffi::c_void, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn __error() -> *mut std::ffi::c_int;
    fn memchr(
        _: *const std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memcmp(
        _: *const std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn memcpy(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn strerror(_: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strdup(_: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    static mut _DefaultRuneLocale: _RuneLocale;
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
        flags_0: std::ffi::c_uint,
        mode_0: mdb_mode_t,
    ) -> std::ffi::c_int;
    fn mdb_env_sync(env: *mut MDB_env, force: std::ffi::c_int) -> std::ffi::c_int;
    fn mdb_env_close(env: *mut MDB_env);
    fn mdb_env_set_mapsize(env: *mut MDB_env, size: mdb_size_t) -> std::ffi::c_int;
    fn mdb_env_set_maxreaders(env: *mut MDB_env, readers: std::ffi::c_uint) -> std::ffi::c_int;
    fn mdb_env_set_maxdbs(env: *mut MDB_env, dbs: MDB_dbi) -> std::ffi::c_int;
    fn mdb_env_get_maxkeysize(env: *mut MDB_env) -> std::ffi::c_int;
    fn mdb_txn_begin(
        env: *mut MDB_env,
        parent: *mut MDB_txn,
        flags_0: std::ffi::c_uint,
        txn: *mut *mut MDB_txn,
    ) -> std::ffi::c_int;
    fn mdb_txn_commit(txn: *mut MDB_txn) -> std::ffi::c_int;
    fn mdb_txn_abort(txn: *mut MDB_txn);
    fn mdb_dbi_open(
        txn: *mut MDB_txn,
        name: *const std::ffi::c_char,
        flags_0: std::ffi::c_uint,
        dbi: *mut MDB_dbi,
    ) -> std::ffi::c_int;
    fn mdb_dbi_close(env: *mut MDB_env, dbi: MDB_dbi);
    fn mdb_set_compare(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        cmp: Option<MDB_cmp_func>,
    ) -> std::ffi::c_int;
    fn mdb_set_dupsort(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        cmp: Option<MDB_cmp_func>,
    ) -> std::ffi::c_int;
    fn mdb_cursor_open(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        cursor: *mut *mut MDB_cursor,
    ) -> std::ffi::c_int;
    fn mdb_cursor_get(
        cursor: *mut MDB_cursor,
        key: *mut MDB_val,
        data: *mut MDB_val,
        op: MDB_cursor_op,
    ) -> std::ffi::c_int;
    fn mdb_cursor_put(
        cursor: *mut MDB_cursor,
        key: *mut MDB_val,
        data: *mut MDB_val,
        flags_0: std::ffi::c_uint,
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
pub type MDB_cmp_func = unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> std::ffi::c_int;
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
    pub len: std::ffi::c_int,
}
#[inline]
unsafe extern "C" fn __isctype(
    mut _c: __darwin_ct_rune_t,
    mut _f: std::ffi::c_ulong,
) -> __darwin_ct_rune_t {
    return if _c < 0 as std::ffi::c_int || _c >= (1 as std::ffi::c_int) << 8 as std::ffi::c_int {
        0 as std::ffi::c_int
    } else {
        (_DefaultRuneLocale.__runetype[_c as usize] as std::ffi::c_ulong & _f != 0)
            as std::ffi::c_int
    };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn isxdigit(mut _c: std::ffi::c_int) -> std::ffi::c_int {
    return __isctype(_c, 0x10000 as std::ffi::c_long as std::ffi::c_ulong);
}
static mut mode: std::ffi::c_int = 0;
static mut subname: *mut std::ffi::c_char = 0 as *const std::ffi::c_char as *mut std::ffi::c_char;
static mut lineno: mdb_size_t = 0;
static mut version: std::ffi::c_int = 0;
static mut flags: std::ffi::c_int = 0;
static mut prog: *mut std::ffi::c_char = 0 as *const std::ffi::c_char as *mut std::ffi::c_char;
static mut Eof: std::ffi::c_int = 0;
static mut info: MDB_envinfo = MDB_envinfo {
    me_mapaddr: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
    me_mapsize: 0,
    me_last_pgno: 0,
    me_last_txnid: 0,
    me_maxreaders: 0,
    me_numreaders: 0,
};
static mut kbuf: MDB_val =
    MDB_val { mv_size: 0, mv_data: 0 as *const std::ffi::c_void as *mut std::ffi::c_void };
static mut dbuf: MDB_val =
    MDB_val { mv_size: 0, mv_data: 0 as *const std::ffi::c_void as *mut std::ffi::c_void };
static mut k0buf: MDB_val =
    MDB_val { mv_size: 0, mv_data: 0 as *const std::ffi::c_void as *mut std::ffi::c_void };
#[no_mangle]
pub static mut dbflags: [flagbit; 7] =
    [flagbit { bit: 0, name: 0 as *mut std::ffi::c_char, len: 0 }; 7];
unsafe extern "C" fn readhdr() {
    let mut ptr: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    flags = 0 as std::ffi::c_int;
    while !(fgets(dbuf.mv_data as *mut std::ffi::c_char, dbuf.mv_size as std::ffi::c_int, __stdinp))
        .is_null()
    {
        lineno = lineno.wrapping_add(1);
        lineno;
        if strncmp(
            dbuf.mv_data as *const std::ffi::c_char,
            b"VERSION=\0" as *const u8 as *const std::ffi::c_char,
            (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
        ) == 0
        {
            version = atoi(
                (dbuf.mv_data as *mut std::ffi::c_char).offset(
                    (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                        as isize,
                ),
            );
            if version > 3 as std::ffi::c_int {
                fprintf(
                    __stderrp,
                    b"%s: line %zu: unsupported VERSION %d\n\0" as *const u8
                        as *const std::ffi::c_char,
                    prog,
                    lineno,
                    version,
                );
                exit(1 as std::ffi::c_int);
            }
        } else {
            if strncmp(
                dbuf.mv_data as *const std::ffi::c_char,
                b"HEADER=END\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            ) == 0
            {
                break;
            }
            if strncmp(
                dbuf.mv_data as *const std::ffi::c_char,
                b"format=\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            ) == 0
            {
                if strncmp(
                    (dbuf.mv_data as *mut std::ffi::c_char).offset(
                        (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                            as isize,
                    ),
                    b"print\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
                ) == 0
                {
                    mode |= 1 as std::ffi::c_int;
                } else if strncmp(
                    (dbuf.mv_data as *mut std::ffi::c_char).offset(
                        (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                            as isize,
                    ),
                    b"bytevalue\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 10]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
                ) != 0
                {
                    fprintf(
                        __stderrp,
                        b"%s: line %zu: unsupported FORMAT %s\n\0" as *const u8
                            as *const std::ffi::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut std::ffi::c_char).offset(
                            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                                as isize,
                        ),
                    );
                    exit(1 as std::ffi::c_int);
                }
            } else if strncmp(
                dbuf.mv_data as *const std::ffi::c_char,
                b"database=\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 10]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            ) == 0
            {
                ptr = memchr(dbuf.mv_data, '\n' as i32, dbuf.mv_size) as *mut std::ffi::c_char;
                if !ptr.is_null() {
                    *ptr = '\0' as i32 as std::ffi::c_char;
                }
                if !subname.is_null() {
                    free(subname as *mut std::ffi::c_void);
                }
                subname = strdup(
                    (dbuf.mv_data as *mut std::ffi::c_char).offset(
                        (::core::mem::size_of::<[std::ffi::c_char; 10]>() as std::ffi::c_ulong)
                            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                            as isize,
                    ),
                );
            } else if strncmp(
                dbuf.mv_data as *const std::ffi::c_char,
                b"type=\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            ) == 0
            {
                if strncmp(
                    (dbuf.mv_data as *mut std::ffi::c_char).offset(
                        (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                            as isize,
                    ),
                    b"btree\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
                ) != 0
                {
                    fprintf(
                        __stderrp,
                        b"%s: line %zu: unsupported type %s\n\0" as *const u8
                            as *const std::ffi::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut std::ffi::c_char).offset(
                            (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                                as isize,
                        ),
                    );
                    exit(1 as std::ffi::c_int);
                }
            } else if strncmp(
                dbuf.mv_data as *const std::ffi::c_char,
                b"mapaddr=\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            ) == 0
            {
                let mut i: std::ffi::c_int = 0;
                ptr = memchr(dbuf.mv_data, '\n' as i32, dbuf.mv_size) as *mut std::ffi::c_char;
                if !ptr.is_null() {
                    *ptr = '\0' as i32 as std::ffi::c_char;
                }
                i = sscanf(
                    (dbuf.mv_data as *mut std::ffi::c_char).offset(
                        (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                            as isize,
                    ),
                    b"%p\0" as *const u8 as *const std::ffi::c_char,
                    &mut info.me_mapaddr as *mut *mut std::ffi::c_void,
                );
                if i != 1 as std::ffi::c_int {
                    fprintf(
                        __stderrp,
                        b"%s: line %zu: invalid mapaddr %s\n\0" as *const u8
                            as *const std::ffi::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut std::ffi::c_char).offset(
                            (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                                as isize,
                        ),
                    );
                    exit(1 as std::ffi::c_int);
                }
            } else if strncmp(
                dbuf.mv_data as *const std::ffi::c_char,
                b"mapsize=\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            ) == 0
            {
                let mut i_0: std::ffi::c_int = 0;
                ptr = memchr(dbuf.mv_data, '\n' as i32, dbuf.mv_size) as *mut std::ffi::c_char;
                if !ptr.is_null() {
                    *ptr = '\0' as i32 as std::ffi::c_char;
                }
                i_0 = sscanf(
                    (dbuf.mv_data as *mut std::ffi::c_char).offset(
                        (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                            as isize,
                    ),
                    b"%zu\0" as *const u8 as *const std::ffi::c_char,
                    &mut info.me_mapsize as *mut mdb_size_t,
                );
                if i_0 != 1 as std::ffi::c_int {
                    fprintf(
                        __stderrp,
                        b"%s: line %zu: invalid mapsize %s\n\0" as *const u8
                            as *const std::ffi::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut std::ffi::c_char).offset(
                            (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                                as isize,
                        ),
                    );
                    exit(1 as std::ffi::c_int);
                }
            } else if strncmp(
                dbuf.mv_data as *const std::ffi::c_char,
                b"maxreaders=\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 12]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
            ) == 0
            {
                let mut i_1: std::ffi::c_int = 0;
                ptr = memchr(dbuf.mv_data, '\n' as i32, dbuf.mv_size) as *mut std::ffi::c_char;
                if !ptr.is_null() {
                    *ptr = '\0' as i32 as std::ffi::c_char;
                }
                i_1 = sscanf(
                    (dbuf.mv_data as *mut std::ffi::c_char).offset(
                        (::core::mem::size_of::<[std::ffi::c_char; 12]>() as std::ffi::c_ulong)
                            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                            as isize,
                    ),
                    b"%u\0" as *const u8 as *const std::ffi::c_char,
                    &mut info.me_maxreaders as *mut std::ffi::c_uint,
                );
                if i_1 != 1 as std::ffi::c_int {
                    fprintf(
                        __stderrp,
                        b"%s: line %zu: invalid maxreaders %s\n\0" as *const u8
                            as *const std::ffi::c_char,
                        prog,
                        lineno,
                        (dbuf.mv_data as *mut std::ffi::c_char).offset(
                            (::core::mem::size_of::<[std::ffi::c_char; 12]>() as std::ffi::c_ulong)
                                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                                as isize,
                        ),
                    );
                    exit(1 as std::ffi::c_int);
                }
            } else {
                let mut i_2: std::ffi::c_int = 0;
                i_2 = 0 as std::ffi::c_int;
                while dbflags[i_2 as usize].bit != 0 {
                    if strncmp(
                        dbuf.mv_data as *const std::ffi::c_char,
                        dbflags[i_2 as usize].name,
                        dbflags[i_2 as usize].len as std::ffi::c_ulong,
                    ) == 0
                        && *(dbuf.mv_data as *mut std::ffi::c_char)
                            .offset(dbflags[i_2 as usize].len as isize)
                            as std::ffi::c_int
                            == '=' as i32
                    {
                        flags |= dbflags[i_2 as usize].bit;
                        break;
                    } else {
                        i_2 += 1;
                        i_2;
                    }
                }
                if dbflags[i_2 as usize].bit == 0 {
                    ptr = memchr(dbuf.mv_data, '=' as i32, dbuf.mv_size) as *mut std::ffi::c_char;
                    if ptr.is_null() {
                        fprintf(
                            __stderrp,
                            b"%s: line %zu: unexpected format\n\0" as *const u8
                                as *const std::ffi::c_char,
                            prog,
                            lineno,
                        );
                        exit(1 as std::ffi::c_int);
                    } else {
                        *ptr = '\0' as i32 as std::ffi::c_char;
                        fprintf(
                            __stderrp,
                            b"%s: line %zu: unrecognized keyword ignored: %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            prog,
                            lineno,
                            dbuf.mv_data as *mut std::ffi::c_char,
                        );
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn badend() {
    fprintf(
        __stderrp,
        b"%s: line %zu: unexpected end of input\n\0" as *const u8 as *const std::ffi::c_char,
        prog,
        lineno,
    );
}
unsafe extern "C" fn unhex(mut c2: *mut std::ffi::c_uchar) -> std::ffi::c_int {
    let mut x: std::ffi::c_int = 0;
    let mut c: std::ffi::c_int = 0;
    let fresh0 = c2;
    c2 = c2.offset(1);
    x = *fresh0 as std::ffi::c_int & 0x4f as std::ffi::c_int;
    if x & 0x40 as std::ffi::c_int != 0 {
        x -= 55 as std::ffi::c_int;
    }
    c = x << 4 as std::ffi::c_int;
    x = *c2 as std::ffi::c_int & 0x4f as std::ffi::c_int;
    if x & 0x40 as std::ffi::c_int != 0 {
        x -= 55 as std::ffi::c_int;
    }
    c |= x;
    return c;
}
unsafe extern "C" fn readline(mut out: *mut MDB_val, mut buf: *mut MDB_val) -> std::ffi::c_int {
    let mut c1: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut c2: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut end: *mut std::ffi::c_uchar = 0 as *mut std::ffi::c_uchar;
    let mut len: size_t = 0;
    let mut l2: size_t = 0;
    let mut c: std::ffi::c_int = 0;
    if mode & 2 as std::ffi::c_int == 0 {
        c = fgetc(__stdinp);
        if c == -(1 as std::ffi::c_int) {
            Eof = 1 as std::ffi::c_int;
            return -(1 as std::ffi::c_int);
        }
        if c != ' ' as i32 {
            lineno = lineno.wrapping_add(1);
            lineno;
            if !(fgets(
                (*buf).mv_data as *mut std::ffi::c_char,
                (*buf).mv_size as std::ffi::c_int,
                __stdinp,
            ))
            .is_null()
            {
                if c == 'D' as i32
                    && strncmp(
                        (*buf).mv_data as *const std::ffi::c_char,
                        b"ATA=END\0" as *const u8 as *const std::ffi::c_char,
                        (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong),
                    ) == 0
                {
                    return -(1 as std::ffi::c_int);
                }
            }
            Eof = 1 as std::ffi::c_int;
            badend();
            return -(1 as std::ffi::c_int);
        }
    }
    if (fgets((*buf).mv_data as *mut std::ffi::c_char, (*buf).mv_size as std::ffi::c_int, __stdinp))
        .is_null()
    {
        Eof = 1 as std::ffi::c_int;
        return -(1 as std::ffi::c_int);
    }
    lineno = lineno.wrapping_add(1);
    lineno;
    c1 = (*buf).mv_data as *mut std::ffi::c_uchar;
    len = strlen(c1 as *mut std::ffi::c_char);
    l2 = len;
    while *c1.offset(len.wrapping_sub(1 as std::ffi::c_int as size_t) as isize) as std::ffi::c_int
        != '\n' as i32
    {
        (*buf).mv_data = realloc((*buf).mv_data, (*buf).mv_size * 2 as std::ffi::c_int as size_t);
        if ((*buf).mv_data).is_null() {
            Eof = 1 as std::ffi::c_int;
            fprintf(
                __stderrp,
                b"%s: line %zu: out of memory, line too long\n\0" as *const u8
                    as *const std::ffi::c_char,
                prog,
                lineno,
            );
            return -(1 as std::ffi::c_int);
        }
        c1 = (*buf).mv_data as *mut std::ffi::c_uchar;
        c1 = c1.offset(l2 as isize);
        if (fgets(
            c1 as *mut std::ffi::c_char,
            ((*buf).mv_size).wrapping_add(1 as std::ffi::c_int as size_t) as std::ffi::c_int,
            __stdinp,
        ))
        .is_null()
        {
            Eof = 1 as std::ffi::c_int;
            badend();
            return -(1 as std::ffi::c_int);
        }
        (*buf).mv_size = (*buf).mv_size * 2 as std::ffi::c_int as size_t;
        len = strlen(c1 as *mut std::ffi::c_char);
        l2 = l2.wrapping_add(len);
    }
    c2 = (*buf).mv_data as *mut std::ffi::c_uchar;
    c1 = c2;
    len = l2;
    len = len.wrapping_sub(1);
    *c1.offset(len as isize) = '\0' as i32 as std::ffi::c_uchar;
    end = c1.offset(len as isize);
    if mode & 1 as std::ffi::c_int != 0 {
        while c2 < end {
            if *c2 as std::ffi::c_int == '\\' as i32 {
                if *c2.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int == '\\' as i32 {
                    let fresh1 = c1;
                    c1 = c1.offset(1);
                    *fresh1 = *c2;
                } else {
                    if c2.offset(3 as std::ffi::c_int as isize) > end
                        || isxdigit(*c2.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int)
                            == 0
                        || isxdigit(*c2.offset(2 as std::ffi::c_int as isize) as std::ffi::c_int)
                            == 0
                    {
                        Eof = 1 as std::ffi::c_int;
                        badend();
                        return -(1 as std::ffi::c_int);
                    }
                    c2 = c2.offset(1);
                    let fresh2 = c1;
                    c1 = c1.offset(1);
                    *fresh2 = unhex(c2) as std::ffi::c_uchar;
                }
                c2 = c2.offset(2 as std::ffi::c_int as isize);
            } else {
                let fresh3 = c2;
                c2 = c2.offset(1);
                let fresh4 = c1;
                c1 = c1.offset(1);
                *fresh4 = *fresh3;
            }
        }
    } else {
        if len & 1 as std::ffi::c_int as size_t != 0 {
            Eof = 1 as std::ffi::c_int;
            badend();
            return -(1 as std::ffi::c_int);
        }
        while c2 < end {
            if isxdigit(*c2 as std::ffi::c_int) == 0
                || isxdigit(*c2.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int) == 0
            {
                Eof = 1 as std::ffi::c_int;
                badend();
                return -(1 as std::ffi::c_int);
            }
            let fresh5 = c1;
            c1 = c1.offset(1);
            *fresh5 = unhex(c2) as std::ffi::c_uchar;
            c2 = c2.offset(2 as std::ffi::c_int as isize);
        }
    }
    (*out).mv_data = (*buf).mv_data;
    c2 = (*out).mv_data as *mut std::ffi::c_uchar;
    (*out).mv_size = c1.offset_from(c2) as std::ffi::c_long as size_t;
    return 0 as std::ffi::c_int;
}
unsafe extern "C" fn usage() {
    fprintf(
        __stderrp,
        b"usage: %s [-V] [-a] [-f input] [-n] [-s name] [-N] [-T] dbpath\n\0" as *const u8
            as *const std::ffi::c_char,
        prog,
    );
    exit(1 as std::ffi::c_int);
}
unsafe extern "C" fn greater(mut a: *const MDB_val, mut b: *const MDB_val) -> std::ffi::c_int {
    return 1 as std::ffi::c_int;
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
    let mut mc: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut dbi: MDB_dbi = 0;
    let mut envname: *mut std::ffi::c_char = 0 as *mut std::ffi::c_char;
    let mut envflags: std::ffi::c_int = 0x10000 as std::ffi::c_int;
    let mut putflags: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut dohdr: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut append: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut prevk: MDB_val =
        MDB_val { mv_size: 0, mv_data: 0 as *const std::ffi::c_void as *mut std::ffi::c_void };
    prog = *argv.offset(0 as std::ffi::c_int as isize);
    if argc < 2 as std::ffi::c_int {
        usage();
    }
    loop {
        i = getopt(
            argc,
            argv as *const *mut std::ffi::c_char,
            b"af:ns:NQTV\0" as *const u8 as *const std::ffi::c_char,
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
            97 => {
                append = 1 as std::ffi::c_int;
            }
            102 => {
                if (freopen(optarg, b"r\0" as *const u8 as *const std::ffi::c_char, __stdinp))
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
            }
            110 => {
                envflags |= 0x4000 as std::ffi::c_int;
            }
            115 => {
                subname = strdup(optarg);
            }
            78 => {
                putflags = 0x10 as std::ffi::c_int | 0x20 as std::ffi::c_int;
            }
            81 => {
                envflags |= 0x10000 as std::ffi::c_int;
            }
            84 => {
                mode |= 2 as std::ffi::c_int | 1 as std::ffi::c_int;
            }
            _ => {
                usage();
            }
        }
    }
    if optind != argc - 1 as std::ffi::c_int {
        usage();
    }
    dbuf.mv_size = 4096 as std::ffi::c_int as size_t;
    dbuf.mv_data = malloc(dbuf.mv_size);
    if mode & 2 as std::ffi::c_int == 0 {
        readhdr();
    }
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
    if info.me_maxreaders != 0 {
        mdb_env_set_maxreaders(env, info.me_maxreaders);
    }
    if info.me_mapsize != 0 {
        mdb_env_set_mapsize(env, info.me_mapsize);
    }
    if !(info.me_mapaddr).is_null() {
        envflags |= 0x1 as std::ffi::c_int;
    }
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
        kbuf.mv_size =
            (mdb_env_get_maxkeysize(env) * 2 as std::ffi::c_int + 2 as std::ffi::c_int) as size_t;
        kbuf.mv_data = malloc(kbuf.mv_size * 2 as std::ffi::c_int as size_t);
        k0buf.mv_size = kbuf.mv_size;
        k0buf.mv_data = (kbuf.mv_data as *mut std::ffi::c_char).offset(kbuf.mv_size as isize)
            as *mut std::ffi::c_void;
        prevk.mv_data = k0buf.mv_data;
        's_222: loop {
            if !(Eof == 0) {
                current_block = 14945635589884787639;
                break;
            }
            let mut key: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
            };
            let mut data: MDB_val = MDB_val {
                mv_size: 0,
                mv_data: 0 as *const std::ffi::c_void as *mut std::ffi::c_void,
            };
            let mut batch: std::ffi::c_int = 0 as std::ffi::c_int;
            let mut appflag: std::ffi::c_int = 0;
            if dohdr == 0 {
                dohdr = 1 as std::ffi::c_int;
            } else if mode & 2 as std::ffi::c_int == 0 {
                readhdr();
            }
            rc = mdb_txn_begin(
                env,
                0 as *mut MDB_txn,
                0 as std::ffi::c_int as std::ffi::c_uint,
                &mut txn,
            );
            if rc != 0 {
                fprintf(
                    __stderrp,
                    b"mdb_txn_begin failed, error %d %s\n\0" as *const u8
                        as *const std::ffi::c_char,
                    rc,
                    mdb_strerror(rc),
                );
                current_block = 16097525410562447625;
                break;
            } else {
                rc = mdb_dbi_open(
                    txn,
                    subname,
                    (flags | 0x40000 as std::ffi::c_int) as std::ffi::c_uint,
                    &mut dbi,
                );
                if rc != 0 {
                    fprintf(
                        __stderrp,
                        b"mdb_dbi_open failed, error %d %s\n\0" as *const u8
                            as *const std::ffi::c_char,
                        rc,
                        mdb_strerror(rc),
                    );
                    current_block = 14945635589884787639;
                    break;
                } else {
                    prevk.mv_size = 0 as std::ffi::c_int as size_t;
                    if append != 0 {
                        mdb_set_compare(
                            txn,
                            dbi,
                            Some(
                                greater
                                    as unsafe extern "C" fn(
                                        *const MDB_val,
                                        *const MDB_val,
                                    )
                                        -> std::ffi::c_int,
                            ),
                        );
                        if flags & 0x4 as std::ffi::c_int != 0 {
                            mdb_set_dupsort(
                                txn,
                                dbi,
                                Some(
                                    greater
                                        as unsafe extern "C" fn(
                                            *const MDB_val,
                                            *const MDB_val,
                                        )
                                            -> std::ffi::c_int,
                                ),
                            );
                        }
                    }
                    rc = mdb_cursor_open(txn, dbi, &mut mc);
                    if rc != 0 {
                        fprintf(
                            __stderrp,
                            b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            rc,
                            mdb_strerror(rc),
                        );
                        current_block = 14945635589884787639;
                        break;
                    } else {
                        loop {
                            rc = readline(&mut key, &mut kbuf);
                            if rc != 0 {
                                break;
                            }
                            rc = readline(&mut data, &mut dbuf);
                            if rc != 0 {
                                fprintf(
                                    __stderrp,
                                    b"%s: line %zu: failed to read key value\n\0" as *const u8
                                        as *const std::ffi::c_char,
                                    prog,
                                    lineno,
                                );
                                current_block = 14945635589884787639;
                                break 's_222;
                            } else {
                                if append != 0 {
                                    appflag = 0x20000 as std::ffi::c_int;
                                    if flags & 0x4 as std::ffi::c_int != 0 {
                                        if prevk.mv_size == key.mv_size
                                            && memcmp(prevk.mv_data, key.mv_data, key.mv_size) == 0
                                        {
                                            appflag = 0x40 as std::ffi::c_int
                                                | 0x40000 as std::ffi::c_int;
                                        } else {
                                            memcpy(prevk.mv_data, key.mv_data, key.mv_size);
                                            prevk.mv_size = key.mv_size;
                                        }
                                    }
                                } else {
                                    appflag = 0 as std::ffi::c_int;
                                }
                                rc = mdb_cursor_put(
                                    mc,
                                    &mut key,
                                    &mut data,
                                    (putflags | appflag) as std::ffi::c_uint,
                                );
                                if rc == -(30799 as std::ffi::c_int) && putflags != 0 {
                                    continue;
                                }
                                if rc != 0 {
                                    fprintf(
                                        __stderrp,
                                        b"%s: line %zu: mdb_cursor_put failed, error %d %s\n\0"
                                            as *const u8
                                            as *const std::ffi::c_char,
                                        prog,
                                        lineno,
                                        rc,
                                        mdb_strerror(rc),
                                    );
                                    current_block = 14945635589884787639;
                                    break 's_222;
                                } else {
                                    batch += 1;
                                    batch;
                                    if !(batch == 100 as std::ffi::c_int) {
                                        continue;
                                    }
                                    rc = mdb_txn_commit(txn);
                                    if rc != 0 {
                                        fprintf(
                                            __stderrp,
                                            b"%s: line %zu: txn_commit: %s\n\0" as *const u8
                                                as *const std::ffi::c_char,
                                            prog,
                                            lineno,
                                            mdb_strerror(rc),
                                        );
                                        current_block = 16097525410562447625;
                                        break 's_222;
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
                                                b"mdb_txn_begin failed, error %d %s\n\0"
                                                    as *const u8
                                                    as *const std::ffi::c_char,
                                                rc,
                                                mdb_strerror(rc),
                                            );
                                            current_block = 16097525410562447625;
                                            break 's_222;
                                        } else {
                                            rc = mdb_cursor_open(txn, dbi, &mut mc);
                                            if rc != 0 {
                                                fprintf(
                                                    __stderrp,
                                                    b"mdb_cursor_open failed, error %d %s\n\0"
                                                        as *const u8
                                                        as *const std::ffi::c_char,
                                                    rc,
                                                    mdb_strerror(rc),
                                                );
                                                current_block = 14945635589884787639;
                                                break 's_222;
                                            } else {
                                                if append != 0 {
                                                    let mut k: MDB_val = MDB_val {
                                                        mv_size: 0,
                                                        mv_data: 0 as *const std::ffi::c_void
                                                            as *mut std::ffi::c_void,
                                                    };
                                                    let mut d: MDB_val = MDB_val {
                                                        mv_size: 0,
                                                        mv_data: 0 as *const std::ffi::c_void
                                                            as *mut std::ffi::c_void,
                                                    };
                                                    mdb_cursor_get(mc, &mut k, &mut d, MDB_LAST);
                                                    memcpy(prevk.mv_data, k.mv_data, k.mv_size);
                                                    prevk.mv_size = k.mv_size;
                                                }
                                                batch = 0 as std::ffi::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        rc = mdb_txn_commit(txn);
                        txn = 0 as *mut MDB_txn;
                        if rc != 0 {
                            fprintf(
                                __stderrp,
                                b"%s: line %zu: txn_commit: %s\n\0" as *const u8
                                    as *const std::ffi::c_char,
                                prog,
                                lineno,
                                mdb_strerror(rc),
                            );
                            current_block = 16097525410562447625;
                            break;
                        } else {
                            if envflags & 0x10000 as std::ffi::c_int != 0 {
                                rc = mdb_env_sync(env, 1 as std::ffi::c_int);
                                if rc != 0 {
                                    fprintf(
                                        __stderrp,
                                        b"mdb_env_sync failed, error %d %s\n\0" as *const u8
                                            as *const std::ffi::c_char,
                                        rc,
                                        mdb_strerror(rc),
                                    );
                                    current_block = 16097525410562447625;
                                    break;
                                }
                            }
                            mdb_dbi_close(env, dbi);
                        }
                    }
                }
            }
        }
        match current_block {
            16097525410562447625 => {}
            _ => {
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
unsafe extern "C" fn run_static_initializers() {
    dbflags = [
        {
            let mut init = flagbit {
                bit: 0x2 as std::ffi::c_int,
                name: b"reversekey\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                len: (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x4 as std::ffi::c_int,
                name: b"dupsort\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_char,
                len: (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x8 as std::ffi::c_int,
                name: b"integerkey\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                len: (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x10 as std::ffi::c_int,
                name: b"dupfixed\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                len: (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x20 as std::ffi::c_int,
                name: b"integerdup\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                len: (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0x40 as std::ffi::c_int,
                name: b"reversedup\0" as *const u8 as *const std::ffi::c_char
                    as *mut std::ffi::c_char,
                len: (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
                    as std::ffi::c_int,
            };
            init
        },
        {
            let mut init = flagbit {
                bit: 0 as std::ffi::c_int,
                name: 0 as *mut std::ffi::c_char,
                len: 0 as std::ffi::c_int,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
