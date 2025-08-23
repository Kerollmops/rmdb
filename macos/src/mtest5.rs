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
    pub type MDB_cursor;
    fn printf(_: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn rand() -> std::ffi::c_int;
    fn srand(_: std::ffi::c_uint);
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn abort() -> !;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn time(_: *mut time_t) -> time_t;
    fn mdb_strerror(err: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn mdb_env_create(env: *mut *mut MDB_env) -> std::ffi::c_int;
    fn mdb_env_open(
        env: *mut MDB_env,
        path: *const std::ffi::c_char,
        flags: std::ffi::c_uint,
        mode: mdb_mode_t,
    ) -> std::ffi::c_int;
    fn mdb_env_stat(env: *mut MDB_env, stat: *mut MDB_stat) -> std::ffi::c_int;
    fn mdb_env_close(env: *mut MDB_env);
    fn mdb_env_set_mapsize(env: *mut MDB_env, size: mdb_size_t) -> std::ffi::c_int;
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
    fn mdb_dbi_close(env: *mut MDB_env, dbi: MDB_dbi);
    fn mdb_del(
        txn: *mut MDB_txn,
        dbi: MDB_dbi,
        key: *mut MDB_val,
        data: *mut MDB_val,
    ) -> std::ffi::c_int;
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
    fn mdb_cursor_put(
        cursor: *mut MDB_cursor,
        key: *mut MDB_val,
        data: *mut MDB_val,
        flags: std::ffi::c_uint,
    ) -> std::ffi::c_int;
}
pub type __uint16_t = std::ffi::c_ushort;
pub type __int64_t = std::ffi::c_longlong;
pub type __darwin_size_t = std::ffi::c_ulong;
pub type __darwin_time_t = std::ffi::c_long;
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
pub type time_t = __darwin_time_t;
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
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int {
    let mut i: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut j: std::ffi::c_int = 0 as std::ffi::c_int;
    let mut rc: std::ffi::c_int = 0;
    let mut env: *mut MDB_env = 0 as *mut MDB_env;
    let mut dbi: MDB_dbi = 0;
    let mut key: MDB_val = MDB_val { mv_size: 0, mv_data: 0 as *mut std::ffi::c_void };
    let mut data: MDB_val = MDB_val { mv_size: 0, mv_data: 0 as *mut std::ffi::c_void };
    let mut txn: *mut MDB_txn = 0 as *mut MDB_txn;
    let mut mst: MDB_stat = MDB_stat {
        ms_psize: 0,
        ms_depth: 0,
        ms_branch_pages: 0,
        ms_leaf_pages: 0,
        ms_overflow_pages: 0,
        ms_entries: 0,
    };
    let mut cursor: *mut MDB_cursor = 0 as *mut MDB_cursor;
    let mut count: std::ffi::c_int = 0;
    let mut values: *mut std::ffi::c_int = 0 as *mut std::ffi::c_int;
    let mut sval: [std::ffi::c_char; 32] = [0; 32];
    let mut kval: [std::ffi::c_char; 4] = [0; 4];
    srand(time(0 as *mut time_t) as std::ffi::c_uint);
    memset(
        sval.as_mut_ptr() as *mut std::ffi::c_void,
        0 as std::ffi::c_int,
        ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong,
    );
    count = rand() % 384 as std::ffi::c_int + 64 as std::ffi::c_int;
    values = malloc(
        (count as std::ffi::c_ulong)
            .wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong),
    ) as *mut std::ffi::c_int;
    i = 0 as std::ffi::c_int;
    while i < count {
        *values.offset(i as isize) = rand() % 1024 as std::ffi::c_int;
        i += 1;
        i;
    }
    rc = mdb_env_create(&mut env);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            52 as std::ffi::c_int,
            b"mdb_env_create(&env)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_set_mapsize(env, 10485760 as std::ffi::c_int as mdb_size_t);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            53 as std::ffi::c_int,
            b"mdb_env_set_mapsize(env, 10485760)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_set_maxdbs(env, 4 as std::ffi::c_int as MDB_dbi);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            54 as std::ffi::c_int,
            b"mdb_env_set_maxdbs(env, 4)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_open(
        env,
        b"./testdb\0" as *const u8 as *const std::ffi::c_char,
        (0x1 as std::ffi::c_int | 0x10000 as std::ffi::c_int) as std::ffi::c_uint,
        0o664 as std::ffi::c_int as mdb_mode_t,
    );
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            55 as std::ffi::c_int,
            b"mdb_env_open(env, \"./testdb\", MDB_FIXEDMAP|MDB_NOSYNC, 0664)\0" as *const u8
                as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_txn_begin(env, 0 as *mut MDB_txn, 0 as std::ffi::c_int as std::ffi::c_uint, &mut txn);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            57 as std::ffi::c_int,
            b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_dbi_open(
        txn,
        b"id2\0" as *const u8 as *const std::ffi::c_char,
        (0x40000 as std::ffi::c_int | 0x4 as std::ffi::c_int) as std::ffi::c_uint,
        &mut dbi,
    );
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            58 as std::ffi::c_int,
            b"mdb_dbi_open(txn, \"id2\", MDB_CREATE|MDB_DUPSORT, &dbi)\0" as *const u8
                as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            59 as std::ffi::c_int,
            b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    key.mv_size = ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong;
    key.mv_data = kval.as_mut_ptr() as *mut std::ffi::c_void;
    data.mv_size = ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong;
    data.mv_data = sval.as_mut_ptr() as *mut std::ffi::c_void;
    printf(b"Adding %d values\n\0" as *const u8 as *const std::ffi::c_char, count);
    i = 0 as std::ffi::c_int;
    while i < count {
        if i & 0xf as std::ffi::c_int == 0 {
            sprintf(
                kval.as_mut_ptr(),
                b"%03x\0" as *const u8 as *const std::ffi::c_char,
                *values.offset(i as isize),
            );
        }
        sprintf(
            sval.as_mut_ptr(),
            b"%03x %d foo bar\0" as *const u8 as *const std::ffi::c_char,
            *values.offset(i as isize),
            *values.offset(i as isize),
        );
        rc = mdb_cursor_put(
            cursor,
            &mut key,
            &mut data,
            0x20 as std::ffi::c_int as std::ffi::c_uint,
        );
        if rc == -(30799 as std::ffi::c_int) || {
            if rc == 0 {
            } else {
                fprintf(
                    __stderrp,
                    b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                    b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
                    71 as std::ffi::c_int,
                    b"mdb_cursor_put(cursor, &key, &data, MDB_NODUPDATA)\0" as *const u8
                        as *const std::ffi::c_char,
                    mdb_strerror(rc),
                );
                abort();
            };
            0 as std::ffi::c_int != 0
        } {
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    if j != 0 {
        printf(b"%d duplicates skipped\n\0" as *const u8 as *const std::ffi::c_char, j);
    }
    mdb_cursor_close(cursor);
    rc = mdb_txn_commit(txn);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            76 as std::ffi::c_int,
            b"mdb_txn_commit(txn)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_stat(env, &mut mst);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            77 as std::ffi::c_int,
            b"mdb_env_stat(env, &mst)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_txn_begin(
        env,
        0 as *mut MDB_txn,
        0x20000 as std::ffi::c_int as std::ffi::c_uint,
        &mut txn,
    );
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            79 as std::ffi::c_int,
            b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            80 as std::ffi::c_int,
            b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
        if !(rc == 0 as std::ffi::c_int) {
            break;
        }
        printf(
            b"key: %p %.*s, data: %p %.*s\n\0" as *const u8 as *const std::ffi::c_char,
            key.mv_data,
            key.mv_size as std::ffi::c_int,
            key.mv_data as *mut std::ffi::c_char,
            data.mv_data,
            data.mv_size as std::ffi::c_int,
            data.mv_data as *mut std::ffi::c_char,
        );
    }
    if rc == -(30798 as std::ffi::c_int) {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            86 as std::ffi::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    j = 0 as std::ffi::c_int;
    i = count - 1 as std::ffi::c_int;
    while i > -(1 as std::ffi::c_int) {
        j += 1;
        j;
        txn = 0 as *mut MDB_txn;
        rc = mdb_txn_begin(
            env,
            0 as *mut MDB_txn,
            0 as std::ffi::c_int as std::ffi::c_uint,
            &mut txn,
        );
        if rc == 0 as std::ffi::c_int {
        } else {
            fprintf(
                __stderrp,
                b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
                95 as std::ffi::c_int,
                b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const std::ffi::c_char,
                mdb_strerror(rc),
            );
            abort();
        };
        sprintf(
            kval.as_mut_ptr(),
            b"%03x\0" as *const u8 as *const std::ffi::c_char,
            *values.offset((i & !(0xf as std::ffi::c_int)) as isize),
        );
        sprintf(
            sval.as_mut_ptr(),
            b"%03x %d foo bar\0" as *const u8 as *const std::ffi::c_char,
            *values.offset(i as isize),
            *values.offset(i as isize),
        );
        key.mv_size = ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong;
        key.mv_data = kval.as_mut_ptr() as *mut std::ffi::c_void;
        data.mv_size = ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong;
        data.mv_data = sval.as_mut_ptr() as *mut std::ffi::c_void;
        rc = mdb_del(txn, dbi, &mut key, &mut data);
        if rc == -(30798 as std::ffi::c_int) || {
            if rc == 0 {
            } else {
                fprintf(
                    __stderrp,
                    b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                    b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
                    102 as std::ffi::c_int,
                    b"mdb_del(txn, dbi, &key, &data)\0" as *const u8 as *const std::ffi::c_char,
                    mdb_strerror(rc),
                );
                abort();
            };
            0 as std::ffi::c_int != 0
        } {
            j -= 1;
            j;
            mdb_txn_abort(txn);
        } else {
            rc = mdb_txn_commit(txn);
            if rc == 0 as std::ffi::c_int {
            } else {
                fprintf(
                    __stderrp,
                    b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                    b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
                    106 as std::ffi::c_int,
                    b"mdb_txn_commit(txn)\0" as *const u8 as *const std::ffi::c_char,
                    mdb_strerror(rc),
                );
                abort();
            };
        }
        i -= rand() % 5 as std::ffi::c_int;
    }
    free(values as *mut std::ffi::c_void);
    printf(b"Deleted %d values\n\0" as *const u8 as *const std::ffi::c_char, j);
    rc = mdb_env_stat(env, &mut mst);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            112 as std::ffi::c_int,
            b"mdb_env_stat(env, &mst)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_txn_begin(
        env,
        0 as *mut MDB_txn,
        0x20000 as std::ffi::c_int as std::ffi::c_uint,
        &mut txn,
    );
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            113 as std::ffi::c_int,
            b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            114 as std::ffi::c_int,
            b"mdb_cursor_open(txn, dbi, &cursor)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    printf(b"Cursor next\n\0" as *const u8 as *const std::ffi::c_char);
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
        if !(rc == 0 as std::ffi::c_int) {
            break;
        }
        printf(
            b"key: %.*s, data: %.*s\n\0" as *const u8 as *const std::ffi::c_char,
            key.mv_size as std::ffi::c_int,
            key.mv_data as *mut std::ffi::c_char,
            data.mv_size as std::ffi::c_int,
            data.mv_data as *mut std::ffi::c_char,
        );
    }
    if rc == -(30798 as std::ffi::c_int) {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            121 as std::ffi::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    printf(b"Cursor prev\n\0" as *const u8 as *const std::ffi::c_char);
    loop {
        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_PREV);
        if !(rc == 0 as std::ffi::c_int) {
            break;
        }
        printf(
            b"key: %.*s, data: %.*s\n\0" as *const u8 as *const std::ffi::c_char,
            key.mv_size as std::ffi::c_int,
            key.mv_data as *mut std::ffi::c_char,
            data.mv_size as std::ffi::c_int,
            data.mv_data as *mut std::ffi::c_char,
        );
    }
    if rc == -(30798 as std::ffi::c_int) {
    } else {
        fprintf(
            __stderrp,
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest5.c\0" as *const u8 as *const std::ffi::c_char,
            128 as std::ffi::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    mdb_dbi_close(env, dbi);
    mdb_env_close(env);
    return 0 as std::ffi::c_int;
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
