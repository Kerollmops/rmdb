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
    fn sprintf(
        __s: *mut std::ffi::c_char,
        __format: *const std::ffi::c_char,
        ...
    ) -> std::ffi::c_int;
    fn rand() -> std::ffi::c_int;
    fn srand(__seed: std::ffi::c_uint);
    fn malloc(__size: size_t) -> *mut std::ffi::c_void;
    fn free(__ptr: *mut std::ffi::c_void);
    fn abort() -> !;
    fn time(__timer: *mut time_t) -> time_t;
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
pub type __time_t = std::ffi::c_long;
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
pub type time_t = __time_t;
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
unsafe fn main_0(
    mut argc: std::ffi::c_int,
    mut argv: *mut *mut std::ffi::c_char,
) -> std::ffi::c_int { unsafe {
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
    let mut sval: [std::ffi::c_char; 32] =
        *::core::mem::transmute::<&[u8; 32], &mut [std::ffi::c_char; 32]>(
            b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        );
    srand(time(0 as *mut time_t) as std::ffi::c_uint);
    count = rand() % 384 as std::ffi::c_int + 64 as std::ffi::c_int;
    values =
        malloc((count as size_t).wrapping_mul(::core::mem::size_of::<std::ffi::c_int>() as size_t))
            as *mut std::ffi::c_int;
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
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            49 as std::ffi::c_int,
            b"mdb_env_create(&env)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_set_maxreaders(env, 1 as std::ffi::c_uint);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            50 as std::ffi::c_int,
            b"mdb_env_set_maxreaders(env, 1)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_set_mapsize(env, 10485760 as mdb_size_t);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            51 as std::ffi::c_int,
            b"mdb_env_set_mapsize(env, 10485760)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_set_maxdbs(env, 4 as MDB_dbi);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            52 as std::ffi::c_int,
            b"mdb_env_set_maxdbs(env, 4)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_open(
        env,
        b"./testdb\0" as *const u8 as *const std::ffi::c_char,
        (0x1 as std::ffi::c_int | 0x10000 as std::ffi::c_int) as std::ffi::c_uint,
        0o664 as mdb_mode_t,
    );
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            53 as std::ffi::c_int,
            b"mdb_env_open(env, \"./testdb\", MDB_FIXEDMAP|MDB_NOSYNC, 0664)\0" as *const u8
                as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_txn_begin(env, 0 as *mut MDB_txn, 0 as std::ffi::c_uint, &mut txn);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            55 as std::ffi::c_int,
            b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_dbi_open(
        txn,
        b"id1\0" as *const u8 as *const std::ffi::c_char,
        0x40000 as std::ffi::c_uint,
        &mut dbi,
    );
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            56 as std::ffi::c_int,
            b"mdb_dbi_open(txn, \"id1\", MDB_CREATE, &dbi)\0" as *const u8
                as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    key.mv_size = ::core::mem::size_of::<std::ffi::c_int>() as std::ffi::c_ulong as size_t;
    key.mv_data = sval.as_mut_ptr() as *mut std::ffi::c_void;
    printf(b"Adding %d values\n\0" as *const u8 as *const std::ffi::c_char, count);
    i = 0 as std::ffi::c_int;
    while i < count {
        sprintf(
            sval.as_mut_ptr(),
            b"%03x %d foo bar\0" as *const u8 as *const std::ffi::c_char,
            *values.offset(i as isize),
            *values.offset(i as isize),
        );
        data.mv_size =
            ::core::mem::size_of::<[std::ffi::c_char; 32]>() as std::ffi::c_ulong as size_t;
        data.mv_data = sval.as_mut_ptr() as *mut std::ffi::c_void;
        rc = mdb_put(txn, dbi, &mut key, &mut data, 0x10 as std::ffi::c_uint);
        if rc == -(30799 as std::ffi::c_int) || {
            if rc == 0 {
            } else {
                fprintf(
                    get_stderr(),
                    b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                    b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
                    66 as std::ffi::c_int,
                    b"mdb_put(txn, dbi, &key, &data, MDB_NOOVERWRITE)\0" as *const u8
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
    rc = mdb_txn_commit(txn);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            70 as std::ffi::c_int,
            b"mdb_txn_commit(txn)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_env_stat(env, &mut mst);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            71 as std::ffi::c_int,
            b"mdb_env_stat(env, &mst)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_txn_begin(env, 0 as *mut MDB_txn, 0x20000 as std::ffi::c_uint, &mut txn);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            73 as std::ffi::c_int,
            b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            74 as std::ffi::c_int,
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
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            80 as std::ffi::c_int,
            b"mdb_cursor_get\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    mdb_cursor_close(cursor);
    mdb_txn_abort(txn);
    j = 0 as std::ffi::c_int;
    key.mv_data = sval.as_mut_ptr() as *mut std::ffi::c_void;
    i = count - 1 as std::ffi::c_int;
    while i > -(1 as std::ffi::c_int) {
        j += 1;
        j;
        txn = 0 as *mut MDB_txn;
        rc = mdb_txn_begin(env, 0 as *mut MDB_txn, 0 as std::ffi::c_uint, &mut txn);
        if rc == 0 as std::ffi::c_int {
        } else {
            fprintf(
                get_stderr(),
                b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
                89 as std::ffi::c_int,
                b"mdb_txn_begin(env, NULL, 0, &txn)\0" as *const u8 as *const std::ffi::c_char,
                mdb_strerror(rc),
            );
            abort();
        };
        sprintf(
            sval.as_mut_ptr(),
            b"%03x \0" as *const u8 as *const std::ffi::c_char,
            *values.offset(i as isize),
        );
        rc = mdb_del(txn, dbi, &mut key, 0 as *mut MDB_val);
        if rc == -(30798 as std::ffi::c_int) || {
            if rc == 0 {
            } else {
                fprintf(
                    get_stderr(),
                    b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                    b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
                    91 as std::ffi::c_int,
                    b"mdb_del(txn, dbi, &key, NULL)\0" as *const u8 as *const std::ffi::c_char,
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
                    get_stderr(),
                    b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
                    b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
                    95 as std::ffi::c_int,
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
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            101 as std::ffi::c_int,
            b"mdb_env_stat(env, &mst)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_txn_begin(env, 0 as *mut MDB_txn, 0x20000 as std::ffi::c_uint, &mut txn);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            102 as std::ffi::c_int,
            b"mdb_txn_begin(env, NULL, MDB_RDONLY, &txn)\0" as *const u8 as *const std::ffi::c_char,
            mdb_strerror(rc),
        );
        abort();
    };
    rc = mdb_cursor_open(txn, dbi, &mut cursor);
    if rc == 0 as std::ffi::c_int {
    } else {
        fprintf(
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            103 as std::ffi::c_int,
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
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            110 as std::ffi::c_int,
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
            get_stderr(),
            b"%s:%d: %s: %s\n\0" as *const u8 as *const std::ffi::c_char,
            b"mtest2.c\0" as *const u8 as *const std::ffi::c_char,
            117 as std::ffi::c_int,
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
