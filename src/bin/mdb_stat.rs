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
    fn fputs(__s: *const std::ffi::c_char, __stream: *mut FILE) -> std::ffi::c_int;
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
    static mut optarg: *mut std::ffi::c_char;
    static mut optind: std::ffi::c_int;
    fn getopt(
        ___argc: std::ffi::c_int,
        ___argv: *const *mut std::ffi::c_char,
        __shortopts: *const std::ffi::c_char,
    ) -> std::ffi::c_int;
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

use rmdb::*;

use crate::mdb_mode_t;
pub type __mode_t = std::ffi::c_uint;
pub type __off_t = std::ffi::c_long;
pub type __off64_t = std::ffi::c_long;
pub type __ssize_t = std::ffi::c_long;
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
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;
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
pub type MDB_msg_func =
    unsafe extern "C" fn(*const std::ffi::c_char, *mut std::ffi::c_void) -> std::ffi::c_int;
pub const EXIT_FAILURE: std::ffi::c_int = 1 as std::ffi::c_int;
pub const EXIT_SUCCESS: std::ffi::c_int = 0 as std::ffi::c_int;
pub const MDB_NOSUBDIR: std::ffi::c_int = 0x4000 as std::ffi::c_int;
pub const MDB_PREVSNAPSHOT: std::ffi::c_int = 0x2000000 as std::ffi::c_int;
pub const MDB_SUCCESS: std::ffi::c_int = 0 as std::ffi::c_int;
unsafe extern "C" fn prstat(mut ms: *mut MDB_stat) {
    unsafe {
        printf(b"  Tree depth: %u\n\0" as *const u8 as *const std::ffi::c_char, (*ms).ms_depth);
        printf(
            b"  Branch pages: %zu\n\0" as *const u8 as *const std::ffi::c_char,
            (*ms).ms_branch_pages,
        );
        printf(
            b"  Leaf pages: %zu\n\0" as *const u8 as *const std::ffi::c_char,
            (*ms).ms_leaf_pages,
        );
        printf(
            b"  Overflow pages: %zu\n\0" as *const u8 as *const std::ffi::c_char,
            (*ms).ms_overflow_pages,
        );
        printf(b"  Entries: %zu\n\0" as *const u8 as *const std::ffi::c_char, (*ms).ms_entries);
    }
}
unsafe extern "C" fn usage(mut prog: *mut std::ffi::c_char) {
    unsafe {
        fprintf(
            get_stderr(),
            b"usage: %s [-V] [-n] [-e] [-r[r]] [-f[f[f]]] [-v] [-a|-s subdb] dbpath\n\0"
                as *const u8 as *const std::ffi::c_char,
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
        let mut current_block: u64;
        let mut i: std::ffi::c_int = 0;
        let mut rc: std::ffi::c_int = 0;
        let mut env: *mut MDB_env = std::ptr::null_mut::<MDB_env>();
        let mut txn: *mut MDB_txn = std::ptr::null_mut::<MDB_txn>();
        let mut dbi: MDB_dbi = 0;
        let mut mst: MDB_stat = MDB_stat {
            ms_psize: 0,
            ms_depth: 0,
            ms_branch_pages: 0,
            ms_leaf_pages: 0,
            ms_overflow_pages: 0,
            ms_entries: 0,
        };
        let mut mei: MDB_envinfo = MDB_envinfo {
            me_mapaddr: std::ptr::null_mut::<std::ffi::c_void>(),
            me_mapsize: 0,
            me_last_pgno: 0,
            me_last_txnid: 0,
            me_maxreaders: 0,
            me_numreaders: 0,
        };
        let mut prog: *mut std::ffi::c_char = *argv.offset(0 as std::ffi::c_int as isize);
        let mut envname: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut subname: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut alldbs: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut envinfo: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut envflags: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut freinfo: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut rdrinfo: std::ffi::c_int = 0 as std::ffi::c_int;
        if argc < 2 as std::ffi::c_int {
            usage(prog);
        }
        loop {
            i = getopt(
                argc,
                argv as *const *mut std::ffi::c_char,
                b"Vaefnrs:v\0" as *const u8 as *const std::ffi::c_char,
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
                97 => {
                    if !subname.is_null() {
                        usage(prog);
                    }
                    alldbs += 1;
                }
                101 => {
                    envinfo += 1;
                }
                102 => {
                    freinfo += 1;
                }
                110 => {
                    envflags |= MDB_NOSUBDIR;
                }
                118 => {
                    envflags |= MDB_PREVSNAPSHOT;
                }
                114 => {
                    rdrinfo += 1;
                }
                115 => {
                    if alldbs != 0 {
                        usage(prog);
                    }
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
            mdb_env_set_maxdbs(env, 4 as MDB_dbi);
        }
        rc = mdb_env_open(
            env,
            envname,
            (envflags | 0x20000 as std::ffi::c_int) as std::ffi::c_uint,
            0o664 as mdb_mode_t as _,
        );
        if rc != 0 {
            fprintf(
                get_stderr(),
                b"mdb_env_open failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
                rc,
                mdb_strerror(rc),
            );
        } else {
            if envinfo != 0 {
                mdb_env_stat(env, &mut mst);
                mdb_env_info(env, &mut mei);
                printf(b"Environment Info\n\0" as *const u8 as *const std::ffi::c_char);
                printf(
                    b"  Map address: %p\n\0" as *const u8 as *const std::ffi::c_char,
                    mei.me_mapaddr,
                );
                printf(
                    b"  Map size: %zu\n\0" as *const u8 as *const std::ffi::c_char,
                    mei.me_mapsize,
                );
                printf(
                    b"  Page size: %u\n\0" as *const u8 as *const std::ffi::c_char,
                    mst.ms_psize,
                );
                printf(
                    b"  Max pages: %zu\n\0" as *const u8 as *const std::ffi::c_char,
                    (mei.me_mapsize).wrapping_div(mst.ms_psize as mdb_size_t as _),
                );
                printf(
                    b"  Number of pages used: %zu\n\0" as *const u8 as *const std::ffi::c_char,
                    (mei.me_last_pgno).wrapping_add(1 as mdb_size_t as _),
                );
                printf(
                    b"  Last transaction ID: %zu\n\0" as *const u8 as *const std::ffi::c_char,
                    mei.me_last_txnid,
                );
                printf(
                    b"  Max readers: %u\n\0" as *const u8 as *const std::ffi::c_char,
                    mei.me_maxreaders,
                );
                printf(
                    b"  Number of readers used: %u\n\0" as *const u8 as *const std::ffi::c_char,
                    mei.me_numreaders,
                );
            }
            if rdrinfo != 0 {
                printf(b"Reader Table Status\n\0" as *const u8 as *const std::ffi::c_char);
                rc = mdb_reader_list(
                    env,
                    ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *const std::ffi::c_char,
                                *mut FILE,
                            ) -> std::ffi::c_int,
                        >,
                        Option<MDB_msg_func>,
                    >(Some(
                        fputs
                            as unsafe extern "C" fn(
                                *const std::ffi::c_char,
                                *mut FILE,
                            ) -> std::ffi::c_int,
                    )),
                    get_stdout() as *mut std::ffi::c_void,
                );
                if rdrinfo > 1 as std::ffi::c_int {
                    let mut dead: std::ffi::c_int = 0;
                    mdb_reader_check(env, &mut dead);
                    printf(
                        b"  %d stale readers cleared.\n\0" as *const u8 as *const std::ffi::c_char,
                        dead,
                    );
                    rc = mdb_reader_list(
                        env,
                        ::core::mem::transmute::<
                            Option<
                                unsafe extern "C" fn(
                                    *const std::ffi::c_char,
                                    *mut FILE,
                                )
                                    -> std::ffi::c_int,
                            >,
                            Option<MDB_msg_func>,
                        >(Some(
                            fputs
                                as unsafe extern "C" fn(
                                    *const std::ffi::c_char,
                                    *mut FILE,
                                )
                                    -> std::ffi::c_int,
                        )),
                        get_stdout() as *mut std::ffi::c_void,
                    );
                }
                if !(!subname.is_null() || alldbs != 0 || freinfo != 0) {
                    current_block = 9536332248080802547;
                } else {
                    current_block = 7420279277351916581;
                }
            } else {
                current_block = 7420279277351916581;
            }
            match current_block {
                9536332248080802547 => {}
                _ => {
                    rc = mdb_txn_begin(
                        env,
                        std::ptr::null_mut::<MDB_txn>(),
                        0x20000 as std::ffi::c_uint,
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
                        if freinfo != 0 {
                            let mut cursor: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                            let mut key: MDB_val = MDB_val {
                                mv_size: 0,
                                mv_data: std::ptr::null_mut::<std::ffi::c_void>(),
                            };
                            let mut data: MDB_val = MDB_val {
                                mv_size: 0,
                                mv_data: std::ptr::null_mut::<std::ffi::c_void>(),
                            };
                            let mut pages: mdb_size_t = 0 as mdb_size_t;
                            let mut iptr: *mut mdb_size_t = std::ptr::null_mut::<mdb_size_t>();
                            printf(b"Freelist Status\n\0" as *const u8 as *const std::ffi::c_char);
                            dbi = 0 as MDB_dbi;
                            rc = mdb_cursor_open(txn, dbi, &mut cursor);
                            if rc != 0 {
                                fprintf(
                                    get_stderr(),
                                    b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                        as *const std::ffi::c_char,
                                    rc,
                                    mdb_strerror(rc),
                                );
                                current_block = 17913902266519338787;
                            } else {
                                rc = mdb_stat(txn, dbi, &mut mst);
                                if rc != 0 {
                                    fprintf(
                                        get_stderr(),
                                        b"mdb_stat failed, error %d %s\n\0" as *const u8
                                            as *const std::ffi::c_char,
                                        rc,
                                        mdb_strerror(rc),
                                    );
                                    current_block = 17913902266519338787;
                                } else {
                                    prstat(&mut mst);
                                    loop {
                                        rc = mdb_cursor_get(cursor, &mut key, &mut data, MDB_NEXT);
                                        if rc != 0 as std::ffi::c_int {
                                            break;
                                        }
                                        iptr = data.mv_data as *mut mdb_size_t;
                                        pages = (pages as std::ffi::c_ulong)
                                            .wrapping_add(*iptr as std::ffi::c_ulong)
                                            as mdb_size_t
                                            as mdb_size_t;
                                        if freinfo > 1 as std::ffi::c_int {
                                            let mut bad: *mut std::ffi::c_char = b"\0" as *const u8
                                                as *const std::ffi::c_char
                                                as *mut std::ffi::c_char;
                                            let mut pg: mdb_size_t = 0;
                                            let mut prev: mdb_size_t = 0;
                                            let mut i_0: ssize_t = 0;
                                            let mut j: ssize_t = 0;
                                            let mut span: ssize_t = 0 as ssize_t;
                                            let fresh0 = iptr;
                                            iptr = iptr.offset(1);
                                            j = *fresh0 as ssize_t;
                                            i_0 = j;
                                            prev = 1 as mdb_size_t;
                                            loop {
                                                i_0 -= 1;
                                                if i_0 < 0 as std::ffi::c_long {
                                                    break;
                                                }
                                                pg = *iptr.offset(i_0 as isize);
                                                if pg <= prev {
                                                    bad = b" [bad sequence]\0" as *const u8
                                                        as *const std::ffi::c_char
                                                        as *mut std::ffi::c_char;
                                                }
                                                prev = pg;
                                                pg = (pg as std::ffi::c_ulong)
                                                    .wrapping_add(span as std::ffi::c_ulong)
                                                    as mdb_size_t
                                                    as mdb_size_t;
                                                while i_0 >= span
                                                    && *iptr.offset((i_0 - span) as isize) == pg
                                                {
                                                    span += 1;
                                                    pg = pg.wrapping_add(1);
                                                }
                                            }
                                            printf(
                                                b"    Transaction %zu, %zd pages, maxspan %zd%s\n\0"
                                                    as *const u8
                                                    as *const std::ffi::c_char,
                                                *(key.mv_data as *mut mdb_size_t),
                                                j,
                                                span,
                                                bad,
                                            );
                                            if freinfo > 2 as std::ffi::c_int {
                                                j -= 1;
                                                while j >= 0 as std::ffi::c_long {
                                                    pg = *iptr.offset(j as isize);
                                                    span = 1 as ssize_t;
                                                    loop {
                                                        j -= 1;
                                                        if !(j >= 0 as std::ffi::c_long
                                                            && *iptr.offset(j as isize)
                                                                == pg.wrapping_add(
                                                                    span as mdb_size_t,
                                                                ))
                                                        {
                                                            break;
                                                        }
                                                        span += 1;
                                                    }
                                                    printf(
                                                        if span > 1 as std::ffi::c_long {
                                                            b"     %9zu[%zd]\n\0" as *const u8
                                                                as *const std::ffi::c_char
                                                        } else {
                                                            b"     %9zu\n\0" as *const u8
                                                                as *const std::ffi::c_char
                                                        },
                                                        pg,
                                                        span,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    mdb_cursor_close(cursor);
                                    printf(
                                        b"  Free pages: %zu\n\0" as *const u8
                                            as *const std::ffi::c_char,
                                        pages,
                                    );
                                    current_block = 14541395414537699361;
                                }
                            }
                        } else {
                            current_block = 14541395414537699361;
                        }
                        if current_block == 14541395414537699361 {
                            rc = mdb_dbi_open(txn, subname, 0 as std::ffi::c_uint, &mut dbi);
                            if rc != 0 {
                                fprintf(
                                    get_stderr(),
                                    b"mdb_open failed, error %d %s\n\0" as *const u8
                                        as *const std::ffi::c_char,
                                    rc,
                                    mdb_strerror(rc),
                                );
                            } else {
                                rc = mdb_stat(txn, dbi, &mut mst);
                                if rc != 0 {
                                    fprintf(
                                        get_stderr(),
                                        b"mdb_stat failed, error %d %s\n\0" as *const u8
                                            as *const std::ffi::c_char,
                                        rc,
                                        mdb_strerror(rc),
                                    );
                                } else {
                                    printf(
                                        b"Status of %s\n\0" as *const u8 as *const std::ffi::c_char,
                                        if !subname.is_null() {
                                            subname as *const std::ffi::c_char
                                        } else {
                                            b"Main DB\0" as *const u8 as *const std::ffi::c_char
                                        },
                                    );
                                    prstat(&mut mst);
                                    if alldbs != 0 {
                                        let mut cursor_0: *mut MDB_cursor =
                                            std::ptr::null_mut::<MDB_cursor>();
                                        let mut key_0: MDB_val = MDB_val {
                                            mv_size: 0,
                                            mv_data: std::ptr::null_mut::<std::ffi::c_void>(),
                                        };
                                        rc = mdb_cursor_open(txn, dbi, &mut cursor_0);
                                        if rc != 0 {
                                            fprintf(
                                                get_stderr(),
                                                b"mdb_cursor_open failed, error %d %s\n\0"
                                                    as *const u8
                                                    as *const std::ffi::c_char,
                                                rc,
                                                mdb_strerror(rc),
                                            );
                                            current_block = 17913902266519338787;
                                        } else {
                                            loop {
                                                rc = mdb_cursor_get(
                                                    cursor_0,
                                                    &mut key_0,
                                                    std::ptr::null_mut::<MDB_val>(),
                                                    MDB_NEXT_NODUP,
                                                );
                                                if rc != 0 as std::ffi::c_int {
                                                    current_block = 17702298541784679949;
                                                    break;
                                                }
                                                let mut str: *mut std::ffi::c_char =
                                                    std::ptr::null_mut::<std::ffi::c_char>();
                                                let mut db2: MDB_dbi = 0;
                                                if !(memchr(
                                                    key_0.mv_data,
                                                    '\0' as i32,
                                                    key_0.mv_size as _,
                                                ))
                                                .is_null()
                                                {
                                                    continue;
                                                }
                                                str = malloc(
                                                    (key_0.mv_size as size_t).wrapping_add(1),
                                                )
                                                    as *mut std::ffi::c_char;
                                                memcpy(
                                                    str as *mut std::ffi::c_void,
                                                    key_0.mv_data,
                                                    key_0.mv_size as _,
                                                );
                                                *str.offset(key_0.mv_size as isize) =
                                                    '\0' as i32 as std::ffi::c_char;
                                                rc = mdb_dbi_open(
                                                    txn,
                                                    str,
                                                    0 as std::ffi::c_uint,
                                                    &mut db2,
                                                );
                                                if rc == MDB_SUCCESS {
                                                    printf(
                                                        b"Status of %s\n\0" as *const u8
                                                            as *const std::ffi::c_char,
                                                        str,
                                                    );
                                                }
                                                free(str as *mut std::ffi::c_void);
                                                if rc != 0 {
                                                    continue;
                                                }
                                                rc = mdb_stat(txn, db2, &mut mst);
                                                if rc != 0 {
                                                    fprintf(
                                                        get_stderr(),
                                                        b"mdb_stat failed, error %d %s\n\0"
                                                            as *const u8
                                                            as *const std::ffi::c_char,
                                                        rc,
                                                        mdb_strerror(rc),
                                                    );
                                                    current_block = 17913902266519338787;
                                                    break;
                                                } else {
                                                    prstat(&mut mst);
                                                    mdb_dbi_close(env, db2);
                                                }
                                            }
                                            match current_block {
                                                17913902266519338787 => {}
                                                _ => {
                                                    mdb_cursor_close(cursor_0);
                                                    current_block = 16778110326724371720;
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 16778110326724371720;
                                    }
                                    match current_block {
                                        17913902266519338787 => {}
                                        _ => {
                                            if rc == -(30798 as std::ffi::c_int) {
                                                rc = MDB_SUCCESS;
                                            }
                                            mdb_dbi_close(env, dbi);
                                        }
                                    }
                                }
                            }
                        }
                        mdb_txn_abort(txn);
                    }
                }
            }
        }
        mdb_env_close(env);
        if rc != 0 {
            EXIT_FAILURE
        } else {
            EXIT_SUCCESS
        }
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
