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
    fn sscanf(
        __s: *const std::ffi::c_char,
        __format: *const std::ffi::c_char,
        ...
    ) -> std::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> std::ffi::c_int;
    fn fgets(
        __s: *mut std::ffi::c_char,
        __n: std::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut std::ffi::c_char;
    fn strtol(
        __nptr: *const std::ffi::c_char,
        __endptr: *mut *mut std::ffi::c_char,
        __base: std::ffi::c_int,
    ) -> std::ffi::c_long;
    fn malloc(__size: size_t) -> *mut std::ffi::c_void;
    fn realloc(__ptr: *mut std::ffi::c_void, __size: size_t) -> *mut std::ffi::c_void;
    fn free(__ptr: *mut std::ffi::c_void);
    fn exit(__status: std::ffi::c_int) -> !;
    fn memcpy(
        __dest: *mut std::ffi::c_void,
        __src: *const std::ffi::c_void,
        __n: size_t,
    ) -> *mut std::ffi::c_void;
    fn memcmp(
        __s1: *const std::ffi::c_void,
        __s2: *const std::ffi::c_void,
        __n: size_t,
    ) -> std::ffi::c_int;
    fn memchr(
        __s: *const std::ffi::c_void,
        __c: std::ffi::c_int,
        __n: size_t,
    ) -> *mut std::ffi::c_void;
    fn strncmp(
        __s1: *const std::ffi::c_char,
        __s2: *const std::ffi::c_char,
        __n: size_t,
    ) -> std::ffi::c_int;
    fn strdup(__s: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strlen(__s: *const std::ffi::c_char) -> size_t;
    fn strerror(__errnum: std::ffi::c_int) -> *mut std::ffi::c_char;
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
const unsafe fn get_stdin() -> *mut FILE {
    unsafe extern "C" {
        static mut stdin: *mut FILE;
    }
    unsafe { stdin }
}
#[cfg(all(target_endian = "little", target_os = "macos", target_pointer_width = "64"))]
const unsafe fn get_stdin() -> *mut FILE {
    unsafe extern "C" {
        static mut __stdinp: *mut FILE;
    }
    unsafe { __stdinp }
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
pub type mdb_size_t = size_t;
pub type MDB_dbi = std::ffi::c_uint;
pub type MDB_cmp_func = unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> std::ffi::c_int;

use rmdb::MDB_LAST;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct flagbit {
    pub bit: std::ffi::c_int,
    pub name: *mut std::ffi::c_char,
    pub len: std::ffi::c_int,
}
pub const EXIT_FAILURE: std::ffi::c_int = 1 as std::ffi::c_int;
pub const EXIT_SUCCESS: std::ffi::c_int = 0 as std::ffi::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const std::ffi::c_char) -> std::ffi::c_int {
    unsafe {
        strtol(
            __nptr,
            std::ptr::null_mut::<std::ffi::c_void>() as *mut *mut std::ffi::c_char,
            10 as std::ffi::c_int,
        ) as std::ffi::c_int
    }
}
pub const MDB_FIXEDMAP: std::ffi::c_int = 0x1 as std::ffi::c_int;
pub const MDB_NOSUBDIR: std::ffi::c_int = 0x4000 as std::ffi::c_int;
pub const MDB_NOSYNC: std::ffi::c_int = 0x10000 as std::ffi::c_int;
pub const MDB_REVERSEKEY: std::ffi::c_int = 0x2 as std::ffi::c_int;
pub const MDB_DUPSORT: std::ffi::c_int = 0x4 as std::ffi::c_int;
pub const MDB_INTEGERKEY: std::ffi::c_int = 0x8 as std::ffi::c_int;
pub const MDB_DUPFIXED: std::ffi::c_int = 0x10 as std::ffi::c_int;
pub const MDB_INTEGERDUP: std::ffi::c_int = 0x20 as std::ffi::c_int;
pub const MDB_REVERSEDUP: std::ffi::c_int = 0x40 as std::ffi::c_int;
pub const MDB_CREATE: std::ffi::c_int = 0x40000 as std::ffi::c_int;
pub const MDB_NOOVERWRITE: std::ffi::c_int = 0x10 as std::ffi::c_int;
pub const MDB_NODUPDATA: std::ffi::c_int = 0x20 as std::ffi::c_int;
pub const MDB_CURRENT: std::ffi::c_int = 0x40 as std::ffi::c_int;
pub const MDB_APPEND: std::ffi::c_int = 0x20000 as std::ffi::c_int;
pub const MDB_APPENDDUP: std::ffi::c_int = 0x40000 as std::ffi::c_int;
pub const PRINT: std::ffi::c_int = 1 as std::ffi::c_int;
pub const NOHDR: std::ffi::c_int = 2 as std::ffi::c_int;
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
#[unsafe(no_mangle)]
pub static mut dbflags: [flagbit; 7] =
    [flagbit { bit: 0, name: 0 as *mut std::ffi::c_char, len: 0 }; 7];
unsafe extern "C" fn readhdr() {
    unsafe {
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        flags = 0 as std::ffi::c_int;
        while !(fgets(
            dbuf.mv_data as *mut std::ffi::c_char,
            dbuf.mv_size as std::ffi::c_int,
            get_stdin(),
        ))
        .is_null()
        {
            lineno = lineno.wrapping_add(1);
            if strncmp(
                dbuf.mv_data as *const std::ffi::c_char,
                b"VERSION=\0" as *const u8 as *const std::ffi::c_char,
                (::core::mem::size_of::<[std::ffi::c_char; 9]>() as size_t)
                    .wrapping_sub(1 as size_t),
            ) == 0
            {
                version = atoi(
                    (dbuf.mv_data as *mut std::ffi::c_char).offset(
                        (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                            .wrapping_sub(1 as std::ffi::c_ulong) as isize,
                    ),
                );
                if version > 3 as std::ffi::c_int {
                    fprintf(
                        get_stderr(),
                        b"%s: line %zu: unsupported VERSION %d\n\0" as *const u8
                            as *const std::ffi::c_char,
                        prog,
                        lineno,
                        version,
                    );
                    exit(EXIT_FAILURE);
                }
            } else {
                if strncmp(
                    dbuf.mv_data as *const std::ffi::c_char,
                    b"HEADER=END\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 11]>() as size_t)
                        .wrapping_sub(1 as size_t),
                ) == 0
                {
                    break;
                }
                if strncmp(
                    dbuf.mv_data as *const std::ffi::c_char,
                    b"format=\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 8]>() as size_t)
                        .wrapping_sub(1 as size_t),
                ) == 0
                {
                    if strncmp(
                        (dbuf.mv_data as *mut std::ffi::c_char).offset(
                            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                                .wrapping_sub(1 as std::ffi::c_ulong)
                                as isize,
                        ),
                        b"print\0" as *const u8 as *const std::ffi::c_char,
                        (::core::mem::size_of::<[std::ffi::c_char; 6]>() as size_t)
                            .wrapping_sub(1 as size_t),
                    ) == 0
                    {
                        mode |= PRINT;
                    } else if strncmp(
                        (dbuf.mv_data as *mut std::ffi::c_char).offset(
                            (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                                .wrapping_sub(1 as std::ffi::c_ulong)
                                as isize,
                        ),
                        b"bytevalue\0" as *const u8 as *const std::ffi::c_char,
                        (::core::mem::size_of::<[std::ffi::c_char; 10]>() as size_t)
                            .wrapping_sub(1 as size_t),
                    ) != 0
                    {
                        fprintf(
                            get_stderr(),
                            b"%s: line %zu: unsupported FORMAT %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            prog,
                            lineno,
                            (dbuf.mv_data as *mut std::ffi::c_char).offset(
                                (::core::mem::size_of::<[std::ffi::c_char; 8]>()
                                    as std::ffi::c_ulong)
                                    .wrapping_sub(1 as std::ffi::c_ulong)
                                    as isize,
                            ),
                        );
                        exit(EXIT_FAILURE);
                    }
                } else if strncmp(
                    dbuf.mv_data as *const std::ffi::c_char,
                    b"database=\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 10]>() as size_t)
                        .wrapping_sub(1 as size_t),
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
                                .wrapping_sub(1 as std::ffi::c_ulong)
                                as isize,
                        ),
                    );
                } else if strncmp(
                    dbuf.mv_data as *const std::ffi::c_char,
                    b"type=\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 6]>() as size_t)
                        .wrapping_sub(1 as size_t),
                ) == 0
                {
                    if strncmp(
                        (dbuf.mv_data as *mut std::ffi::c_char).offset(
                            (::core::mem::size_of::<[std::ffi::c_char; 6]>() as std::ffi::c_ulong)
                                .wrapping_sub(1 as std::ffi::c_ulong)
                                as isize,
                        ),
                        b"btree\0" as *const u8 as *const std::ffi::c_char,
                        (::core::mem::size_of::<[std::ffi::c_char; 6]>() as size_t)
                            .wrapping_sub(1 as size_t),
                    ) != 0
                    {
                        fprintf(
                            get_stderr(),
                            b"%s: line %zu: unsupported type %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            prog,
                            lineno,
                            (dbuf.mv_data as *mut std::ffi::c_char).offset(
                                (::core::mem::size_of::<[std::ffi::c_char; 6]>()
                                    as std::ffi::c_ulong)
                                    .wrapping_sub(1 as std::ffi::c_ulong)
                                    as isize,
                            ),
                        );
                        exit(EXIT_FAILURE);
                    }
                } else if strncmp(
                    dbuf.mv_data as *const std::ffi::c_char,
                    b"mapaddr=\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 9]>() as size_t)
                        .wrapping_sub(1 as size_t),
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
                                .wrapping_sub(1 as std::ffi::c_ulong)
                                as isize,
                        ),
                        b"%p\0" as *const u8 as *const std::ffi::c_char,
                        &raw mut info.me_mapaddr,
                    );
                    if i != 1 as std::ffi::c_int {
                        fprintf(
                            get_stderr(),
                            b"%s: line %zu: invalid mapaddr %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            prog,
                            lineno,
                            (dbuf.mv_data as *mut std::ffi::c_char).offset(
                                (::core::mem::size_of::<[std::ffi::c_char; 9]>()
                                    as std::ffi::c_ulong)
                                    .wrapping_sub(1 as std::ffi::c_ulong)
                                    as isize,
                            ),
                        );
                        exit(EXIT_FAILURE);
                    }
                } else if strncmp(
                    dbuf.mv_data as *const std::ffi::c_char,
                    b"mapsize=\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 9]>() as size_t)
                        .wrapping_sub(1 as size_t),
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
                                .wrapping_sub(1 as std::ffi::c_ulong)
                                as isize,
                        ),
                        b"%zu\0" as *const u8 as *const std::ffi::c_char,
                        &raw mut info.me_mapsize,
                    );
                    if i_0 != 1 as std::ffi::c_int {
                        fprintf(
                            get_stderr(),
                            b"%s: line %zu: invalid mapsize %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            prog,
                            lineno,
                            (dbuf.mv_data as *mut std::ffi::c_char).offset(
                                (::core::mem::size_of::<[std::ffi::c_char; 9]>()
                                    as std::ffi::c_ulong)
                                    .wrapping_sub(1 as std::ffi::c_ulong)
                                    as isize,
                            ),
                        );
                        exit(EXIT_FAILURE);
                    }
                } else if strncmp(
                    dbuf.mv_data as *const std::ffi::c_char,
                    b"maxreaders=\0" as *const u8 as *const std::ffi::c_char,
                    (::core::mem::size_of::<[std::ffi::c_char; 12]>() as size_t)
                        .wrapping_sub(1 as size_t),
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
                                .wrapping_sub(1 as std::ffi::c_ulong)
                                as isize,
                        ),
                        b"%u\0" as *const u8 as *const std::ffi::c_char,
                        &raw mut info.me_maxreaders,
                    );
                    if i_1 != 1 as std::ffi::c_int {
                        fprintf(
                            get_stderr(),
                            b"%s: line %zu: invalid maxreaders %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            prog,
                            lineno,
                            (dbuf.mv_data as *mut std::ffi::c_char).offset(
                                (::core::mem::size_of::<[std::ffi::c_char; 12]>()
                                    as std::ffi::c_ulong)
                                    .wrapping_sub(1 as std::ffi::c_ulong)
                                    as isize,
                            ),
                        );
                        exit(EXIT_FAILURE);
                    }
                } else {
                    let mut i_2: std::ffi::c_int = 0;
                    i_2 = 0 as std::ffi::c_int;
                    while dbflags[i_2 as usize].bit != 0 {
                        if strncmp(
                            dbuf.mv_data as *const std::ffi::c_char,
                            dbflags[i_2 as usize].name,
                            dbflags[i_2 as usize].len as size_t,
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
                        }
                    }
                    if dbflags[i_2 as usize].bit == 0 {
                        ptr =
                            memchr(dbuf.mv_data, '=' as i32, dbuf.mv_size) as *mut std::ffi::c_char;
                        if ptr.is_null() {
                            fprintf(
                                get_stderr(),
                                b"%s: line %zu: unexpected format\n\0" as *const u8
                                    as *const std::ffi::c_char,
                                prog,
                                lineno,
                            );
                            exit(EXIT_FAILURE);
                        } else {
                            *ptr = '\0' as i32 as std::ffi::c_char;
                            fprintf(
                                get_stderr(),
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
}
unsafe extern "C" fn badend() {
    unsafe {
        fprintf(
            get_stderr(),
            b"%s: line %zu: unexpected end of input\n\0" as *const u8 as *const std::ffi::c_char,
            prog,
            lineno,
        );
    }
}
unsafe extern "C" fn unhex(mut c2: *mut std::ffi::c_uchar) -> std::ffi::c_int {
    unsafe {
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
        c
    }
}
unsafe extern "C" fn readline(mut out: *mut MDB_val, mut buf: *mut MDB_val) -> std::ffi::c_int {
    unsafe {
        let mut c1: *mut std::ffi::c_uchar = std::ptr::null_mut::<std::ffi::c_uchar>();
        let mut c2: *mut std::ffi::c_uchar = std::ptr::null_mut::<std::ffi::c_uchar>();
        let mut end: *mut std::ffi::c_uchar = std::ptr::null_mut::<std::ffi::c_uchar>();
        let mut len: size_t = 0;
        let mut l2: size_t = 0;
        let mut c: std::ffi::c_int = 0;
        if mode & NOHDR == 0 {
            c = fgetc(get_stdin());
            if c == -(1 as std::ffi::c_int) {
                Eof = 1 as std::ffi::c_int;
                return -(1 as std::ffi::c_int);
            }
            if c != ' ' as i32 {
                lineno = lineno.wrapping_add(1);
                if !(fgets(
                    (*buf).mv_data as *mut std::ffi::c_char,
                    (*buf).mv_size as std::ffi::c_int,
                    get_stdin(),
                ))
                .is_null()
                    && c == 'D' as i32
                    && strncmp(
                        (*buf).mv_data as *const std::ffi::c_char,
                        b"ATA=END\0" as *const u8 as *const std::ffi::c_char,
                        (::core::mem::size_of::<[std::ffi::c_char; 8]>() as size_t)
                            .wrapping_sub(1 as size_t),
                    ) == 0
                {
                    return -(1 as std::ffi::c_int);
                }
                Eof = 1 as std::ffi::c_int;
                badend();
                return -(1 as std::ffi::c_int);
            }
        }
        if (fgets(
            (*buf).mv_data as *mut std::ffi::c_char,
            (*buf).mv_size as std::ffi::c_int,
            get_stdin(),
        ))
        .is_null()
        {
            Eof = 1 as std::ffi::c_int;
            return -(1 as std::ffi::c_int);
        }
        lineno = lineno.wrapping_add(1);
        c1 = (*buf).mv_data as *mut std::ffi::c_uchar;
        len = strlen(c1 as *mut std::ffi::c_char);
        l2 = len;
        while *c1.offset(len.wrapping_sub(1 as size_t) as isize) as std::ffi::c_int != '\n' as i32 {
            (*buf).mv_data = realloc((*buf).mv_data, ((*buf).mv_size).wrapping_mul(2 as size_t));
            if ((*buf).mv_data).is_null() {
                Eof = 1 as std::ffi::c_int;
                fprintf(
                    get_stderr(),
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
                ((*buf).mv_size).wrapping_add(1 as size_t) as std::ffi::c_int,
                get_stdin(),
            ))
            .is_null()
            {
                Eof = 1 as std::ffi::c_int;
                badend();
                return -(1 as std::ffi::c_int);
            }
            (*buf).mv_size = ((*buf).mv_size as std::ffi::c_ulong)
                .wrapping_mul(2 as std::ffi::c_ulong) as size_t
                as size_t;
            len = strlen(c1 as *mut std::ffi::c_char);
            l2 = (l2 as std::ffi::c_ulong).wrapping_add(len as std::ffi::c_ulong) as size_t
                as size_t;
        }
        c2 = (*buf).mv_data as *mut std::ffi::c_uchar;
        c1 = c2;
        len = l2;
        len = len.wrapping_sub(1);
        *c1.offset(len as isize) = '\0' as i32 as std::ffi::c_uchar;
        end = c1.offset(len as isize);
        if mode & PRINT != 0 {
            while c2 < end {
                if *c2 as std::ffi::c_int == '\\' as i32 {
                    if *c2.offset(1 as std::ffi::c_int as isize) as std::ffi::c_int == '\\' as i32 {
                        let fresh1 = c1;
                        c1 = c1.offset(1);
                        *fresh1 = *c2;
                    } else {
                        if c2.offset(3 as std::ffi::c_int as isize) > end
                            || !(*c2.offset(1 as std::ffi::c_int as isize)).is_ascii_hexdigit()
                            || !(*c2.offset(2 as std::ffi::c_int as isize)).is_ascii_hexdigit()
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
            if len & 1 as size_t != 0 {
                Eof = 1 as std::ffi::c_int;
                badend();
                return -(1 as std::ffi::c_int);
            }
            while c2 < end {
                if !((*c2 as std::ffi::c_int as isize) as u8).is_ascii_hexdigit()
                    || !(*c2.offset(1 as std::ffi::c_int as isize)).is_ascii_hexdigit()
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
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn usage() {
    unsafe {
        fprintf(
            get_stderr(),
            b"usage: %s [-V] [-a] [-f input] [-n] [-s name] [-N] [-T] dbpath\n\0" as *const u8
                as *const std::ffi::c_char,
            prog,
        );
        exit(EXIT_FAILURE);
    }
}
unsafe extern "C" fn greater(mut _a: *const MDB_val, mut _b: *const MDB_val) -> std::ffi::c_int {
    1
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
        let mut mc: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut dbi: MDB_dbi = 0;
        let mut envname: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut envflags: std::ffi::c_int = MDB_NOSYNC;
        let mut putflags: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut dohdr: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut append: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut prevk: MDB_val = MDB_val {
            mv_size: 0,
            mv_data: std::ptr::null::<std::ffi::c_void>() as *mut std::ffi::c_void,
        };
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
                    append = 1 as std::ffi::c_int;
                }
                102 => {
                    if (freopen(
                        optarg,
                        b"r\0" as *const u8 as *const std::ffi::c_char,
                        get_stdin(),
                    ))
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
                }
                110 => {
                    envflags |= MDB_NOSUBDIR;
                }
                115 => {
                    subname = strdup(optarg);
                }
                78 => {
                    putflags = MDB_NOOVERWRITE | MDB_NODUPDATA;
                }
                81 => {
                    envflags |= MDB_NOSYNC;
                }
                84 => {
                    mode |= NOHDR | PRINT;
                }
                _ => {
                    usage();
                }
            }
        }
        if optind != argc - 1 as std::ffi::c_int {
            usage();
        }
        dbuf.mv_size = 4096 as size_t;
        dbuf.mv_data = malloc(dbuf.mv_size);
        if mode & NOHDR == 0 {
            readhdr();
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
        mdb_env_set_maxdbs(env, 2 as MDB_dbi);
        if info.me_maxreaders != 0 {
            mdb_env_set_maxreaders(env, info.me_maxreaders);
        }
        if info.me_mapsize != 0 {
            mdb_env_set_mapsize(env, info.me_mapsize);
        }
        if !(info.me_mapaddr).is_null() {
            envflags |= MDB_FIXEDMAP;
        }
        rc = mdb_env_open(env, envname, envflags as std::ffi::c_uint, 0o664 as mdb_mode_t);
        if rc != 0 {
            fprintf(
                get_stderr(),
                b"mdb_env_open failed, error %d %s\n\0" as *const u8 as *const std::ffi::c_char,
                rc,
                mdb_strerror(rc),
            );
        } else {
            kbuf.mv_size = (mdb_env_get_maxkeysize(env) * 2 as std::ffi::c_int
                + 2 as std::ffi::c_int) as size_t;
            kbuf.mv_data = malloc((kbuf.mv_size).wrapping_mul(2 as size_t));
            k0buf.mv_size = kbuf.mv_size;
            k0buf.mv_data = (kbuf.mv_data as *mut std::ffi::c_char).offset(kbuf.mv_size as isize)
                as *mut std::ffi::c_void;
            prevk.mv_data = k0buf.mv_data;
            's_222: loop {
                if Eof != 0 {
                    current_block = 13973796120831625861;
                    break;
                }
                let mut key: MDB_val = MDB_val {
                    mv_size: 0,
                    mv_data: std::ptr::null::<std::ffi::c_void>() as *mut std::ffi::c_void,
                };
                let mut data: MDB_val = MDB_val {
                    mv_size: 0,
                    mv_data: std::ptr::null::<std::ffi::c_void>() as *mut std::ffi::c_void,
                };
                let mut batch: std::ffi::c_int = 0 as std::ffi::c_int;
                let mut appflag: std::ffi::c_int = 0;
                if dohdr == 0 {
                    dohdr = 1 as std::ffi::c_int;
                } else if mode & NOHDR == 0 {
                    readhdr();
                }
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
                    current_block = 5797318274898257760;
                    break;
                } else {
                    rc = mdb_dbi_open(
                        txn,
                        subname,
                        (flags | MDB_CREATE) as std::ffi::c_uint,
                        &mut dbi,
                    );
                    if rc != 0 {
                        fprintf(
                            get_stderr(),
                            b"mdb_dbi_open failed, error %d %s\n\0" as *const u8
                                as *const std::ffi::c_char,
                            rc,
                            mdb_strerror(rc),
                        );
                        current_block = 13973796120831625861;
                        break;
                    } else {
                        prevk.mv_size = 0 as size_t;
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
                            if flags & MDB_DUPSORT != 0 {
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
                                get_stderr(),
                                b"mdb_cursor_open failed, error %d %s\n\0" as *const u8
                                    as *const std::ffi::c_char,
                                rc,
                                mdb_strerror(rc),
                            );
                            current_block = 13973796120831625861;
                            break;
                        } else {
                            loop {
                                rc = readline(&mut key, &raw mut kbuf);
                                if rc != 0 {
                                    break;
                                }
                                rc = readline(&mut data, &raw mut dbuf);
                                if rc != 0 {
                                    fprintf(
                                        get_stderr(),
                                        b"%s: line %zu: failed to read key value\n\0" as *const u8
                                            as *const std::ffi::c_char,
                                        prog,
                                        lineno,
                                    );
                                    current_block = 13973796120831625861;
                                    break 's_222;
                                } else {
                                    if append != 0 {
                                        appflag = MDB_APPEND;
                                        if flags & MDB_DUPSORT != 0 {
                                            if prevk.mv_size == key.mv_size
                                                && memcmp(prevk.mv_data, key.mv_data, key.mv_size)
                                                    == 0
                                            {
                                                appflag = MDB_CURRENT | MDB_APPENDDUP;
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
                                            get_stderr(),
                                            b"%s: line %zu: mdb_cursor_put failed, error %d %s\n\0"
                                                as *const u8
                                                as *const std::ffi::c_char,
                                            prog,
                                            lineno,
                                            rc,
                                            mdb_strerror(rc),
                                        );
                                        current_block = 13973796120831625861;
                                        break 's_222;
                                    } else {
                                        batch += 1;
                                        if batch != 100 as std::ffi::c_int {
                                            continue;
                                        }
                                        rc = mdb_txn_commit(txn);
                                        if rc != 0 {
                                            fprintf(
                                                get_stderr(),
                                                b"%s: line %zu: txn_commit: %s\n\0" as *const u8
                                                    as *const std::ffi::c_char,
                                                prog,
                                                lineno,
                                                mdb_strerror(rc),
                                            );
                                            current_block = 5797318274898257760;
                                            break 's_222;
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
                                                    b"mdb_txn_begin failed, error %d %s\n\0"
                                                        as *const u8
                                                        as *const std::ffi::c_char,
                                                    rc,
                                                    mdb_strerror(rc),
                                                );
                                                current_block = 5797318274898257760;
                                                break 's_222;
                                            } else {
                                                rc = mdb_cursor_open(txn, dbi, &mut mc);
                                                if rc != 0 {
                                                    fprintf(
                                                        get_stderr(),
                                                        b"mdb_cursor_open failed, error %d %s\n\0"
                                                            as *const u8
                                                            as *const std::ffi::c_char,
                                                        rc,
                                                        mdb_strerror(rc),
                                                    );
                                                    current_block = 13973796120831625861;
                                                    break 's_222;
                                                } else {
                                                    if append != 0 {
                                                        let mut k: MDB_val = MDB_val {
                                                            mv_size: 0,
                                                            mv_data: std::ptr::null::<
                                                                std::ffi::c_void,
                                                            >(
                                                            )
                                                                as *mut std::ffi::c_void,
                                                        };
                                                        let mut d: MDB_val = MDB_val {
                                                            mv_size: 0,
                                                            mv_data: std::ptr::null::<
                                                                std::ffi::c_void,
                                                            >(
                                                            )
                                                                as *mut std::ffi::c_void,
                                                        };
                                                        mdb_cursor_get(
                                                            mc, &mut k, &mut d, MDB_LAST,
                                                        );
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
                            txn = std::ptr::null_mut::<MDB_txn>();
                            if rc != 0 {
                                fprintf(
                                    get_stderr(),
                                    b"%s: line %zu: txn_commit: %s\n\0" as *const u8
                                        as *const std::ffi::c_char,
                                    prog,
                                    lineno,
                                    mdb_strerror(rc),
                                );
                                current_block = 5797318274898257760;
                                break;
                            } else {
                                if envflags & MDB_NOSYNC != 0 {
                                    rc = mdb_env_sync(env, 1 as std::ffi::c_int);
                                    if rc != 0 {
                                        fprintf(
                                            get_stderr(),
                                            b"mdb_env_sync failed, error %d %s\n\0" as *const u8
                                                as *const std::ffi::c_char,
                                            rc,
                                            mdb_strerror(rc),
                                        );
                                        current_block = 5797318274898257760;
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
                5797318274898257760 => {}
                _ => {
                    mdb_txn_abort(txn);
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
unsafe extern "C" fn run_static_initializers() {
    unsafe {
        dbflags = [
            {
                flagbit {
                    bit: MDB_REVERSEKEY,
                    name: b"reversekey\0" as *const u8 as *const std::ffi::c_char
                        as *mut std::ffi::c_char,
                    len: (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_ulong)
                        as std::ffi::c_int,
                }
            },
            {
                flagbit {
                    bit: MDB_DUPSORT,
                    name: b"dupsort\0" as *const u8 as *const std::ffi::c_char
                        as *mut std::ffi::c_char,
                    len: (::core::mem::size_of::<[std::ffi::c_char; 8]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_ulong)
                        as std::ffi::c_int,
                }
            },
            {
                flagbit {
                    bit: MDB_INTEGERKEY,
                    name: b"integerkey\0" as *const u8 as *const std::ffi::c_char
                        as *mut std::ffi::c_char,
                    len: (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_ulong)
                        as std::ffi::c_int,
                }
            },
            {
                flagbit {
                    bit: MDB_DUPFIXED,
                    name: b"dupfixed\0" as *const u8 as *const std::ffi::c_char
                        as *mut std::ffi::c_char,
                    len: (::core::mem::size_of::<[std::ffi::c_char; 9]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_ulong)
                        as std::ffi::c_int,
                }
            },
            {
                flagbit {
                    bit: MDB_INTEGERDUP,
                    name: b"integerdup\0" as *const u8 as *const std::ffi::c_char
                        as *mut std::ffi::c_char,
                    len: (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_ulong)
                        as std::ffi::c_int,
                }
            },
            {
                flagbit {
                    bit: MDB_REVERSEDUP,
                    name: b"reversedup\0" as *const u8 as *const std::ffi::c_char
                        as *mut std::ffi::c_char,
                    len: (::core::mem::size_of::<[std::ffi::c_char; 11]>() as std::ffi::c_ulong)
                        .wrapping_sub(1 as std::ffi::c_ulong)
                        as std::ffi::c_int,
                }
            },
            {
                flagbit {
                    bit: 0 as std::ffi::c_int,
                    name: std::ptr::null_mut::<std::ffi::c_char>(),
                    len: 0 as std::ffi::c_int,
                }
            },
        ];
    }
}
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".CRT$XIB"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
