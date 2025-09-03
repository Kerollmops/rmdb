#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
unsafe extern "C" {
    pub type __sFILEX;
    fn fstat(_: std::ffi::c_int, _: *mut stat) -> std::ffi::c_int;
    fn writev(_: std::ffi::c_int, _: *const iovec, _: std::ffi::c_int) -> ssize_t;
    fn mmap(
        _: *mut std::ffi::c_void,
        _: size_t,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: off_t,
    ) -> *mut std::ffi::c_void;
    fn msync(_: *mut std::ffi::c_void, _: size_t, _: std::ffi::c_int) -> std::ffi::c_int;
    fn munmap(_: *mut std::ffi::c_void, _: size_t) -> std::ffi::c_int;
    fn madvise(_: *mut std::ffi::c_void, _: size_t, _: std::ffi::c_int) -> std::ffi::c_int;
    fn open(_: *const std::ffi::c_char, _: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn fcntl(_: std::ffi::c_int, _: std::ffi::c_int, _: ...) -> std::ffi::c_int;
    fn __error() -> *mut std::ffi::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn sprintf(_: *mut std::ffi::c_char, _: *const std::ffi::c_char, _: ...) -> std::ffi::c_int;
    fn malloc(_: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn calloc(_: std::ffi::c_ulong, _: std::ffi::c_ulong) -> *mut std::ffi::c_void;
    fn free(_: *mut std::ffi::c_void);
    fn posix_memalign(
        __memptr: *mut *mut std::ffi::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> std::ffi::c_int;
    fn abort() -> !;
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
    fn memmove(
        _: *mut std::ffi::c_void,
        _: *const std::ffi::c_void,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn memset(
        _: *mut std::ffi::c_void,
        _: std::ffi::c_int,
        _: std::ffi::c_ulong,
    ) -> *mut std::ffi::c_void;
    fn strcpy(_: *mut std::ffi::c_char, _: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn strerror(_: std::ffi::c_int) -> *mut std::ffi::c_char;
    fn strlen(_: *const std::ffi::c_char) -> std::ffi::c_ulong;
    fn strncmp(
        _: *const std::ffi::c_char,
        _: *const std::ffi::c_char,
        _: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
    fn strdup(_: *const std::ffi::c_char) -> *mut std::ffi::c_char;
    fn close(_: std::ffi::c_int) -> std::ffi::c_int;
    fn getpid() -> pid_t;
    fn lseek(_: std::ffi::c_int, _: off_t, _: std::ffi::c_int) -> off_t;
    fn sysconf(_: std::ffi::c_int) -> std::ffi::c_long;
    fn write(__fd: std::ffi::c_int, __buf: *const std::ffi::c_void, __nbyte: size_t) -> ssize_t;
    fn pread(
        __fd: std::ffi::c_int,
        __buf: *mut std::ffi::c_void,
        __nbyte: size_t,
        __offset: off_t,
    ) -> ssize_t;
    fn pwrite(
        __fd: std::ffi::c_int,
        __buf: *const std::ffi::c_void,
        __nbyte: size_t,
        __offset: off_t,
    ) -> ssize_t;
    fn ftruncate(_: std::ffi::c_int, _: off_t) -> std::ffi::c_int;
    fn pthread_cond_destroy(_: *mut pthread_cond_t) -> std::ffi::c_int;
    fn pthread_cond_init(_: *mut pthread_cond_t, _: *const pthread_condattr_t) -> std::ffi::c_int;
    fn pthread_cond_signal(_: *mut pthread_cond_t) -> std::ffi::c_int;
    fn pthread_cond_wait(_: *mut pthread_cond_t, _: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_create(
        _: *mut pthread_t,
        _: *const pthread_attr_t,
        _: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> *mut std::ffi::c_void>,
        _: *mut std::ffi::c_void,
    ) -> std::ffi::c_int;
    fn pthread_getspecific(_: pthread_key_t) -> *mut std::ffi::c_void;
    fn pthread_join(_: pthread_t, _: *mut *mut std::ffi::c_void) -> std::ffi::c_int;
    fn pthread_key_create(
        _: *mut pthread_key_t,
        _: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>,
    ) -> std::ffi::c_int;
    fn pthread_key_delete(_: pthread_key_t) -> std::ffi::c_int;
    fn pthread_mutex_destroy(_: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_mutex_init(
        _: *mut pthread_mutex_t,
        _: *const pthread_mutexattr_t,
    ) -> std::ffi::c_int;
    fn pthread_mutex_lock(_: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_mutex_unlock(_: *mut pthread_mutex_t) -> std::ffi::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_setspecific(_: pthread_key_t, _: *const std::ffi::c_void) -> std::ffi::c_int;
    fn pthread_sigmask(_: std::ffi::c_int, _: *const sigset_t, _: *mut sigset_t)
    -> std::ffi::c_int;
    fn sigwait(_: *const sigset_t, _: *mut std::ffi::c_int) -> std::ffi::c_int;
    fn ftok(_: *const std::ffi::c_char, _: std::ffi::c_int) -> key_t;
    fn semctl(
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: std::ffi::c_int,
        _: ...
    ) -> std::ffi::c_int;
    fn semget(_: key_t, _: std::ffi::c_int, _: std::ffi::c_int) -> std::ffi::c_int;
    fn semop(_: std::ffi::c_int, _: *mut sembuf, _: size_t) -> std::ffi::c_int;
}

use std::mem::offset_of;

use bitflags::bitflags;
use libc::ENOMEM;

use crate::MDB_cursor_op::*;
use crate::midl::*;
use crate::{MDB_CURRENT, MDB_NOTFOUND, MDB_RESERVE, MDB_SUCCESS, MDB_WRITEMAP, MDB_cursor_op};

pub type __uint16_t = std::ffi::c_ushort;
pub type __int32_t = std::ffi::c_int;
pub type __uint32_t = std::ffi::c_uint;
pub type __int64_t = std::ffi::c_longlong;
pub type __uint64_t = std::ffi::c_ulonglong;
pub type __darwin_size_t = std::ffi::c_ulong;
pub type __darwin_ssize_t = std::ffi::c_long;
pub type __darwin_time_t = std::ffi::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_uid_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: Option<unsafe extern "C" fn(*mut std::ffi::c_void) -> ()>,
    pub __arg: *mut std::ffi::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_attr_t {
    pub __sig: std::ffi::c_long,
    pub __opaque: [std::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_cond_t {
    pub __sig: std::ffi::c_long,
    pub __opaque: [std::ffi::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: std::ffi::c_long,
    pub __opaque: [std::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: std::ffi::c_long,
    pub __opaque: [std::ffi::c_char; 56],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: std::ffi::c_long,
    pub __opaque: [std::ffi::c_char; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _opaque_pthread_t {
    pub __sig: std::ffi::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [std::ffi::c_char; 8176],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = std::ffi::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type dev_t = __darwin_dev_t;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type gid_t = __darwin_gid_t;
pub type key_t = __int32_t;
pub type mode_t = __darwin_mode_t;
pub type nlink_t = __uint16_t;
pub type pid_t = __darwin_pid_t;
pub type off_t = __darwin_off_t;
pub type uid_t = __darwin_uid_t;
pub type size_t = __darwin_size_t;
pub type ssize_t = __darwin_ssize_t;
pub type time_t = __darwin_time_t;
pub type pthread_attr_t = __darwin_pthread_attr_t;
pub type pthread_cond_t = __darwin_pthread_cond_t;
pub type pthread_condattr_t = __darwin_pthread_condattr_t;
pub type pthread_mutex_t = __darwin_pthread_mutex_t;
pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
pub type pthread_t = __darwin_pthread_t;
pub type pthread_key_t = __darwin_pthread_key_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: std::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2],
}
pub type sigset_t = __darwin_sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut std::ffi::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flock {
    pub l_start: off_t,
    pub l_len: off_t,
    pub l_pid: pid_t,
    pub l_type: std::ffi::c_short,
    pub l_whence: std::ffi::c_short,
}
pub type uint16_t = std::ffi::c_ushort;
pub type uint32_t = std::ffi::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct ipc_perm {
    pub uid: uid_t,
    pub gid: gid_t,
    pub cuid: uid_t,
    pub cgid: gid_t,
    pub mode: mode_t,
    pub _seq: std::ffi::c_ushort,
    pub _key: key_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed(4))]
pub struct __semid_ds_new {
    pub sem_perm: ipc_perm,
    pub sem_base: __int32_t,
    pub sem_nsems: std::ffi::c_ushort,
    pub sem_otime: time_t,
    pub sem_pad1: __int32_t,
    pub sem_ctime: time_t,
    pub sem_pad2: __int32_t,
    pub sem_pad3: [__int32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sembuf {
    pub sem_num: std::ffi::c_ushort,
    pub sem_op: std::ffi::c_short,
    pub sem_flg: std::ffi::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union semun {
    pub val: std::ffi::c_int,
    pub buf: *mut __semid_ds_new,
    pub array: *mut std::ffi::c_ushort,
}
pub type mdb_mode_t = mode_t;
pub type mdb_size_t = size_t;
pub type mdb_filehandle_t = std::ffi::c_int;

#[derive(Clone)]
pub struct MDB_env {
    pub me_fd: std::ffi::c_int,
    pub me_lfd: std::ffi::c_int,
    pub me_mfd: std::ffi::c_int,
    pub me_flags: uint32_t,
    pub me_psize: std::ffi::c_uint,
    pub me_os_psize: std::ffi::c_uint,
    pub me_maxreaders: std::ffi::c_uint,
    pub me_close_readers: std::ffi::c_int,
    pub me_numdbs: MDB_dbi,
    pub me_maxdbs: MDB_dbi,
    pub me_pid: pid_t,
    pub me_path: *mut std::ffi::c_char,
    pub me_map: *mut std::ffi::c_char,
    pub me_txns: *mut MDB_txninfo,
    pub me_metas: [*mut MDB_meta; 2],
    pub me_pbuf: *mut std::ffi::c_void,
    pub me_txn: *mut MDB_txn,
    pub me_txn0: *mut MDB_txn,
    pub me_mapsize: mdb_size_t,
    pub me_size: off_t,
    pub me_maxpg: pgno_t,
    pub me_dbxs: *mut MDB_dbx,
    pub me_dbflags: *mut uint16_t,
    pub me_dbiseqs: *mut std::ffi::c_uint,
    pub me_txkey: pthread_key_t,
    pub me_pgoldest: txnid_t,
    pub me_pgstate: MDB_pgstate,
    pub me_dpages: *mut MDB_page,
    pub me_free_pgs: Option<MDB_IDL>,
    pub me_dirty_list: MDB_ID2L,
    pub me_maxfree_1pg: std::ffi::c_int,
    pub me_nodemax: std::ffi::c_uint,
    pub me_live_reader: std::ffi::c_int,
    pub me_rmutex: mdb_mutex_t,
    pub me_wmutex: mdb_mutex_t,
    pub me_userctx: *mut std::ffi::c_void,
    pub me_assert_func: Option<MDB_assert_func>,
}
pub type MDB_assert_func = unsafe extern "C" fn(*mut MDB_env, *const std::ffi::c_char) -> ();
pub type mdb_mutex_t = [mdb_mutex; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdb_mutex {
    pub semid: std::ffi::c_int,
    pub semnum: std::ffi::c_int,
    pub locked: *mut std::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_page {
    pub mp_p: C2RustUnnamed_1,
    pub mp_pad: uint16_t,
    pub mp_flags: u16,
    pub mp_pb: C2RustUnnamed,
    pub mp_ptrs: [indx_t; 0],
}
pub type indx_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub pb: C2RustUnnamed_0,
    pub pb_pages: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub pb_lower: indx_t,
    pub pb_upper: indx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub p_pgno: pgno_t,
    pub p_next: *mut MDB_page,
}
pub type pgno_t = MDB_ID;
#[derive(Clone)]
pub struct MDB_pgstate {
    pub mf_pghead: Option<MDB_IDL>,
    pub mf_pglast: txnid_t,
}
pub type txnid_t = MDB_ID;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_dbx {
    pub md_name: MDB_val,
    pub md_cmp: Option<MDB_cmp_func>,
    pub md_dcmp: Option<MDB_cmp_func>,
    pub md_rel: Option<MDB_rel_func>,
    pub md_relctx: *mut std::ffi::c_void,
}
pub type MDB_rel_func = unsafe extern "C" fn(
    *mut MDB_val,
    *mut std::ffi::c_void,
    *mut std::ffi::c_void,
    *mut std::ffi::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_val {
    pub mv_size: size_t,
    pub mv_data: *mut std::ffi::c_void,
}
pub type MDB_cmp_func = unsafe extern "C" fn(*const MDB_val, *const MDB_val) -> std::ffi::c_int;
#[derive(Clone)]
#[repr(C)]
pub struct MDB_txn {
    pub mt_parent: *mut MDB_txn,
    pub mt_child: *mut MDB_txn,
    pub mt_next_pgno: pgno_t,
    pub mt_txnid: txnid_t,
    pub mt_env: *mut MDB_env,
    pub mt_free_pgs: MDB_IDL,
    pub mt_loose_pgs: *mut MDB_page,
    pub mt_loose_count: std::ffi::c_int,
    pub mt_spill_pgs: Option<MDB_IDL>,
    pub mt_u: C2RustUnnamed_2,
    pub mt_dbxs: *mut MDB_dbx,
    pub mt_dbs: *mut MDB_db,
    pub mt_dbiseqs: *mut std::ffi::c_uint,
    pub mt_cursors: *mut *mut MDB_cursor,
    pub mt_dbflags: *mut std::ffi::c_uchar,
    pub mt_numdbs: MDB_dbi,
    pub mt_flags: std::ffi::c_uint,
    pub mt_dirty_room: std::ffi::c_uint,
}
pub type MDB_dbi = std::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_cursor {
    pub mc_next: *mut MDB_cursor,
    pub mc_backup: *mut MDB_cursor,
    pub mc_xcursor: *mut MDB_xcursor,
    pub mc_txn: *mut MDB_txn,
    pub mc_dbi: MDB_dbi,
    pub mc_db: *mut MDB_db,
    pub mc_dbx: *mut MDB_dbx,
    pub mc_dbflag: *mut std::ffi::c_uchar,
    pub mc_snum: std::ffi::c_ushort,
    pub mc_top: std::ffi::c_ushort,
    pub mc_flags: std::ffi::c_uint,
    pub mc_pg: [*mut MDB_page; 32],
    pub mc_ki: [indx_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_db {
    pub md_pad: uint32_t,
    pub md_flags: uint16_t,
    pub md_depth: uint16_t,
    pub md_branch_pages: pgno_t,
    pub md_leaf_pages: pgno_t,
    pub md_overflow_pages: pgno_t,
    pub md_entries: mdb_size_t,
    pub md_root: pgno_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_xcursor {
    pub mx_cursor: MDB_cursor,
    pub mx_db: MDB_db,
    pub mx_dbx: MDB_dbx,
    pub mx_dbflag: std::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub dirty_list: MDB_ID2L,
    pub reader: *mut MDB_reader,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_reader {
    pub mru: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub mrx: MDB_rxbody,
    pub pad: [std::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_rxbody {
    pub mrb_txnid: txnid_t,
    pub mrb_pid: pid_t,
    pub mrb_tid: pthread_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_meta {
    pub mm_magic: uint32_t,
    pub mm_version: uint32_t,
    pub mm_address: *mut std::ffi::c_void,
    pub mm_mapsize: mdb_size_t,
    pub mm_dbs: [MDB_db; 2],
    pub mm_last_pg: pgno_t,
    pub mm_txnid: txnid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txninfo {
    pub mt1: C2RustUnnamed_5,
    pub mt2: C2RustUnnamed_4,
    pub mti_readers: [MDB_reader; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub mt2_wlocked: std::ffi::c_int,
    pub pad: [std::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub mtb: MDB_txbody,
    pub pad: [std::ffi::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_txbody {
    pub mtb_magic: uint32_t,
    pub mtb_format: uint32_t,
    pub mtb_txnid: txnid_t,
    pub mtb_numreaders: std::ffi::c_uint,
    pub mtb_semid: std::ffi::c_int,
    pub mtb_rlocked: std::ffi::c_int,
}
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
pub type mdb_nchar_t = std::ffi::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_name {
    pub mn_len: std::ffi::c_int,
    pub mn_alloced: std::ffi::c_int,
    pub mn_val: *mut mdb_nchar_t,
}
pub type mdb_fopen_type = std::ffi::c_uint;
pub const MDB_O_LOCKS: mdb_fopen_type = 16777734;
pub const MDB_O_MASK: mdb_fopen_type = 20974083;
pub const MDB_O_COPY: mdb_fopen_type = 16779777;
pub const MDB_O_META: mdb_fopen_type = 20971521;
pub const MDB_O_RDWR: mdb_fopen_type = 514;
pub const MDB_O_RDONLY: mdb_fopen_type = 0;

const P_INVALID: pgno_t = !(0 as pgno_t);

bitflags! {
    /// Flags for the page headers.
    #[derive(Debug, Clone, Copy)]
    struct PageFlags: u16 {
        /// branch page
        const P_BRANCH   = 0x01;
        /// leaf page
        const P_LEAF     = 0x02;
        /// overflow page
        const P_OVERFLOW = 0x04;
        /// meta page
        const P_META     = 0x08;
        /// dirty page, also set for #P_SUBP pages
        const P_DIRTY    = 0x10;
        /// for #MDB_DUPFIXED records
        const P_LEAF2    = 0x20;
        /// for #MDB_DUPSORT sub-pages
        const P_SUBP     = 0x40;
        /// page was dirtied then freed, can be reused
        const P_LOOSE    = 0x4000;
        /// leave this page alone during spill
        const P_KEEP     = 0x8000;
    }
}

bitflags! {
    /// Cursor state flags.
    #[derive(Debug, Clone, Copy)]
    struct CursorFlags: u32 {
        /// cursor has been initialized and is valid
        const C_INITIALIZED     = 0x01;
        /// No more data
        const C_EOF             = 0x02;
        /// Cursor is a sub-cursor
        const C_SUB             = 0x04;
        /// last op was a cursor_del
        const C_DEL             = 0x08;
        /// Un-track cursor when closing
        const C_UNTRACK         = 0x40;
        /// Copy of txn flag
        const C_WRITEMAP        = TransactionFlags::MDB_TXN_WRITEMAP.bits();
        /// Read-only cursor into the txn's original snapshot in the map.
        ///
        /// Set for read-only txns, and in #mdb_page_alloc() for #FREE_DBI when
        /// #MDB_DEVEL & 2. Only implements code which is necessary for this.
        const C_ORIG_RDONLY     = TransactionFlags::MDB_TXN_RDONLY.bits();
    }
}

bitflags! {
    /// Transaction Flags
    #[derive(Debug, Clone, Copy)]
    struct TransactionFlags: u32 {
        /// #mdb_txn_begin() flags
        const MDB_TXN_BEGIN_FLAGS   = EnvironmentFlags::MDB_NOMETASYNC.bits() | EnvironmentFlags::MDB_NOSYNC.bits() | EnvironmentFlags::MDB_RDONLY.bits();
        /// don't sync meta for this txn on commit
        const MDB_TXN_NOMETASYNC    = EnvironmentFlags::MDB_NOMETASYNC.bits();
        /// don't sync this txn on commit
        const MDB_TXN_NOSYNC        = EnvironmentFlags::MDB_NOSYNC.bits();
        /// read-only transaction
        const MDB_TXN_RDONLY        = EnvironmentFlags::MDB_RDONLY.bits();
        // /* internal txn flags */
        /// /**< copy of #MDB_env flag in writers
        const MDB_TXN_WRITEMAP      = EnvironmentFlags::MDB_WRITEMAP.bits();
        /// txn is finished or never began
        const MDB_TXN_FINISHED      = 0x01;
        /// txn is unusable after an error
        const MDB_TXN_ERROR         = 0x02;
        /// must write, even if dirty list is empty
        const MDB_TXN_DIRTY         = 0x04;
        /// txn or a parent has spilled pages
        const MDB_TXN_SPILLS        = 0x08;
        /// txn has an #MDB_txn.%mt_child
        const MDB_TXN_HAS_CHILD     = 0x10;
        /// most operations on the txn are currently illegal
        const MDB_TXN_BLOCKED       = TransactionFlags::MDB_TXN_FINISHED.bits() | TransactionFlags::MDB_TXN_ERROR.bits() | TransactionFlags::MDB_TXN_HAS_CHILD.bits();
    }
}

bitflags! {
    /// Environment Flags
    #[derive(Debug, Clone, Copy)]
    struct EnvironmentFlags: u32 {
        /// mmap at a fixed address (experimental)
        const MDB_FIXEDMAP     = 0x01;
        /// no environment directory
        const MDB_NOSUBDIR     = 0x4000;
        /// don't fsync after commit
        const MDB_NOSYNC       = 0x10000;
        /// read only
        const MDB_RDONLY       = 0x20000;
        /// don't fsync metapage after commit
        const MDB_NOMETASYNC   = 0x40000;
        /// use writable mmap
        const MDB_WRITEMAP     = 0x80000;
        /// use asynchronous msync when #MDB_WRITEMAP is used
        const MDB_MAPASYNC     = 0x100000;
        /// tie reader locktable slots to #MDB_txn objects instead of to threads
        const MDB_NOTLS        = 0x200000;
        /// don't do any locking, caller must manage their own locks
        const MDB_NOLOCK       = 0x400000;
        /// don't do readahead (no effect on Windows)
        const MDB_NORDAHEAD    = 0x800000;
        /// don't initialize malloc'd memory before writing to datafile
        const MDB_NOMEMINIT    = 0x1000000;
        /// use the previous snapshot rather than the latest one
        const MDB_PREVSNAPSHOT = 0x2000000;
    }
}

bitflags! {
    /// Flags for node headers.
    #[derive(Debug, Clone, Copy)]
    struct NodeFlags: u32 {
        /// data put on overflow page
        const F_BIGDATA = 0x01;
        /// data is a sub-database
        const F_SUBDATA = 0x02;
        /// data has duplicates
        const F_DUPDATA = 0x04;
        /// valid flags for #mdb_node_add()
        const NODE_ADD_FLAGS = NodeFlags::F_DUPDATA.bits() | NodeFlags::F_SUBDATA.bits() | WriteFlags::MDB_RESERVE.bits() | WriteFlags::MDB_APPEND.bits();
    }
}

bitflags! {
    /// Write Flags
    #[derive(Debug, Clone, Copy)]
    struct WriteFlags: u32 {
        /// For put: Don't write if the key already exists.
        const MDB_NOOVERWRITE   = 0x10;
        /// Only for #MDB_DUPSORT
        /// For put: don't write if the key and data pair already exist.<br>
        /// For mdb_cursor_del: remove all duplicate data items.
        const MDB_NODUPDATA     = 0x20;
        /// For mdb_cursor_put: overwrite the current key/data pair */
        const MDB_CURRENT       = 0x40;
        /// For put: Just reserve space for data, don't copy it. Return a
        /// pointer to the reserved space.
        const MDB_RESERVE       = 0x10000;
        /// Data is being appended, don't split full pages.
        const MDB_APPEND        = 0x20000;
        /// Duplicate data is being appended, don't split full pages.
        const MDB_APPENDDUP     = 0x40000;
        /// Store multiple data items in one call. Only for #MDB_DUPFIXED.
        const MDB_MULTIPLE      = 0x80000;
    }
}

bitflags! {
    /// Transaction DB Flags
    #[derive(Debug, Clone, Copy)]
    struct TransactionDbFlags: u8 {
        /// DB was written in this txn
        const DB_DIRTY = 0x01;
        /// Named-DB record is older than txnID
        const DB_STALE = 0x02;
        /// Named-DB handle opened in this txn
        const DB_NEW = 0x04;
        /// DB handle is valid, see also #MDB_VALID
        const DB_VALID = 0x08;
        /// As #DB_VALID, but not set for #FREE_DBI
        const DB_USRVALID = 0x10;
        /// DB is #MDB_DUPSORT data
        const DB_DUPDATA = 0x20;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union MDB_metabuf {
    pub mb_page: MDB_page,
    pub mb_metabuf: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub mm_pad: [std::ffi::c_char; 16],
    pub mm_meta: MDB_meta,
}
pub const Size: C2RustUnnamed_7 = 152;
pub type C2RustUnnamed_7 = std::ffi::c_uint;
pub const MDB_lock_desc: C2RustUnnamed_11 = 166042;
pub const MDB_END_ABORT: C2RustUnnamed_12 = 2;

#[derive(Clone)]
pub struct MDB_ntxn {
    pub mnt_txn: MDB_txn,
    pub mnt_pgstate: MDB_pgstate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_page2 {
    pub mp2_p: [uint16_t; 4],
    pub mp2_pad: uint16_t,
    pub mp2_flags: uint16_t,
    pub mp2_lower: indx_t,
    pub mp2_upper: indx_t,
    pub mp2_ptrs: [indx_t; 0],
}
pub type mdb_mutexref_t = *mut mdb_mutex;
pub const MDB_END_FAIL_BEGIN: C2RustUnnamed_12 = 5;
pub type Pidlock_op = std::ffi::c_uint;
pub const Pidcheck: Pidlock_op = 7;
pub const Pidset: Pidlock_op = 8;
pub const MDB_END_RESET_TMP: C2RustUnnamed_12 = 4;
pub const MDB_END_FAIL_BEGINCHILD: C2RustUnnamed_12 = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdb_copy {
    pub mc_env: *mut MDB_env,
    pub mc_txn: *mut MDB_txn,
    pub mc_mutex: pthread_mutex_t,
    pub mc_cond: pthread_cond_t,
    pub mc_wbuf: [*mut std::ffi::c_char; 2],
    pub mc_over: [*mut std::ffi::c_char; 2],
    pub mc_wlen: [size_t; 2],
    pub mc_olen: [size_t; 2],
    pub mc_next_pgno: pgno_t,
    pub mc_fd: std::ffi::c_int,
    pub mc_toggle: std::ffi::c_int,
    pub mc_new: std::ffi::c_int,
    pub mc_error: std::ffi::c_int,
}
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct MDB_node {
    pub mn_lo: std::ffi::c_ushort,
    pub mn_hi: std::ffi::c_ushort,
    pub mn_flags: std::ffi::c_ushort,
    pub mn_ksize: std::ffi::c_ushort,
    pub mn_data: [std::ffi::c_char; 1],
}
pub const Align: C2RustUnnamed_8 = 8;
pub type C2RustUnnamed_8 = std::ffi::c_uint;
pub const Paranoid: C2RustUnnamed_9 = 0;
pub type C2RustUnnamed_9 = std::ffi::c_uint;
pub const Max_retries: C2RustUnnamed_9 = 2147483647;
pub const MDB_END_COMMITTED: C2RustUnnamed_12 = 0;
// pub const mask: C2RustUnnamed_10 = 49232;
pub type C2RustUnnamed_10 = std::ffi::c_uint;
pub const MDB_END_EMPTY_COMMIT: C2RustUnnamed_12 = 1;
pub const MDB_END_RESET: C2RustUnnamed_12 = 3;
pub type MDB_msg_func =
    unsafe extern "C" fn(*const std::ffi::c_char, *mut std::ffi::c_void) -> std::ffi::c_int;
pub type C2RustUnnamed_11 = std::ffi::c_uint;
pub type C2RustUnnamed_12 = std::ffi::c_uint;
#[inline(always)]
unsafe extern "C" fn __sigbits(mut __signo: std::ffi::c_int) -> std::ffi::c_int {
    if __signo > 32 as std::ffi::c_int {
        0 as std::ffi::c_int
    } else {
        (1 as std::ffi::c_int) << (__signo - 1 as std::ffi::c_int)
    }
}
unsafe extern "C" fn mdb_sem_wait(mut sem: mdb_mutexref_t) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0;
        let mut locked: *mut std::ffi::c_int = (*sem).locked;
        let mut sb: sembuf = {
            sembuf {
                sem_num: 0 as std::ffi::c_int as std::ffi::c_ushort,
                sem_op: -(1 as std::ffi::c_int) as std::ffi::c_short,
                sem_flg: 0o10000 as std::ffi::c_int as std::ffi::c_short,
            }
        };
        sb.sem_num = (*sem).semnum as std::ffi::c_ushort;
        loop {
            if semop((*sem).semid, &mut sb, 1 as std::ffi::c_int as size_t) == 0 {
                rc = if *locked != 0 {
                    -(30779 as std::ffi::c_int) + 11 as std::ffi::c_int
                } else {
                    0 as std::ffi::c_int
                };
                *locked = 1 as std::ffi::c_int;
                break;
            } else {
                rc = *__error();
                if rc != 4 as std::ffi::c_int {
                    break;
                }
            }
        }
        rc
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_version(
    mut major: *mut std::ffi::c_int,
    mut minor: *mut std::ffi::c_int,
    mut patch: *mut std::ffi::c_int,
) -> *mut std::ffi::c_char {
    unsafe {
        if !major.is_null() {
            *major = 0 as std::ffi::c_int;
        }
        if !minor.is_null() {
            *minor = 9 as std::ffi::c_int;
        }
        if !patch.is_null() {
            *patch = 70 as std::ffi::c_int;
        }
        b"LMDB 0.9.70: (December 19, 2015)\0" as *const u8 as *const std::ffi::c_char
            as *mut std::ffi::c_char
    }
}
static mut mdb_errstr: [*mut std::ffi::c_char; 21] = [
    b"MDB_KEYEXIST: Key/data pair already exists\0" as *const u8 as *const std::ffi::c_char
        as *mut std::ffi::c_char,
    b"MDB_NOTFOUND: No matching key/data pair found\0" as *const u8 as *const std::ffi::c_char
        as *mut std::ffi::c_char,
    b"MDB_PAGE_NOTFOUND: Requested page not found\0" as *const u8 as *const std::ffi::c_char
        as *mut std::ffi::c_char,
    b"MDB_CORRUPTED: Located page was wrong type\0" as *const u8 as *const std::ffi::c_char
        as *mut std::ffi::c_char,
    b"MDB_PANIC: Update of meta page failed or environment had fatal error\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_VERSION_MISMATCH: Database environment version mismatch\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_INVALID: File is not an LMDB file\0" as *const u8 as *const std::ffi::c_char
        as *mut std::ffi::c_char,
    b"MDB_MAP_FULL: Environment mapsize limit reached\0" as *const u8 as *const std::ffi::c_char
        as *mut std::ffi::c_char,
    b"MDB_DBS_FULL: Environment maxdbs limit reached\0" as *const u8 as *const std::ffi::c_char
        as *mut std::ffi::c_char,
    b"MDB_READERS_FULL: Environment maxreaders limit reached\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_TLS_FULL: Thread-local storage keys full - too many environments open\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_TXN_FULL: Transaction has too many dirty pages - transaction too big\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_CURSOR_FULL: Internal error - cursor stack limit reached\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_PAGE_FULL: Internal error - page has no more space\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_MAP_RESIZED: Database contents grew beyond environment mapsize\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_INCOMPATIBLE: Operation and DB incompatible, or DB flags changed\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_BAD_RSLOT: Invalid reuse of reader locktable slot\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_BAD_TXN: Transaction must abort, has a child, or is invalid\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_BAD_VALSIZE: Unsupported size of key/DB name/data, or wrong DUPFIXED size\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_BAD_DBI: The specified DBI handle was closed/changed unexpectedly\0" as *const u8
        as *const std::ffi::c_char as *mut std::ffi::c_char,
    b"MDB_PROBLEM: Unexpected problem - txn should abort\0" as *const u8 as *const std::ffi::c_char
        as *mut std::ffi::c_char,
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_strerror(mut err: std::ffi::c_int) -> *mut std::ffi::c_char {
    unsafe {
        let mut i: std::ffi::c_int = 0;
        if err == 0 {
            return b"Successful return: 0\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char;
        }
        if err >= -(30799 as std::ffi::c_int) && err <= -(30779 as std::ffi::c_int) {
            i = err - -(30799 as std::ffi::c_int);
            return mdb_errstr[i as usize];
        }
        if err < 0 as std::ffi::c_int {
            return b"Invalid error code\0" as *const u8 as *const std::ffi::c_char
                as *mut std::ffi::c_char;
        }
        strerror(err)
    }
}
#[cold]
unsafe extern "C" fn mdb_assert_fail(
    mut env: *mut MDB_env,
    mut expr_txt: *const std::ffi::c_char,
    mut func: *const std::ffi::c_char,
    mut file: *const std::ffi::c_char,
    mut line: std::ffi::c_int,
) {
    unsafe {
        let mut buf: [std::ffi::c_char; 400] = [0; 400];
        sprintf(
            buf.as_mut_ptr(),
            b"%.100s:%d: Assertion '%.200s' failed in %.40s()\0" as *const u8
                as *const std::ffi::c_char,
            file,
            line,
            expr_txt,
            func,
        );
        if ((*env).me_assert_func).is_some() {
            ((*env).me_assert_func).expect("non-null function pointer")(env, buf.as_mut_ptr());
        }
        fprintf(__stderrp, b"%s\n\0" as *const u8 as *const std::ffi::c_char, buf.as_mut_ptr());
        abort();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cmp(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> std::ffi::c_int {
    unsafe {
        ((*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp).expect("non-null function pointer")(a, b)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_dcmp(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> std::ffi::c_int {
    #[allow(unpredictable_function_pointer_comparisons)]
    unsafe {
        let mut dcmp: Option<MDB_cmp_func> = (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
        // It's NEED_CMP_CLONG
        if (0xffffffff as std::ffi::c_uint as std::ffi::c_ulong)
            < 18446744073709551615 as std::ffi::c_ulong
            && dcmp == Some(mdb_cmp_int as MDB_cmp_func)
            && (*a).mv_size == ::core::mem::size_of::<mdb_size_t>() as std::ffi::c_ulong
        {
            dcmp = Some(mdb_cmp_cint as MDB_cmp_func);
        }
        dcmp.expect("non-null function pointer")(a, b)
    }
}
unsafe extern "C" fn mdb_page_malloc(
    mut txn: *mut MDB_txn,
    mut num: std::ffi::c_uint,
) -> *mut MDB_page {
    unsafe {
        let mut env: *mut MDB_env = (*txn).mt_env;
        let mut ret: *mut MDB_page = (*env).me_dpages;
        let mut psize: size_t = (*env).me_psize as size_t;
        let mut sz: size_t = psize;
        let mut off: size_t = 0;
        if num == 1 as std::ffi::c_int as std::ffi::c_uint {
            if !ret.is_null() {
                (*env).me_dpages = (*ret).mp_p.p_next;
                return ret;
            }
            off = 16 as std::ffi::c_ulong as std::ffi::c_uint as size_t;
            psize = psize.wrapping_sub(off);
        } else {
            sz *= num as size_t;
            off = sz.wrapping_sub(psize);
        }
        ret = malloc(sz) as *mut MDB_page;
        if !ret.is_null() {
            if (*env).me_flags & 0x1000000 as std::ffi::c_int as uint32_t == 0 {
                memset(
                    (ret as *mut std::ffi::c_char).offset(off as isize) as *mut std::ffi::c_void,
                    0 as std::ffi::c_int,
                    psize,
                );
                (*ret).mp_pad = 0 as std::ffi::c_int as uint16_t;
            }
        } else {
            (*txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
        }
        ret
    }
}
unsafe extern "C" fn mdb_page_free(mut env: *mut MDB_env, mut mp: *mut MDB_page) {
    unsafe {
        (*mp).mp_p.p_next = (*env).me_dpages;
        (*env).me_dpages = mp;
    }
}
unsafe extern "C" fn mdb_dpage_free(mut env: *mut MDB_env, mut dp: *mut MDB_page) {
    unsafe {
        if ((*(dp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x4 as std::ffi::c_int
            != 0x4 as std::ffi::c_int)
            || (*dp).mp_pb.pb_pages == 1 as std::ffi::c_int as uint32_t
        {
            mdb_page_free(env, dp);
        } else {
            free(dp as *mut std::ffi::c_void);
        };
    }
}
unsafe extern "C" fn mdb_dlist_free(mut txn: *mut MDB_txn) {
    unsafe {
        let mut env: *mut MDB_env = (*txn).mt_env;
        let mut dl: MDB_ID2L = (*txn).mt_u.dirty_list;
        let mut i: std::ffi::c_uint = 0;
        let mut n: std::ffi::c_uint =
            (*dl.offset(0 as std::ffi::c_int as isize)).mid as std::ffi::c_uint;
        i = 1 as std::ffi::c_int as std::ffi::c_uint;
        while i <= n {
            mdb_dpage_free(env, (*dl.offset(i as isize)).mptr as *mut MDB_page);
            i = i.wrapping_add(1);
        }
        (*dl.offset(0 as std::ffi::c_int as isize)).mid = 0 as std::ffi::c_int as MDB_ID;
    }
}
unsafe extern "C" fn mdb_page_loose(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> std::ffi::c_int {
    unsafe {
        let mut loose: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut pgno: pgno_t = (*mp).mp_p.p_pgno;
        let mut txn: *mut MDB_txn = (*mc).mc_txn;
        if (*mp).mp_flags as std::ffi::c_int & 0x10 as std::ffi::c_int != 0
            && (*mc).mc_dbi != 0 as std::ffi::c_int as MDB_dbi
        {
            if !((*txn).mt_parent).is_null() {
                let mut dl: *mut MDB_ID2 = (*txn).mt_u.dirty_list;
                if (*dl.offset(0 as std::ffi::c_int as isize)).mid != 0 {
                    let mut x: std::ffi::c_uint = mdb_mid2l_search(dl, pgno);
                    if x as MDB_ID <= (*dl.offset(0 as std::ffi::c_int as isize)).mid
                        && (*dl.offset(x as isize)).mid == pgno
                    {
                        if mp != (*dl.offset(x as isize)).mptr as *mut MDB_page {
                            (*mc).mc_flags &= !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                as std::ffi::c_uint;
                            (*txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                            return -(30779 as std::ffi::c_int);
                        }
                        loose = 1 as std::ffi::c_int;
                    }
                }
            } else {
                loose = 1 as std::ffi::c_int;
            }
        }
        if loose != 0 {
            let fresh0 = &mut (*(mp.offset(2 as std::ffi::c_int as isize) as *mut *mut MDB_page));
            *fresh0 = (*txn).mt_loose_pgs;
            (*txn).mt_loose_pgs = mp;
            (*txn).mt_loose_count += 1;
            (*mp).mp_flags =
                ((*mp).mp_flags as std::ffi::c_int | 0x4000 as std::ffi::c_int) as uint16_t;
        } else {
            let mut rc: std::ffi::c_int = mdb_midl_append(&mut (*txn).mt_free_pgs, pgno);
            if rc != 0 {
                return rc;
            }
        }
        0 as std::ffi::c_int
    }
}

/// Set or clear P_KEEP in dirty, non-overflow, non-sub pages watched by txn.
///
/// @param[in] mc A cursor handle for the current operation.
/// @param[in] pflags Flags of the pages to update:
/// P_DIRTY to set P_KEEP, P_DIRTY|P_KEEP to clear it.
/// @param[in] all No shortcuts. Needed except after a full #mdb_page_flush().
/// @return 0 on success, non-zero on failure.
unsafe extern "C" fn mdb_pages_xkeep(
    mut mc: *mut MDB_cursor,
    mut pflags: u32,
    mut all: i32,
) -> i32 {
    unsafe {
        let mask = PageFlags::P_SUBP | PageFlags::P_DIRTY | PageFlags::P_LOOSE | PageFlags::P_KEEP;

        let mut txn: *mut MDB_txn = (*mc).mc_txn;
        let mut m3: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut m0: *mut MDB_cursor = mc;
        let mut mx: *mut MDB_xcursor = std::ptr::null_mut::<MDB_xcursor>();
        let mut dp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut i = 0;
        let mut j = 0;
        let mut rc = MDB_SUCCESS;
        let mut level = 0;

        // Mark pages seen by cursors: First m0, then tracked cursors
        i = (*txn).mt_numdbs;
        'outer: loop {
            if (*mc).mc_flags & CursorFlags::C_INITIALIZED.bits() as u32 != 0 {
                m3 = mc;
                loop {
                    mp = std::ptr::null_mut::<MDB_page>();
                    j = 0;
                    while j < (*m3).mc_snum as u32 {
                        mp = (*m3).mc_pg[j as usize];
                        if ((*mp).mp_flags & mask.bits()) as u32 == pflags {
                            (*mp).mp_flags ^= PageFlags::P_KEEP.bits();
                        }
                        j += 1;
                    }
                    mx = (*m3).mc_xcursor;
                    // Proceed to mx if it is at a sub-database
                    if !(!mx.is_null()
                        && (*mx).mx_cursor.mc_flags & CursorFlags::C_INITIALIZED.bits() as u32 != 0)
                    {
                        break;
                    }
                    if !(!mp.is_null() && (*mp).mp_flags & PageFlags::P_LEAF.bits() != 0) {
                        break;
                    }
                    // leaf = NODEPTR(mp, m3->mc_ki[j-1]);
                    leaf = (mp as *mut std::ffi::c_char)
                        .offset(
                            *((*(mp as *mut MDB_page2)).mp2_ptrs)
                                .as_mut_ptr()
                                .offset((*m3).mc_ki[j.wrapping_sub(1) as usize] as isize)
                                as std::ffi::c_int as isize,
                        )
                        .offset((if 0 as std::ffi::c_int != 0 { 16 } else { 0 }) as isize)
                        as *mut MDB_node;
                    if (*leaf).mn_flags as u32 & NodeFlags::F_SUBDATA.bits() == 0 {
                        break;
                    }
                    m3 = &mut (*mx).mx_cursor;
                }
            }

            mc = (*mc).mc_next;
            while mc.is_null() || mc == m0 {
                if i == 0 {
                    break 'outer;
                }
                i -= i;
                mc = *((*txn).mt_cursors).offset(i as isize);
            }
        }

        if all != 0 {
            // Mark dirty root pages
            i = 0;
            while i < (*txn).mt_numdbs {
                if *((*txn).mt_dbflags).offset(i as isize) & TransactionDbFlags::DB_DIRTY.bits()
                    != 0
                {
                    let pgno = (*((*txn).mt_dbs).offset(i as isize)).md_root;
                    if pgno != P_INVALID {
                        rc = mdb_page_get(m0, pgno, &mut dp, &mut level);
                        if rc != 0 {
                            break;
                        }
                        if ((*dp).mp_flags & mask.bits()) as u32 == pflags && level <= 1 {
                            (*dp).mp_flags ^= PageFlags::P_KEEP.bits();
                        }
                    }
                }
                i += 1;
            }
        }

        return rc;
    }
}
unsafe extern "C" fn mdb_page_spill(
    mut m0: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut txn: *mut MDB_txn = (*m0).mc_txn;
        let mut dp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut dl: MDB_ID2L = (*txn).mt_u.dirty_list;
        let mut i: std::ffi::c_uint = 0;
        let mut j: std::ffi::c_uint = 0;
        let mut need: std::ffi::c_uint = 0;
        let mut rc: std::ffi::c_int = 0;
        if (*m0).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
            return 0 as std::ffi::c_int;
        }
        i = (*(*m0).mc_db).md_depth as std::ffi::c_uint;
        if (*m0).mc_dbi >= 2 as std::ffi::c_int as MDB_dbi {
            i = i.wrapping_add(
                (*((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize)).md_depth
                    as std::ffi::c_uint,
            );
        }
        if !key.is_null() {
            i = (i as std::ffi::c_ulong).wrapping_add(
                (8 as std::ffi::c_ulong)
                    .wrapping_add((*key).mv_size)
                    .wrapping_add((*data).mv_size)
                    .wrapping_add((*(*txn).mt_env).me_psize as std::ffi::c_ulong)
                    .wrapping_div((*(*txn).mt_env).me_psize as std::ffi::c_ulong),
            ) as std::ffi::c_uint as std::ffi::c_uint;
        }
        i = i.wrapping_add(i);
        need = i;
        if (*txn).mt_dirty_room > i {
            return 0 as std::ffi::c_int;
        }
        match (*txn).mt_spill_pgs.as_mut() {
            None => {
                (*txn).mt_spill_pgs = mdb_midl_alloc(((1) << (16 + 1)) - 1);
                if (*txn).mt_spill_pgs.is_none() {
                    return ENOMEM;
                }
            }
            Some(sl) => {
                /* purge deleted slots */
                sl.retain(|id| !(id & 1) == 0);
            }
        }

        rc = mdb_pages_xkeep(m0, 0x10 as std::ffi::c_int as std::ffi::c_uint, 1 as std::ffi::c_int);
        if rc == 0 as std::ffi::c_int {
            if need
                < ((((1 as std::ffi::c_int) << (16 as std::ffi::c_int + 1 as std::ffi::c_int))
                    - 1 as std::ffi::c_int)
                    / 8 as std::ffi::c_int) as std::ffi::c_uint
            {
                need = ((((1 as std::ffi::c_int)
                    << (16 as std::ffi::c_int + 1 as std::ffi::c_int))
                    - 1 as std::ffi::c_int)
                    / 8 as std::ffi::c_int) as std::ffi::c_uint;
            }
            i = (*dl.offset(0 as std::ffi::c_int as isize)).mid as std::ffi::c_uint;
            loop {
                if !(i != 0 && need != 0) {
                    current_block = 980989089337379490;
                    break;
                }
                let mut pn: MDB_ID = (*dl.offset(i as isize)).mid << 1 as std::ffi::c_int;
                dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                if (*dp).mp_flags & (PageFlags::P_LOOSE.bits() | PageFlags::P_KEEP.bits()) == 0 {
                    if !((*txn).mt_parent).is_null() {
                        let mut tx2: *mut MDB_txn = std::ptr::null_mut::<MDB_txn>();
                        tx2 = (*txn).mt_parent;
                        while !tx2.is_null() {
                            // if (tx2->mt_spill_pgs) {
                            //     j = mdb_midl_search(tx2->mt_spill_pgs, pn);
                            //     if (j <= tx2->mt_spill_pgs[0] && tx2->mt_spill_pgs[j] == pn) {
                            //         dp->mp_flags |= P_KEEP;
                            //         break;
                            //     }
                            // }
                            if let Some(spill_pgs) = (*tx2).mt_spill_pgs.as_mut() {
                                j = mdb_midl_search(spill_pgs, pn);
                                if j as usize <= spill_pgs.len() && spill_pgs[j as usize] == pn {
                                    (*dp).mp_flags |= PageFlags::P_KEEP.bits();
                                    break;
                                }
                            }
                            tx2 = (*tx2).mt_parent;
                        }
                        if !tx2.is_null() {
                            current_block = 13472856163611868459;
                        } else {
                            current_block = 8180496224585318153;
                        }
                    } else {
                        current_block = 8180496224585318153;
                    }
                    match current_block {
                        13472856163611868459 => {}
                        _ => {
                            rc = mdb_midl_append((*txn).mt_spill_pgs.as_mut().unwrap(), pn);
                            if rc != 0 {
                                current_block = 17826759996572660473;
                                break;
                            }
                            need = need.wrapping_sub(1);
                        }
                    }
                }
                i = i.wrapping_sub(1);
            }
            match current_block {
                17826759996572660473 => {}
                _ => {
                    mdb_midl_sort((*txn).mt_spill_pgs.as_mut().unwrap());
                    rc = mdb_page_flush(txn, i as std::ffi::c_int);
                    if rc == 0 as std::ffi::c_int {
                        rc = mdb_pages_xkeep(
                            m0,
                            (0x10 as std::ffi::c_int | 0x8000 as std::ffi::c_int)
                                as std::ffi::c_uint,
                            i as std::ffi::c_int,
                        );
                    }
                }
            }
        }
        (*txn).mt_flags |= (if rc != 0 { 0x2 as std::ffi::c_int } else { 0x8 as std::ffi::c_int })
            as std::ffi::c_uint;
        rc
    }
}
unsafe extern "C" fn mdb_find_oldest(mut txn: *mut MDB_txn) -> txnid_t {
    unsafe {
        let mut i: std::ffi::c_int = 0;
        let mut mr: txnid_t = 0;
        let mut oldest: txnid_t = ((*txn).mt_txnid).wrapping_sub(1 as std::ffi::c_int as txnid_t);
        if !((*(*txn).mt_env).me_txns).is_null() {
            let mut r: *mut MDB_reader = ((*(*(*txn).mt_env).me_txns).mti_readers).as_mut_ptr();
            i = (*(*(*txn).mt_env).me_txns).mt1.mtb.mtb_numreaders as std::ffi::c_int;
            loop {
                i -= 1;
                if i < 0 as std::ffi::c_int {
                    break;
                }
                if (*r.offset(i as isize)).mru.mrx.mrb_pid != 0 {
                    mr = (*r.offset(i as isize)).mru.mrx.mrb_txnid;
                    if oldest > mr {
                        oldest = mr;
                    }
                }
            }
        }
        oldest
    }
}
unsafe extern "C" fn mdb_page_dirty(mut txn: *mut MDB_txn, mut mp: *mut MDB_page) {
    unsafe {
        let mut mid: MDB_ID2 = MDB_ID2 { mid: 0, mptr: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut rc: std::ffi::c_int = 0;
        let mut insert: Option<unsafe fn(MDB_ID2L, *mut MDB_ID2) -> std::ffi::c_int> = None;
        if (*txn).mt_flags & 0x80000 as std::ffi::c_int as std::ffi::c_uint != 0 {
            insert = Some(mdb_mid2l_append as unsafe fn(MDB_ID2L, *mut MDB_ID2) -> std::ffi::c_int);
        } else {
            insert = Some(mdb_mid2l_insert as unsafe fn(MDB_ID2L, *mut MDB_ID2) -> std::ffi::c_int);
        }
        mid.mid = (*mp).mp_p.p_pgno;
        mid.mptr = mp as *mut std::ffi::c_void;
        rc = insert.expect("non-null function pointer")((*txn).mt_u.dirty_list, &mut mid);
        if rc == 0 as std::ffi::c_int {
        } else {
            mdb_assert_fail(
                (*txn).mt_env,
                b"rc == 0\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 15], &[std::ffi::c_char; 15]>(
                    b"mdb_page_dirty\0",
                ))
                .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                2471 as std::ffi::c_int,
            );
        };
        (*txn).mt_dirty_room = ((*txn).mt_dirty_room).wrapping_sub(1);
    }
}

/// Allocate page numbers and memory for writing.  Maintain me_pglast,
/// me_pghead and mt_next_pgno.  Set #MDB_TXN_ERROR on failure.
///
/// If there are free pages available from older transactions, they
/// are re-used first. Otherwise allocate a new page at mt_next_pgno.
/// Do not modify the freedB, just merge freeDB records into me_pghead[]
/// and move me_pglast to say which records were consumed.  Only this
/// function can create me_pghead and move me_pglast/mt_next_pgno.
/// When #MDB_DEVEL & 2, it is not affected by #mdb_freelist_save(): it
/// then uses the transaction's original snapshot of the freeDB.
/// @param[in] mc cursor A cursor handle identifying the transaction and
///	database for which we are allocating.
/// @param[in] num the number of pages to allocate.
/// @param[out] mp Address of the allocated page(s). Requests for multiple pages
///  will always be satisfied by a single contiguous chunk of memory.
/// @return 0 on success, non-zero on failure.
unsafe extern "C" fn mdb_page_alloc(
    mut mc: *mut MDB_cursor,
    mut num: std::ffi::c_int,
    mut mp: *mut *mut MDB_page,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut rc: std::ffi::c_int = 0;
        let mut retry: std::ffi::c_int = num * 60 as std::ffi::c_int;
        let mut txn: *mut MDB_txn = (*mc).mc_txn;
        let mut env: *mut MDB_env = (*txn).mt_env;
        let mut pgno: pgno_t = 0;
        let mut mop: Option<&mut MDB_IDL> = (*env).me_pgstate.mf_pghead.as_mut();
        let mut i: std::ffi::c_uint = 0;
        let mut j: std::ffi::c_uint = 0;
        let mut mop_len: std::ffi::c_uint = mop.as_ref().map_or(0, |m| m.len() as u32);
        let mut n2: u32 = (num as u32).checked_sub(1).unwrap();
        let mut np: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut oldest: txnid_t = 0 as std::ffi::c_int as txnid_t;
        let mut last: txnid_t = 0;
        let mut op: MDB_cursor_op = MDB_FIRST;
        let mut m2: MDB_cursor = MDB_cursor {
            mc_next: std::ptr::null_mut::<MDB_cursor>(),
            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
            mc_txn: std::ptr::null_mut::<MDB_txn>(),
            mc_dbi: 0,
            mc_db: std::ptr::null_mut::<MDB_db>(),
            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
            mc_ki: [0; 32],
        };
        let mut found_old: std::ffi::c_int = 0 as std::ffi::c_int;
        if num == 1 as std::ffi::c_int && !((*txn).mt_loose_pgs).is_null() {
            np = (*txn).mt_loose_pgs;
            (*txn).mt_loose_pgs = *(np.offset(2 as std::ffi::c_int as isize) as *mut *mut MDB_page);
            (*txn).mt_loose_count -= 1;
            *mp = np;
            return 0 as std::ffi::c_int;
        }
        *mp = std::ptr::null_mut::<MDB_page>();
        if (*txn).mt_dirty_room == 0 as std::ffi::c_int as std::ffi::c_uint {
            rc = -(30788 as std::ffi::c_int);
        } else {
            op = MDB_FIRST;
            's_69: loop {
                let mut key: MDB_val =
                    MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
                let mut data: MDB_val =
                    MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
                let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                let mut idl: *mut pgno_t = std::ptr::null_mut::<pgno_t>();
                if let Some(mop) = mop.as_mut()
                    && mop_len > n2
                {
                    i = mop_len;
                    loop {
                        pgno = mop[i as usize];
                        if mop[i.wrapping_sub(n2) as usize] == pgno.wrapping_add(n2 as pgno_t) {
                            current_block = 11643083498027723715;
                            break 's_69;
                        }
                        i = i.wrapping_sub(1);
                        if i <= n2 {
                            break;
                        }
                    }
                    retry -= 1;
                    if retry < 0 as std::ffi::c_int {
                        current_block = 6721012065216013753;
                        break;
                    }
                }
                if op as std::ffi::c_uint == MDB_FIRST as std::ffi::c_int as std::ffi::c_uint {
                    last = (*env).me_pgstate.mf_pglast;
                    oldest = (*env).me_pgoldest;
                    mdb_cursor_init(
                        &mut m2,
                        txn,
                        0 as std::ffi::c_int as MDB_dbi,
                        std::ptr::null_mut::<MDB_xcursor>(),
                    );
                    if last != 0 {
                        op = MDB_SET_RANGE;
                        key.mv_data = &mut last as *mut txnid_t as *mut std::ffi::c_void;
                        key.mv_size = ::core::mem::size_of::<txnid_t>() as std::ffi::c_ulong;
                    }
                    if Paranoid as std::ffi::c_int != 0
                        && (*mc).mc_dbi == 0 as std::ffi::c_int as MDB_dbi
                    {
                        retry = -(1 as std::ffi::c_int);
                    }
                }
                if Paranoid as std::ffi::c_int != 0 && retry < 0 as std::ffi::c_int && mop_len != 0
                {
                    current_block = 6721012065216013753;
                    break;
                }
                last = last.wrapping_add(1);
                if oldest <= last {
                    if found_old == 0 {
                        oldest = mdb_find_oldest(txn);
                        (*env).me_pgoldest = oldest;
                        found_old = 1 as std::ffi::c_int;
                    }
                    if oldest <= last {
                        current_block = 6721012065216013753;
                        break;
                    }
                }
                rc = mdb_cursor_get(&mut m2, &mut key, std::ptr::null_mut::<MDB_val>(), op);
                if rc != 0 {
                    if rc == -(30798 as std::ffi::c_int) {
                        current_block = 6721012065216013753;
                        break;
                    } else {
                        current_block = 11154531721185249356;
                        break;
                    }
                } else {
                    last = *(key.mv_data as *mut txnid_t);
                    if oldest <= last {
                        if found_old == 0 {
                            oldest = mdb_find_oldest(txn);
                            (*env).me_pgoldest = oldest;
                            found_old = 1 as std::ffi::c_int;
                        }
                        if oldest <= last {
                            current_block = 6721012065216013753;
                            break;
                        }
                    }
                    np = m2.mc_pg[m2.mc_top as usize];
                    leaf = (np as *mut std::ffi::c_char)
                        .offset(
                            *((*(np as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                .as_mut_ptr()
                                .offset(m2.mc_ki[m2.mc_top as usize] as isize)
                                as std::ffi::c_int as isize,
                        )
                        .offset(
                            (if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    rc = mdb_node_read(&mut m2, leaf, &mut data);
                    if rc != 0 as std::ffi::c_int {
                        current_block = 11154531721185249356;
                        break;
                    }
                    idl = data.mv_data as *mut MDB_ID;
                    i = *idl.offset(0 as std::ffi::c_int as isize) as std::ffi::c_uint;
                    if mop.is_none() {
                        (*env).me_pgstate.mf_pghead = mdb_midl_alloc(i as _);
                        mop = (*env).me_pgstate.mf_pghead.as_mut();
                        if (*env).me_pgstate.mf_pghead.is_none() {
                            rc = ENOMEM;
                            current_block = 11154531721185249356;
                            break;
                        }
                    } else {
                        rc = mdb_midl_need(
                            (*env).me_pgstate.mf_pghead.as_mut().unwrap(),
                            i as usize,
                        );
                        if rc != 0 {
                            current_block = 11154531721185249356;
                            break;
                        }
                        mop = (*env).me_pgstate.mf_pghead.as_mut();
                    }
                    (*env).me_pgstate.mf_pglast = last;
                    // Merge in descending sorted order
                    mdb_midl_xmerge_ref(mop.as_mut().unwrap(), MDB_IDLRef::from_pointer(idl));
                    mop_len = mop.as_ref().map_or(0, |m| m.len() as u32);
                    op = MDB_NEXT;
                }
            }
            match current_block {
                11154531721185249356 => {}
                _ => {
                    if current_block == 6721012065216013753 {
                        i = 0 as std::ffi::c_int as std::ffi::c_uint;
                        pgno = (*txn).mt_next_pgno;
                        if pgno.wrapping_add(num as pgno_t) >= (*env).me_maxpg {
                            rc = -(30792 as std::ffi::c_int);
                            current_block = 11154531721185249356;
                        } else {
                            current_block = 11643083498027723715;
                        }
                    }
                    match current_block {
                        11154531721185249356 => {}
                        _ => {
                            if (*env).me_flags & 0x80000 as std::ffi::c_int as uint32_t != 0 {
                                np = ((*env).me_map)
                                    .offset(((*env).me_psize as pgno_t * pgno) as isize)
                                    as *mut MDB_page;
                                current_block = 16779030619667747692;
                            } else {
                                np = mdb_page_malloc(txn, num as std::ffi::c_uint);
                                if np.is_null() {
                                    rc = 12 as std::ffi::c_int;
                                    current_block = 11154531721185249356;
                                } else {
                                    current_block = 16779030619667747692;
                                }
                            }
                            match current_block {
                                11154531721185249356 => {}
                                _ => {
                                    if let Some(mop) = mop.as_mut()
                                        && i != 0
                                    {
                                        mop_len = mop_len.wrapping_sub(num as u32);
                                        // *mop.offset(0 as isize) = mop_len as pgno_t;
                                        // Note: I do a truncate just after the loop this way
                                        //       I can continue to access the mop elements freely.
                                        /* Move any stragglers down */
                                        j = i.wrapping_sub(num as u32);
                                        while j < mop_len {
                                            i = i.wrapping_add(1);
                                            j = j.wrapping_add(1);
                                            mop[j as usize] = mop[i as usize];
                                        }
                                        mop.truncate(mop_len as usize);
                                    } else {
                                        (*txn).mt_next_pgno = pgno.wrapping_add(num as pgno_t);
                                    }
                                    (*np).mp_p.p_pgno = pgno;
                                    mdb_page_dirty(txn, np);
                                    *mp = np;
                                    return 0; // MDB_SUCCESS
                                }
                            }
                        }
                    }
                }
            }
        }
        (*txn).mt_flags |= 0x2 as std::ffi::c_uint;
        rc
    }
}
unsafe extern "C" fn mdb_page_copy(
    mut dst: *mut MDB_page,
    mut src: *mut MDB_page,
    mut psize: std::ffi::c_uint,
) {
    unsafe {
        let mut upper: indx_t = (*src).mp_pb.pb.pb_upper;
        let mut lower: indx_t = (*src).mp_pb.pb.pb_lower;
        let mut unused: indx_t = (upper as std::ffi::c_int - lower as std::ffi::c_int) as indx_t;
        unused = (unused as std::ffi::c_int & -(Align as std::ffi::c_int)) as indx_t;
        if unused as std::ffi::c_int != 0
            && ((*(src as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                & 0x20 as std::ffi::c_int
                != 0x20 as std::ffi::c_int)
        {
            upper = ((upper as std::ffi::c_uint).wrapping_add(if 0 as std::ffi::c_int != 0 {
                16 as std::ffi::c_ulong as std::ffi::c_uint
            } else {
                0 as std::ffi::c_int as std::ffi::c_uint
            }) & -(Align as std::ffi::c_int) as std::ffi::c_uint) as indx_t;
            memcpy(
                dst as *mut std::ffi::c_void,
                src as *const std::ffi::c_void,
                ((lower as std::ffi::c_uint)
                    .wrapping_add(if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    })
                    .wrapping_add(
                        (Align as std::ffi::c_int - 1 as std::ffi::c_int) as std::ffi::c_uint,
                    )
                    & -(Align as std::ffi::c_int) as std::ffi::c_uint)
                    as std::ffi::c_ulong,
            );
            memcpy(
                (dst as *mut std::ffi::c_char).offset(upper as std::ffi::c_int as isize)
                    as *mut pgno_t as *mut std::ffi::c_void,
                (src as *mut std::ffi::c_char).offset(upper as std::ffi::c_int as isize)
                    as *mut pgno_t as *const std::ffi::c_void,
                psize.wrapping_sub(upper as std::ffi::c_uint) as std::ffi::c_ulong,
            );
        } else {
            memcpy(
                dst as *mut std::ffi::c_void,
                src as *const std::ffi::c_void,
                psize.wrapping_sub(unused as std::ffi::c_uint) as std::ffi::c_ulong,
            );
        };
    }
}
unsafe extern "C" fn mdb_page_unspill(
    mut txn: *mut MDB_txn,
    mut mp: *mut MDB_page,
    mut ret: *mut *mut MDB_page,
) -> std::ffi::c_int {
    unsafe {
        let mut env: *mut MDB_env = (*txn).mt_env;
        let mut tx2: *mut MDB_txn = std::ptr::null_mut::<MDB_txn>();
        let mut x: std::ffi::c_uint = 0;
        let mut pgno: pgno_t = (*mp).mp_p.p_pgno;
        let mut pn: pgno_t = pgno << 1 as std::ffi::c_int;
        tx2 = txn;
        while !tx2.is_null() {
            if let Some(spill_pgs) = ((*tx2).mt_spill_pgs).as_mut() {
                x = mdb_midl_search(spill_pgs, pn);
                if x as MDB_ID <= spill_pgs.len() && spill_pgs[x as usize] == pn {
                    let mut np: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
                    let mut num: std::ffi::c_int = 0;
                    if (*txn).mt_dirty_room == 0 as std::ffi::c_uint {
                        return -(30788 as std::ffi::c_int);
                    }
                    if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                        as std::ffi::c_int
                        & 0x4 as std::ffi::c_int
                        == 0x4 as std::ffi::c_int
                    {
                        num = (*mp).mp_pb.pb_pages as std::ffi::c_int;
                    } else {
                        num = 1 as std::ffi::c_int;
                    }
                    if (*env).me_flags & 0x80000 as uint32_t != 0 {
                        np = mp;
                    } else {
                        np = mdb_page_malloc(txn, num as std::ffi::c_uint);
                        if np.is_null() {
                            return 12 as std::ffi::c_int;
                        }
                        if num > 1 {
                            memcpy(
                                np as *mut std::ffi::c_void,
                                mp as *const std::ffi::c_void,
                                (num as std::ffi::c_uint).wrapping_mul((*env).me_psize)
                                    as std::ffi::c_ulong,
                            );
                        } else {
                            mdb_page_copy(np, mp, (*env).me_psize);
                        }
                    }
                    if std::ptr::eq(tx2, txn) {
                        if x as MDB_ID == spill_pgs.len() {
                            // let fresh1 =
                            //     &mut (*((*txn).mt_spill_pgs).offset(0 as std::ffi::c_int as isize));
                            // *fresh1 = (*fresh1).wrapping_sub(1);
                            spill_pgs.pop();
                        } else {
                            spill_pgs[x as usize] |= 1 as MDB_ID;
                        }
                    }
                    mdb_page_dirty(txn, np);
                    (*np).mp_flags =
                        ((*np).mp_flags as std::ffi::c_int | 0x10 as std::ffi::c_int) as uint16_t;
                    *ret = np;
                    break;
                }
            }
            tx2 = (*tx2).mt_parent;
        }
        0 as std::ffi::c_int
    }
}

/// Touch a page: make it dirty and re-insert into tree with updated pgno.
///
/// Set #MDB_TXN_ERROR on failure.
/// @param[in] mc cursor pointing to the page to be touched
/// @return 0 on success, non-zero on failure.
unsafe extern "C" fn mdb_page_touch(mut mc: *mut MDB_cursor) -> i32 {
    unsafe {
        let mut current_block: u64;
        let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
        let mut np: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut txn: *mut MDB_txn = (*mc).mc_txn;
        let mut m2: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut m3: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut pgno: pgno_t = 0;
        let mut rc: std::ffi::c_int = 0;
        if std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2).mp2_flags
            as std::ffi::c_int
            & 0x10 as std::ffi::c_int
            != 0x10 as std::ffi::c_int
        {
            if (*txn).mt_flags & 0x8 as std::ffi::c_int as std::ffi::c_uint != 0 {
                np = std::ptr::null_mut::<MDB_page>();
                rc = mdb_page_unspill(txn, mp, &mut np);
                if rc != 0 {
                    current_block = 14630967659647133411;
                } else if !np.is_null() {
                    current_block = 11181162895269274365;
                } else {
                    current_block = 11006700562992250127;
                }
            } else {
                current_block = 11006700562992250127;
            }
            match current_block {
                11181162895269274365 => {}
                _ => {
                    if current_block == 11006700562992250127 {
                        rc = mdb_midl_need(&mut (*txn).mt_free_pgs, 1);
                        if rc != 0 || {
                            rc = mdb_page_alloc(mc, 1 as std::ffi::c_int, &mut np);
                            rc != 0
                        } {
                            current_block = 14630967659647133411;
                        } else {
                            pgno = (*np).mp_p.p_pgno;
                            if (*mp).mp_p.p_pgno != pgno {
                            } else {
                                mdb_assert_fail(
                                    (*(*mc).mc_txn).mt_env,
                                    b"mp->mp_pgno != pgno\0" as *const u8
                                        as *const std::ffi::c_char,
                                    (*::core::mem::transmute::<&[u8; 15], &[std::ffi::c_char; 15]>(
                                        b"mdb_page_touch\0",
                                    ))
                                    .as_ptr(),
                                    b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                                    2800 as std::ffi::c_int,
                                );
                            };
                            // let mut xidl: &mut MDB_IDL = &mut (*txn).mt_free_pgs;
                            // let fresh2 = &mut (*xidl.offset(0 as std::ffi::c_int as isize));
                            // *fresh2 = (*fresh2).wrapping_add(1);
                            // let mut xlen: MDB_ID = *fresh2;
                            // *xidl.offset(xlen as isize) = (*mp).mp_p.p_pgno;
                            mdb_midl_xappend(&mut (*txn).mt_free_pgs, (*mp).mp_p.p_pgno);
                            /* Update the parent page, if any, to point to the new page */
                            if (*mc).mc_top != 0 {
                                let mut parent: *mut MDB_page = (*mc).mc_pg[((*mc).mc_top
                                    as std::ffi::c_int
                                    - 1 as std::ffi::c_int)
                                    as usize];
                                let mut node: *mut MDB_node = (parent as *mut std::ffi::c_char)
                                    .offset(
                                        *((*(parent as *mut std::ffi::c_void as *mut MDB_page2))
                                            .mp2_ptrs)
                                            .as_mut_ptr()
                                            .offset(
                                                (*mc).mc_ki[((*mc).mc_top as std::ffi::c_int
                                                    - 1 as std::ffi::c_int)
                                                    as usize]
                                                    as isize,
                                            )
                                            as std::ffi::c_int
                                            as isize,
                                    )
                                    .offset(
                                        (if 0 as std::ffi::c_int != 0 {
                                            16 as std::ffi::c_ulong as std::ffi::c_uint
                                        } else {
                                            0 as std::ffi::c_int as std::ffi::c_uint
                                        }) as isize,
                                    )
                                    as *mut MDB_node;
                                (*node).mn_lo = (pgno & 0xffff as std::ffi::c_int as pgno_t)
                                    as std::ffi::c_ushort;
                                (*node).mn_hi =
                                    (pgno >> 16 as std::ffi::c_int) as std::ffi::c_ushort;
                                if if -(1 as std::ffi::c_int) as pgno_t
                                    > 0xffffffff as std::ffi::c_uint as pgno_t
                                {
                                    32 as std::ffi::c_int
                                } else {
                                    0 as std::ffi::c_int
                                } != 0
                                {
                                    (*node).mn_flags = (pgno
                                        >> (if -(1 as std::ffi::c_int) as pgno_t
                                            > 0xffffffff as std::ffi::c_uint as pgno_t
                                        {
                                            32 as std::ffi::c_int
                                        } else {
                                            0 as std::ffi::c_int
                                        }))
                                        as std::ffi::c_ushort;
                                }
                            } else {
                                (*(*mc).mc_db).md_root = pgno;
                            }
                            current_block = 13131896068329595644;
                        }
                    }
                    match current_block {
                        13131896068329595644 => {}
                        _ => {
                            (*txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                            return rc;
                        }
                    }
                }
            }
        } else {
            if !((*txn).mt_parent).is_null()
                && ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                    & 0x40 as std::ffi::c_int
                    != 0x40 as std::ffi::c_int)
            {
                let mut mid: MDB_ID2 =
                    MDB_ID2 { mid: 0, mptr: std::ptr::null_mut::<std::ffi::c_void>() };
                let mut dl: *mut MDB_ID2 = (*txn).mt_u.dirty_list;
                pgno = (*mp).mp_p.p_pgno;
                if (*dl.offset(0 as std::ffi::c_int as isize)).mid != 0 {
                    let mut x: std::ffi::c_uint = mdb_mid2l_search(dl, pgno);
                    if x as MDB_ID <= (*dl.offset(0 as std::ffi::c_int as isize)).mid
                        && (*dl.offset(x as isize)).mid == pgno
                    {
                        if mp != (*dl.offset(x as isize)).mptr as *mut MDB_page {
                            (*mc).mc_flags &= !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                as std::ffi::c_uint;
                            (*txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                            return -(30779 as std::ffi::c_int);
                        }
                        return 0 as std::ffi::c_int;
                    }
                }
                if (*dl.offset(0 as std::ffi::c_int as isize)).mid
                    < (((1 as std::ffi::c_int) << (16 as std::ffi::c_int + 1 as std::ffi::c_int))
                        - 1 as std::ffi::c_int) as MDB_ID
                {
                } else {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"dl[0].mid < MDB_IDL_UM_MAX\0" as *const u8 as *const std::ffi::c_char,
                        (*::core::mem::transmute::<&[u8; 15], &[std::ffi::c_char; 15]>(
                            b"mdb_page_touch\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                        2827 as std::ffi::c_int,
                    );
                };
                np = mdb_page_malloc(txn, 1 as std::ffi::c_int as std::ffi::c_uint);
                if np.is_null() {
                    return 12 as std::ffi::c_int;
                }
                mid.mid = pgno;
                mid.mptr = np as *mut std::ffi::c_void;
                rc = mdb_mid2l_insert(dl, &mut mid);
                if rc == 0 as std::ffi::c_int {
                } else {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"rc == 0\0" as *const u8 as *const std::ffi::c_char,
                        (*::core::mem::transmute::<&[u8; 15], &[std::ffi::c_char; 15]>(
                            b"mdb_page_touch\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                        2835 as std::ffi::c_int,
                    );
                };
            } else {
                return 0 as std::ffi::c_int;
            }
            current_block = 13131896068329595644;
        }
        if current_block == 13131896068329595644 {
            mdb_page_copy(np, mp, (*(*txn).mt_env).me_psize);
            (*np).mp_p.p_pgno = pgno;
            (*np).mp_flags =
                ((*np).mp_flags as std::ffi::c_int | 0x10 as std::ffi::c_int) as uint16_t;
        }
        (*mc).mc_pg[(*mc).mc_top as usize] = np;
        m2 = *((*txn).mt_cursors).offset((*mc).mc_dbi as isize);
        if (*mc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
            while !m2.is_null() {
                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                if (((*m3).mc_snum as std::ffi::c_int) >= (*mc).mc_snum as std::ffi::c_int)
                    && (*m3).mc_pg[(*mc).mc_top as usize] == mp
                {
                    (*m3).mc_pg[(*mc).mc_top as usize] = np;
                }
                m2 = (*m2).mc_next;
            }
        } else {
            while !m2.is_null() {
                if (((*m2).mc_snum as std::ffi::c_int) >= (*mc).mc_snum as std::ffi::c_int)
                    && (m2 != mc)
                    && (*m2).mc_pg[(*mc).mc_top as usize] == mp
                {
                    (*m2).mc_pg[(*mc).mc_top as usize] = np;
                    if (*(np as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                        as std::ffi::c_int
                        & 0x2 as std::ffi::c_int
                        == 0x2 as std::ffi::c_int
                    {
                        let mut xr_pg: *mut MDB_page = np;
                        let mut xr_node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                        if !(!(!((*m2).mc_xcursor).is_null()
                            && (*(*m2).mc_xcursor).mx_cursor.mc_flags
                                & 0x1 as std::ffi::c_int as std::ffi::c_uint
                                != 0)
                            || (*m2).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint
                                >= ((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                                    as std::ffi::c_uint)
                                    .wrapping_sub(
                                        (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                            if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            },
                                        ),
                                    )
                                    >> 1 as std::ffi::c_int)
                        {
                            xr_node = (xr_pg as *mut std::ffi::c_char)
                                .offset(
                                    *((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2))
                                        .mp2_ptrs)
                                        .as_mut_ptr()
                                        .offset((*m2).mc_ki[(*mc).mc_top as usize] as isize)
                                        as std::ffi::c_int
                                        as isize,
                                )
                                .offset(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            if (*xr_node).mn_flags as std::ffi::c_int
                                & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                == 0x4 as std::ffi::c_int
                            {
                                (*(*m2).mc_xcursor).mx_cursor.mc_pg
                                    [0 as std::ffi::c_int as usize] = ((*xr_node).mn_data)
                                    .as_mut_ptr()
                                    .offset((*xr_node).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void
                                    as *mut MDB_page;
                            }
                        }
                    }
                }
                m2 = (*m2).mc_next;
            }
        }
        0 as std::ffi::c_int
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_env_sync0(
    mut env: *mut MDB_env,
    mut force: std::ffi::c_int,
    mut numpgs: pgno_t,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        if (*env).me_flags & 0x20000 as std::ffi::c_int as uint32_t != 0 {
            return 13 as std::ffi::c_int;
        }
        if force != 0 || (*env).me_flags & 0x10000 as std::ffi::c_int as uint32_t == 0 {
            if (*env).me_flags & 0x80000 as std::ffi::c_int as uint32_t != 0 {
                let mut flags: std::ffi::c_int =
                    if (*env).me_flags & 0x100000 as std::ffi::c_int as uint32_t != 0 && force == 0
                    {
                        0x1 as std::ffi::c_int
                    } else {
                        0x10 as std::ffi::c_int
                    };
                if msync(
                    (*env).me_map as *mut std::ffi::c_void,
                    ((*env).me_psize as pgno_t * numpgs) as u64,
                    flags,
                ) != 0
                {
                    rc = *__error();
                } else if flags == 0x10 as std::ffi::c_int
                    && fcntl((*env).me_fd, 51 as std::ffi::c_int) != 0
                {
                    rc = *__error();
                }
            } else if fcntl((*env).me_fd, 51 as std::ffi::c_int) != 0 {
                rc = *__error();
            }
        }
        rc
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_env_sync(
    mut env: *mut MDB_env,
    mut force: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut m: *mut MDB_meta = mdb_env_pick_meta(env);
        mdb_env_sync0(env, force, ((*m).mm_last_pg).wrapping_add(1 as std::ffi::c_int as pgno_t))
    }
}

unsafe extern "C" fn mdb_cursor_shadow(
    mut src: *mut MDB_txn,
    mut dst: *mut MDB_txn,
) -> std::ffi::c_int {
    unsafe {
        let mut mc: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut bk: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut mx: *mut MDB_xcursor = std::ptr::null_mut::<MDB_xcursor>();
        let mut size: size_t = 0;
        let mut i: std::ffi::c_int = 0;
        i = (*src).mt_numdbs as std::ffi::c_int;
        loop {
            i -= 1;
            if i < 0 as std::ffi::c_int {
                break;
            }
            mc = *((*src).mt_cursors).offset(i as isize);
            if !mc.is_null() {
                size = ::core::mem::size_of::<MDB_cursor>() as std::ffi::c_ulong;
                if !((*mc).mc_xcursor).is_null() {
                    size = (size as std::ffi::c_ulong)
                        .wrapping_add(::core::mem::size_of::<MDB_xcursor>() as std::ffi::c_ulong)
                        as size_t as size_t;
                }
                while !mc.is_null() {
                    bk = malloc(size) as *mut MDB_cursor;
                    if bk.is_null() {
                        return 12 as std::ffi::c_int;
                    }
                    *bk = *mc;
                    (*mc).mc_backup = bk;
                    (*mc).mc_db = &mut *((*dst).mt_dbs).offset(i as isize) as *mut MDB_db;
                    (*mc).mc_txn = dst;
                    (*mc).mc_dbflag =
                        &mut *((*dst).mt_dbflags).offset(i as isize) as *mut std::ffi::c_uchar;
                    mx = (*mc).mc_xcursor;
                    if !mx.is_null() {
                        *(bk.offset(1 as std::ffi::c_int as isize) as *mut MDB_xcursor) = *mx;
                        (*mx).mx_cursor.mc_txn = dst;
                    }
                    (*mc).mc_next = *((*dst).mt_cursors).offset(i as isize);
                    let fresh3 = &mut (*((*dst).mt_cursors).offset(i as isize));
                    *fresh3 = mc;
                    mc = (*bk).mc_next;
                }
            }
        }
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_cursors_close(mut txn: *mut MDB_txn, mut merge: std::ffi::c_uint) {
    unsafe {
        let mut cursors: *mut *mut MDB_cursor = (*txn).mt_cursors;
        let mut mc: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut next: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut bk: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut mx: *mut MDB_xcursor = std::ptr::null_mut::<MDB_xcursor>();
        let mut i: std::ffi::c_int = 0;
        i = (*txn).mt_numdbs as std::ffi::c_int;
        loop {
            i -= 1;
            if i < 0 as std::ffi::c_int {
                break;
            }
            mc = *cursors.offset(i as isize);
            while !mc.is_null() {
                next = (*mc).mc_next;
                bk = (*mc).mc_backup;
                if !bk.is_null() {
                    if merge != 0 {
                        (*mc).mc_next = (*bk).mc_next;
                        (*mc).mc_backup = (*bk).mc_backup;
                        (*mc).mc_txn = (*bk).mc_txn;
                        (*mc).mc_db = (*bk).mc_db;
                        (*mc).mc_dbflag = (*bk).mc_dbflag;
                        mx = (*mc).mc_xcursor;
                        if !mx.is_null() {
                            (*mx).mx_cursor.mc_txn = (*bk).mc_txn;
                        }
                    } else {
                        *mc = *bk;
                        mx = (*mc).mc_xcursor;
                        if !mx.is_null() {
                            *mx = *(bk.offset(1 as std::ffi::c_int as isize) as *mut MDB_xcursor);
                        }
                    }
                    mc = bk;
                }
                free(mc as *mut std::ffi::c_void);
                mc = next;
            }
            let fresh4 = &mut (*cursors.offset(i as isize));
            *fresh4 = std::ptr::null_mut::<MDB_cursor>();
        }
    }
}
unsafe extern "C" fn mdb_reader_pid(
    mut env: *mut MDB_env,
    mut op: Pidlock_op,
    mut pid: pid_t,
) -> std::ffi::c_int {
    unsafe {
        loop {
            let mut rc: std::ffi::c_int = 0;
            let mut lock_info: flock =
                flock { l_start: 0, l_len: 0, l_pid: 0, l_type: 0, l_whence: 0 };
            memset(
                &mut lock_info as *mut flock as *mut std::ffi::c_void,
                0 as std::ffi::c_int,
                ::core::mem::size_of::<flock>() as std::ffi::c_ulong,
            );
            lock_info.l_type = 3 as std::ffi::c_int as std::ffi::c_short;
            lock_info.l_whence = 0 as std::ffi::c_int as std::ffi::c_short;
            lock_info.l_start = pid as off_t;
            lock_info.l_len = 1 as std::ffi::c_int as off_t;
            rc = fcntl((*env).me_lfd, op as std::ffi::c_int, &mut lock_info as *mut flock);
            if rc == 0 as std::ffi::c_int {
                if op as std::ffi::c_uint == 7 as std::ffi::c_int as std::ffi::c_uint
                    && lock_info.l_type as std::ffi::c_int != 2 as std::ffi::c_int
                {
                    rc = -(1 as std::ffi::c_int);
                }
            } else {
                rc = *__error();
                if rc == 4 as std::ffi::c_int {
                    continue;
                }
            }
            return rc;
        }
    }
}
unsafe extern "C" fn mdb_txn_renew0(mut txn: *mut MDB_txn) -> std::ffi::c_int {
    unsafe {
        let mut env: *mut MDB_env = (*txn).mt_env;
        let mut ti: *mut MDB_txninfo = (*env).me_txns;
        let mut meta: *mut MDB_meta = std::ptr::null_mut::<MDB_meta>();
        let mut i: std::ffi::c_uint = 0;
        let mut nr: std::ffi::c_uint = 0;
        let mut flags: std::ffi::c_uint = (*txn).mt_flags;
        let mut x: uint16_t = 0;
        let mut rc: std::ffi::c_int = 0;
        let mut new_notls: std::ffi::c_int = 0 as std::ffi::c_int;
        flags &= 0x20000 as std::ffi::c_int as std::ffi::c_uint;
        if flags != 0 as std::ffi::c_int as std::ffi::c_uint {
            if ti.is_null() {
                meta = mdb_env_pick_meta(env);
                (*txn).mt_txnid = (*meta).mm_txnid;
                (*txn).mt_u.reader = std::ptr::null_mut::<MDB_reader>();
            } else {
                let mut r: *mut MDB_reader =
                    (if (*env).me_flags & 0x200000 as std::ffi::c_int as uint32_t != 0 {
                        (*txn).mt_u.reader as *mut std::ffi::c_void
                    } else {
                        pthread_getspecific((*env).me_txkey)
                    }) as *mut MDB_reader;
                if !r.is_null() {
                    if (*r).mru.mrx.mrb_pid != (*env).me_pid
                        || (*r).mru.mrx.mrb_txnid != -(1 as std::ffi::c_int) as txnid_t
                    {
                        return -(30783 as std::ffi::c_int);
                    }
                } else {
                    let mut pid: pid_t = (*env).me_pid;
                    let mut tid: pthread_t = pthread_self();
                    let mut rmutex: mdb_mutexref_t = ((*env).me_rmutex).as_mut_ptr();
                    if (*env).me_live_reader == 0 {
                        rc = mdb_reader_pid(env, Pidset, pid);
                        if rc != 0 {
                            return rc;
                        }
                        (*env).me_live_reader = 1 as std::ffi::c_int;
                    }
                    rc = mdb_sem_wait(rmutex);
                    if rc != 0 && {
                        rc = mdb_mutex_failed(env, rmutex, rc);
                        rc != 0
                    } {
                        return rc;
                    }
                    nr = (*ti).mt1.mtb.mtb_numreaders;
                    i = 0 as std::ffi::c_int as std::ffi::c_uint;
                    while i < nr {
                        if (*((*ti).mti_readers).as_mut_ptr().offset(i as isize)).mru.mrx.mrb_pid
                            == 0 as std::ffi::c_int
                        {
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                    if i == (*env).me_maxreaders {
                        let mut sb: sembuf = {
                            sembuf {
                                sem_num: 0 as std::ffi::c_int as std::ffi::c_ushort,
                                sem_op: 1 as std::ffi::c_int as std::ffi::c_short,
                                sem_flg: 0o10000 as std::ffi::c_int as std::ffi::c_short,
                            }
                        };
                        sb.sem_num = (*rmutex).semnum as std::ffi::c_ushort;
                        *(*rmutex).locked = 0 as std::ffi::c_int;
                        semop((*rmutex).semid, &mut sb, 1 as std::ffi::c_int as size_t);
                        return -(30790 as std::ffi::c_int);
                    }
                    r = &mut *((*ti).mti_readers).as_mut_ptr().offset(i as isize)
                        as *mut MDB_reader;
                    ::core::ptr::write_volatile(
                        &mut (*r).mru.mrx.mrb_pid as *mut pid_t,
                        0 as std::ffi::c_int,
                    );
                    ::core::ptr::write_volatile(
                        &mut (*r).mru.mrx.mrb_txnid as *mut txnid_t,
                        -(1 as std::ffi::c_int) as txnid_t,
                    );
                    ::core::ptr::write_volatile(&mut (*r).mru.mrx.mrb_tid as *mut pthread_t, tid);
                    if i == nr {
                        nr = nr.wrapping_add(1);
                        ::core::ptr::write_volatile(
                            &mut (*ti).mt1.mtb.mtb_numreaders as *mut std::ffi::c_uint,
                            nr,
                        );
                    }
                    ::core::ptr::write_volatile(
                        &mut (*env).me_close_readers as *mut std::ffi::c_int,
                        nr as std::ffi::c_int,
                    );
                    ::core::ptr::write_volatile(&mut (*r).mru.mrx.mrb_pid as *mut pid_t, pid);
                    let mut sb_0: sembuf = {
                        sembuf {
                            sem_num: 0 as std::ffi::c_int as std::ffi::c_ushort,
                            sem_op: 1 as std::ffi::c_int as std::ffi::c_short,
                            sem_flg: 0o10000 as std::ffi::c_int as std::ffi::c_short,
                        }
                    };
                    sb_0.sem_num = (*rmutex).semnum as std::ffi::c_ushort;
                    *(*rmutex).locked = 0 as std::ffi::c_int;
                    semop((*rmutex).semid, &mut sb_0, 1 as std::ffi::c_int as size_t);
                    new_notls = ((*env).me_flags & 0x200000 as std::ffi::c_int as uint32_t)
                        as std::ffi::c_int;
                    if new_notls == 0 && {
                        rc = pthread_setspecific((*env).me_txkey, r as *const std::ffi::c_void);
                        rc != 0
                    } {
                        ::core::ptr::write_volatile(
                            &mut (*r).mru.mrx.mrb_pid as *mut pid_t,
                            0 as std::ffi::c_int,
                        );
                        return rc;
                    }
                }
                loop {
                    ::core::ptr::write_volatile(
                        &mut (*r).mru.mrx.mrb_txnid as *mut txnid_t,
                        (*ti).mt1.mtb.mtb_txnid,
                    );
                    if (*r).mru.mrx.mrb_txnid == (*ti).mt1.mtb.mtb_txnid {
                        break;
                    }
                }
                if (*r).mru.mrx.mrb_txnid == 0
                    && (*env).me_flags & 0x20000 as std::ffi::c_int as uint32_t != 0
                {
                    meta = mdb_env_pick_meta(env);
                    ::core::ptr::write_volatile(
                        &mut (*r).mru.mrx.mrb_txnid as *mut txnid_t,
                        (*meta).mm_txnid,
                    );
                } else {
                    meta = (*env).me_metas
                        [((*r).mru.mrx.mrb_txnid & 1 as std::ffi::c_int as txnid_t) as usize];
                }
                (*txn).mt_txnid = (*r).mru.mrx.mrb_txnid;
                (*txn).mt_u.reader = r;
            }
        } else {
            if !ti.is_null() {
                rc = mdb_sem_wait(((*env).me_wmutex).as_mut_ptr());
                if rc != 0 && {
                    rc = mdb_mutex_failed(env, ((*env).me_wmutex).as_mut_ptr(), rc);
                    rc != 0
                } {
                    return rc;
                }
                (*txn).mt_txnid = (*ti).mt1.mtb.mtb_txnid;
                meta =
                    (*env).me_metas[((*txn).mt_txnid & 1 as std::ffi::c_int as txnid_t) as usize];
            } else {
                meta = mdb_env_pick_meta(env);
                (*txn).mt_txnid = (*meta).mm_txnid;
            }
            (*txn).mt_txnid = ((*txn).mt_txnid).wrapping_add(1);
            (*txn).mt_txnid;
            (*txn).mt_child = std::ptr::null_mut::<MDB_txn>();
            (*txn).mt_loose_pgs = std::ptr::null_mut::<MDB_page>();
            (*txn).mt_loose_count = 0 as std::ffi::c_int;
            (*txn).mt_dirty_room = (((1 as std::ffi::c_int)
                << (16 as std::ffi::c_int + 1 as std::ffi::c_int))
                - 1 as std::ffi::c_int) as std::ffi::c_uint;
            (*txn).mt_u.dirty_list = (*env).me_dirty_list;
            (*((*txn).mt_u.dirty_list).offset(0 as std::ffi::c_int as isize)).mid =
                0 as std::ffi::c_int as MDB_ID;
            // (*txn).mt_free_pgs = (*env).me_free_pgs;
            // *((*txn).mt_free_pgs).offset(0 as std::ffi::c_int as isize) =
            //     0 as std::ffi::c_int as MDB_ID;
            // Note: I loose the capacity, for now
            (*txn).mt_free_pgs = MDB_IDL::default();
            (*txn).mt_spill_pgs = None;
            (*env).me_txn = txn;
            memcpy(
                (*txn).mt_dbiseqs as *mut std::ffi::c_void,
                (*env).me_dbiseqs as *const std::ffi::c_void,
                ((*env).me_maxdbs as std::ffi::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong),
            );
        }
        memcpy(
            (*txn).mt_dbs as *mut std::ffi::c_void,
            ((*meta).mm_dbs).as_mut_ptr() as *const std::ffi::c_void,
            (2 as std::ffi::c_int as std::ffi::c_ulong)
                .wrapping_mul(::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong),
        );
        (*txn).mt_next_pgno = ((*meta).mm_last_pg).wrapping_add(1 as std::ffi::c_int as pgno_t);
        (*txn).mt_flags = flags;
        (*txn).mt_numdbs = (*env).me_numdbs;
        i = 2 as std::ffi::c_int as std::ffi::c_uint;
        while i < (*txn).mt_numdbs {
            x = *((*env).me_dbflags).offset(i as isize);
            (*((*txn).mt_dbs).offset(i as isize)).md_flags = (x as std::ffi::c_int
                & (0xffff as std::ffi::c_int & !(0x8000 as std::ffi::c_int)))
                as uint16_t;
            *((*txn).mt_dbflags).offset(i as isize) =
                (if x as std::ffi::c_int & 0x8000 as std::ffi::c_int != 0 {
                    0x8 as std::ffi::c_int | 0x10 as std::ffi::c_int | 0x2 as std::ffi::c_int
                } else {
                    0 as std::ffi::c_int
                }) as std::ffi::c_uchar;
            i = i.wrapping_add(1);
        }
        *((*txn).mt_dbflags).offset(1 as std::ffi::c_int as isize) =
            (0x8 as std::ffi::c_int | 0x10 as std::ffi::c_int) as std::ffi::c_uchar;
        *((*txn).mt_dbflags).offset(0 as std::ffi::c_int as isize) =
            0x8 as std::ffi::c_int as std::ffi::c_uchar;
        if (*env).me_flags & 0x80000000 as std::ffi::c_uint != 0 {
            rc = -(30795 as std::ffi::c_int);
        } else if (*env).me_maxpg < (*txn).mt_next_pgno {
            rc = -(30785 as std::ffi::c_int);
        } else {
            return 0 as std::ffi::c_int;
        }
        mdb_txn_end(txn, (new_notls | MDB_END_FAIL_BEGIN as std::ffi::c_int) as std::ffi::c_uint);
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_txn_renew(mut txn: *mut MDB_txn) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0;
        if txn.is_null()
            || ((*txn).mt_flags
                & (0x20000 as std::ffi::c_int | 0x1 as std::ffi::c_int) as std::ffi::c_uint
                != (0x20000 as std::ffi::c_int | 0x1 as std::ffi::c_int) as std::ffi::c_uint)
        {
            return 22 as std::ffi::c_int; // EINVAL
        }
        rc = mdb_txn_renew0(txn);
        // rc == 0 as std::ffi::c_int; // DPRINTF WAS DISABLED
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_txn_begin(
    mut env: *mut MDB_env,
    mut parent: *mut MDB_txn,
    mut flags: std::ffi::c_uint,
    mut ret: *mut *mut MDB_txn,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut txn: *mut MDB_txn = std::ptr::null_mut::<MDB_txn>();
        let mut ntxn: *mut MDB_ntxn = std::ptr::null_mut::<MDB_ntxn>();
        let mut rc: std::ffi::c_int = 0;
        let mut size: std::ffi::c_int = 0;
        let mut tsize: std::ffi::c_int = 0;
        flags &= (0x40000 as std::ffi::c_int
            | 0x10000 as std::ffi::c_int
            | 0x20000 as std::ffi::c_int) as std::ffi::c_uint;
        flags |= (*env).me_flags & 0x80000 as std::ffi::c_int as uint32_t;
        if (*env).me_flags & 0x20000 as std::ffi::c_int as uint32_t & !flags != 0 {
            return 13 as std::ffi::c_int;
        }
        if !parent.is_null() {
            flags |= (*parent).mt_flags;
            if flags
                & (0x20000 as std::ffi::c_int
                    | 0x80000 as std::ffi::c_int
                    | (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int))
                    as std::ffi::c_uint
                != 0
            {
                return if (*parent).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    22 as std::ffi::c_int
                } else {
                    -(30782 as std::ffi::c_int)
                };
            }
            size = ((*env).me_maxdbs as std::ffi::c_ulong).wrapping_mul(
                (::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong)
                    .wrapping_add(::core::mem::size_of::<*mut MDB_cursor>() as std::ffi::c_ulong)
                    .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong),
            ) as std::ffi::c_int;
            tsize = ::core::mem::size_of::<MDB_ntxn>() as std::ffi::c_ulong as std::ffi::c_int;
            size += tsize;
            current_block = 12800627514080957624;
        } else if flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0 {
            size = ((*env).me_maxdbs as std::ffi::c_ulong).wrapping_mul(
                (::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong)
                    .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong),
            ) as std::ffi::c_int;
            tsize = ::core::mem::size_of::<MDB_txn>() as std::ffi::c_ulong as std::ffi::c_int;
            size += tsize;
            current_block = 12800627514080957624;
        } else {
            txn = (*env).me_txn0;
            current_block = 16107657000161174531;
        }
        if current_block == 12800627514080957624 {
            txn = calloc(1 as std::ffi::c_int as std::ffi::c_ulong, size as std::ffi::c_ulong)
                as *mut MDB_txn;
            if txn.is_null() {
                return 12 as std::ffi::c_int;
            }
            (*txn).mt_dbxs = (*env).me_dbxs;
            (*txn).mt_dbs = (txn as *mut std::ffi::c_char).offset(tsize as isize) as *mut MDB_db;
            (*txn).mt_dbflags = (txn as *mut std::ffi::c_uchar)
                .offset(size as isize)
                .offset(-((*env).me_maxdbs as isize));
            (*txn).mt_flags = flags;
            (*txn).mt_env = env;
            if !parent.is_null() {
                let mut i: std::ffi::c_uint = 0;
                (*txn).mt_cursors =
                    ((*txn).mt_dbs).offset((*env).me_maxdbs as isize) as *mut *mut MDB_cursor;
                (*txn).mt_dbiseqs = (*parent).mt_dbiseqs;
                (*txn).mt_u.dirty_list =
                    malloc((::core::mem::size_of::<MDB_ID2>() as std::ffi::c_ulong).wrapping_mul(
                        ((1 as std::ffi::c_int) << (16 as std::ffi::c_int + 1 as std::ffi::c_int))
                            as std::ffi::c_ulong,
                    )) as MDB_ID2L;
                if ((*txn).mt_u.dirty_list).is_null() || {
                    // Note: We must not unwrap here and return an actual ENOMEM error
                    (*txn).mt_free_pgs = mdb_midl_alloc(((1) << (16 + 1)) - 1).unwrap_or_default();
                    // ((*txn).mt_free_pgs).is_null()
                    (*txn).mt_free_pgs.capacity() == 0
                } {
                    free((*txn).mt_u.dirty_list as *mut std::ffi::c_void);
                    free(txn as *mut std::ffi::c_void);
                    return ENOMEM;
                }
                (*txn).mt_txnid = (*parent).mt_txnid;
                (*txn).mt_dirty_room = (*parent).mt_dirty_room;
                (*((*txn).mt_u.dirty_list).offset(0 as std::ffi::c_int as isize)).mid =
                    0 as std::ffi::c_int as MDB_ID;
                (*txn).mt_spill_pgs = None;
                (*txn).mt_next_pgno = (*parent).mt_next_pgno;
                (*parent).mt_flags |= 0x10 as std::ffi::c_int as std::ffi::c_uint;
                (*parent).mt_child = txn;
                (*txn).mt_parent = parent;
                (*txn).mt_numdbs = (*parent).mt_numdbs;
                memcpy(
                    (*txn).mt_dbs as *mut std::ffi::c_void,
                    (*parent).mt_dbs as *const std::ffi::c_void,
                    ((*txn).mt_numdbs as std::ffi::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong),
                );
                i = 0 as std::ffi::c_int as std::ffi::c_uint;
                while i < (*txn).mt_numdbs {
                    *((*txn).mt_dbflags).offset(i as isize) =
                        (*((*parent).mt_dbflags).offset(i as isize) as std::ffi::c_int
                            & !(0x4 as std::ffi::c_int))
                            as std::ffi::c_uchar;
                    i = i.wrapping_add(1);
                }
                rc = 0 as std::ffi::c_int;
                ntxn = txn as *mut MDB_ntxn;
                // Note: This complex part is literally just a clone with a check for
                //       memory allocation failures. We will handle allocation failures later.
                // (*ntxn).mnt_pgstate = (*env).me_pgstate; /* save parent me_pghead & co */
                (*ntxn).mnt_pgstate = MDB_pgstate {
                    mf_pghead: (*env).me_pgstate.mf_pghead.clone(),
                    mf_pglast: (*env).me_pgstate.mf_pglast,
                };
                if rc == 0 {
                    rc = mdb_cursor_shadow(parent, txn);
                }
                if rc != 0 {
                    mdb_txn_end(
                        txn,
                        MDB_END_FAIL_BEGINCHILD as std::ffi::c_int as std::ffi::c_uint,
                    );
                }
                current_block = 10150597327160359210;
            } else {
                (*txn).mt_dbiseqs = (*env).me_dbiseqs;
                current_block = 16107657000161174531;
            }
        }
        if current_block == 16107657000161174531 {
            rc = mdb_txn_renew0(txn);
        }
        if rc != 0 {
            if txn != (*env).me_txn0 {
                free((*txn).mt_u.dirty_list as *mut std::ffi::c_void);
                free(txn as *mut std::ffi::c_void);
            }
        } else {
            (*txn).mt_flags |= flags;
            *ret = txn;
        }
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_txn_env(mut txn: *mut MDB_txn) -> *mut MDB_env {
    unsafe {
        if txn.is_null() {
            return std::ptr::null_mut::<MDB_env>();
        }
        (*txn).mt_env
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_txn_id(mut txn: *mut MDB_txn) -> mdb_size_t {
    unsafe {
        if txn.is_null() {
            return 0 as std::ffi::c_int as mdb_size_t;
        }
        (*txn).mt_txnid as _
    }
}
unsafe extern "C" fn mdb_dbis_update(mut txn: *mut MDB_txn, mut keep: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = 0;
        let mut n: MDB_dbi = (*txn).mt_numdbs;
        let mut env: *mut MDB_env = (*txn).mt_env;
        let mut tdbflags: *mut std::ffi::c_uchar = (*txn).mt_dbflags;
        i = n as std::ffi::c_int;
        loop {
            i -= 1;
            if i < 2 as std::ffi::c_int {
                break;
            }
            if *tdbflags.offset(i as isize) as std::ffi::c_int & 0x4 as std::ffi::c_int != 0 {
                if keep != 0 {
                    *((*env).me_dbflags).offset(i as isize) =
                        ((*((*txn).mt_dbs).offset(i as isize)).md_flags as std::ffi::c_int
                            | 0x8000 as std::ffi::c_int) as uint16_t;
                } else {
                    let mut ptr: *mut std::ffi::c_char =
                        (*((*env).me_dbxs).offset(i as isize)).md_name.mv_data
                            as *mut std::ffi::c_char;
                    if !ptr.is_null() {
                        let fresh5 = &mut (*((*env).me_dbxs).offset(i as isize)).md_name.mv_data;
                        *fresh5 = std::ptr::null_mut::<std::ffi::c_void>();
                        (*((*env).me_dbxs).offset(i as isize)).md_name.mv_size =
                            0 as std::ffi::c_int as size_t;
                        *((*env).me_dbflags).offset(i as isize) = 0 as std::ffi::c_int as uint16_t;
                        let fresh6 = &mut (*((*env).me_dbiseqs).offset(i as isize));
                        *fresh6 = (*fresh6).wrapping_add(1);
                        free(ptr as *mut std::ffi::c_void);
                    }
                }
            }
        }
        if keep != 0 && (*env).me_numdbs < n {
            (*env).me_numdbs = n;
        }
    }
}
unsafe extern "C" fn mdb_txn_end(mut txn: *mut MDB_txn, mut mode: std::ffi::c_uint) {
    unsafe {
        let mut env: *mut MDB_env = (*txn).mt_env;
        mdb_dbis_update(
            txn,
            (mode & 0x10 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int,
        );
        if (*txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint
            == 0x20000 as std::ffi::c_int as std::ffi::c_uint
        {
            if !((*txn).mt_u.reader).is_null() {
                ::core::ptr::write_volatile(
                    &mut (*(*txn).mt_u.reader).mru.mrx.mrb_txnid as *mut txnid_t,
                    -(1 as std::ffi::c_int) as txnid_t,
                );
                if (*env).me_flags & 0x200000 as std::ffi::c_int as uint32_t == 0 {
                    (*txn).mt_u.reader = std::ptr::null_mut::<MDB_reader>();
                } else if mode & 0x200000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    ::core::ptr::write_volatile(
                        &mut (*(*txn).mt_u.reader).mru.mrx.mrb_pid as *mut pid_t,
                        0 as std::ffi::c_int,
                    );
                    (*txn).mt_u.reader = std::ptr::null_mut::<MDB_reader>();
                }
            }
            (*txn).mt_numdbs = 0 as std::ffi::c_int as MDB_dbi;
            (*txn).mt_flags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
        } else if (*txn).mt_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint
            != 0x1 as std::ffi::c_int as std::ffi::c_uint
        {
            // let mut pghead: Option<&mut MDB_IDL> = (*env).me_pgstate.mf_pghead.as_mut();
            if mode & 0x10 == 0 {
                mdb_cursors_close(txn, 0 as std::ffi::c_uint);
            }
            if (*env).me_flags & 0x80000 as uint32_t == 0 {
                mdb_dlist_free(txn);
            }
            (*txn).mt_numdbs = 0 as std::ffi::c_int as MDB_dbi;
            (*txn).mt_flags = 0x1 as std::ffi::c_int as std::ffi::c_uint;
            if ((*txn).mt_parent).is_null() {
                mdb_midl_shrink(&mut (*txn).mt_free_pgs);
                (*env).me_free_pgs = Some(std::mem::take(&mut (*txn).mt_free_pgs));
                (*env).me_pgstate.mf_pghead = None;
                (*env).me_pgstate.mf_pglast = 0;
                (*env).me_txn = std::ptr::null_mut::<MDB_txn>();
                mode = 0 as std::ffi::c_int as std::ffi::c_uint;
                if !((*env).me_txns).is_null() {
                    let mut sb: sembuf = {
                        sembuf {
                            sem_num: 0 as std::ffi::c_int as std::ffi::c_ushort,
                            sem_op: 1 as std::ffi::c_int as std::ffi::c_short,
                            sem_flg: 0o10000 as std::ffi::c_int as std::ffi::c_short,
                        }
                    };
                    sb.sem_num = (*((*env).me_wmutex).as_mut_ptr()).semnum as std::ffi::c_ushort;
                    *(*((*env).me_wmutex).as_mut_ptr()).locked = 0 as std::ffi::c_int;
                    semop(
                        (*((*env).me_wmutex).as_mut_ptr()).semid,
                        &mut sb,
                        1 as std::ffi::c_int as size_t,
                    );
                }
            } else {
                (*(*txn).mt_parent).mt_child = std::ptr::null_mut::<MDB_txn>();
                (*(*txn).mt_parent).mt_flags &= !(0x10 as std::ffi::c_int) as std::ffi::c_uint;
                // Note: not sure about the ownership transfer
                // (*env).me_pgstate = (*(txn as *mut MDB_ntxn)).mnt_pgstate;
                (*env).me_pgstate = MDB_pgstate {
                    mf_pghead: (*(txn as *mut MDB_ntxn)).mnt_pgstate.mf_pghead.take(),
                    mf_pglast: (*(txn as *mut MDB_ntxn)).mnt_pgstate.mf_pglast,
                };
                mdb_midl_free(std::mem::take(&mut (*txn).mt_free_pgs));
                free((*txn).mt_u.dirty_list as *mut std::ffi::c_void);
            }
            if let Some(spill_pgs) = (*txn).mt_spill_pgs.take() {
                mdb_midl_free(spill_pgs);
            }
            // Note: Freed above when owner assigned to None
            // mdb_midl_free(pghead);
        }
        if mode & 0x20 as std::ffi::c_int as std::ffi::c_uint != 0 {
            free(txn as *mut std::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_txn_reset(mut txn: *mut MDB_txn) {
    unsafe {
        if txn.is_null() {
            return;
        }
        if (*txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint == 0 {
            return;
        }
        mdb_txn_end(txn, MDB_END_RESET as std::ffi::c_int as std::ffi::c_uint);
    }
}
unsafe extern "C" fn _mdb_txn_abort(mut txn: *mut MDB_txn) {
    unsafe {
        if txn.is_null() {
            return;
        }
        if !((*txn).mt_child).is_null() {
            _mdb_txn_abort((*txn).mt_child);
        }
        mdb_txn_end(
            txn,
            (MDB_END_ABORT as std::ffi::c_int
                | 0x200000 as std::ffi::c_int
                | 0x20 as std::ffi::c_int) as std::ffi::c_uint,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_txn_abort(mut txn: *mut MDB_txn) {
    unsafe {
        _mdb_txn_abort(txn);
    }
}

/// Save the freelist as of this transaction to the freeDB.
/// This changes the freelist. Keep trying until it stabilizes.
///
/// When (MDB_DEVEL) & 2, the changes do not affect #mdb_page_alloc(),
/// it then uses the transaction's original snapshot of the freeDB.
unsafe extern "C" fn mdb_freelist_save(mut txn: *mut MDB_txn) -> std::ffi::c_int {
    unsafe {
        // env->me_pghead[] can grow and shrink during this call.
        // env->me_pglast and txn->mt_free_pgs[] can only grow.
        // Page numbers cannot disappear from txn->mt_free_pgs[].
        let mut mc: MDB_cursor = MDB_cursor {
            mc_next: std::ptr::null_mut::<MDB_cursor>(),
            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
            mc_txn: std::ptr::null_mut::<MDB_txn>(),
            mc_dbi: 0,
            mc_db: std::ptr::null_mut::<MDB_db>(),
            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
            mc_ki: [0; 32],
        };
        let mut env: *mut MDB_env = (*txn).mt_env;
        let mut rc: std::ffi::c_int = 0;
        let mut maxfree_1pg: std::ffi::c_int = (*env).me_maxfree_1pg;
        let mut more: std::ffi::c_int = 1 as std::ffi::c_int;
        let mut pglast: txnid_t = 0 as std::ffi::c_int as txnid_t;
        let mut head_id: txnid_t = 0 as std::ffi::c_int as txnid_t;
        let mut freecnt: pgno_t = 0 as std::ffi::c_int as pgno_t;
        let mut free_pgs: &mut MDB_IDL;
        let mut mop: Option<&mut MDB_IDL> = None;
        let mut head_room: ssize_t = 0 as std::ffi::c_int as ssize_t;
        let mut total_room: ssize_t = 0 as std::ffi::c_int as ssize_t;
        let mut mop_len: ssize_t = 0;
        let mut clean_limit: ssize_t = 0;

        mdb_cursor_init(
            &mut mc,
            txn,
            0 as std::ffi::c_int as MDB_dbi,
            std::ptr::null_mut::<MDB_xcursor>(),
        );

        if ((*env).me_pgstate.mf_pghead).is_some() {
            /* Make sure first page of freeDB is touched and on freelist */
            rc = mdb_page_search(
                &mut mc,
                std::ptr::null_mut::<MDB_val>(),
                4 as std::ffi::c_int | 1 as std::ffi::c_int,
            );
            if rc != 0 && rc != MDB_NOTFOUND {
                return rc;
            }
        }
        if ((*env).me_pgstate.mf_pghead).is_some() && !((*txn).mt_loose_pgs).is_null() {
            // Put loose page numbers in mt_free_pgs, since
            // we may be unable to return them to me_pghead.
            let mut mp: *mut MDB_page = (*txn).mt_loose_pgs;
            let mut dl: *mut MDB_ID2 = (*txn).mt_u.dirty_list;
            let mut x: std::ffi::c_uint = 0;
            rc = mdb_midl_need(&mut (*txn).mt_free_pgs, (*txn).mt_loose_count as usize);
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
            while !mp.is_null() {
                mdb_midl_xappend(&mut (*txn).mt_free_pgs, (*mp).mp_p.p_pgno);
                // must also remove from dirty list
                if (*txn).mt_flags & 0x80000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    x = 1 as std::ffi::c_int as std::ffi::c_uint;
                    while x as MDB_ID <= (*dl.offset(0 as std::ffi::c_int as isize)).mid {
                        if (*dl.offset(x as isize)).mid == (*mp).mp_p.p_pgno {
                            break;
                        }
                        x = x.wrapping_add(1);
                    }
                    if x as MDB_ID <= (*dl.offset(0 as std::ffi::c_int as isize)).mid {
                    } else {
                        mdb_assert_fail(
                            (*txn).mt_env,
                            b"x <= dl[0].mid\0" as *const u8 as *const std::ffi::c_char,
                            (*::core::mem::transmute::<&[u8; 18], &[std::ffi::c_char; 18]>(
                                b"mdb_freelist_save\0",
                            ))
                            .as_ptr(),
                            b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                            3547 as std::ffi::c_int,
                        );
                    };
                } else {
                    x = mdb_mid2l_search(dl, (*mp).mp_p.p_pgno);
                    if (*dl.offset(x as isize)).mid == (*mp).mp_p.p_pgno {
                    } else {
                        mdb_assert_fail(
                            (*txn).mt_env,
                            b"dl[x].mid == mp->mp_pgno\0" as *const u8 as *const std::ffi::c_char,
                            (*::core::mem::transmute::<&[u8; 18], &[std::ffi::c_char; 18]>(
                                b"mdb_freelist_save\0",
                            ))
                            .as_ptr(),
                            b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                            3550 as std::ffi::c_int,
                        );
                    };
                    mdb_dpage_free(env, mp);
                }
                let fresh8 = &mut (*dl.offset(x as isize)).mptr;
                *fresh8 = std::ptr::null_mut::<std::ffi::c_void>();
                mp = *(mp.offset(2 as std::ffi::c_int as isize) as *mut *mut MDB_page);
            }
            let mut y: std::ffi::c_uint = 0;
            y = 1 as std::ffi::c_int as std::ffi::c_uint;
            while !((*dl.offset(y as isize)).mptr).is_null()
                && y as MDB_ID <= (*dl.offset(0 as std::ffi::c_int as isize)).mid
            {
                y = y.wrapping_add(1);
            }
            if y as MDB_ID <= (*dl.offset(0 as std::ffi::c_int as isize)).mid {
                x = y;
                y = y.wrapping_add(1);
                loop {
                    while ((*dl.offset(y as isize)).mptr).is_null()
                        && y as MDB_ID <= (*dl.offset(0 as std::ffi::c_int as isize)).mid
                    {
                        y = y.wrapping_add(1);
                    }
                    if y as MDB_ID > (*dl.offset(0 as std::ffi::c_int as isize)).mid {
                        break;
                    }
                    let fresh9 = y;
                    y = y.wrapping_add(1);
                    let fresh10 = x;
                    x = x.wrapping_add(1);
                    *dl.offset(fresh10 as isize) = *dl.offset(fresh9 as isize);
                }
                (*dl.offset(0 as std::ffi::c_int as isize)).mid =
                    x.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as MDB_ID;
            } else {
                (*dl.offset(0 as std::ffi::c_int as isize)).mid = 0 as std::ffi::c_int as MDB_ID;
            }
            (*txn).mt_loose_pgs = std::ptr::null_mut::<MDB_page>();
            (*txn).mt_loose_count = 0 as std::ffi::c_int;
        }
        clean_limit = if (*env).me_flags
            & (0x1000000 as std::ffi::c_int | 0x80000 as std::ffi::c_int) as uint32_t
            != 0
        {
            0x7fffffffffffffff as std::ffi::c_long
        } else {
            maxfree_1pg as std::ffi::c_long
        };
        loop {
            /* Come back here after each Put() in case freelist changed */
            let mut key: MDB_val =
                MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
            let mut data: MDB_val =
                MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
            let mut pgs: *mut pgno_t = std::ptr::null_mut::<pgno_t>();
            let mut j: ssize_t = 0;

            // If using records from freeDB which we have not yet
            // deleted, delete them and any we reserved for me_pghead.
            while pglast < (*env).me_pgstate.mf_pglast {
                rc = mdb_cursor_first(&mut mc, &mut key, std::ptr::null_mut::<MDB_val>());
                if rc != 0 {
                    return rc;
                }
                head_id = *(key.mv_data as *mut txnid_t);
                pglast = head_id;
                head_room = 0 as std::ffi::c_int as ssize_t;
                total_room = head_room;
                if pglast <= (*env).me_pgstate.mf_pglast {
                } else {
                    mdb_assert_fail(
                        (*txn).mt_env,
                        b"pglast <= env->me_pglast\0" as *const u8 as *const std::ffi::c_char,
                        (*::core::mem::transmute::<&[u8; 18], &[std::ffi::c_char; 18]>(
                            b"mdb_freelist_save\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                        3594 as std::ffi::c_int,
                    );
                };
                rc = _mdb_cursor_del(&mut mc, 0 as std::ffi::c_int as std::ffi::c_uint);
                if rc != 0 {
                    return rc;
                }
            }

            // Save the IDL of pages freed by this txn, to a single record
            if freecnt < (*txn).mt_free_pgs.len() {
                if freecnt == 0 {
                    // Make sure last page of freeDB is touched and on freelist
                    rc = mdb_page_search(
                        &mut mc,
                        std::ptr::null_mut::<MDB_val>(),
                        8 as std::ffi::c_int | 1 as std::ffi::c_int,
                    );
                    if rc != 0 && rc != -(30798 as std::ffi::c_int) {
                        return rc;
                    }
                }
                free_pgs = &mut (*txn).mt_free_pgs;
                // Write to last page of freeDB
                key.mv_size = ::core::mem::size_of::<txnid_t>() as std::ffi::c_ulong;
                key.mv_data = &mut (*txn).mt_txnid as *mut txnid_t as *mut std::ffi::c_void;
                loop {
                    freecnt = free_pgs.len();
                    data.mv_size = free_pgs
                        .len()
                        .wrapping_add(1 as std::ffi::c_int as pgno_t)
                        .wrapping_mul(::core::mem::size_of::<MDB_ID>() as _)
                        as _;
                    rc = _mdb_cursor_put(&mut mc, &mut key, &mut data, MDB_RESERVE);
                    if rc != 0 {
                        return rc;
                    }
                    // Retry if mt_free_pgs[] grew during the Put()
                    free_pgs = &mut (*txn).mt_free_pgs;
                    if freecnt >= free_pgs.len() {
                        break;
                    }
                }
                mdb_midl_sort(free_pgs);
                // memcpy(data.mv_data, free_pgs as *const std::ffi::c_void, data.mv_size);
                free_pgs.copy_with_len_into_data(data);
            } else {
                mop = (*env).me_pgstate.mf_pghead.as_mut();
                // mop_len = (if !mop.is_null() {
                //     *mop.offset(0 as std::ffi::c_int as isize)
                // } else {
                //     0 as std::ffi::c_int as pgno_t
                // })
                // .wrapping_add((*txn).mt_loose_count as pgno_t) as ssize_t;
                mop_len = mop.as_ref().map_or(0, |m| m.len() as i64) + (*txn).mt_loose_count as i64;
                if total_room >= mop_len {
                    if total_room == mop_len || {
                        more -= 1;
                        more < 0 as std::ffi::c_int
                    } {
                        break;
                    }
                } else if head_room >= maxfree_1pg as ssize_t
                    && head_id > 1 as std::ffi::c_int as txnid_t
                {
                    head_id = head_id.wrapping_sub(1);
                    head_room = 0 as std::ffi::c_int as ssize_t;
                }
                total_room -= head_room;
                head_room = mop_len - total_room;
                if head_room > maxfree_1pg as ssize_t && head_id > 1 as std::ffi::c_int as txnid_t {
                    head_room = (head_room as txnid_t / head_id) as ssize_t as ssize_t;
                    head_room += maxfree_1pg as ssize_t
                        - head_room % (maxfree_1pg + 1 as std::ffi::c_int) as ssize_t;
                } else if head_room < 0 as std::ffi::c_int as ssize_t {
                    head_room = 0 as std::ffi::c_int as ssize_t;
                }
                key.mv_size = ::core::mem::size_of::<txnid_t>() as std::ffi::c_ulong;
                key.mv_data = &mut head_id as *mut txnid_t as *mut std::ffi::c_void;
                data.mv_size = ((head_room + 1 as std::ffi::c_int as ssize_t) as std::ffi::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong);
                rc = _mdb_cursor_put(
                    &mut mc,
                    &mut key,
                    &mut data,
                    0x10000 as std::ffi::c_int as std::ffi::c_uint,
                );
                if rc != 0 {
                    return rc;
                }
                pgs = data.mv_data as *mut pgno_t;
                j = if head_room > clean_limit {
                    head_room
                } else {
                    0 as std::ffi::c_int as ssize_t
                };
                loop {
                    *pgs.offset(j as isize) = 0 as std::ffi::c_int as pgno_t;
                    j -= 1;
                    if j < 0 as std::ffi::c_int as ssize_t {
                        break;
                    }
                }
                total_room += head_room;
            }
        }

        // Return loose page numbers to me_pghead, though usually none are
        // left at this point.  The pages themselves remain in dirty_list.
        if !((*txn).mt_loose_pgs).is_null()
            && let Some(mop) = mop.as_mut()
        {
            // Note: This block is tricky to read and understand, the reason is because
            //       they decided to use the tail of an MDB_IDL to temporarily store
            //       the loose pgno and finally xmerge them with the front of the MDB_IDL.
            //       I'll not do that for now and use a temp MDB_IDL.
            let mut mp: *mut MDB_page = (*txn).mt_loose_pgs;
            let mut loose = mdb_midl_alloc((*txn).mt_loose_count as usize).unwrap();

            while !mp.is_null() {
                mdb_midl_append(&mut loose, (*mp).mp_p.p_pgno);
                mp = *(mp.offset(2) as *mut *mut MDB_page);
            }

            mdb_midl_sort(&mut loose);
            mdb_midl_xmerge(mop, loose);
            (*txn).mt_loose_pgs = std::ptr::null_mut::<MDB_page>();
            (*txn).mt_loose_count = 0;
            mop_len = mop.len() as i64;
        }

        /* Fill in the reserved me_pghead records */
        rc = 0; // MDB_SUCCESS
        if let Some(mop) = mop.as_ref()
            && mop_len != 0
        {
            // Note: I rewrote this function to use a temporary buffer
            //       The reason is because the MDB_IDL should be changed to be shapped
            //       like the one in the original C library. They store the lenght of the
            //       IDL at the front of the MDB_IDs and use this property to directly
            //       memcpy into the mmap. In our case, we use a Vec and the length is
            //       not at the front of the allocation. This will help us avoid useless memcpys.
            let mut key = MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
            let mut data =
                MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
            let mut buffer = vec![0; 1 + mop_len as usize]; // len + MDB_IDs

            // We read the MDB_IDs from the back
            // mop = mop.offset(mop_len as isize);
            let mut mop_index = mop.len();
            rc = mdb_cursor_first(&mut mc, &mut key, &mut data);
            while rc == 0 {
                let mut id: txnid_t = (key.mv_data as *mut txnid_t).read();
                let mut len = (data.mv_size).wrapping_div(::core::mem::size_of::<MDB_ID>() as u64)
                    as ssize_t
                    - 1;
                // let mut save: MDB_ID = 0;

                // mdb_tassert
                if len >= 0 as ssize_t && id <= (*env).me_pgstate.mf_pglast {
                } else {
                    mdb_assert_fail(
                        (*txn).mt_env,
                        b"len >= 0 && id <= env->me_pglast\0" as *const u8
                            as *const std::ffi::c_char,
                        (*::core::mem::transmute::<&[u8; 18], &[std::ffi::c_char; 18]>(
                            b"mdb_freelist_save\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                        3710 as std::ffi::c_int,
                    );
                };
                key.mv_data = &mut id as *mut txnid_t as *mut std::ffi::c_void;
                if len > mop_len {
                    len = mop_len;
                    data.mv_size = (len as u64 + 1) * (::core::mem::size_of::<MDB_ID>() as u64);
                }
                // mop = mop.offset(-(len as isize));
                mop_index -= len as usize;
                // Note: this is the extra memcpy we need to get rid of
                // *mop.offset(0 as std::ffi::c_int as isize) = len as pgno_t;
                // buffer[0] = len as pgno_t; // already done below
                mop.copy_range_with_len_into_buffer(
                    mop_index..mop_index + len as usize,
                    &mut buffer[..len as usize + 1],
                );
                // data.mv_data = mop as *mut std::ffi::c_void;
                data.mv_data = buffer.as_ptr() as *mut std::ffi::c_void;
                // save = *mop.offset(0);
                rc = _mdb_cursor_put(&mut mc, &mut key, &mut data, MDB_CURRENT);
                // *mop.offset(0) = save;
                if rc != 0 || {
                    mop_len -= len;
                    mop_len == 0
                } {
                    break;
                }
                rc = mdb_cursor_next(&mut mc, &mut key, &mut data, MDB_NEXT);
            }
        }
        rc
    }
}
unsafe extern "C" fn mdb_page_flush(
    mut txn: *mut MDB_txn,
    mut keep: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut env: *mut MDB_env = (*txn).mt_env;
        let mut dl: MDB_ID2L = (*txn).mt_u.dirty_list;
        let mut psize: std::ffi::c_uint = (*env).me_psize;
        let mut j: std::ffi::c_uint = 0;
        let mut i: std::ffi::c_int = 0;
        let mut pagecount: std::ffi::c_int =
            (*dl.offset(0 as std::ffi::c_int as isize)).mid as std::ffi::c_int;
        let mut rc: std::ffi::c_int = 0;
        let mut size: size_t = 0 as std::ffi::c_int as size_t;
        let mut pos: off_t = 0 as std::ffi::c_int as off_t;
        let mut pgno: pgno_t = 0 as std::ffi::c_int as pgno_t;
        let mut dp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut iov: [iovec; 64] =
            [iovec { iov_base: std::ptr::null_mut::<std::ffi::c_void>(), iov_len: 0 }; 64];
        let mut fd: std::ffi::c_int = (*env).me_fd;
        let mut wsize: ssize_t = 0 as std::ffi::c_int as ssize_t;
        let mut wres: ssize_t = 0;
        let mut wpos: off_t = 0 as std::ffi::c_int as off_t;
        let mut next_pos: off_t = 1 as std::ffi::c_int as off_t;
        let mut n: std::ffi::c_int = 0 as std::ffi::c_int;
        i = keep;
        j = i as std::ffi::c_uint;
        if (*env).me_flags & 0x80000 as std::ffi::c_int as uint32_t != 0 {
            loop {
                i += 1;
                if i > pagecount {
                    break;
                }
                dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                if (*dp).mp_flags as std::ffi::c_int
                    & (0x4000 as std::ffi::c_int | 0x8000 as std::ffi::c_int)
                    != 0
                {
                    (*dp).mp_flags = ((*dp).mp_flags as std::ffi::c_int
                        & !(0x8000 as std::ffi::c_int))
                        as uint16_t;
                    j = j.wrapping_add(1);
                    *dl.offset(j as isize) = *dl.offset(i as isize);
                } else {
                    (*dp).mp_flags = ((*dp).mp_flags as std::ffi::c_int
                        & !(0x10 as std::ffi::c_int))
                        as uint16_t;
                }
            }
        } else {
            loop {
                i += 1;
                if i <= pagecount {
                    dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                    if (*dp).mp_flags as std::ffi::c_int
                        & (0x4000 as std::ffi::c_int | 0x8000 as std::ffi::c_int)
                        != 0
                    {
                        (*dp).mp_flags = ((*dp).mp_flags as std::ffi::c_int
                            & !(0x8000 as std::ffi::c_int))
                            as uint16_t;
                        (*dl.offset(i as isize)).mid = 0 as std::ffi::c_int as MDB_ID;
                        continue;
                    } else {
                        pgno = (*dl.offset(i as isize)).mid;
                        (*dp).mp_flags = ((*dp).mp_flags as std::ffi::c_int
                            & !(0x10 as std::ffi::c_int))
                            as uint16_t;
                        pos = (pgno * psize as pgno_t) as off_t;
                        size = psize as size_t;
                        if (*(dp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                            as std::ffi::c_int
                            & 0x4 as std::ffi::c_int
                            == 0x4 as std::ffi::c_int
                        {
                            size *= (*dp).mp_pb.pb_pages as size_t;
                        }
                    }
                }
                if pos != next_pos
                    || n == 64 as std::ffi::c_int
                    || (wsize as size_t).wrapping_add(size)
                        > (0x40000000 as std::ffi::c_uint
                            >> (::core::mem::size_of::<ssize_t>() as std::ffi::c_ulong
                                == 4 as std::ffi::c_int as std::ffi::c_ulong)
                                as std::ffi::c_int) as size_t
                {
                    if n != 0 {
                        's_208: {
                            loop {
                                if n == 1 as std::ffi::c_int {
                                    wres = pwrite(
                                        fd,
                                        iov[0 as std::ffi::c_int as usize].iov_base,
                                        wsize as size_t,
                                        wpos,
                                    );
                                } else {
                                    's_156: {
                                        loop {
                                            if lseek(fd, wpos, 0 as std::ffi::c_int)
                                                == -(1 as std::ffi::c_int) as off_t
                                            {
                                                rc = *__error();
                                                if rc == 4 as std::ffi::c_int {
                                                    continue;
                                                }
                                                return rc;
                                            } else {
                                                break 's_156;
                                            }
                                        }
                                    }
                                    wres = writev(fd, iov.as_mut_ptr(), n);
                                }
                                if wres != wsize {
                                    if wres < 0 as std::ffi::c_int as ssize_t {
                                        rc = *__error();
                                        if rc != 4 as std::ffi::c_int {
                                            break;
                                        }
                                    } else {
                                        rc = 5 as std::ffi::c_int;
                                        break;
                                    }
                                } else {
                                    n = 0 as std::ffi::c_int;
                                    break 's_208;
                                }
                            }
                            return rc;
                        }
                    }
                    if i > pagecount {
                        break;
                    }
                    wpos = pos;
                    wsize = 0 as std::ffi::c_int as ssize_t;
                }
                iov[n as usize].iov_len = size;
                iov[n as usize].iov_base = dp as *mut std::ffi::c_char as *mut std::ffi::c_void;
                next_pos = (pos as std::ffi::c_ulonglong)
                    .wrapping_add(size as std::ffi::c_ulonglong)
                    as off_t;
                wsize = (wsize as size_t).wrapping_add(size) as ssize_t as ssize_t;
                n += 1;
            }
            if (*env).me_flags & 0x80000 as std::ffi::c_int as uint32_t == 0 {
                i = keep;
                loop {
                    i += 1;
                    if i > pagecount {
                        break;
                    }
                    dp = (*dl.offset(i as isize)).mptr as *mut MDB_page;
                    if (*dl.offset(i as isize)).mid == 0 {
                        j = j.wrapping_add(1);
                        *dl.offset(j as isize) = *dl.offset(i as isize);
                        (*dl.offset(j as isize)).mid = (*dp).mp_p.p_pgno;
                    } else {
                        mdb_dpage_free(env, dp);
                    }
                }
            }
        }
        i -= 1;
        (*txn).mt_dirty_room =
            ((*txn).mt_dirty_room).wrapping_add((i as std::ffi::c_uint).wrapping_sub(j));
        (*dl.offset(0 as std::ffi::c_int as isize)).mid = j as MDB_ID;
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn _mdb_txn_commit(mut txn: *mut MDB_txn) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut rc: std::ffi::c_int = 0;
        let mut i: std::ffi::c_uint = 0;
        let mut end_mode: std::ffi::c_uint = 0;
        let mut env: *mut MDB_env = std::ptr::null_mut::<MDB_env>();
        if txn.is_null() {
            return 22 as std::ffi::c_int;
        }
        end_mode = (MDB_END_EMPTY_COMMIT as std::ffi::c_int
            | 0x10 as std::ffi::c_int
            | 0x200000 as std::ffi::c_int
            | 0x20 as std::ffi::c_int) as std::ffi::c_uint;
        if !((*txn).mt_child).is_null() {
            rc = _mdb_txn_commit((*txn).mt_child);
            if rc != 0 {
                current_block = 9928889463419475167;
            } else {
                current_block = 11875828834189669668;
            }
        } else {
            current_block = 11875828834189669668;
        }
        if current_block == 11875828834189669668 {
            env = (*txn).mt_env;
            if (*txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint
                == 0x20000 as std::ffi::c_int as std::ffi::c_uint
            {
                current_block = 9264303918316504035;
            } else if (*txn).mt_flags
                & (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint
                != 0
            {
                if !((*txn).mt_parent).is_null() {
                    (*(*txn).mt_parent).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                }
                rc = -(30782 as std::ffi::c_int);
                current_block = 9928889463419475167;
            } else if !((*txn).mt_parent).is_null() {
                let mut parent: *mut MDB_txn = (*txn).mt_parent;
                let mut lp: *mut *mut MDB_page = std::ptr::null_mut::<*mut MDB_page>();
                let mut dst: MDB_ID2L = std::ptr::null_mut::<MDB_ID2>();
                let mut src: MDB_ID2L = std::ptr::null_mut::<MDB_ID2>();
                let mut pspill: Option<&mut MDB_IDL> = None;
                let mut x: std::ffi::c_uint = 0;
                let mut y: std::ffi::c_uint = 0;
                let mut len: std::ffi::c_uint = 0;
                let mut ps_len: std::ffi::c_uint = 0;
                rc = mdb_midl_append_list(
                    &mut (*parent).mt_free_pgs,
                    std::mem::take(&mut (*txn).mt_free_pgs),
                );
                if rc != 0 {
                    current_block = 9928889463419475167;
                } else {
                    mdb_midl_free(std::mem::take(&mut (*txn).mt_free_pgs));
                    (*parent).mt_next_pgno = (*txn).mt_next_pgno;
                    (*parent).mt_flags = (*txn).mt_flags;
                    mdb_cursors_close(txn, 1 as std::ffi::c_int as std::ffi::c_uint);
                    memcpy(
                        (*parent).mt_dbs as *mut std::ffi::c_void,
                        (*txn).mt_dbs as *const std::ffi::c_void,
                        ((*txn).mt_numdbs as std::ffi::c_ulong)
                            .wrapping_mul(::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong),
                    );
                    (*parent).mt_numdbs = (*txn).mt_numdbs;
                    *((*parent).mt_dbflags).offset(0 as std::ffi::c_int as isize) =
                        *((*txn).mt_dbflags).offset(0 as std::ffi::c_int as isize);
                    *((*parent).mt_dbflags).offset(1 as std::ffi::c_int as isize) =
                        *((*txn).mt_dbflags).offset(1 as std::ffi::c_int as isize);
                    i = 2 as std::ffi::c_int as std::ffi::c_uint;
                    while i < (*txn).mt_numdbs {
                        x = (*((*parent).mt_dbflags).offset(i as isize) as std::ffi::c_int
                            & 0x4 as std::ffi::c_int)
                            as std::ffi::c_uint;
                        *((*parent).mt_dbflags).offset(i as isize) =
                            (*((*txn).mt_dbflags).offset(i as isize) as std::ffi::c_uint | x)
                                as std::ffi::c_uchar;
                        i = i.wrapping_add(1);
                    }
                    dst = (*parent).mt_u.dirty_list;
                    src = (*txn).mt_u.dirty_list;
                    pspill = (*parent).mt_spill_pgs.as_mut();
                    /* Remove anything in our dirty list from parent's spill list */
                    if let Some(pspill) = pspill
                        && {
                            ps_len = pspill.len() as u32;
                            ps_len != 0
                        }
                    {
                        y = ps_len;
                        x = y;
                        // Note: This is not optimal, but it's a workaround to make this part
                        //       more understandable for now. We will optimize it later.
                        pspill.retain(|&id| {
                            (1..=(*src.offset(0)).mid).any(|i| {
                                let pn = (*src.offset(i as isize)).mid << 1;
                                pn == id
                            })
                        });
                    }

                    /* Remove anything in our spill list from parent's dirty list */
                    if let Some(spill_pgs) =
                        (*txn).mt_spill_pgs.as_mut().filter(|pgs| !pgs.is_empty())
                    {
                        i = 1 as std::ffi::c_int as std::ffi::c_uint;
                        while i as MDB_ID <= spill_pgs.len() {
                            let mut pn_0: MDB_ID = spill_pgs[i as usize];
                            if pn_0 & 1 as std::ffi::c_int as MDB_ID == 0 {
                                pn_0 >>= 1 as std::ffi::c_int;
                                y = mdb_mid2l_search(dst, pn_0);
                                if y as MDB_ID <= (*dst.offset(0 as std::ffi::c_int as isize)).mid
                                    && (*dst.offset(y as isize)).mid == pn_0
                                {
                                    free((*dst.offset(y as isize)).mptr);
                                    while (y as MDB_ID)
                                        < (*dst.offset(0 as std::ffi::c_int as isize)).mid
                                    {
                                        *dst.offset(y as isize) = *dst.offset(
                                            y.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint)
                                                as isize,
                                        );
                                        y = y.wrapping_add(1);
                                    }
                                    let fresh11 =
                                        &mut (*dst.offset(0 as std::ffi::c_int as isize)).mid;
                                    *fresh11 = (*fresh11).wrapping_sub(1);
                                }
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                    x = (*dst.offset(0 as std::ffi::c_int as isize)).mid as std::ffi::c_uint;
                    (*dst.offset(0 as std::ffi::c_int as isize)).mid =
                        0 as std::ffi::c_int as MDB_ID;
                    if !((*parent).mt_parent).is_null() {
                        len = (x as MDB_ID)
                            .wrapping_add((*src.offset(0 as std::ffi::c_int as isize)).mid)
                            as std::ffi::c_uint;
                        y = (mdb_mid2l_search(
                            src,
                            ((*dst.offset(x as isize)).mid)
                                .wrapping_add(1 as std::ffi::c_int as MDB_ID),
                        ))
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint);
                        i = x;
                        while y != 0 && i != 0 {
                            let mut yp: pgno_t = (*src.offset(y as isize)).mid;
                            while yp < (*dst.offset(i as isize)).mid {
                                i = i.wrapping_sub(1);
                            }
                            if yp == (*dst.offset(i as isize)).mid {
                                i = i.wrapping_sub(1);
                                len = len.wrapping_sub(1);
                            }
                            y = y.wrapping_sub(1);
                        }
                    } else {
                        len = ((((1 as std::ffi::c_int)
                            << (16 as std::ffi::c_int + 1 as std::ffi::c_int))
                            - 1 as std::ffi::c_int)
                            as std::ffi::c_uint)
                            .wrapping_sub((*txn).mt_dirty_room);
                    }
                    y = (*src.offset(0 as std::ffi::c_int as isize)).mid as std::ffi::c_uint;
                    i = len;
                    while y != 0 {
                        let mut yp_0: pgno_t = (*src.offset(y as isize)).mid;
                        while yp_0 < (*dst.offset(x as isize)).mid {
                            let fresh12 = x;
                            x = x.wrapping_sub(1);
                            let fresh13 = i;
                            i = i.wrapping_sub(1);
                            *dst.offset(fresh13 as isize) = *dst.offset(fresh12 as isize);
                        }
                        if yp_0 == (*dst.offset(x as isize)).mid {
                            let fresh14 = x;
                            x = x.wrapping_sub(1);
                            free((*dst.offset(fresh14 as isize)).mptr);
                        }
                        let fresh15 = y;
                        y = y.wrapping_sub(1);
                        let fresh16 = i;
                        i = i.wrapping_sub(1);
                        *dst.offset(fresh16 as isize) = *src.offset(fresh15 as isize);
                    }
                    if i == x {
                    } else {
                        mdb_assert_fail(
                            (*txn).mt_env,
                            b"i == x\0" as *const u8 as *const std::ffi::c_char,
                            (*::core::mem::transmute::<&[u8; 16], &[std::ffi::c_char; 16]>(
                                b"_mdb_txn_commit\0",
                            ))
                            .as_ptr(),
                            b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                            4101 as std::ffi::c_int,
                        );
                    };
                    (*dst.offset(0 as std::ffi::c_int as isize)).mid = len as MDB_ID;
                    free((*txn).mt_u.dirty_list as *mut std::ffi::c_void);
                    (*parent).mt_dirty_room = (*txn).mt_dirty_room;
                    if let Some(spill_pgs) = (*txn).mt_spill_pgs.take() {
                        if let Some(parent_spill_pgs) = (*parent).mt_spill_pgs.as_mut() {
                            rc = mdb_midl_append_list(parent_spill_pgs, spill_pgs);
                            if rc != 0 {
                                (*parent).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                            }
                            // Note: Already freed above
                            // mdb_midl_free((*txn).mt_spill_pgs);
                            mdb_midl_sort(parent_spill_pgs);
                        } else {
                            // Note: not sure about the ownership transfer
                            (*parent).mt_spill_pgs = (*txn).mt_spill_pgs.take();
                        }
                    }
                    lp = &mut (*parent).mt_loose_pgs;
                    while !(*lp).is_null() {
                        lp = (*lp).offset(2 as std::ffi::c_int as isize) as *mut *mut MDB_page;
                    }
                    *lp = (*txn).mt_loose_pgs;
                    (*parent).mt_loose_count += (*txn).mt_loose_count;
                    (*parent).mt_child = std::ptr::null_mut::<MDB_txn>();
                    if let Some(pghead) = (*(txn as *mut MDB_ntxn)).mnt_pgstate.mf_pghead.take() {
                        mdb_midl_free(pghead);
                    }
                    free(txn as *mut std::ffi::c_void);
                    return rc;
                }
            } else if txn != (*env).me_txn {
                rc = 22 as std::ffi::c_int;
                current_block = 9928889463419475167;
            } else {
                mdb_cursors_close(txn, 0 as std::ffi::c_int as std::ffi::c_uint);
                if (*((*txn).mt_u.dirty_list).offset(0 as std::ffi::c_int as isize)).mid == 0
                    && (*txn).mt_flags
                        & (0x4 as std::ffi::c_int | 0x8 as std::ffi::c_int) as std::ffi::c_uint
                        == 0
                {
                    current_block = 9264303918316504035;
                } else {
                    if (*txn).mt_numdbs > 2 as std::ffi::c_int as MDB_dbi {
                        let mut mc: MDB_cursor = MDB_cursor {
                            mc_next: std::ptr::null_mut::<MDB_cursor>(),
                            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                            mc_txn: std::ptr::null_mut::<MDB_txn>(),
                            mc_dbi: 0,
                            mc_db: std::ptr::null_mut::<MDB_db>(),
                            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                            mc_snum: 0,
                            mc_top: 0,
                            mc_flags: 0,
                            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                            mc_ki: [0; 32],
                        };
                        let mut i_0: MDB_dbi = 0;
                        let mut data: MDB_val = MDB_val {
                            mv_size: 0,
                            mv_data: std::ptr::null_mut::<std::ffi::c_void>(),
                        };
                        data.mv_size = ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong;
                        mdb_cursor_init(
                            &mut mc,
                            txn,
                            1 as std::ffi::c_int as MDB_dbi,
                            std::ptr::null_mut::<MDB_xcursor>(),
                        );
                        i_0 = 2 as std::ffi::c_int as MDB_dbi;
                        loop {
                            if i_0 >= (*txn).mt_numdbs {
                                current_block = 8656139126282042408;
                                break;
                            }
                            if *((*txn).mt_dbflags).offset(i_0 as isize) as std::ffi::c_int
                                & 0x1 as std::ffi::c_int
                                != 0
                            {
                                if *((*txn).mt_dbiseqs).offset(i_0 as isize)
                                    != *((*(*txn).mt_env).me_dbiseqs).offset(i_0 as isize)
                                {
                                    rc = -(30780 as std::ffi::c_int);
                                    current_block = 9928889463419475167;
                                    break;
                                } else {
                                    data.mv_data = &mut *((*txn).mt_dbs).offset(i_0 as isize)
                                        as *mut MDB_db
                                        as *mut std::ffi::c_void;
                                    rc = _mdb_cursor_put(
                                        &mut mc,
                                        &mut (*((*txn).mt_dbxs).offset(i_0 as isize)).md_name,
                                        &mut data,
                                        0x2 as std::ffi::c_int as std::ffi::c_uint,
                                    );
                                    if rc != 0 {
                                        current_block = 9928889463419475167;
                                        break;
                                    }
                                }
                            }
                            i_0 = i_0.wrapping_add(1);
                        }
                    } else {
                        current_block = 8656139126282042408;
                    }
                    match current_block {
                        9928889463419475167 => {}
                        _ => {
                            rc = mdb_freelist_save(txn);
                            if rc != 0 {
                                current_block = 9928889463419475167;
                            } else {
                                if let Some(pghead) = (*env).me_pgstate.mf_pghead.take() {
                                    mdb_midl_free(pghead);
                                    // (*env).me_pgstate.mf_pghead = std::ptr::null_mut::<pgno_t>();
                                }
                                mdb_midl_shrink(&mut (*txn).mt_free_pgs);
                                rc = mdb_page_flush(txn, 0 as std::ffi::c_int);
                                if rc != 0 {
                                    current_block = 9928889463419475167;
                                } else if ((*txn).mt_flags
                                    & 0x10000 as std::ffi::c_int as std::ffi::c_uint
                                    != 0x10000 as std::ffi::c_int as std::ffi::c_uint)
                                    && {
                                        rc = mdb_env_sync0(
                                            env,
                                            0 as std::ffi::c_int,
                                            (*txn).mt_next_pgno,
                                        );
                                        rc != 0
                                    }
                                {
                                    current_block = 9928889463419475167;
                                } else {
                                    rc = mdb_env_write_meta(txn);
                                    if rc != 0 {
                                        current_block = 9928889463419475167;
                                    } else {
                                        end_mode = (MDB_END_COMMITTED as std::ffi::c_int
                                            | 0x10 as std::ffi::c_int)
                                            as std::ffi::c_uint;
                                        if (*env).me_flags
                                            & 0x2000000 as std::ffi::c_int as uint32_t
                                            != 0
                                        {
                                            if (*env).me_flags
                                                & 0x400000 as std::ffi::c_int as uint32_t
                                                == 0
                                            {
                                                let mut excl: std::ffi::c_int = 0;
                                                rc = mdb_env_share_locks(env, &mut excl);
                                                if rc != 0 {
                                                    current_block = 9928889463419475167;
                                                } else {
                                                    current_block = 1604201581803946138;
                                                }
                                            } else {
                                                current_block = 1604201581803946138;
                                            }
                                            match current_block {
                                                9928889463419475167 => {}
                                                _ => {
                                                    (*env).me_flags ^=
                                                        0x2000000 as std::ffi::c_int as uint32_t;
                                                    current_block = 9264303918316504035;
                                                }
                                            }
                                        } else {
                                            current_block = 9264303918316504035;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match current_block {
                9928889463419475167 => {}
                _ => {
                    mdb_txn_end(txn, end_mode);
                    return 0 as std::ffi::c_int;
                }
            }
        }
        _mdb_txn_abort(txn);
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_txn_commit(mut txn: *mut MDB_txn) -> std::ffi::c_int {
    unsafe { _mdb_txn_commit(txn) }
}
#[cold]
unsafe extern "C" fn mdb_env_read_header(
    mut env: *mut MDB_env,
    mut prev: std::ffi::c_int,
    mut meta: *mut MDB_meta,
) -> std::ffi::c_int {
    unsafe {
        let mut pbuf: MDB_metabuf = MDB_metabuf {
            mb_page: MDB_page {
                mp_p: C2RustUnnamed_1 { p_pgno: 0 },
                mp_pad: 0,
                mp_flags: 0,
                mp_pb: C2RustUnnamed { pb: C2RustUnnamed_0 { pb_lower: 0, pb_upper: 0 } },
                mp_ptrs: [0; 0],
            },
        };
        let mut p: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut m: *mut MDB_meta = std::ptr::null_mut::<MDB_meta>();
        let mut i: std::ffi::c_int = 0;
        let mut rc: std::ffi::c_int = 0;
        let mut off: std::ffi::c_int = 0;
        off = 0 as std::ffi::c_int;
        i = off;
        while i < 2 as std::ffi::c_int {
            rc = pread(
                (*env).me_fd,
                &mut pbuf as *mut MDB_metabuf as *mut std::ffi::c_void,
                Size as std::ffi::c_int as size_t,
                off as off_t,
            ) as std::ffi::c_int;
            if rc != Size as std::ffi::c_int {
                if rc == 0 as std::ffi::c_int && off == 0 as std::ffi::c_int {
                    return 2 as std::ffi::c_int;
                }
                rc = if rc < 0 as std::ffi::c_int {
                    *__error()
                } else {
                    -(30793 as std::ffi::c_int)
                };
                return rc;
            }
            p = &mut pbuf as *mut MDB_metabuf as *mut MDB_page;
            if (*p).mp_flags as std::ffi::c_int & 0x8 as std::ffi::c_int != 0x8 as std::ffi::c_int {
                return -(30793 as std::ffi::c_int);
            }
            m = (p as *mut std::ffi::c_char)
                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                as *mut std::ffi::c_void as *mut MDB_meta;
            if (*m).mm_magic != 0xbeefc0de as std::ffi::c_uint {
                return -(30793 as std::ffi::c_int);
            }
            if (*m).mm_version
                != (if 0 as std::ffi::c_int != 0 {
                    999 as std::ffi::c_int
                } else {
                    1 as std::ffi::c_int
                }) as uint32_t
            {
                return -(30794 as std::ffi::c_int);
            }
            if off == 0 as std::ffi::c_int
                || (if prev != 0 {
                    ((*m).mm_txnid < (*meta).mm_txnid) as std::ffi::c_int
                } else {
                    ((*m).mm_txnid > (*meta).mm_txnid) as std::ffi::c_int
                }) != 0
            {
                *meta = *m;
            }
            i += 1;
            off = (off as uint32_t)
                .wrapping_add((*meta).mm_dbs[0 as std::ffi::c_int as usize].md_pad)
                as std::ffi::c_int as std::ffi::c_int;
        }
        0 as std::ffi::c_int
    }
}
#[cold]
unsafe extern "C" fn mdb_env_init_meta0(mut env: *mut MDB_env, mut meta: *mut MDB_meta) {
    unsafe {
        (*meta).mm_magic = 0xbeefc0de as std::ffi::c_uint;
        (*meta).mm_version =
            (if 0 as std::ffi::c_int != 0 { 999 as std::ffi::c_int } else { 1 as std::ffi::c_int })
                as uint32_t;
        (*meta).mm_mapsize = (*env).me_mapsize;
        (*meta).mm_dbs[0 as std::ffi::c_int as usize].md_pad = (*env).me_psize;
        (*meta).mm_last_pg = (2 as std::ffi::c_int - 1 as std::ffi::c_int) as pgno_t;
        (*meta).mm_dbs[0 as std::ffi::c_int as usize].md_flags =
            ((*env).me_flags & 0xffff as std::ffi::c_int as uint32_t) as uint16_t;
        (*meta).mm_dbs[0 as std::ffi::c_int as usize].md_flags =
            ((*meta).mm_dbs[0 as std::ffi::c_int as usize].md_flags as std::ffi::c_int
                | 0x8 as std::ffi::c_int) as uint16_t;
        (*meta).mm_dbs[0 as std::ffi::c_int as usize].md_root = !(0 as std::ffi::c_int as pgno_t);
        (*meta).mm_dbs[1 as std::ffi::c_int as usize].md_root = !(0 as std::ffi::c_int as pgno_t);
    }
}
#[cold]
unsafe extern "C" fn mdb_env_init_meta(
    mut env: *mut MDB_env,
    mut meta: *mut MDB_meta,
) -> std::ffi::c_int {
    unsafe {
        let mut p: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut q: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut rc: std::ffi::c_int = 0;
        let mut psize: std::ffi::c_uint = 0;
        let mut len: std::ffi::c_int = 0;
        psize = (*env).me_psize;
        p = calloc(2 as std::ffi::c_int as std::ffi::c_ulong, psize as std::ffi::c_ulong)
            as *mut MDB_page;
        if p.is_null() {
            return 12 as std::ffi::c_int;
        }
        (*p).mp_p.p_pgno = 0 as std::ffi::c_int as pgno_t;
        (*p).mp_flags = 0x8 as std::ffi::c_int as uint16_t;
        *((p as *mut std::ffi::c_char).offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
            as *mut std::ffi::c_void as *mut MDB_meta) = *meta;
        q = (p as *mut std::ffi::c_char).offset(psize as isize) as *mut MDB_page;
        (*q).mp_p.p_pgno = 1 as std::ffi::c_int as pgno_t;
        (*q).mp_flags = 0x8 as std::ffi::c_int as uint16_t;
        *((q as *mut std::ffi::c_char).offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
            as *mut std::ffi::c_void as *mut MDB_meta) = *meta;
        loop {
            len = pwrite(
                (*env).me_fd,
                p as *const std::ffi::c_void,
                psize.wrapping_mul(2 as std::ffi::c_int as std::ffi::c_uint) as size_t,
                0 as std::ffi::c_int as off_t,
            ) as std::ffi::c_int;
            if len == -(1 as std::ffi::c_int) && *__error() == 4 as std::ffi::c_int {
                continue;
            }
            rc = (len >= 0 as std::ffi::c_int) as std::ffi::c_int;
            break;
        }
        if rc == 0 {
            rc = *__error();
        } else if len as std::ffi::c_uint
            == psize.wrapping_mul(2 as std::ffi::c_int as std::ffi::c_uint)
        {
            rc = 0 as std::ffi::c_int;
        } else {
            rc = 28 as std::ffi::c_int;
        }
        free(p as *mut std::ffi::c_void);
        rc
    }
}
unsafe extern "C" fn mdb_env_write_meta(mut txn: *mut MDB_txn) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut env: *mut MDB_env = std::ptr::null_mut::<MDB_env>();
        let mut meta: MDB_meta = MDB_meta {
            mm_magic: 0,
            mm_version: 0,
            mm_address: std::ptr::null_mut::<std::ffi::c_void>(),
            mm_mapsize: 0,
            mm_dbs: [MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            }; 2],
            mm_last_pg: 0,
            mm_txnid: 0,
        };
        let mut metab: MDB_meta = MDB_meta {
            mm_magic: 0,
            mm_version: 0,
            mm_address: std::ptr::null_mut::<std::ffi::c_void>(),
            mm_mapsize: 0,
            mm_dbs: [MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            }; 2],
            mm_last_pg: 0,
            mm_txnid: 0,
        };
        let mut mp: *mut MDB_meta = std::ptr::null_mut::<MDB_meta>();
        let mut flags: std::ffi::c_uint = 0;
        let mut mapsize: mdb_size_t = 0;
        let mut off: off_t = 0;
        let mut rc: std::ffi::c_int = 0;
        let mut len: std::ffi::c_int = 0;
        let mut toggle: std::ffi::c_int = 0;
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut mfd: std::ffi::c_int = 0;
        let mut r2: std::ffi::c_int = 0;
        toggle = ((*txn).mt_txnid & 1 as std::ffi::c_int as txnid_t) as std::ffi::c_int;
        env = (*txn).mt_env;
        flags = (*txn).mt_flags | (*env).me_flags;
        mp = (*env).me_metas[toggle as usize];
        mapsize = (*(*env).me_metas[(toggle ^ 1 as std::ffi::c_int) as usize]).mm_mapsize;
        if mapsize < (*env).me_mapsize {
            mapsize = (*env).me_mapsize;
        }
        if flags & 0x80000 as std::ffi::c_int as std::ffi::c_uint != 0 {
            (*mp).mm_mapsize = mapsize;
            (*mp).mm_dbs[0 as std::ffi::c_int as usize] =
                *((*txn).mt_dbs).offset(0 as std::ffi::c_int as isize);
            (*mp).mm_dbs[1 as std::ffi::c_int as usize] =
                *((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize);
            (*mp).mm_last_pg = ((*txn).mt_next_pgno).wrapping_sub(1 as std::ffi::c_int as pgno_t);
            ::core::ptr::write_volatile(&mut (*mp).mm_txnid as *mut txnid_t, (*txn).mt_txnid);
            if flags & (0x40000 as std::ffi::c_int | 0x10000 as std::ffi::c_int) as std::ffi::c_uint
                == 0
            {
                let mut meta_size: std::ffi::c_uint = (*env).me_psize;
                rc = if (*env).me_flags & 0x100000 as std::ffi::c_int as uint32_t != 0 {
                    0x1 as std::ffi::c_int
                } else {
                    0x10 as std::ffi::c_int
                };
                ptr = (mp as *mut std::ffi::c_char)
                    .offset(-(16 as std::ffi::c_ulong as std::ffi::c_uint as isize));
                r2 = (ptr.offset_from((*env).me_map) as std::ffi::c_long
                    & ((*env).me_os_psize).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                        as std::ffi::c_long) as std::ffi::c_int;
                ptr = ptr.offset(-(r2 as isize));
                meta_size = meta_size.wrapping_add(r2 as std::ffi::c_uint);
                if msync(ptr as *mut std::ffi::c_void, meta_size as size_t, rc) != 0 {
                    rc = *__error();
                    current_block = 8566817178116531278;
                } else {
                    current_block = 15483493740570082785;
                }
            } else {
                current_block = 15483493740570082785;
            }
        } else {
            ::core::ptr::write_volatile(&mut metab.mm_txnid as *mut txnid_t, (*mp).mm_txnid);
            metab.mm_last_pg = (*mp).mm_last_pg;
            meta.mm_mapsize = mapsize;
            meta.mm_dbs[0 as std::ffi::c_int as usize] =
                *((*txn).mt_dbs).offset(0 as std::ffi::c_int as isize);
            meta.mm_dbs[1 as std::ffi::c_int as usize] =
                *((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize);
            meta.mm_last_pg = ((*txn).mt_next_pgno).wrapping_sub(1 as std::ffi::c_int as pgno_t);
            ::core::ptr::write_volatile(&mut meta.mm_txnid as *mut txnid_t, (*txn).mt_txnid);
            off = 16 as std::ffi::c_ulong as off_t;
            ptr = (&mut meta as *mut MDB_meta as *mut std::ffi::c_char).offset(off as isize);
            len = (::core::mem::size_of::<MDB_meta>() as std::ffi::c_ulong as std::ffi::c_ulonglong)
                .wrapping_sub(off as std::ffi::c_ulonglong) as std::ffi::c_int;
            off += (mp as *mut std::ffi::c_char).offset_from((*env).me_map) as std::ffi::c_long
                as off_t;
            mfd = if flags
                & (0x10000 as std::ffi::c_int | 0x40000 as std::ffi::c_int) as std::ffi::c_uint
                != 0
            {
                (*env).me_fd
            } else {
                (*env).me_mfd
            };
            loop {
                rc = pwrite(mfd, ptr as *const std::ffi::c_void, len as size_t, off)
                    as std::ffi::c_int;
                if rc == len {
                    current_block = 15483493740570082785;
                    break;
                }
                rc = if rc < 0 as std::ffi::c_int { *__error() } else { 5 as std::ffi::c_int };
                if rc == 4 as std::ffi::c_int {
                    continue;
                }
                meta.mm_last_pg = metab.mm_last_pg;
                ::core::ptr::write_volatile(&mut meta.mm_txnid as *mut txnid_t, metab.mm_txnid);
                r2 = pwrite((*env).me_fd, ptr as *const std::ffi::c_void, len as size_t, off)
                    as std::ffi::c_int;
                current_block = 8566817178116531278;
                break;
            }
        }
        match current_block {
            15483493740570082785 => {
                if !((*env).me_txns).is_null() {
                    ::core::ptr::write_volatile(
                        &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
                        (*txn).mt_txnid,
                    );
                }
                0 as std::ffi::c_int
            }
            _ => {
                (*env).me_flags |= 0x80000000 as std::ffi::c_uint;
                rc
            }
        }
    }
}
unsafe extern "C" fn mdb_env_pick_meta(mut env: *const MDB_env) -> *mut MDB_meta {
    unsafe {
        let mut metas: *const *mut MDB_meta = ((*env).me_metas).as_ptr();
        *metas.offset(
            (((**metas.offset(0 as std::ffi::c_int as isize)).mm_txnid
                < (**metas.offset(1 as std::ffi::c_int as isize)).mm_txnid)
                as std::ffi::c_int
                ^ ((*env).me_flags & 0x2000000 as std::ffi::c_int as uint32_t
                    != 0 as std::ffi::c_int as uint32_t) as std::ffi::c_int) as isize,
        )
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_create(mut env: *mut *mut MDB_env) -> std::ffi::c_int {
    unsafe {
        let mut e: *mut MDB_env = std::ptr::null_mut::<MDB_env>();
        e = calloc(
            1 as std::ffi::c_int as std::ffi::c_ulong,
            ::core::mem::size_of::<MDB_env>() as std::ffi::c_ulong,
        ) as *mut MDB_env;
        if e.is_null() {
            return 12 as std::ffi::c_int;
        }
        (*e).me_maxreaders = 126 as std::ffi::c_int as std::ffi::c_uint;
        (*e).me_numdbs = 2 as std::ffi::c_int as MDB_dbi;
        (*e).me_maxdbs = (*e).me_numdbs;
        (*e).me_fd = -(1 as std::ffi::c_int);
        (*e).me_lfd = -(1 as std::ffi::c_int);
        (*e).me_mfd = -(1 as std::ffi::c_int);
        (*((*e).me_rmutex).as_mut_ptr()).semid = -(1 as std::ffi::c_int);
        (*((*e).me_wmutex).as_mut_ptr()).semid = -(1 as std::ffi::c_int);
        (*e).me_pid = getpid();
        (*e).me_os_psize = sysconf(29 as std::ffi::c_int) as std::ffi::c_uint;
        *env = e;
        0 as std::ffi::c_int
    }
}
#[cold]
unsafe extern "C" fn mdb_env_map(
    mut env: *mut MDB_env,
    mut addr: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    unsafe {
        let mut p: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut flags: std::ffi::c_uint = (*env).me_flags;
        let mut mmap_flags: std::ffi::c_int = 0x1 as std::ffi::c_int;
        let mut prot: std::ffi::c_int = 0x1 as std::ffi::c_int;
        if flags & 0x80000 as std::ffi::c_int as std::ffi::c_uint != 0 {
            prot |= 0x2 as std::ffi::c_int;
            if ftruncate((*env).me_fd, (*env).me_mapsize as off_t) < 0 as std::ffi::c_int {
                return *__error();
            }
        }
        (*env).me_map = mmap(
            addr,
            (*env).me_mapsize as _,
            prot,
            mmap_flags,
            (*env).me_fd,
            0 as std::ffi::c_int as off_t,
        ) as *mut std::ffi::c_char;
        if (*env).me_map
            == -(1 as std::ffi::c_int) as *mut std::ffi::c_void as *mut std::ffi::c_char
        {
            (*env).me_map = std::ptr::null_mut::<std::ffi::c_char>();
            return *__error();
        }
        if flags & 0x800000 as std::ffi::c_int as std::ffi::c_uint != 0 {
            madvise(
                (*env).me_map as *mut std::ffi::c_void,
                (*env).me_mapsize as _,
                1 as std::ffi::c_int,
            );
        }
        if !addr.is_null() && (*env).me_map != addr as *mut std::ffi::c_char {
            return 16 as std::ffi::c_int;
        }
        p = (*env).me_map as *mut MDB_page;
        (*env).me_metas[0 as std::ffi::c_int as usize] = (p as *mut std::ffi::c_char)
            .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
            as *mut std::ffi::c_void
            as *mut MDB_meta;
        (*env).me_metas[1 as std::ffi::c_int as usize] =
            ((*env).me_metas[0 as std::ffi::c_int as usize] as *mut std::ffi::c_char)
                .offset((*env).me_psize as isize) as *mut MDB_meta;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_set_mapsize(
    mut env: *mut MDB_env,
    mut size: mdb_size_t,
) -> std::ffi::c_int {
    unsafe {
        if !((*env).me_map).is_null() {
            let mut meta: *mut MDB_meta = std::ptr::null_mut::<MDB_meta>();
            let mut old: *mut std::ffi::c_void = std::ptr::null_mut::<std::ffi::c_void>();
            let mut rc: std::ffi::c_int = 0;
            if !((*env).me_txn).is_null() {
                return 22 as std::ffi::c_int;
            }
            meta = mdb_env_pick_meta(env);
            if size == 0 {
                size = (*meta).mm_mapsize;
            }
            let mut minsize: mdb_size_t = (((*meta).mm_last_pg)
                .wrapping_add(1 as std::ffi::c_int as pgno_t)
                * (*env).me_psize as pgno_t) as _;
            if size < minsize {
                size = minsize;
            }
            munmap((*env).me_map as *mut std::ffi::c_void, (*env).me_mapsize as _);
            (*env).me_mapsize = size;
            old = (if (*env).me_flags & 0x1 as std::ffi::c_int as uint32_t != 0 {
                (*env).me_map
            } else {
                std::ptr::null_mut::<std::ffi::c_char>()
            }) as *mut std::ffi::c_void;
            rc = mdb_env_map(env, old);
            if rc != 0 {
                return rc;
            }
        }
        (*env).me_mapsize = size;
        if (*env).me_psize != 0 {
            (*env).me_maxpg = ((*env).me_mapsize / (*env).me_psize as mdb_size_t) as _;
        }
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_set_maxdbs(
    mut env: *mut MDB_env,
    mut dbs: MDB_dbi,
) -> std::ffi::c_int {
    unsafe {
        if !((*env).me_map).is_null() {
            return 22 as std::ffi::c_int;
        }
        (*env).me_maxdbs = dbs.wrapping_add(2 as std::ffi::c_int as MDB_dbi);
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_set_maxreaders(
    mut env: *mut MDB_env,
    mut readers: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        if !((*env).me_map).is_null() || readers < 1 as std::ffi::c_int as std::ffi::c_uint {
            return 22 as std::ffi::c_int;
        }
        (*env).me_maxreaders = readers;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_get_maxreaders(
    mut env: *mut MDB_env,
    mut readers: *mut std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        if env.is_null() || readers.is_null() {
            return 22 as std::ffi::c_int;
        }
        *readers = (*env).me_maxreaders;
        0 as std::ffi::c_int
    }
}
#[cold]
unsafe extern "C" fn mdb_fsize(
    mut fd: std::ffi::c_int,
    mut size: *mut mdb_size_t,
) -> std::ffi::c_int {
    unsafe {
        let mut st: stat = stat {
            st_dev: 0,
            st_mode: 0,
            st_nlink: 0,
            st_ino: 0,
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            st_atimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_birthtimespec: timespec { tv_sec: 0, tv_nsec: 0 },
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            st_flags: 0,
            st_gen: 0,
            st_lspare: 0,
            st_qspare: [0; 2],
        };
        if fstat(fd, &mut st) != 0 {
            return *__error();
        }
        *size = st.st_size as mdb_size_t;
        0 as std::ffi::c_int
    }
}
static mut mdb_suffixes: [[*const mdb_nchar_t; 2]; 2] = [
    [
        b"/data.mdb\0" as *const u8 as *const std::ffi::c_char,
        b"\0" as *const u8 as *const std::ffi::c_char,
    ],
    [
        b"/lock.mdb\0" as *const u8 as *const std::ffi::c_char,
        b"-lock\0" as *const u8 as *const std::ffi::c_char,
    ],
];
#[cold]
unsafe extern "C" fn mdb_fname_init(
    mut path: *const std::ffi::c_char,
    mut envflags: std::ffi::c_uint,
    mut fname: *mut MDB_name,
) -> std::ffi::c_int {
    unsafe {
        let mut no_suffix: std::ffi::c_int = (envflags
            & (0x4000 as std::ffi::c_int | 0x400000 as std::ffi::c_int) as std::ffi::c_uint
            == (0x4000 as std::ffi::c_int | 0x400000 as std::ffi::c_int) as std::ffi::c_uint)
            as std::ffi::c_int;
        (*fname).mn_alloced = 0 as std::ffi::c_int;
        (*fname).mn_len = strlen(path) as std::ffi::c_int;
        if no_suffix != 0 {
            (*fname).mn_val = path as *mut std::ffi::c_char;
        } else {
            (*fname).mn_val = malloc(
                ((*fname).mn_len + 9 as std::ffi::c_int + 1 as std::ffi::c_int)
                    as std::ffi::c_ulong,
            ) as *mut mdb_nchar_t;
            if !((*fname).mn_val).is_null() {
                (*fname).mn_alloced = 1 as std::ffi::c_int;
                strcpy((*fname).mn_val, path);
            } else {
                return 12 as std::ffi::c_int;
            }
        }
        0 as std::ffi::c_int
    }
}
#[cold]
unsafe extern "C" fn mdb_fopen(
    mut env: *const MDB_env,
    mut fname: *mut MDB_name,
    mut which: mdb_fopen_type,
    mut mode: mdb_mode_t,
    mut res: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut fd: std::ffi::c_int = 0;
        let mut flags: std::ffi::c_int = 0;
        if (*fname).mn_alloced != 0 {
            strcpy(
                ((*fname).mn_val).offset((*fname).mn_len as isize),
                mdb_suffixes[(which as std::ffi::c_uint
                    == MDB_O_LOCKS as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_int as usize][((*env).me_flags
                    & 0x4000 as std::ffi::c_int as uint32_t
                    == 0x4000 as std::ffi::c_int as uint32_t)
                    as std::ffi::c_int as usize],
            );
        }
        fd = open(
            (*fname).mn_val,
            (which as std::ffi::c_uint & MDB_O_MASK as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int,
            mode as std::ffi::c_int,
        );
        if fd == -(1 as std::ffi::c_int) {
            rc = *__error();
        } else {
            if which as std::ffi::c_uint != MDB_O_RDONLY as std::ffi::c_int as std::ffi::c_uint
                && which as std::ffi::c_uint != MDB_O_RDWR as std::ffi::c_int as std::ffi::c_uint
                && 0x1000000 as std::ffi::c_int == 0
                && {
                    flags = fcntl(fd, 1 as std::ffi::c_int);
                    flags != -(1 as std::ffi::c_int)
                }
            {
                fcntl(fd, 2 as std::ffi::c_int, flags | 1 as std::ffi::c_int);
            }
            if which as std::ffi::c_uint == MDB_O_COPY as std::ffi::c_int as std::ffi::c_uint
                && (*env).me_psize >= (*env).me_os_psize
            {
                fcntl(fd, 48 as std::ffi::c_int, 1 as std::ffi::c_int);
            }
        }
        *res = fd;
        rc
    }
}
#[cold]
unsafe extern "C" fn mdb_env_open2(
    mut env: *mut MDB_env,
    mut prev: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut flags: std::ffi::c_uint = (*env).me_flags;
        let mut i: std::ffi::c_int = 0;
        let mut newenv: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut rc: std::ffi::c_int = 0;
        let mut meta: MDB_meta = MDB_meta {
            mm_magic: 0,
            mm_version: 0,
            mm_address: std::ptr::null_mut::<std::ffi::c_void>(),
            mm_mapsize: 0,
            mm_dbs: [MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            }; 2],
            mm_last_pg: 0,
            mm_txnid: 0,
        };
        i = mdb_env_read_header(env, prev, &mut meta);
        if i != 0 as std::ffi::c_int {
            if i != 2 as std::ffi::c_int {
                return i;
            }
            newenv = 1 as std::ffi::c_int;
            (*env).me_psize = (*env).me_os_psize;
            if (*env).me_psize
                > (if (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) != 0
                {
                    0x10000 as std::ffi::c_int
                } else {
                    0x8000 as std::ffi::c_int
                }) as std::ffi::c_uint
            {
                (*env).me_psize = (if if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                } != 0
                {
                    0x10000 as std::ffi::c_int
                } else {
                    0x8000 as std::ffi::c_int
                }) as std::ffi::c_uint;
            }
            memset(
                &mut meta as *mut MDB_meta as *mut std::ffi::c_void,
                0 as std::ffi::c_int,
                ::core::mem::size_of::<MDB_meta>() as std::ffi::c_ulong,
            );
            mdb_env_init_meta0(env, &mut meta);
            meta.mm_mapsize = 1048576 as std::ffi::c_int as mdb_size_t;
        } else {
            (*env).me_psize = meta.mm_dbs[0 as std::ffi::c_int as usize].md_pad;
        }
        if (*env).me_mapsize == 0 {
            (*env).me_mapsize = meta.mm_mapsize;
        }
        let mut minsize: mdb_size_t =
            ((meta.mm_last_pg).wrapping_add(1 as std::ffi::c_int as pgno_t)
                * meta.mm_dbs[0 as std::ffi::c_int as usize].md_pad as pgno_t) as _;
        if (*env).me_mapsize < minsize {
            (*env).me_mapsize = minsize;
        }
        meta.mm_mapsize = (*env).me_mapsize;
        if newenv != 0 && flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0 {
            rc = mdb_env_init_meta(env, &mut meta);
            if rc != 0 {
                return rc;
            }
            newenv = 0 as std::ffi::c_int;
        }
        rc = mdb_env_map(
            env,
            if flags & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0 {
                meta.mm_address
            } else {
                std::ptr::null_mut::<std::ffi::c_void>()
            },
        );
        if rc != 0 {
            return rc;
        }
        if newenv != 0 {
            if flags & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0 {
                meta.mm_address = (*env).me_map as *mut std::ffi::c_void;
            }
            i = mdb_env_init_meta(env, &mut meta);
            if i != 0 as std::ffi::c_int {
                return i;
            }
        }
        (*env).me_maxfree_1pg = (((*env).me_psize)
            .wrapping_sub(16 as std::ffi::c_ulong as std::ffi::c_uint)
            as std::ffi::c_ulong)
            .wrapping_div(::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong)
            .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_ulong)
            as std::ffi::c_int;
        (*env).me_nodemax = ((((*env).me_psize)
            .wrapping_sub(16 as std::ffi::c_ulong as std::ffi::c_uint)
            .wrapping_div(2 as std::ffi::c_int as std::ffi::c_uint)
            & -(2 as std::ffi::c_int) as std::ffi::c_uint)
            as std::ffi::c_ulong)
            .wrapping_sub(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
            as std::ffi::c_uint;
        (*env).me_maxpg = ((*env).me_mapsize / (*env).me_psize as mdb_size_t) as _;
        if prev != 0 && !((*env).me_txns).is_null() {
            ::core::ptr::write_volatile(
                &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
                meta.mm_txnid,
            );
        }
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_env_reader_dest(mut ptr: *mut std::ffi::c_void) {
    unsafe {
        let mut reader: *mut MDB_reader = ptr as *mut MDB_reader;
        if (*reader).mru.mrx.mrb_pid == getpid() {
            ::core::ptr::write_volatile(
                &mut (*reader).mru.mrx.mrb_pid as *mut pid_t,
                0 as std::ffi::c_int,
            );
        }
    }
}
#[cold]
unsafe extern "C" fn mdb_env_share_locks(
    mut env: *mut MDB_env,
    mut excl: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut meta: *mut MDB_meta = mdb_env_pick_meta(env);
        ::core::ptr::write_volatile(
            &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
            (*meta).mm_txnid,
        );
        let mut lock_info: flock = flock { l_start: 0, l_len: 0, l_pid: 0, l_type: 0, l_whence: 0 };
        memset(
            &mut lock_info as *mut flock as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            ::core::mem::size_of::<flock>() as std::ffi::c_ulong,
        );
        lock_info.l_type = 1 as std::ffi::c_int as std::ffi::c_short;
        lock_info.l_whence = 0 as std::ffi::c_int as std::ffi::c_short;
        lock_info.l_start = 0 as std::ffi::c_int as off_t;
        lock_info.l_len = 1 as std::ffi::c_int as off_t;
        loop {
            rc = fcntl((*env).me_lfd, 8 as std::ffi::c_int, &mut lock_info as *mut flock);
            if !(rc != 0 && {
                rc = *__error();
                rc == 4 as std::ffi::c_int
            }) {
                break;
            }
        }
        *excl = if rc != 0 { -(1 as std::ffi::c_int) } else { 0 as std::ffi::c_int };
        rc
    }
}
#[cold]
unsafe extern "C" fn mdb_env_excl_lock(
    mut env: *mut MDB_env,
    mut excl: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut lock_info: flock = flock { l_start: 0, l_len: 0, l_pid: 0, l_type: 0, l_whence: 0 };
        memset(
            &mut lock_info as *mut flock as *mut std::ffi::c_void,
            0 as std::ffi::c_int,
            ::core::mem::size_of::<flock>() as std::ffi::c_ulong,
        );
        lock_info.l_type = 3 as std::ffi::c_int as std::ffi::c_short;
        lock_info.l_whence = 0 as std::ffi::c_int as std::ffi::c_short;
        lock_info.l_start = 0 as std::ffi::c_int as off_t;
        lock_info.l_len = 1 as std::ffi::c_int as off_t;
        loop {
            rc = fcntl((*env).me_lfd, 8 as std::ffi::c_int, &mut lock_info as *mut flock);
            if !(rc != 0 && {
                rc = *__error();
                rc == 4 as std::ffi::c_int
            }) {
                break;
            }
        }
        if rc == 0 {
            *excl = 1 as std::ffi::c_int;
        } else if *excl < 0 as std::ffi::c_int {
            lock_info.l_type = 1 as std::ffi::c_int as std::ffi::c_short;
            loop {
                rc = fcntl((*env).me_lfd, 9 as std::ffi::c_int, &mut lock_info as *mut flock);
                if !(rc != 0 && {
                    rc = *__error();
                    rc == 4 as std::ffi::c_int
                }) {
                    break;
                }
            }
            if rc == 0 as std::ffi::c_int {
                *excl = 0 as std::ffi::c_int;
            }
        }
        rc
    }
}
#[cold]
unsafe extern "C" fn mdb_env_setup_locks(
    mut env: *mut MDB_env,
    mut fname: *mut MDB_name,
    mut mode: std::ffi::c_int,
    mut excl: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut semid: std::ffi::c_int = 0;
        let mut semu: semun = semun { val: 0 };
        let mut rc: std::ffi::c_int = 0;
        let mut size: off_t = 0;
        let mut rsize: off_t = 0;
        rc = mdb_fopen(env, fname, MDB_O_LOCKS, mode as mdb_mode_t, &mut (*env).me_lfd);
        if rc != 0 {
            if rc == 30 as std::ffi::c_int
                && (*env).me_flags & 0x20000 as std::ffi::c_int as uint32_t != 0
            {
                return 0 as std::ffi::c_int;
            }
        } else {
            if (*env).me_flags & 0x200000 as std::ffi::c_int as uint32_t == 0 {
                rc = pthread_key_create(
                    &mut (*env).me_txkey,
                    Some(mdb_env_reader_dest as unsafe extern "C" fn(*mut std::ffi::c_void) -> ()),
                );
                if rc != 0 {
                    current_block = 18368835354285971245;
                } else {
                    (*env).me_flags |= 0x10000000 as std::ffi::c_uint;
                    current_block = 5720623009719927633;
                }
            } else {
                current_block = 5720623009719927633;
            }
            match current_block {
                18368835354285971245 => {}
                _ => {
                    rc = mdb_env_excl_lock(env, excl);
                    if rc == 0 {
                        size = lseek(
                            (*env).me_lfd,
                            0 as std::ffi::c_int as off_t,
                            2 as std::ffi::c_int,
                        );
                        if size == -(1 as std::ffi::c_int) as off_t {
                            current_block = 2284073165394886733;
                        } else {
                            rsize = (((*env).me_maxreaders)
                                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                                as std::ffi::c_ulong)
                                .wrapping_mul(
                                    ::core::mem::size_of::<MDB_reader>() as std::ffi::c_ulong
                                )
                                .wrapping_add(
                                    ::core::mem::size_of::<MDB_txninfo>() as std::ffi::c_ulong
                                ) as off_t;
                            if size < rsize && *excl > 0 as std::ffi::c_int {
                                if ftruncate((*env).me_lfd, rsize) != 0 as std::ffi::c_int {
                                    current_block = 2284073165394886733;
                                } else {
                                    current_block = 17833034027772472439;
                                }
                            } else {
                                rsize = size;
                                size = (rsize as std::ffi::c_ulonglong)
                                    .wrapping_sub(::core::mem::size_of::<MDB_txninfo>()
                                        as std::ffi::c_ulong
                                        as std::ffi::c_ulonglong)
                                    as off_t;
                                (*env).me_maxreaders = (size as std::ffi::c_ulonglong)
                                    .wrapping_div(::core::mem::size_of::<MDB_reader>()
                                        as std::ffi::c_ulong
                                        as std::ffi::c_ulonglong)
                                    .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulonglong)
                                    as std::ffi::c_uint;
                                current_block = 17833034027772472439;
                            }
                            match current_block {
                                2284073165394886733 => {}
                                _ => {
                                    let mut m: *mut std::ffi::c_void = mmap(
                                        std::ptr::null_mut::<std::ffi::c_void>(),
                                        rsize as size_t,
                                        0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int,
                                        0x1 as std::ffi::c_int,
                                        (*env).me_lfd,
                                        0 as std::ffi::c_int as off_t,
                                    );
                                    if m == -(1 as std::ffi::c_int) as *mut std::ffi::c_void {
                                        current_block = 2284073165394886733;
                                    } else {
                                        (*env).me_txns = m as *mut MDB_txninfo;
                                        if *excl > 0 as std::ffi::c_int {
                                            let mut vals: [std::ffi::c_ushort; 2] = [
                                                1 as std::ffi::c_int as std::ffi::c_ushort,
                                                1 as std::ffi::c_int as std::ffi::c_ushort,
                                            ];
                                            let mut key: key_t = ftok((*fname).mn_val, 'M' as i32);
                                            if key == -(1 as std::ffi::c_int) {
                                                current_block = 2284073165394886733;
                                            } else {
                                                semid = semget(
                                                    key,
                                                    2 as std::ffi::c_int,
                                                    mode & 0o777 as std::ffi::c_int
                                                        | 0o1000 as std::ffi::c_int,
                                                );
                                                if semid < 0 as std::ffi::c_int {
                                                    current_block = 2284073165394886733;
                                                } else {
                                                    semu.array = vals.as_mut_ptr();
                                                    if semctl(
                                                        semid,
                                                        0 as std::ffi::c_int,
                                                        9 as std::ffi::c_int,
                                                        semu,
                                                    ) < 0 as std::ffi::c_int
                                                    {
                                                        current_block = 2284073165394886733;
                                                    } else {
                                                        (*(*env).me_txns).mt1.mtb.mtb_semid = semid;
                                                        (*(*env).me_txns).mt1.mtb.mtb_rlocked =
                                                            0 as std::ffi::c_int;
                                                        (*(*env).me_txns).mt2.mt2_wlocked =
                                                            0 as std::ffi::c_int;
                                                        (*(*env).me_txns).mt1.mtb.mtb_magic =
                                                            0xbeefc0de as std::ffi::c_uint;
                                                        (*(*env).me_txns).mt1.mtb.mtb_format =
                                                        ((if 0 as std::ffi::c_int != 0 {
                                                            999 as std::ffi::c_int
                                                        } else {
                                                            2 as std::ffi::c_int
                                                        })
                                                            as std::ffi::c_uint)
                                                            .wrapping_rem(
                                                                (1 as std::ffi::c_uint)
                                                                    << 12 as std::ffi::c_int,
                                                            )
                                                            .wrapping_add(
                                                                (MDB_lock_desc as std::ffi::c_int
                                                                    as std::ffi::c_uint)
                                                                    .wrapping_mul(
                                                                        (1 as std::ffi::c_uint)
                                                                            << 12
                                                                                as std::ffi::c_int,
                                                                    ),
                                                            );
                                                        ::core::ptr::write_volatile(
                                                            &mut (*(*env).me_txns).mt1.mtb.mtb_txnid
                                                                as *mut txnid_t,
                                                            0 as std::ffi::c_int as txnid_t,
                                                        );
                                                        ::core::ptr::write_volatile(
                                                            &mut (*(*env).me_txns)
                                                                .mt1
                                                                .mtb
                                                                .mtb_numreaders
                                                                as *mut std::ffi::c_uint,
                                                            0 as std::ffi::c_int
                                                                as std::ffi::c_uint,
                                                        );
                                                        current_block = 5330834795799507926;
                                                    }
                                                }
                                            }
                                        } else {
                                            let mut buf: __semid_ds_new = __semid_ds_new {
                                                sem_perm: ipc_perm {
                                                    uid: 0,
                                                    gid: 0,
                                                    cuid: 0,
                                                    cgid: 0,
                                                    mode: 0,
                                                    _seq: 0,
                                                    _key: 0,
                                                },
                                                sem_base: 0,
                                                sem_nsems: 0,
                                                sem_otime: 0,
                                                sem_pad1: 0,
                                                sem_ctime: 0,
                                                sem_pad2: 0,
                                                sem_pad3: [0; 4],
                                            };
                                            if (*(*env).me_txns).mt1.mtb.mtb_magic
                                                != 0xbeefc0de as std::ffi::c_uint
                                            {
                                                rc = -(30793 as std::ffi::c_int);
                                                current_block = 18368835354285971245;
                                            } else if (*(*env).me_txns).mt1.mtb.mtb_format
                                                != ((if 0 as std::ffi::c_int != 0 {
                                                    999 as std::ffi::c_int
                                                } else {
                                                    2 as std::ffi::c_int
                                                })
                                                    as std::ffi::c_uint)
                                                    .wrapping_rem(
                                                        (1 as std::ffi::c_uint)
                                                            << 12 as std::ffi::c_int,
                                                    )
                                                    .wrapping_add(
                                                        (MDB_lock_desc as std::ffi::c_int
                                                            as std::ffi::c_uint)
                                                            .wrapping_mul(
                                                                (1 as std::ffi::c_uint)
                                                                    << 12 as std::ffi::c_int,
                                                            ),
                                                    )
                                            {
                                                rc = -(30794 as std::ffi::c_int);
                                                current_block = 18368835354285971245;
                                            } else {
                                                rc = *__error();
                                                if rc != 0
                                                    && rc != 13 as std::ffi::c_int
                                                    && rc != 35 as std::ffi::c_int
                                                {
                                                    current_block = 18368835354285971245;
                                                } else {
                                                    semid = (*(*env).me_txns).mt1.mtb.mtb_semid;
                                                    semu.buf = &mut buf;
                                                    if semctl(
                                                        semid,
                                                        0 as std::ffi::c_int,
                                                        2 as std::ffi::c_int,
                                                        semu,
                                                    ) < 0 as std::ffi::c_int
                                                    {
                                                        current_block = 2284073165394886733;
                                                    } else if semctl(
                                                        semid,
                                                        0 as std::ffi::c_int,
                                                        1 as std::ffi::c_int,
                                                        semu,
                                                    ) < 0 as std::ffi::c_int
                                                    {
                                                        current_block = 2284073165394886733;
                                                    } else {
                                                        current_block = 5330834795799507926;
                                                    }
                                                }
                                            }
                                        }
                                        match current_block {
                                            2284073165394886733 => {}
                                            18368835354285971245 => {}
                                            _ => {
                                                (*((*env).me_rmutex).as_mut_ptr()).semid = semid;
                                                (*((*env).me_wmutex).as_mut_ptr()).semid = semid;
                                                (*((*env).me_rmutex).as_mut_ptr()).semnum =
                                                    0 as std::ffi::c_int;
                                                (*((*env).me_wmutex).as_mut_ptr()).semnum =
                                                    1 as std::ffi::c_int;
                                                let fresh17 =
                                                    &mut (*((*env).me_rmutex).as_mut_ptr()).locked;
                                                *fresh17 =
                                                    &mut (*(*env).me_txns).mt1.mtb.mtb_rlocked;
                                                let fresh18 =
                                                    &mut (*((*env).me_wmutex).as_mut_ptr()).locked;
                                                *fresh18 = &mut (*(*env).me_txns).mt2.mt2_wlocked;
                                                return 0 as std::ffi::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        match current_block {
                            18368835354285971245 => {}
                            _ => {
                                rc = *__error();
                            }
                        }
                    }
                }
            }
        }
        rc
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_open(
    mut env: *mut MDB_env,
    mut path: *const std::ffi::c_char,
    mut flags: std::ffi::c_uint,
    mut mode: mdb_mode_t,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut rc: std::ffi::c_int = 0;
        let mut excl: std::ffi::c_int = -(1 as std::ffi::c_int);
        let mut fname: MDB_name =
            MDB_name { mn_len: 0, mn_alloced: 0, mn_val: std::ptr::null_mut::<mdb_nchar_t>() };
        if (*env).me_fd != -(1 as std::ffi::c_int)
            || flags
                & !(0x10000 as std::ffi::c_int
                    | 0x40000 as std::ffi::c_int
                    | 0x100000 as std::ffi::c_int
                    | 0x1000000 as std::ffi::c_int
                    | (0x1 as std::ffi::c_int
                        | 0x4000 as std::ffi::c_int
                        | 0x20000 as std::ffi::c_int
                        | 0x80000 as std::ffi::c_int
                        | 0x200000 as std::ffi::c_int
                        | 0x400000 as std::ffi::c_int
                        | 0x800000 as std::ffi::c_int
                        | 0x2000000 as std::ffi::c_int)) as std::ffi::c_uint
                != 0
        {
            return 22 as std::ffi::c_int;
        }
        flags |= (*env).me_flags;
        rc = mdb_fname_init(path, flags, &mut fname);
        if rc != 0 {
            return rc;
        }
        flags |= 0x20000000 as std::ffi::c_uint;
        if flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0 {
            flags &= !(0x80000 as std::ffi::c_int) as std::ffi::c_uint;
        } else {
            (*env).me_free_pgs = mdb_midl_alloc(((1) << (16 + 1)) - 1);
            if !(((*env).me_free_pgs).is_some() && {
                (*env).me_dirty_list = calloc(
                    ((1 as std::ffi::c_int) << (16 as std::ffi::c_int + 1 as std::ffi::c_int))
                        as std::ffi::c_ulong,
                    ::core::mem::size_of::<MDB_ID2>() as std::ffi::c_ulong,
                ) as MDB_ID2L;
                !((*env).me_dirty_list).is_null()
            }) {
                rc = 12 as std::ffi::c_int;
            }
        }
        (*env).me_flags = flags;
        if rc == 0 {
            (*env).me_path = strdup(path);
            (*env).me_dbxs = calloc(
                (*env).me_maxdbs as std::ffi::c_ulong,
                ::core::mem::size_of::<MDB_dbx>() as std::ffi::c_ulong,
            ) as *mut MDB_dbx;
            (*env).me_dbflags = calloc(
                (*env).me_maxdbs as std::ffi::c_ulong,
                ::core::mem::size_of::<uint16_t>() as std::ffi::c_ulong,
            ) as *mut uint16_t;
            (*env).me_dbiseqs = calloc(
                (*env).me_maxdbs as std::ffi::c_ulong,
                ::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong,
            ) as *mut std::ffi::c_uint;
            if !(!((*env).me_dbxs).is_null()
                && !((*env).me_path).is_null()
                && !((*env).me_dbflags).is_null()
                && !((*env).me_dbiseqs).is_null())
            {
                rc = 12 as std::ffi::c_int;
            } else {
                let fresh19 = &mut (*((*env).me_dbxs).offset(0 as std::ffi::c_int as isize)).md_cmp;
                *fresh19 = Some(mdb_cmp_long as MDB_cmp_func);
                if flags
                    & (0x20000 as std::ffi::c_int | 0x400000 as std::ffi::c_int) as std::ffi::c_uint
                    == 0
                {
                    rc = mdb_env_setup_locks(env, &mut fname, mode as std::ffi::c_int, &mut excl);
                    if rc != 0 {
                        current_block = 2122094917359643297;
                    } else if flags & 0x2000000 as std::ffi::c_int as std::ffi::c_uint != 0
                        && excl == 0
                    {
                        rc = 35 as std::ffi::c_int;
                        current_block = 2122094917359643297;
                    } else {
                        current_block = 5783071609795492627;
                    }
                } else {
                    current_block = 5783071609795492627;
                }
                match current_block {
                    2122094917359643297 => {}
                    _ => {
                        rc = mdb_fopen(
                            env,
                            &mut fname,
                            (if flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                                MDB_O_RDONLY as std::ffi::c_int
                            } else {
                                MDB_O_RDWR as std::ffi::c_int
                            }) as mdb_fopen_type,
                            mode,
                            &mut (*env).me_fd,
                        );
                        if rc == 0 {
                            if flags
                                & (0x20000 as std::ffi::c_int | 0x400000 as std::ffi::c_int)
                                    as std::ffi::c_uint
                                == 0x20000 as std::ffi::c_int as std::ffi::c_uint
                            {
                                rc = mdb_env_setup_locks(
                                    env,
                                    &mut fname,
                                    mode as std::ffi::c_int,
                                    &mut excl,
                                );
                                if rc != 0 {
                                    current_block = 2122094917359643297;
                                } else {
                                    current_block = 4068382217303356765;
                                }
                            } else {
                                current_block = 4068382217303356765;
                            }
                            match current_block {
                                2122094917359643297 => {}
                                _ => {
                                    rc = mdb_env_open2(
                                        env,
                                        (flags & 0x2000000 as std::ffi::c_int as std::ffi::c_uint)
                                            as std::ffi::c_int,
                                    );
                                    if rc == 0 as std::ffi::c_int {
                                        if flags
                                            & (0x20000 as std::ffi::c_int
                                                | 0x80000 as std::ffi::c_int)
                                                as std::ffi::c_uint
                                            == 0
                                        {
                                            rc = mdb_fopen(
                                                env,
                                                &mut fname,
                                                MDB_O_META,
                                                mode,
                                                &mut (*env).me_mfd,
                                            );
                                            if rc != 0 {
                                                current_block = 2122094917359643297;
                                            } else {
                                                current_block = 8693738493027456495;
                                            }
                                        } else {
                                            current_block = 8693738493027456495;
                                        }
                                        match current_block {
                                            2122094917359643297 => {}
                                            _ => {
                                                if excl > 0 as std::ffi::c_int
                                                    && flags
                                                        & 0x2000000 as std::ffi::c_int
                                                            as std::ffi::c_uint
                                                        == 0
                                                {
                                                    rc = mdb_env_share_locks(env, &mut excl);
                                                    if rc != 0 {
                                                        current_block = 2122094917359643297;
                                                    } else {
                                                        current_block = 14136749492126903395;
                                                    }
                                                } else {
                                                    current_block = 14136749492126903395;
                                                }
                                                match current_block {
                                                    2122094917359643297 => {}
                                                    _ => {
                                                        if flags
                                                            & 0x20000 as std::ffi::c_int
                                                                as std::ffi::c_uint
                                                            == 0
                                                        {
                                                            let mut txn: *mut MDB_txn =
                                                                std::ptr::null_mut::<MDB_txn>();
                                                            let mut tsize: std::ffi::c_int =
                                                                ::core::mem::size_of::<MDB_txn>()
                                                                    as std::ffi::c_ulong
                                                                    as std::ffi::c_int;
                                                            let mut size: std::ffi::c_int = (tsize as std::ffi::c_ulong)
                                                            .wrapping_add(
                                                                ((*env).me_maxdbs as std::ffi::c_ulong)
                                                                    .wrapping_mul(
                                                                        (::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong)
                                                                            .wrapping_add(
                                                                                ::core::mem::size_of::<*mut MDB_cursor>() as std::ffi::c_ulong,
                                                                            )
                                                                            .wrapping_add(
                                                                                ::core::mem::size_of::<std::ffi::c_uint>() as std::ffi::c_ulong,
                                                                            )
                                                                            .wrapping_add(1 as std::ffi::c_int as std::ffi::c_ulong),
                                                                    ),
                                                            ) as std::ffi::c_int;
                                                            (*env).me_pbuf = calloc(
                                                                1 as std::ffi::c_int
                                                                    as std::ffi::c_ulong,
                                                                (*env).me_psize
                                                                    as std::ffi::c_ulong,
                                                            );
                                                            if !((*env).me_pbuf).is_null() && {
                                                                txn = calloc(
                                                                    1 as std::ffi::c_int
                                                                        as std::ffi::c_ulong,
                                                                    size as std::ffi::c_ulong,
                                                                )
                                                                    as *mut MDB_txn;
                                                                !txn.is_null()
                                                            } {
                                                                (*txn).mt_dbs = (txn
                                                                    as *mut std::ffi::c_char)
                                                                    .offset(tsize as isize)
                                                                    as *mut MDB_db;
                                                                (*txn).mt_cursors = ((*txn).mt_dbs)
                                                                    .offset(
                                                                        (*env).me_maxdbs as isize,
                                                                    )
                                                                    as *mut *mut MDB_cursor;
                                                                (*txn).mt_dbiseqs =
                                                                    ((*txn).mt_cursors).offset(
                                                                        (*env).me_maxdbs as isize,
                                                                    )
                                                                        as *mut std::ffi::c_uint;
                                                                (*txn).mt_dbflags =
                                                                    ((*txn).mt_dbiseqs).offset(
                                                                        (*env).me_maxdbs as isize,
                                                                    )
                                                                        as *mut std::ffi::c_uchar;
                                                                (*txn).mt_env = env;
                                                                (*txn).mt_dbxs = (*env).me_dbxs;
                                                                (*txn).mt_flags = 0x1
                                                                    as std::ffi::c_int
                                                                    as std::ffi::c_uint;
                                                                (*env).me_txn0 = txn;
                                                            } else {
                                                                rc = 12 as std::ffi::c_int;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if rc != 0 {
            mdb_env_close0(env, excl);
        }
        if fname.mn_alloced != 0 {
            free(fname.mn_val as *mut std::ffi::c_void);
        }
        rc
    }
}
#[cold]
unsafe extern "C" fn mdb_env_close0(mut env: *mut MDB_env, mut excl: std::ffi::c_int) {
    unsafe {
        let mut i: std::ffi::c_int = 0;
        if (*env).me_flags & 0x20000000 as std::ffi::c_uint == 0 {
            return;
        }
        if !((*env).me_dbxs).is_null() {
            i = (*env).me_maxdbs as std::ffi::c_int;
            loop {
                i -= 1;
                if i < 2 as std::ffi::c_int {
                    break;
                }
                free((*((*env).me_dbxs).offset(i as isize)).md_name.mv_data);
            }
            free((*env).me_dbxs as *mut std::ffi::c_void);
        }
        free((*env).me_pbuf);
        free((*env).me_dbiseqs as *mut std::ffi::c_void);
        free((*env).me_dbflags as *mut std::ffi::c_void);
        free((*env).me_path as *mut std::ffi::c_void);
        free((*env).me_dirty_list as *mut std::ffi::c_void);
        free((*env).me_txn0 as *mut std::ffi::c_void);
        // Note: will be freed later by dropping the environment
        if let Some(free_pages) = (*env).me_free_pgs.take() {
            mdb_midl_free(free_pages);
        }
        if (*env).me_flags & 0x10000000 as std::ffi::c_uint != 0 {
            pthread_key_delete((*env).me_txkey);
        }
        if !((*env).me_map).is_null() {
            munmap((*env).me_map as *mut std::ffi::c_void, (*env).me_mapsize as _);
        }
        if (*env).me_mfd != -(1 as std::ffi::c_int) {
            close((*env).me_mfd);
        }
        if (*env).me_fd != -(1 as std::ffi::c_int) {
            close((*env).me_fd);
        }
        if !((*env).me_txns).is_null() {
            let mut pid: pid_t = getpid();
            i = (*env).me_close_readers;
            loop {
                i -= 1;
                if i < 0 as std::ffi::c_int {
                    break;
                }
                if (*((*(*env).me_txns).mti_readers).as_mut_ptr().offset(i as isize))
                    .mru
                    .mrx
                    .mrb_pid
                    == pid
                {
                    ::core::ptr::write_volatile(
                        &mut (*((*(*env).me_txns).mti_readers).as_mut_ptr().offset(i as isize))
                            .mru
                            .mrx
                            .mrb_pid as *mut pid_t,
                        0 as std::ffi::c_int,
                    );
                }
            }
            if (*((*env).me_rmutex).as_mut_ptr()).semid != -(1 as std::ffi::c_int) {
                if excl == 0 as std::ffi::c_int {
                    mdb_env_excl_lock(env, &mut excl);
                }
                if excl > 0 as std::ffi::c_int {
                    semctl(
                        (*((*env).me_rmutex).as_mut_ptr()).semid,
                        0 as std::ffi::c_int,
                        0 as std::ffi::c_int,
                    );
                }
            }
            munmap(
                (*env).me_txns as *mut std::ffi::c_void,
                (((*env).me_maxreaders).wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                    as std::ffi::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<MDB_reader>() as std::ffi::c_ulong)
                    .wrapping_add(::core::mem::size_of::<MDB_txninfo>() as std::ffi::c_ulong),
            );
        }
        if (*env).me_lfd != -(1 as std::ffi::c_int) {
            close((*env).me_lfd);
        }
        (*env).me_flags &= !(0x20000000 as std::ffi::c_uint | 0x10000000 as std::ffi::c_uint);
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_close(mut env: *mut MDB_env) {
    unsafe {
        let mut dp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        if env.is_null() {
            return;
        }
        loop {
            dp = (*env).me_dpages;
            if dp.is_null() {
                break;
            }
            (*env).me_dpages = (*dp).mp_p.p_next;
            free(dp as *mut std::ffi::c_void);
        }
        mdb_env_close0(env, 0 as std::ffi::c_int);
        free(env as *mut std::ffi::c_void);
    }
}
unsafe extern "C" fn mdb_cmp_long(mut a: *const MDB_val, mut b: *const MDB_val) -> std::ffi::c_int {
    unsafe {
        if *((*a).mv_data as *mut mdb_size_t) < *((*b).mv_data as *mut mdb_size_t) {
            -(1 as std::ffi::c_int)
        } else {
            (*((*a).mv_data as *mut mdb_size_t) > *((*b).mv_data as *mut mdb_size_t))
                as std::ffi::c_int
        }
    }
}
unsafe extern "C" fn mdb_cmp_int(mut a: *const MDB_val, mut b: *const MDB_val) -> std::ffi::c_int {
    unsafe {
        if *((*a).mv_data as *mut std::ffi::c_uint) < *((*b).mv_data as *mut std::ffi::c_uint) {
            -(1 as std::ffi::c_int)
        } else {
            (*((*a).mv_data as *mut std::ffi::c_uint) > *((*b).mv_data as *mut std::ffi::c_uint))
                as std::ffi::c_int
        }
    }
}
unsafe extern "C" fn mdb_cmp_cint(mut a: *const MDB_val, mut b: *const MDB_val) -> std::ffi::c_int {
    unsafe {
        let mut u: *mut std::ffi::c_ushort = std::ptr::null_mut::<std::ffi::c_ushort>();
        let mut c: *mut std::ffi::c_ushort = std::ptr::null_mut::<std::ffi::c_ushort>();
        let mut x: std::ffi::c_int = 0;
        u = ((*a).mv_data as *mut std::ffi::c_char).offset((*a).mv_size as isize)
            as *mut std::ffi::c_ushort;
        c = ((*b).mv_data as *mut std::ffi::c_char).offset((*a).mv_size as isize)
            as *mut std::ffi::c_ushort;
        loop {
            u = u.offset(-1);
            c = c.offset(-1);
            x = *u as std::ffi::c_int - *c as std::ffi::c_int;
            if !(x == 0 && u > (*a).mv_data as *mut std::ffi::c_ushort) {
                break;
            }
        }
        x
    }
}
unsafe extern "C" fn mdb_cmp_memn(mut a: *const MDB_val, mut b: *const MDB_val) -> std::ffi::c_int {
    unsafe {
        let mut diff: std::ffi::c_int = 0;
        let mut len_diff: ssize_t = 0;
        let mut len: std::ffi::c_uint = 0;
        len = (*a).mv_size as std::ffi::c_uint;
        len_diff = (*a).mv_size as ssize_t - (*b).mv_size as ssize_t;
        if len_diff > 0 as std::ffi::c_int as ssize_t {
            len = (*b).mv_size as std::ffi::c_uint;
            len_diff = 1 as std::ffi::c_int as ssize_t;
        }
        diff = memcmp((*a).mv_data, (*b).mv_data, len as std::ffi::c_ulong);
        (if diff != 0 {
            diff as ssize_t
        } else if len_diff < 0 as std::ffi::c_int as ssize_t {
            -(1 as std::ffi::c_int) as ssize_t
        } else {
            len_diff
        }) as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_cmp_memnr(
    mut a: *const MDB_val,
    mut b: *const MDB_val,
) -> std::ffi::c_int {
    unsafe {
        let mut p1: *const std::ffi::c_uchar = std::ptr::null::<std::ffi::c_uchar>();
        let mut p2: *const std::ffi::c_uchar = std::ptr::null::<std::ffi::c_uchar>();
        let mut p1_lim: *const std::ffi::c_uchar = std::ptr::null::<std::ffi::c_uchar>();
        let mut len_diff: ssize_t = 0;
        let mut diff: std::ffi::c_int = 0;
        p1_lim = (*a).mv_data as *const std::ffi::c_uchar;
        p1 = ((*a).mv_data as *const std::ffi::c_uchar).offset((*a).mv_size as isize);
        p2 = ((*b).mv_data as *const std::ffi::c_uchar).offset((*b).mv_size as isize);
        len_diff = (*a).mv_size as ssize_t - (*b).mv_size as ssize_t;
        if len_diff > 0 as std::ffi::c_int as ssize_t {
            p1_lim = p1_lim.offset(len_diff as isize);
            len_diff = 1 as std::ffi::c_int as ssize_t;
        }
        while p1 > p1_lim {
            p1 = p1.offset(-1);
            p2 = p2.offset(-1);
            diff = *p1 as std::ffi::c_int - *p2 as std::ffi::c_int;
            if diff != 0 {
                return diff;
            }
        }
        (if len_diff < 0 as std::ffi::c_int as ssize_t {
            -(1 as std::ffi::c_int) as ssize_t
        } else {
            len_diff
        }) as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_node_search(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut exactp: *mut std::ffi::c_int,
) -> *mut MDB_node {
    #[allow(unpredictable_function_pointer_comparisons)]
    unsafe {
        let mut i: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
        let mut nkeys: std::ffi::c_uint = 0;
        let mut low: std::ffi::c_int = 0;
        let mut high: std::ffi::c_int = 0;
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
        let mut node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut nodekey: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut cmp: Option<MDB_cmp_func> = None;
        nkeys = ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_uint)
            .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                },
            ))
            >> 1 as std::ffi::c_int;
        low = if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            == 0x2 as std::ffi::c_int
        {
            0 as std::ffi::c_int
        } else {
            1 as std::ffi::c_int
        };
        high = nkeys.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
        cmp = (*(*mc).mc_dbx).md_cmp;
        if cmp == Some(mdb_cmp_cint as MDB_cmp_func)
            && (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                & 0x1 as std::ffi::c_int
                == 0x1 as std::ffi::c_int
        {
            if (*((mp as *mut std::ffi::c_char)
                .offset(
                    *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(1 as std::ffi::c_int as isize)
                        as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node))
                .mn_ksize as std::ffi::c_ulong
                == ::core::mem::size_of::<mdb_size_t>() as std::ffi::c_ulong
            {
                cmp = Some(mdb_cmp_long as MDB_cmp_func);
            } else {
                cmp = Some(mdb_cmp_int as MDB_cmp_func);
            }
        }
        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            nodekey.mv_size = (*(*mc).mc_db).md_pad as size_t;
            node = (mp as *mut std::ffi::c_char)
                .offset(
                    *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(0 as std::ffi::c_int as isize)
                        as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            while low <= high {
                i = ((low + high) >> 1 as std::ffi::c_int) as std::ffi::c_uint;
                nodekey.mv_data = (mp as *mut std::ffi::c_char)
                    .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                    .offset((i as size_t * nodekey.mv_size) as isize)
                    as *mut std::ffi::c_void;
                rc = cmp.expect("non-null function pointer")(key, &mut nodekey);
                if rc == 0 as std::ffi::c_int {
                    break;
                }
                if rc > 0 as std::ffi::c_int {
                    low =
                        i.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
                } else {
                    high =
                        i.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
                }
            }
        } else {
            while low <= high {
                i = ((low + high) >> 1 as std::ffi::c_int) as std::ffi::c_uint;
                node = (mp as *mut std::ffi::c_char)
                    .offset(
                        *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                            .as_mut_ptr()
                            .offset(i as isize) as std::ffi::c_int as isize,
                    )
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                nodekey.mv_size = (*node).mn_ksize as size_t;
                nodekey.mv_data = ((*node).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                rc = cmp.expect("non-null function pointer")(key, &mut nodekey);
                if rc == 0 as std::ffi::c_int {
                    break;
                }
                if rc > 0 as std::ffi::c_int {
                    low =
                        i.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
                } else {
                    high =
                        i.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_int;
                }
            }
        }
        if rc > 0 as std::ffi::c_int {
            i = i.wrapping_add(1);
            if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                & 0x20 as std::ffi::c_int
                != 0x20 as std::ffi::c_int
            {
                node = (mp as *mut std::ffi::c_char)
                    .offset(
                        *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                            .as_mut_ptr()
                            .offset(i as isize) as std::ffi::c_int as isize,
                    )
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
            }
        }
        if !exactp.is_null() {
            *exactp = (rc == 0 as std::ffi::c_int
                && nkeys > 0 as std::ffi::c_int as std::ffi::c_uint)
                as std::ffi::c_int;
        }
        (*mc).mc_ki[(*mc).mc_top as usize] = i as indx_t;
        if i >= nkeys {
            return std::ptr::null_mut::<MDB_node>();
        }
        node
    }
}
unsafe extern "C" fn mdb_cursor_pop(mut mc: *mut MDB_cursor) {
    unsafe {
        if (*mc).mc_snum != 0 {
            (*mc).mc_snum = ((*mc).mc_snum).wrapping_sub(1);
            (*mc).mc_snum;
            if (*mc).mc_snum != 0 {
                (*mc).mc_top = ((*mc).mc_top).wrapping_sub(1);
                (*mc).mc_top;
            } else {
                (*mc).mc_flags &= !(0x1 as std::ffi::c_int) as std::ffi::c_uint;
            }
        }
    }
}
unsafe extern "C" fn mdb_cursor_push(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> std::ffi::c_int {
    unsafe {
        if (*mc).mc_snum as std::ffi::c_int >= 32 as std::ffi::c_int {
            (*(*mc).mc_txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
            return -(30787 as std::ffi::c_int);
        }
        let fresh20 = (*mc).mc_snum;
        (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
        (*mc).mc_top = fresh20;
        (*mc).mc_pg[(*mc).mc_top as usize] = mp;
        (*mc).mc_ki[(*mc).mc_top as usize] = 0 as std::ffi::c_int as indx_t;
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_page_get(
    mut mc: *mut MDB_cursor,
    mut pgno: pgno_t,
    mut ret: *mut *mut MDB_page,
    mut lvl: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut txn: *mut MDB_txn = (*mc).mc_txn;
        let mut p: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut level: std::ffi::c_int = 0;
        if (*mc).mc_flags
            & (0x20000 as std::ffi::c_int | 0x80000 as std::ffi::c_int) as std::ffi::c_uint
            == 0
        {
            let mut tx2: *mut MDB_txn = txn;
            level = 1 as std::ffi::c_int;
            loop {
                let mut dl: MDB_ID2L = (*tx2).mt_u.dirty_list;
                let mut x: u32 = 0;
                if let Some(spill_pgs) = (*tx2).mt_spill_pgs.as_mut() {
                    let mut pn: MDB_ID = pgno << 1 as std::ffi::c_int;
                    x = mdb_midl_search(spill_pgs, pn);
                    if x as MDB_ID <= spill_pgs.len() && spill_pgs[x as usize] == pn {
                        current_block = 18052530224793637264;
                        break;
                    }
                }
                if (*dl.offset(0 as std::ffi::c_int as isize)).mid != 0 {
                    let mut x_0: std::ffi::c_uint = mdb_mid2l_search(dl, pgno);
                    if x_0 as MDB_ID <= (*dl.offset(0 as std::ffi::c_int as isize)).mid
                        && (*dl.offset(x_0 as isize)).mid == pgno
                    {
                        p = (*dl.offset(x_0 as isize)).mptr as *mut MDB_page;
                        current_block = 139318302626647418;
                        break;
                    }
                }
                level += 1;
                tx2 = (*tx2).mt_parent;
                if tx2.is_null() {
                    current_block = 3512920355445576850;
                    break;
                }
            }
        } else {
            current_block = 3512920355445576850;
        }
        if current_block == 3512920355445576850 {
            if pgno >= (*txn).mt_next_pgno {
                (*txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                return -(30797 as std::ffi::c_int);
            }
            level = 0 as std::ffi::c_int;
            current_block = 18052530224793637264;
        }
        if current_block == 18052530224793637264 {
            let mut env: *mut MDB_env = (*txn).mt_env;
            p = ((*env).me_map).offset(((*env).me_psize as pgno_t * pgno) as isize)
                as *mut MDB_page;
        }
        *ret = p;
        if !lvl.is_null() {
            *lvl = level;
        }
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_page_search_root(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut flags: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
        let mut rc: std::ffi::c_int = 0;
        while (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x1 as std::ffi::c_int
            == 0x1 as std::ffi::c_int
        {
            let mut current_block_30: u64;
            let mut node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
            let mut i: indx_t = 0;
            if (*mc).mc_dbi == 0
                || ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                    as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int
                    > 1 as std::ffi::c_int as std::ffi::c_uint
            {
            } else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"!mc->mc_dbi || NUMKEYS(mp) > 1\0" as *const u8 as *const std::ffi::c_char,
                    (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                        b"mdb_page_search_root\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                    6560 as std::ffi::c_int,
                );
            };
            if flags & (4 as std::ffi::c_int | 8 as std::ffi::c_int) != 0 {
                i = 0 as std::ffi::c_int as indx_t;
                if flags & 8 as std::ffi::c_int != 0 {
                    i = (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                        as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                        as indx_t;
                    if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0 {
                        if (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int
                            == i as std::ffi::c_int
                        {
                            let fresh21 = (*mc).mc_snum;
                            (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
                            (*mc).mc_top = fresh21;
                            mp = (*mc).mc_pg[(*mc).mc_top as usize];
                            current_block_30 = 11028031168362772848;
                        } else {
                            current_block_30 = 4495394744059808450;
                        }
                    } else {
                        current_block_30 = 4495394744059808450;
                    }
                } else {
                    current_block_30 = 4495394744059808450;
                }
            } else {
                let mut exact: std::ffi::c_int = 0;
                node = mdb_node_search(mc, key, &mut exact);
                if node.is_null() {
                    i = (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                        as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                        as indx_t;
                } else {
                    i = (*mc).mc_ki[(*mc).mc_top as usize];
                    if exact == 0 {
                        if i as std::ffi::c_int > 0 as std::ffi::c_int {
                        } else {
                            mdb_assert_fail(
                                (*(*mc).mc_txn).mt_env,
                                b"i > 0\0" as *const u8 as *const std::ffi::c_char,
                                (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                                    b"mdb_page_search_root\0",
                                ))
                                .as_ptr(),
                                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                                6584 as std::ffi::c_int,
                            );
                        };
                        i = i.wrapping_sub(1);
                    }
                }
                current_block_30 = 4495394744059808450;
            }
            if current_block_30 == 4495394744059808450 {
                if (i as std::ffi::c_uint)
                    < ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                        as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int
                {
                } else {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"i < NUMKEYS(mp)\0" as *const u8 as *const std::ffi::c_char,
                        (*::core::mem::transmute::<&[u8; 21], &[std::ffi::c_char; 21]>(
                            b"mdb_page_search_root\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                        6591 as std::ffi::c_int,
                    );
                };
                node = (mp as *mut std::ffi::c_char)
                    .offset(
                        *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                            .as_mut_ptr()
                            .offset(i as isize) as std::ffi::c_int as isize,
                    )
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                rc = mdb_page_get(
                    mc,
                    (*node).mn_lo as pgno_t
                        | ((*node).mn_hi as pgno_t) << 16 as std::ffi::c_int
                        | (if (if -(1 as std::ffi::c_int) as pgno_t
                            > 0xffffffff as std::ffi::c_uint as pgno_t
                        {
                            32 as std::ffi::c_int
                        } else {
                            0 as std::ffi::c_int
                        }) != 0
                        {
                            ((*node).mn_flags as pgno_t)
                                << (if -(1 as std::ffi::c_int) as pgno_t
                                    > 0xffffffff as std::ffi::c_uint as pgno_t
                                {
                                    32 as std::ffi::c_int
                                } else {
                                    0 as std::ffi::c_int
                                })
                        } else {
                            0 as std::ffi::c_int as pgno_t
                        }),
                    &mut mp,
                    std::ptr::null_mut::<std::ffi::c_int>(),
                );
                if rc != 0 as std::ffi::c_int {
                    return rc;
                }
                (*mc).mc_ki[(*mc).mc_top as usize] = i;
                rc = mdb_cursor_push(mc, mp);
                if rc != 0 {
                    return rc;
                }
            }
            if flags & 1 as std::ffi::c_int != 0 {
                rc = mdb_page_touch(mc);
                if rc != 0 as std::ffi::c_int {
                    return rc;
                }
                mp = (*mc).mc_pg[(*mc).mc_top as usize];
            }
        }
        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            != 0x2 as std::ffi::c_int
        {
            (*(*mc).mc_txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
            return -(30796 as std::ffi::c_int);
        }
        (*mc).mc_flags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
        (*mc).mc_flags &= !(0x2 as std::ffi::c_int) as std::ffi::c_uint;
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_page_search_lowest(mut mc: *mut MDB_cursor) -> std::ffi::c_int {
    unsafe {
        let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
        let mut node: *mut MDB_node = (mp as *mut std::ffi::c_char)
            .offset(
                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(0 as std::ffi::c_int as isize) as std::ffi::c_int
                    as isize,
            )
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        let mut rc: std::ffi::c_int = 0;
        rc = mdb_page_get(
            mc,
            (*node).mn_lo as pgno_t
                | ((*node).mn_hi as pgno_t) << 16 as std::ffi::c_int
                | (if (if -(1 as std::ffi::c_int) as pgno_t
                    > 0xffffffff as std::ffi::c_uint as pgno_t
                {
                    32 as std::ffi::c_int
                } else {
                    0 as std::ffi::c_int
                }) != 0
                {
                    ((*node).mn_flags as pgno_t)
                        << (if -(1 as std::ffi::c_int) as pgno_t
                            > 0xffffffff as std::ffi::c_uint as pgno_t
                        {
                            32 as std::ffi::c_int
                        } else {
                            0 as std::ffi::c_int
                        })
                } else {
                    0 as std::ffi::c_int as pgno_t
                }),
            &mut mp,
            std::ptr::null_mut::<std::ffi::c_int>(),
        );
        if rc != 0 as std::ffi::c_int {
            return rc;
        }
        (*mc).mc_ki[(*mc).mc_top as usize] = 0 as std::ffi::c_int as indx_t;
        rc = mdb_cursor_push(mc, mp);
        if rc != 0 {
            return rc;
        }
        mdb_page_search_root(mc, std::ptr::null_mut::<MDB_val>(), 4 as std::ffi::c_int)
    }
}
unsafe extern "C" fn mdb_page_search(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut flags: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0;
        let mut root: pgno_t = 0;
        if (*(*mc).mc_txn).mt_flags
            & (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int)
                as std::ffi::c_uint
            != 0
        {
            return -(30782 as std::ffi::c_int);
        } else {
            if *(*mc).mc_dbflag as std::ffi::c_int & 0x2 as std::ffi::c_int != 0 {
                let mut mc2: MDB_cursor = MDB_cursor {
                    mc_next: std::ptr::null_mut::<MDB_cursor>(),
                    mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                    mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                    mc_txn: std::ptr::null_mut::<MDB_txn>(),
                    mc_dbi: 0,
                    mc_db: std::ptr::null_mut::<MDB_db>(),
                    mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                    mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                    mc_snum: 0,
                    mc_top: 0,
                    mc_flags: 0,
                    mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                    mc_ki: [0; 32],
                };
                if *((*(*mc).mc_txn).mt_dbiseqs).offset((*mc).mc_dbi as isize)
                    != *((*(*(*mc).mc_txn).mt_env).me_dbiseqs).offset((*mc).mc_dbi as isize)
                {
                    return -(30780 as std::ffi::c_int);
                }
                mdb_cursor_init(
                    &mut mc2,
                    (*mc).mc_txn,
                    1 as std::ffi::c_int as MDB_dbi,
                    std::ptr::null_mut::<MDB_xcursor>(),
                );
                rc = mdb_page_search(&mut mc2, &mut (*(*mc).mc_dbx).md_name, 0 as std::ffi::c_int);
                if rc != 0 {
                    return rc;
                }
                let mut data: MDB_val =
                    MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
                let mut exact: std::ffi::c_int = 0 as std::ffi::c_int;
                let mut flags_0: uint16_t = 0;
                let mut leaf: *mut MDB_node =
                    mdb_node_search(&mut mc2, &mut (*(*mc).mc_dbx).md_name, &mut exact);
                if exact == 0 {
                    return -(30780 as std::ffi::c_int);
                }
                if (*leaf).mn_flags as std::ffi::c_int
                    & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                    != 0x2 as std::ffi::c_int
                {
                    return -(30784 as std::ffi::c_int);
                }
                rc = mdb_node_read(&mut mc2, leaf, &mut data);
                if rc != 0 {
                    return rc;
                }
                memcpy(
                    &mut flags_0 as *mut uint16_t as *mut std::ffi::c_void,
                    (data.mv_data as *mut std::ffi::c_char).offset(4 as std::ffi::c_ulong as isize)
                        as *const std::ffi::c_void,
                    ::core::mem::size_of::<uint16_t>() as std::ffi::c_ulong,
                );
                if (*(*mc).mc_db).md_flags as std::ffi::c_int
                    & (0xffff as std::ffi::c_int & !(0x8000 as std::ffi::c_int))
                    != flags_0 as std::ffi::c_int
                {
                    return -(30784 as std::ffi::c_int);
                }
                memcpy(
                    (*mc).mc_db as *mut std::ffi::c_void,
                    data.mv_data,
                    ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong,
                );
                *(*mc).mc_dbflag = (*(*mc).mc_dbflag as std::ffi::c_int & !(0x2 as std::ffi::c_int))
                    as std::ffi::c_uchar;
            }
            root = (*(*mc).mc_db).md_root;
            if root == !(0 as std::ffi::c_int as pgno_t) {
                return -(30798 as std::ffi::c_int);
            }
        }
        if root > 1 as std::ffi::c_int as pgno_t {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"root > 1\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 16], &[std::ffi::c_char; 16]>(
                    b"mdb_page_search\0",
                ))
                .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                6711 as std::ffi::c_int,
            );
        };
        if ((*mc).mc_pg[0 as std::ffi::c_int as usize]).is_null()
            || (*(*mc).mc_pg[0 as std::ffi::c_int as usize]).mp_p.p_pgno != root
        {
            rc = mdb_page_get(
                mc,
                root,
                &mut *((*mc).mc_pg).as_mut_ptr().offset(0 as std::ffi::c_int as isize),
                std::ptr::null_mut::<std::ffi::c_int>(),
            );
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
        }
        (*mc).mc_snum = 1 as std::ffi::c_int as std::ffi::c_ushort;
        (*mc).mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
        if flags & 1 as std::ffi::c_int != 0 {
            rc = mdb_page_touch(mc);
            if rc != 0 {
                return rc;
            }
        }
        if flags & 2 as std::ffi::c_int != 0 {
            return 0 as std::ffi::c_int;
        }
        mdb_page_search_root(mc, key, flags)
    }
}
unsafe extern "C" fn mdb_ovpage_free(
    mut mc: *mut MDB_cursor,
    mut mp: *mut MDB_page,
) -> std::ffi::c_int {
    unsafe {
        let mut txn: *mut MDB_txn = (*mc).mc_txn;
        let mut pg: pgno_t = (*mp).mp_p.p_pgno;
        let mut x: u32 = 0;
        let mut ovpages = (*mp).mp_pb.pb_pages;
        let mut env: *mut MDB_env = (*txn).mt_env;
        let mut sl: Option<&mut MDB_IDL> = (*txn).mt_spill_pgs.as_mut();
        let mut pn: MDB_ID = pg << 1;
        let mut rc: std::ffi::c_int = 0;

        // If the page is dirty or on the spill list we just acquired it,
        // so we should give it back to our current free list, if any.
        // Otherwise put it onto the list of pages we freed in this txn.
        //
        // Won't create me_pghead: me_pglast must be inited along with it.
        // Unsupported in nested txns: They would need to hide the page
        // range in ancestor txns' dirty and spilled lists.
        if let Some(pg_head) = ((*env).me_pgstate.mf_pghead).as_mut()
            && ((*txn).mt_parent).is_null()
            && ((*mp).mp_flags & PageFlags::P_DIRTY.bits() != 0
                || (sl.as_mut().map_or(false, |sl| {
                    x = mdb_midl_search(sl, pn);
                    x as MDB_ID <= sl.len()
                }) && sl.as_mut().map_or(false, |sl| sl[x as usize] == pn)))
        {
            let mut i: std::ffi::c_uint = 0;
            let mut j: std::ffi::c_uint = 0;
            // let mut mop = None;
            let mut dl: *mut MDB_ID2 = std::ptr::null_mut::<MDB_ID2>();
            let mut ix: MDB_ID2 =
                MDB_ID2 { mid: 0, mptr: std::ptr::null_mut::<std::ffi::c_void>() };
            let mut iy: MDB_ID2 =
                MDB_ID2 { mid: 0, mptr: std::ptr::null_mut::<std::ffi::c_void>() };
            rc = mdb_midl_need(pg_head, ovpages as usize);
            if rc != 0 {
                return rc;
            }
            if (*mp).mp_flags & PageFlags::P_DIRTY.bits() == 0
                && let Some(sl) = sl.as_mut()
            {
                /* This page is no longer spilled */
                if x as MDB_ID == sl.len() {
                    sl.pop();
                } else {
                    sl[x as usize] |= 1 as MDB_ID;
                }
            } else {
                /* Remove from dirty list */
                dl = (*txn).mt_u.dirty_list;
                let fresh23 = &mut (*dl.offset(0 as std::ffi::c_int as isize)).mid;
                let fresh24 = *fresh23;
                *fresh23 = (*fresh23).wrapping_sub(1);
                x = fresh24 as std::ffi::c_uint;
                ix = *dl.offset(x as isize);
                while ix.mptr != mp as *mut std::ffi::c_void {
                    if x > 1 as std::ffi::c_int as std::ffi::c_uint {
                        x = x.wrapping_sub(1);
                        iy = *dl.offset(x as isize);
                        *dl.offset(x as isize) = ix;
                    } else {
                        if x > 1 as std::ffi::c_int as std::ffi::c_uint {
                        } else {
                            mdb_assert_fail(
                                (*(*mc).mc_txn).mt_env,
                                b"x > 1\0" as *const u8 as *const std::ffi::c_char,
                                (*::core::mem::transmute::<&[u8; 16], &[std::ffi::c_char; 16]>(
                                    b"mdb_ovpage_free\0",
                                ))
                                .as_ptr(),
                                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                                6793 as std::ffi::c_int,
                            );
                        };
                        let fresh25 = &mut (*dl.offset(0 as std::ffi::c_int as isize)).mid;
                        *fresh25 = (*fresh25).wrapping_add(1);
                        j = *fresh25 as std::ffi::c_uint;
                        *dl.offset(j as isize) = ix;
                        (*txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                        return -(30779 as std::ffi::c_int);
                    }
                    ix = iy;
                }
                (*txn).mt_dirty_room += 1;
                if (*env).me_flags & MDB_WRITEMAP == 0 {
                    mdb_dpage_free(env, mp);
                }
            }
            /* Insert in me_pghead */
            // mop = (*env).me_pgstate.mf_pghead;
            let mop = pg_head;
            // j = (*mop.offset(0 as isize)).wrapping_add(ovpages as pgno_t) as std::ffi::c_uint;
            j = mop.len() as u32 + ovpages;
            // i = *mop.offset(0 as isize) as std::ffi::c_uint;
            i = mop.len() as u32;
            mop.extend(std::iter::repeat_n(0, ovpages as usize));
            while i != 0 && mop[i as usize] < pg {
                let fresh26 = j;
                j = j.wrapping_sub(1);
                // *mop.offset(fresh26 as isize) = *mop.offset(i as isize);
                mop[fresh26 as usize] = mop[i as usize];
                i = i.wrapping_sub(1);
            }
            while j > i {
                let fresh27 = pg;
                pg = pg.wrapping_add(1);
                let fresh28 = j;
                j = j.wrapping_sub(1);
                // *mop.offset(fresh28 as isize) = fresh27;
                mop[fresh28 as usize] = fresh27;
            }
            // Note: we extend the vector with zeros MDB_ID above the loops
            // let fresh29 = &mut (*mop.offset(0 as std::ffi::c_int as isize));
            // *fresh29 = (*fresh29).wrapping_add(ovpages as pgno_t);
        } else {
            rc = mdb_midl_append_range(&mut (*txn).mt_free_pgs, pg, ovpages as usize);
            if rc != 0 {
                return rc;
            }
        }
        (*(*mc).mc_db).md_overflow_pages =
            ((*(*mc).mc_db).md_overflow_pages).wrapping_sub(ovpages as pgno_t);
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_node_read(
    mut mc: *mut MDB_cursor,
    mut leaf: *mut MDB_node,
    mut data: *mut MDB_val,
) -> std::ffi::c_int {
    unsafe {
        let mut omp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut pgno: pgno_t = 0;
        let mut rc: std::ffi::c_int = 0;
        std::ptr::null_mut::<MDB_page>().is_null();
        if (*leaf).mn_flags as std::ffi::c_int & 0x1 as std::ffi::c_int != 0x1 as std::ffi::c_int {
            (*data).mv_size = ((*leaf).mn_lo as std::ffi::c_uint
                | ((*leaf).mn_hi as std::ffi::c_uint) << 16 as std::ffi::c_int)
                as size_t;
            (*data).mv_data =
                ((*leaf).mn_data).as_mut_ptr().offset((*leaf).mn_ksize as std::ffi::c_int as isize)
                    as *mut std::ffi::c_void;
            return 0 as std::ffi::c_int;
        }
        (*data).mv_size = ((*leaf).mn_lo as std::ffi::c_uint
            | ((*leaf).mn_hi as std::ffi::c_uint) << 16 as std::ffi::c_int)
            as size_t;
        memcpy(
            &mut pgno as *mut pgno_t as *mut std::ffi::c_void,
            ((*leaf).mn_data).as_mut_ptr().offset((*leaf).mn_ksize as std::ffi::c_int as isize)
                as *mut std::ffi::c_void,
            ::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong,
        );
        rc = mdb_page_get(mc, pgno, &mut omp, std::ptr::null_mut::<std::ffi::c_int>());
        if rc != 0 as std::ffi::c_int {
            return rc;
        }
        (*data).mv_data = (omp as *mut std::ffi::c_char)
            .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
            as *mut std::ffi::c_void;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_get(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> std::ffi::c_int {
    unsafe {
        let mut mc: MDB_cursor = MDB_cursor {
            mc_next: std::ptr::null_mut::<MDB_cursor>(),
            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
            mc_txn: std::ptr::null_mut::<MDB_txn>(),
            mc_dbi: 0,
            mc_db: std::ptr::null_mut::<MDB_db>(),
            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
            mc_ki: [0; 32],
        };
        let mut mx: MDB_xcursor = MDB_xcursor {
            mx_cursor: MDB_cursor {
                mc_next: std::ptr::null_mut::<MDB_cursor>(),
                mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                mc_dbi: 0,
                mc_db: std::ptr::null_mut::<MDB_db>(),
                mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                mc_ki: [0; 32],
            },
            mx_db: MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            },
            mx_dbx: MDB_dbx {
                md_name: MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() },
                md_cmp: None,
                md_dcmp: None,
                md_rel: None,
                md_relctx: std::ptr::null_mut::<std::ffi::c_void>(),
            },
            mx_dbflag: 0,
        };
        let mut exact: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut rc: std::ffi::c_int = 0;
        if key.is_null()
            || data.is_null()
            || !(!txn.is_null()
                && dbi < (*txn).mt_numdbs
                && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                    & 0x10 as std::ffi::c_int
                    != 0)
        {
            return 22 as std::ffi::c_int;
        }
        if (*txn).mt_flags
            & (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int)
                as std::ffi::c_uint
            != 0
        {
            return -(30782 as std::ffi::c_int);
        }
        mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
        rc = mdb_cursor_set(&mut mc, key, data, MDB_SET, &mut exact);
        rc
    }
}
unsafe extern "C" fn mdb_cursor_sibling(
    mut mc: *mut MDB_cursor,
    mut move_right: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0;
        let mut indx: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        if ((*mc).mc_snum as std::ffi::c_int) < 2 as std::ffi::c_int {
            return -(30798 as std::ffi::c_int);
        }
        mdb_cursor_pop(mc);
        if if move_right != 0 {
            (((*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint)
                .wrapping_add(1 as std::ffi::c_uint)
                >= ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_lower as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int) as std::ffi::c_int
        } else {
            ((*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int == 0 as std::ffi::c_int)
                as std::ffi::c_int
        } != 0
        {
            rc = mdb_cursor_sibling(mc, move_right);
            if rc != 0 as std::ffi::c_int {
                (*mc).mc_top = ((*mc).mc_top).wrapping_add(1);
                (*mc).mc_top;
                (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
                (*mc).mc_snum;
                return rc;
            }
        } else if move_right != 0 {
            (*mc).mc_ki[(*mc).mc_top as usize] =
                ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
            (*mc).mc_ki[(*mc).mc_top as usize];
        } else {
            (*mc).mc_ki[(*mc).mc_top as usize] =
                ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_sub(1);
            (*mc).mc_ki[(*mc).mc_top as usize];
        }
        if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
            .mp2_flags as std::ffi::c_int
            & 0x1 as std::ffi::c_int
            == 0x1 as std::ffi::c_int
        {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"IS_BRANCH(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 19], &[std::ffi::c_char; 19]>(
                    b"mdb_cursor_sibling\0",
                ))
                .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                6935 as std::ffi::c_int,
            );
        };
        indx = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_char)
            .offset(
                *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                    as std::ffi::c_int as isize,
            )
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        rc = mdb_page_get(
            mc,
            (*indx).mn_lo as pgno_t
                | ((*indx).mn_hi as pgno_t) << 16 as std::ffi::c_int
                | (if (if -(1 as std::ffi::c_int) as pgno_t
                    > 0xffffffff as std::ffi::c_uint as pgno_t
                {
                    32 as std::ffi::c_int
                } else {
                    0 as std::ffi::c_int
                }) != 0
                {
                    ((*indx).mn_flags as pgno_t)
                        << (if -(1 as std::ffi::c_int) as pgno_t
                            > 0xffffffff as std::ffi::c_uint as pgno_t
                        {
                            32 as std::ffi::c_int
                        } else {
                            0 as std::ffi::c_int
                        })
                } else {
                    0 as std::ffi::c_int as pgno_t
                }),
            &mut mp,
            std::ptr::null_mut::<std::ffi::c_int>(),
        );
        if rc != 0 as std::ffi::c_int {
            (*mc).mc_flags &=
                !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
            return rc;
        }
        mdb_cursor_push(mc, mp);
        if move_right == 0 {
            (*mc).mc_ki[(*mc).mc_top as usize] =
                (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                    as indx_t;
        }
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_cursor_next(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> std::ffi::c_int {
    unsafe {
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut rc: std::ffi::c_int = 0;
        if (*mc).mc_flags & 0x8 as std::ffi::c_int as std::ffi::c_uint != 0
            && op as std::ffi::c_uint == MDB_NEXT_DUP as std::ffi::c_int as std::ffi::c_uint
        {
            return -(30798 as std::ffi::c_int);
        }
        if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0 {
            return mdb_cursor_first(mc, key, data);
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        if (*mc).mc_flags & 0x2 as std::ffi::c_int as std::ffi::c_uint != 0 {
            if (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint
                >= (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                    as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
            {
                return -(30798 as std::ffi::c_int);
            }
            (*mc).mc_flags ^= 0x2 as std::ffi::c_int as std::ffi::c_uint;
        }
        if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x4 as std::ffi::c_int != 0 {
            leaf = (mp as *mut std::ffi::c_char)
                .offset(
                    *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                        as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            if (*leaf).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int
                == 0x4 as std::ffi::c_int
            {
                if op as std::ffi::c_uint == MDB_NEXT as std::ffi::c_int as std::ffi::c_uint
                    || op as std::ffi::c_uint == MDB_NEXT_DUP as std::ffi::c_int as std::ffi::c_uint
                {
                    rc = mdb_cursor_next(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        data,
                        std::ptr::null_mut::<MDB_val>(),
                        MDB_NEXT,
                    );
                    if op as std::ffi::c_uint != MDB_NEXT as std::ffi::c_int as std::ffi::c_uint
                        || rc != -(30798 as std::ffi::c_int)
                    {
                        if rc == 0 as std::ffi::c_int && !key.is_null() {
                            (*key).mv_size = std::ptr::read_unaligned(
                                (leaf as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize)
                                    as *mut u16,
                            ) as size_t;
                            (*key).mv_data =
                                ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                        }
                        return rc;
                    }
                }
            } else {
                (*(*mc).mc_xcursor).mx_cursor.mc_flags &=
                    !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
                if op as std::ffi::c_uint == MDB_NEXT_DUP as std::ffi::c_int as std::ffi::c_uint {
                    return -(30798 as std::ffi::c_int);
                }
            }
        }
        if (*mc).mc_flags & 0x8 as std::ffi::c_int as std::ffi::c_uint != 0 {
            (*mc).mc_flags ^= 0x8 as std::ffi::c_int as std::ffi::c_uint;
        } else if ((*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint)
            .wrapping_add(1 as std::ffi::c_uint)
            >= (std::ptr::read_unaligned(
                (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_lower) as isize) as *mut u16,
            ) as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int
        {
            rc = mdb_cursor_sibling(mc, 1 as std::ffi::c_int);
            if rc != 0 as std::ffi::c_int {
                (*mc).mc_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
        } else {
            (*mc).mc_ki[(*mc).mc_top as usize] =
                ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
            (*mc).mc_ki[(*mc).mc_top as usize];
        }
        if std::ptr::read_unaligned(
            (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_flags) as isize) as *mut u16,
        ) as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key).mv_data = (mp as *mut std::ffi::c_char)
                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                .offset(((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size) as isize)
                as *mut std::ffi::c_void;
            return 0 as std::ffi::c_int;
        }
        if std::ptr::read_unaligned(
            (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_flags) as isize) as *mut u16,
        ) as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            == 0x2 as std::ffi::c_int
        {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"IS_LEAF(mp)\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 16], &[std::ffi::c_char; 16]>(
                    b"mdb_cursor_next\0",
                ))
                .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                7024 as std::ffi::c_int,
            );
        };
        leaf = (mp as *mut std::ffi::c_char)
            .offset(std::ptr::read_unaligned(
                ((mp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize) as *mut u16)
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize),
            ) as std::ffi::c_int as isize)
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if std::ptr::read_unaligned(
            (leaf as *mut u8).offset(offset_of!(MDB_node, mn_flags) as isize) as *mut u8,
        ) as std::ffi::c_int
            & 0x4 as std::ffi::c_int
            == 0x4 as std::ffi::c_int
        {
            mdb_xcursor_init1(mc, leaf);
            rc = mdb_cursor_first(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                std::ptr::null_mut::<MDB_val>(),
            );
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
        } else if !data.is_null() {
            rc = mdb_node_read(mc, leaf, data);
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
        }
        if !key.is_null() {
            (*key).mv_size = std::ptr::read_unaligned(
                (leaf as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize) as *mut u16,
            ) as size_t;
            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
        }
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_cursor_prev(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> std::ffi::c_int {
    unsafe {
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut rc: std::ffi::c_int = 0;
        if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0 {
            rc = mdb_cursor_last(mc, key, data);
            if rc != 0 {
                return rc;
            }
            (*mc).mc_ki[(*mc).mc_top as usize] =
                ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
            (*mc).mc_ki[(*mc).mc_top as usize];
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x4 as std::ffi::c_int != 0
            && ((*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint)
                < ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int
        {
            leaf = (mp as *mut std::ffi::c_char)
                .offset(
                    *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                        as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            if (*leaf).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int
                == 0x4 as std::ffi::c_int
            {
                if op as std::ffi::c_uint == MDB_PREV as std::ffi::c_int as std::ffi::c_uint
                    || op as std::ffi::c_uint == MDB_PREV_DUP as std::ffi::c_int as std::ffi::c_uint
                {
                    rc = mdb_cursor_prev(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        data,
                        std::ptr::null_mut::<MDB_val>(),
                        MDB_PREV,
                    );
                    if op as std::ffi::c_uint != MDB_PREV as std::ffi::c_int as std::ffi::c_uint
                        || rc != -(30798 as std::ffi::c_int)
                    {
                        if rc == 0 as std::ffi::c_int {
                            if !key.is_null() {
                                (*key).mv_size = std::ptr::read_unaligned(
                                    (leaf as *mut u8)
                                        .offset(offset_of!(MDB_node, mn_ksize) as isize)
                                        as *mut u16,
                                ) as size_t;
                                (*key).mv_data =
                                    ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                            }
                            (*mc).mc_flags &= !(0x2 as std::ffi::c_int) as std::ffi::c_uint;
                        }
                        return rc;
                    }
                }
            } else {
                (*(*mc).mc_xcursor).mx_cursor.mc_flags &=
                    !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
                if op as std::ffi::c_uint == MDB_PREV_DUP as std::ffi::c_int as std::ffi::c_uint {
                    return -(30798 as std::ffi::c_int);
                }
            }
        }
        (*mc).mc_flags &= !(0x2 as std::ffi::c_int | 0x8 as std::ffi::c_int) as std::ffi::c_uint;
        if (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int == 0 as std::ffi::c_int {
            rc = mdb_cursor_sibling(mc, 0 as std::ffi::c_int);
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
            (*mc).mc_ki[(*mc).mc_top as usize] =
                (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                    as indx_t;
        } else {
            (*mc).mc_ki[(*mc).mc_top as usize] =
                ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_sub(1);
            (*mc).mc_ki[(*mc).mc_top as usize];
        }
        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            != 0x2 as std::ffi::c_int
        {
            return -(30796 as std::ffi::c_int);
        }
        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
            (*key).mv_data = (mp as *mut std::ffi::c_char)
                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                .offset(((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size) as isize)
                as *mut std::ffi::c_void;
            return 0 as std::ffi::c_int;
        }
        leaf = (mp as *mut std::ffi::c_char)
            .offset(
                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                    as std::ffi::c_int as isize,
            )
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*leaf).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int == 0x4 as std::ffi::c_int {
            mdb_xcursor_init1(mc, leaf);
            rc = mdb_cursor_last(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                std::ptr::null_mut::<MDB_val>(),
            );
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
        } else if !data.is_null() {
            rc = mdb_node_read(mc, leaf, data);
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
        }
        if !key.is_null() {
            (*key).mv_size = std::ptr::read_unaligned(
                (leaf as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize) as *mut u16,
            ) as size_t;
            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
        }
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_cursor_set(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
    mut exactp: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    #[allow(unpredictable_function_pointer_comparisons)]
    unsafe {
        let mut current_block: u64;
        let mut rc: std::ffi::c_int = 0;
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        if (*key).mv_size == 0 as std::ffi::c_int as size_t {
            return -(30781 as std::ffi::c_int);
        }
        if !((*mc).mc_xcursor).is_null() {
            (*(*mc).mc_xcursor).mx_cursor.mc_flags &=
                !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
        }
        if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0 {
            let mut nodekey: MDB_val =
                MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
            if (std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2).mp2_lower
                as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int
                == 0
            {
                (*mc).mc_ki[(*mc).mc_top as usize] = 0 as std::ffi::c_int as indx_t;
                return -(30798 as std::ffi::c_int);
            }
            if std::ptr::read_unaligned(
                (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_flags) as isize) as *mut u16,
            ) as std::ffi::c_int
                & 0x20 as std::ffi::c_int
                != 0
            {
                nodekey.mv_size = (*(*mc).mc_db).md_pad as size_t;
                nodekey.mv_data = (mp as *mut std::ffi::c_char)
                    .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                    .offset((0 as std::ffi::c_int as size_t * nodekey.mv_size) as isize)
                    as *mut std::ffi::c_void;
            } else {
                leaf = (mp as *mut std::ffi::c_char)
                    .offset(std::ptr::read_unaligned(
                        ((mp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize)
                            as *mut u16)
                            .offset(0 as isize),
                    ) as isize)
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                nodekey.mv_size = std::ptr::read_unaligned(
                    (leaf as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize) as *mut u16,
                ) as size_t;
                nodekey.mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
            }
            rc = ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")(key, &mut nodekey);
            if rc == 0 as std::ffi::c_int {
                (*mc).mc_ki[(*mc).mc_top as usize] = 0 as std::ffi::c_int as indx_t;
                if !exactp.is_null() {
                    *exactp = 1 as std::ffi::c_int;
                }
                current_block = 6408777650154919156;
            } else {
                if rc > 0 as std::ffi::c_int {
                    let mut i: std::ffi::c_uint = 0;
                    let mut nkeys: std::ffi::c_uint = ((*(mp as *mut std::ffi::c_void
                        as *mut MDB_page2))
                        .mp2_lower
                        as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int;
                    if nkeys > 1 as std::ffi::c_int as std::ffi::c_uint {
                        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                            as std::ffi::c_int
                            & 0x20 as std::ffi::c_int
                            != 0
                        {
                            nodekey.mv_data = (mp as *mut std::ffi::c_char)
                                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                                .offset(
                                    (nkeys.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                                        as size_t
                                        * nodekey.mv_size)
                                        as isize,
                                )
                                as *mut std::ffi::c_void;
                        } else {
                            leaf = (mp as *mut std::ffi::c_char)
                                .offset(
                                    *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                        .as_mut_ptr()
                                        .offset(
                                            nkeys.wrapping_sub(
                                                1 as std::ffi::c_int as std::ffi::c_uint,
                                            ) as isize,
                                        ) as std::ffi::c_int
                                        as isize,
                                )
                                .offset(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            nodekey.mv_size = std::ptr::read_unaligned(
                                (leaf as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize)
                                    as *mut u16,
                            ) as size_t;
                            nodekey.mv_data =
                                ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                        }
                        rc = ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")(
                            key,
                            &mut nodekey,
                        );
                        if rc == 0 as std::ffi::c_int {
                            (*mc).mc_ki[(*mc).mc_top as usize] = nkeys
                                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                                as indx_t;
                            if !exactp.is_null() {
                                *exactp = 1 as std::ffi::c_int;
                            }
                            current_block = 6408777650154919156;
                        } else if rc < 0 as std::ffi::c_int {
                            if ((*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint)
                                < ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                                    as std::ffi::c_uint)
                                    .wrapping_sub(
                                        (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                            if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            },
                                        ),
                                    )
                                    >> 1 as std::ffi::c_int
                            {
                                if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                                    as std::ffi::c_int
                                    & 0x20 as std::ffi::c_int
                                    != 0
                                {
                                    nodekey.mv_data = (mp as *mut std::ffi::c_char)
                                        .offset(
                                            16 as std::ffi::c_ulong as std::ffi::c_uint as isize,
                                        )
                                        .offset(
                                            ((*mc).mc_ki[(*mc).mc_top as usize] as size_t
                                                * nodekey.mv_size)
                                                as isize,
                                        )
                                        as *mut std::ffi::c_void;
                                } else {
                                    leaf = (mp as *mut std::ffi::c_char)
                                        .offset(
                                            *((*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                                .mp2_ptrs)
                                                .as_mut_ptr()
                                                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                                as std::ffi::c_int
                                                as isize,
                                        )
                                        .offset(
                                            (if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            }) as isize,
                                        )
                                        as *mut MDB_node;
                                    nodekey.mv_size = std::ptr::read_unaligned(
                                        (leaf as *mut u8)
                                            .offset(offset_of!(MDB_node, mn_ksize) as isize)
                                            as *mut u16,
                                    )
                                        as size_t;
                                    nodekey.mv_data =
                                        ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                                }
                                rc = ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")(
                                    key,
                                    &mut nodekey,
                                );
                                if rc == 0 as std::ffi::c_int {
                                    if !exactp.is_null() {
                                        *exactp = 1 as std::ffi::c_int;
                                    }
                                    current_block = 6408777650154919156;
                                } else {
                                    current_block = 3160140712158701372;
                                }
                            } else {
                                current_block = 3160140712158701372;
                            }
                            match current_block {
                                6408777650154919156 => {}
                                _ => {
                                    rc = 0 as std::ffi::c_int;
                                    (*mc).mc_flags &= !(0x2 as std::ffi::c_int) as std::ffi::c_uint;
                                    current_block = 4280090506452962206;
                                }
                            }
                        } else {
                            current_block = 13619784596304402172;
                        }
                    } else {
                        current_block = 13619784596304402172;
                    }
                    match current_block {
                        4280090506452962206 => {}
                        6408777650154919156 => {}
                        _ => {
                            i = 0 as std::ffi::c_int as std::ffi::c_uint;
                            while i < (*mc).mc_top as std::ffi::c_uint {
                                if ((*mc).mc_ki[i as usize] as std::ffi::c_uint)
                                    < (((*((*mc).mc_pg[i as usize] as *mut std::ffi::c_void
                                        as *mut MDB_page2))
                                        .mp2_lower
                                        as std::ffi::c_uint)
                                        .wrapping_sub(
                                            (16 as std::ffi::c_ulong as std::ffi::c_uint)
                                                .wrapping_sub(if 0 as std::ffi::c_int != 0 {
                                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                                } else {
                                                    0 as std::ffi::c_int as std::ffi::c_uint
                                                }),
                                        )
                                        >> 1 as std::ffi::c_int)
                                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                                {
                                    break;
                                }
                                i = i.wrapping_add(1);
                            }
                            if i == (*mc).mc_top as std::ffi::c_uint {
                                (*mc).mc_ki[(*mc).mc_top as usize] = nkeys as indx_t;
                                return -(30798 as std::ffi::c_int);
                            }
                            current_block = 8347882395825654554;
                        }
                    }
                } else {
                    current_block = 8347882395825654554;
                }
                match current_block {
                    6408777650154919156 => {}
                    4280090506452962206 => {}
                    _ => {
                        if (*mc).mc_top == 0 {
                            (*mc).mc_ki[(*mc).mc_top as usize] = 0 as std::ffi::c_int as indx_t;
                            if op as std::ffi::c_uint
                                == MDB_SET_RANGE as std::ffi::c_int as std::ffi::c_uint
                                && exactp.is_null()
                            {
                                rc = 0 as std::ffi::c_int;
                            } else {
                                return -(30798 as std::ffi::c_int);
                            }
                            current_block = 6408777650154919156;
                        } else {
                            current_block = 15594603006322722090;
                        }
                    }
                }
            }
        } else {
            (*mc).mc_pg[0 as std::ffi::c_int as usize] = std::ptr::null_mut::<MDB_page>();
            current_block = 15594603006322722090;
        }
        if current_block == 15594603006322722090 {
            rc = mdb_page_search(mc, key, 0 as std::ffi::c_int);
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
            if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                & 0x2 as std::ffi::c_int
                == 0x2 as std::ffi::c_int
            {
            } else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"IS_LEAF(mp)\0" as *const u8 as *const std::ffi::c_char,
                    (*::core::mem::transmute::<&[u8; 15], &[std::ffi::c_char; 15]>(
                        b"mdb_cursor_set\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                    7243 as std::ffi::c_int,
                );
            };
            current_block = 4280090506452962206;
        }
        if current_block == 4280090506452962206 {
            leaf = mdb_node_search(mc, key, exactp);
            if !exactp.is_null() && *exactp == 0 {
                return -(30798 as std::ffi::c_int);
            }
            if leaf.is_null() {
                rc = mdb_cursor_sibling(mc, 1 as std::ffi::c_int);
                if rc != 0 as std::ffi::c_int {
                    (*mc).mc_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                    return rc;
                }
                mp = (*mc).mc_pg[(*mc).mc_top as usize];
                if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                    & 0x2 as std::ffi::c_int
                    == 0x2 as std::ffi::c_int
                {
                } else {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"IS_LEAF(mp)\0" as *const u8 as *const std::ffi::c_char,
                        (*::core::mem::transmute::<&[u8; 15], &[std::ffi::c_char; 15]>(
                            b"mdb_cursor_set\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                        7259 as std::ffi::c_int,
                    );
                };
                leaf = (mp as *mut std::ffi::c_char)
                    .offset(std::ptr::read_unaligned(
                        ((mp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize)
                            as *mut u16)
                            .offset(0 as isize),
                    ) as isize)
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
            }
        }
        (*mc).mc_flags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
        (*mc).mc_flags &= !(0x2 as std::ffi::c_int) as std::ffi::c_uint;
        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            if op as std::ffi::c_uint == MDB_SET_RANGE as std::ffi::c_int as std::ffi::c_uint
                || op as std::ffi::c_uint == MDB_SET_KEY as std::ffi::c_int as std::ffi::c_uint
            {
                (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
                (*key).mv_data = (mp as *mut std::ffi::c_char)
                    .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                    .offset(
                        ((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size) as isize,
                    ) as *mut std::ffi::c_void;
            }
            return 0 as std::ffi::c_int;
        }
        if (*leaf).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int == 0x4 as std::ffi::c_int {
            mdb_xcursor_init1(mc, leaf);
            if op as std::ffi::c_uint == MDB_SET as std::ffi::c_int as std::ffi::c_uint
                || op as std::ffi::c_uint == MDB_SET_KEY as std::ffi::c_int as std::ffi::c_uint
                || op as std::ffi::c_uint == MDB_SET_RANGE as std::ffi::c_int as std::ffi::c_uint
            {
                rc = mdb_cursor_first(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    std::ptr::null_mut::<MDB_val>(),
                );
            } else {
                let mut ex2: std::ffi::c_int = 0;
                let mut ex2p: *mut std::ffi::c_int = std::ptr::null_mut::<std::ffi::c_int>();
                if op as std::ffi::c_uint == MDB_GET_BOTH as std::ffi::c_int as std::ffi::c_uint {
                    ex2p = &mut ex2;
                    ex2 = 0 as std::ffi::c_int;
                } else {
                    ex2p = std::ptr::null_mut::<std::ffi::c_int>();
                }
                rc = mdb_cursor_set(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    std::ptr::null_mut::<MDB_val>(),
                    MDB_SET_RANGE,
                    ex2p,
                );
                if rc != 0 as std::ffi::c_int {
                    return rc;
                }
            }
        } else if !data.is_null() {
            if op as std::ffi::c_uint == MDB_GET_BOTH as std::ffi::c_int as std::ffi::c_uint
                || op as std::ffi::c_uint
                    == MDB_GET_BOTH_RANGE as std::ffi::c_int as std::ffi::c_uint
            {
                let mut olddata: MDB_val =
                    MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
                let mut dcmp: Option<MDB_cmp_func> = None;
                rc = mdb_node_read(mc, leaf, &mut olddata);
                if rc != 0 as std::ffi::c_int {
                    return rc;
                }
                dcmp = (*(*mc).mc_dbx).md_dcmp;
                if (0xffffffff as std::ffi::c_uint as std::ffi::c_ulong)
                    < 18446744073709551615 as std::ffi::c_ulong
                    && dcmp == Some(mdb_cmp_int as MDB_cmp_func)
                    && olddata.mv_size == ::core::mem::size_of::<mdb_size_t>() as std::ffi::c_ulong
                {
                    dcmp = Some(mdb_cmp_cint as MDB_cmp_func);
                }
                rc = dcmp.expect("non-null function pointer")(data, &mut olddata);
                if rc != 0 {
                    if op as std::ffi::c_uint == MDB_GET_BOTH as std::ffi::c_int as std::ffi::c_uint
                        || rc > 0 as std::ffi::c_int
                    {
                        return -(30798 as std::ffi::c_int);
                    }
                    rc = 0 as std::ffi::c_int;
                }
                *data = olddata;
            } else {
                if !((*mc).mc_xcursor).is_null() {
                    (*(*mc).mc_xcursor).mx_cursor.mc_flags &=
                        !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
                }
                rc = mdb_node_read(mc, leaf, data);
                if rc != 0 as std::ffi::c_int {
                    return rc;
                }
            }
        }
        if (op as std::ffi::c_uint == MDB_SET_RANGE as std::ffi::c_int as std::ffi::c_uint
            || op as std::ffi::c_uint == MDB_SET_KEY as std::ffi::c_int as std::ffi::c_uint)
            && !key.is_null()
        {
            (*key).mv_size = std::ptr::read_unaligned(
                (leaf as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize) as *mut u16,
            ) as size_t;
            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
        }
        rc
    }
}
unsafe extern "C" fn mdb_cursor_first(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0;
        let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        if !((*mc).mc_xcursor).is_null() {
            (*(*mc).mc_xcursor).mx_cursor.mc_flags &=
                !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
        }
        if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0
            || (*mc).mc_top as std::ffi::c_int != 0
        {
            rc = mdb_page_search(mc, std::ptr::null_mut::<MDB_val>(), 4 as std::ffi::c_int);
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
        }
        if std::ptr::read_unaligned(
            ((*mc).mc_pg[(*mc).mc_top as usize] as *mut u8)
                .offset(offset_of!(MDB_page2, mp2_flags) as isize) as *mut u16,
        ) as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            == 0x2 as std::ffi::c_int
        {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"IS_LEAF(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 17], &[std::ffi::c_char; 17]>(
                    b"mdb_cursor_first\0",
                ))
                .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                7341 as std::ffi::c_int,
            );
        };
        leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_char)
            .offset(std::ptr::read_unaligned(
                (((*mc).mc_pg[(*mc).mc_top as usize] as *mut u8)
                    .offset(offset_of!(MDB_page2, mp2_ptrs) as isize) as *mut u16)
                    .offset(0 as std::ffi::c_int as isize),
            ) as std::ffi::c_int as isize)
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        (*mc).mc_flags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
        (*mc).mc_flags &= !(0x2 as std::ffi::c_int) as std::ffi::c_uint;
        (*mc).mc_ki[(*mc).mc_top as usize] = 0 as std::ffi::c_int as indx_t;
        if std::ptr::read_unaligned(
            ((*mc).mc_pg[(*mc).mc_top as usize] as *mut u8)
                .offset(offset_of!(MDB_page2, mp2_flags) as isize) as *mut u16,
        ) as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            if !key.is_null() {
                (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
                (*key).mv_data = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_char)
                    .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                    .offset((0 as std::ffi::c_int as size_t * (*key).mv_size) as isize)
                    as *mut std::ffi::c_void;
            }
            return 0 as std::ffi::c_int;
        }
        if std::ptr::read_unaligned(
            (leaf as *mut u8).offset(offset_of!(MDB_node, mn_flags) as isize) as *mut u8,
        ) as std::ffi::c_int
            & 0x4 as std::ffi::c_int
            == 0x4 as std::ffi::c_int
        {
            mdb_xcursor_init1(mc, leaf);
            rc = mdb_cursor_first(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                std::ptr::null_mut::<MDB_val>(),
            );
            if rc != 0 {
                return rc;
            }
        } else if !data.is_null() {
            rc = mdb_node_read(mc, leaf, data);
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
        }
        if !key.is_null() {
            (*key).mv_size = std::ptr::read_unaligned(
                (leaf as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize) as *mut u16,
            ) as size_t;
            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
        }
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_cursor_last(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0;
        let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        if !((*mc).mc_xcursor).is_null() {
            (*(*mc).mc_xcursor).mx_cursor.mc_flags &=
                !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
        }
        if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0
            || (*mc).mc_top as std::ffi::c_int != 0
        {
            rc = mdb_page_search(mc, std::ptr::null_mut::<MDB_val>(), 8 as std::ffi::c_int);
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
        }
        if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
            .mp2_flags as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            == 0x2 as std::ffi::c_int
        {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"IS_LEAF(mc->mc_pg[mc->mc_top])\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 16], &[std::ffi::c_char; 16]>(
                    b"mdb_cursor_last\0",
                ))
                .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                7388 as std::ffi::c_int,
            );
        };
        (*mc).mc_ki[(*mc).mc_top as usize] =
            (((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                .mp2_lower as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int)
                .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as indx_t;
        (*mc).mc_flags |= (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
        leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_char)
            .offset(
                *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                    as std::ffi::c_int as isize,
            )
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
            .mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            if !key.is_null() {
                (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
                (*key).mv_data = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_char)
                    .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                    .offset(
                        ((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size) as isize,
                    ) as *mut std::ffi::c_void;
            }
            return 0 as std::ffi::c_int;
        }
        if (*leaf).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int == 0x4 as std::ffi::c_int {
            mdb_xcursor_init1(mc, leaf);
            rc = mdb_cursor_last(
                &mut (*(*mc).mc_xcursor).mx_cursor,
                data,
                std::ptr::null_mut::<MDB_val>(),
            );
            if rc != 0 {
                return rc;
            }
        } else if !data.is_null() {
            rc = mdb_node_read(mc, leaf, data);
            if rc != 0 as std::ffi::c_int {
                return rc;
            }
        }
        if !key.is_null() {
            (*key).mv_size = std::ptr::read_unaligned(
                (leaf as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize) as *mut u16,
            ) as size_t;
            (*key).mv_data = ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
        }
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cursor_get(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut op: MDB_cursor_op,
) -> std::ffi::c_int {
    unsafe {
        let mut mx: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut current_block: u64;
        let mut rc: std::ffi::c_int = 0;
        let mut exact: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut mfunc: Option<
            unsafe extern "C" fn(*mut MDB_cursor, *mut MDB_val, *mut MDB_val) -> std::ffi::c_int,
        > = None;
        if mc.is_null() {
            return 22 as std::ffi::c_int;
        }
        if (*(*mc).mc_txn).mt_flags
            & (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int)
                as std::ffi::c_uint
            != 0
        {
            return -(30782 as std::ffi::c_int);
        }
        match op as std::ffi::c_uint {
            4 => {
                if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0 {
                    rc = 22 as std::ffi::c_int;
                } else {
                    let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
                    let mut nkeys: std::ffi::c_int = (((*(mp as *mut std::ffi::c_void
                        as *mut MDB_page2))
                        .mp2_lower
                        as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int)
                        as std::ffi::c_int;
                    if nkeys == 0 || (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int >= nkeys
                    {
                        (*mc).mc_ki[(*mc).mc_top as usize] = nkeys as indx_t;
                        rc = -(30798 as std::ffi::c_int);
                    } else {
                        rc = 0 as std::ffi::c_int;
                        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                            as std::ffi::c_int
                            & 0x20 as std::ffi::c_int
                            == 0x20 as std::ffi::c_int
                        {
                            (*key).mv_size = (*(*mc).mc_db).md_pad as size_t;
                            (*key).mv_data = (mp as *mut std::ffi::c_char)
                                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                                .offset(
                                    ((*mc).mc_ki[(*mc).mc_top as usize] as size_t * (*key).mv_size)
                                        as isize,
                                )
                                as *mut std::ffi::c_void;
                        } else {
                            let mut leaf: *mut MDB_node = (mp as *mut std::ffi::c_char)
                                .offset(
                                    *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                        .as_mut_ptr()
                                        .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                        as std::ffi::c_int
                                        as isize,
                                )
                                .offset(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as isize,
                                )
                                as *mut MDB_node;
                            if !key.is_null() {
                                (*key).mv_size = std::ptr::read_unaligned(
                                    (leaf as *mut u8)
                                        .offset(offset_of!(MDB_node, mn_ksize) as isize)
                                        as *mut u16,
                                ) as size_t;
                                (*key).mv_data =
                                    ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                            }
                            if !data.is_null() {
                                if (*leaf).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int
                                    == 0x4 as std::ffi::c_int
                                {
                                    rc = mdb_cursor_get(
                                        &mut (*(*mc).mc_xcursor).mx_cursor,
                                        data,
                                        std::ptr::null_mut::<MDB_val>(),
                                        MDB_GET_CURRENT,
                                    );
                                } else {
                                    rc = mdb_node_read(mc, leaf, data);
                                }
                            }
                        }
                    }
                }
                current_block = 1765866445182206997;
            }
            2 | 3 => {
                if data.is_null() {
                    rc = 22 as std::ffi::c_int;
                    current_block = 1765866445182206997;
                } else if ((*mc).mc_xcursor).is_null() {
                    rc = -(30784 as std::ffi::c_int);
                    current_block = 1765866445182206997;
                } else {
                    current_block = 15086062982616303049;
                }
            }
            15..=17 => {
                current_block = 15086062982616303049;
            }
            5 => {
                if data.is_null()
                    || (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0
                {
                    rc = 22 as std::ffi::c_int;
                    current_block = 1765866445182206997;
                } else if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x10 as std::ffi::c_int == 0
                {
                    rc = -(30784 as std::ffi::c_int);
                    current_block = 1765866445182206997;
                } else {
                    rc = 0 as std::ffi::c_int;
                    if (*(*mc).mc_xcursor).mx_cursor.mc_flags
                        & 0x1 as std::ffi::c_int as std::ffi::c_uint
                        == 0
                        || (*(*mc).mc_xcursor).mx_cursor.mc_flags
                            & 0x2 as std::ffi::c_int as std::ffi::c_uint
                            != 0
                    {
                        current_block = 1765866445182206997;
                    } else {
                        current_block = 6189203605807418917;
                    }
                }
            }
            10 => {
                if data.is_null() {
                    rc = 22 as std::ffi::c_int;
                    current_block = 1765866445182206997;
                } else if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x10 as std::ffi::c_int == 0
                {
                    rc = -(30784 as std::ffi::c_int);
                    current_block = 1765866445182206997;
                } else {
                    rc = mdb_cursor_next(mc, key, data, MDB_NEXT_DUP);
                    if rc == 0 as std::ffi::c_int {
                        if (*(*mc).mc_xcursor).mx_cursor.mc_flags
                            & 0x1 as std::ffi::c_int as std::ffi::c_uint
                            != 0
                        {
                            mx = std::ptr::null_mut::<MDB_cursor>();
                            current_block = 6189203605807418917;
                        } else {
                            rc = -(30798 as std::ffi::c_int);
                            current_block = 1765866445182206997;
                        }
                    } else {
                        current_block = 1765866445182206997;
                    }
                }
            }
            18 => {
                if data.is_null() {
                    rc = 22 as std::ffi::c_int;
                    current_block = 1765866445182206997;
                } else if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x10 as std::ffi::c_int == 0
                {
                    rc = -(30784 as std::ffi::c_int);
                    current_block = 1765866445182206997;
                } else {
                    if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0 {
                        rc = mdb_cursor_last(mc, key, data);
                    } else {
                        rc = 0 as std::ffi::c_int;
                    }
                    if rc == 0 as std::ffi::c_int {
                        let mut mx_0: *mut MDB_cursor = &mut (*(*mc).mc_xcursor).mx_cursor;
                        if (*mx_0).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0 {
                            rc = mdb_cursor_sibling(mx_0, 0 as std::ffi::c_int);
                            if rc == 0 as std::ffi::c_int {
                                current_block = 6189203605807418917;
                            } else {
                                current_block = 1765866445182206997;
                            }
                        } else {
                            rc = -(30798 as std::ffi::c_int);
                            current_block = 1765866445182206997;
                        }
                    } else {
                        current_block = 1765866445182206997;
                    }
                }
            }
            8 | 9 | 11 => {
                rc = mdb_cursor_next(mc, key, data, op);
                current_block = 1765866445182206997;
            }
            12..=14 => {
                rc = mdb_cursor_prev(mc, key, data, op);
                current_block = 1765866445182206997;
            }
            0 => {
                rc = mdb_cursor_first(mc, key, data);
                current_block = 1765866445182206997;
            }
            1 => {
                mfunc = Some(
                    mdb_cursor_first
                        as unsafe extern "C" fn(
                            *mut MDB_cursor,
                            *mut MDB_val,
                            *mut MDB_val,
                        ) -> std::ffi::c_int,
                );
                current_block = 18048101459110804040;
            }
            6 => {
                rc = mdb_cursor_last(mc, key, data);
                current_block = 1765866445182206997;
            }
            7 => {
                mfunc = Some(
                    mdb_cursor_last
                        as unsafe extern "C" fn(
                            *mut MDB_cursor,
                            *mut MDB_val,
                            *mut MDB_val,
                        ) -> std::ffi::c_int,
                );
                current_block = 18048101459110804040;
            }
            _ => {
                rc = 22 as std::ffi::c_int;
                current_block = 1765866445182206997;
            }
        }
        match current_block {
            18048101459110804040 => {
                if data.is_null()
                    || (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0
                {
                    rc = 22 as std::ffi::c_int;
                } else if ((*mc).mc_xcursor).is_null() {
                    rc = -(30784 as std::ffi::c_int);
                } else if (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint
                    >= ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                        as *mut MDB_page2))
                        .mp2_lower as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int
                {
                    (*mc).mc_ki[(*mc).mc_top as usize] = (((*((*mc).mc_pg[(*mc).mc_top as usize]
                        as *mut std::ffi::c_void
                        as *mut MDB_page2))
                        .mp2_lower
                        as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int)
                        as indx_t;
                    rc = -(30798 as std::ffi::c_int);
                } else {
                    (*mc).mc_flags &= !(0x2 as std::ffi::c_int) as std::ffi::c_uint;
                    let mut leaf_0: *mut MDB_node = ((*mc).mc_pg[(*mc).mc_top as usize]
                        as *mut std::ffi::c_char)
                        .offset(
                            *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                                as *mut MDB_page2))
                                .mp2_ptrs)
                                .as_mut_ptr()
                                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                as std::ffi::c_int as isize,
                        )
                        .offset(
                            (if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    if (*leaf_0).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int
                        != 0x4 as std::ffi::c_int
                    {
                        if !key.is_null() {
                            (*key).mv_size = (*leaf_0).mn_ksize as size_t;
                            (*key).mv_data =
                                ((*leaf_0).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                        }
                        rc = mdb_node_read(mc, leaf_0, data);
                    } else if (*(*mc).mc_xcursor).mx_cursor.mc_flags
                        & 0x1 as std::ffi::c_int as std::ffi::c_uint
                        == 0
                    {
                        rc = 22 as std::ffi::c_int;
                    } else {
                        rc = mfunc.expect("non-null function pointer")(
                            &mut (*(*mc).mc_xcursor).mx_cursor,
                            data,
                            std::ptr::null_mut::<MDB_val>(),
                        );
                    }
                }
            }
            6189203605807418917 => {
                mx = &mut (*(*mc).mc_xcursor).mx_cursor;
                (*data).mv_size = (((*((*mx).mc_pg[(*mx).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_lower as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int)
                    .wrapping_mul((*(*mx).mc_db).md_pad)
                    as size_t;
                (*data).mv_data = ((*mx).mc_pg[(*mx).mc_top as usize] as *mut std::ffi::c_char)
                    .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                    as *mut std::ffi::c_void;
                (*mx).mc_ki[(*mx).mc_top as usize] =
                    (((*((*mx).mc_pg[(*mx).mc_top as usize] as *mut std::ffi::c_void
                        as *mut MDB_page2))
                        .mp2_lower as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int)
                        .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                        as indx_t;
            }
            15086062982616303049 => {
                if key.is_null() {
                    rc = 22 as std::ffi::c_int;
                } else {
                    rc = mdb_cursor_set(
                        mc,
                        key,
                        data,
                        op,
                        if op as std::ffi::c_uint
                            == MDB_SET_RANGE as std::ffi::c_int as std::ffi::c_uint
                        {
                            std::ptr::null_mut::<std::ffi::c_int>()
                        } else {
                            &mut exact
                        },
                    );
                }
            }
            _ => {}
        }
        if (*mc).mc_flags & 0x8 as std::ffi::c_int as std::ffi::c_uint != 0 {
            (*mc).mc_flags ^= 0x8 as std::ffi::c_int as std::ffi::c_uint;
        }
        rc
    }
}
unsafe extern "C" fn mdb_cursor_touch(mut mc: *mut MDB_cursor) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        if (*mc).mc_dbi >= 2 as std::ffi::c_int as MDB_dbi
            && *(*mc).mc_dbflag as std::ffi::c_int
                & (0x1 as std::ffi::c_int | 0x20 as std::ffi::c_int)
                == 0
        {
            let mut mc2: MDB_cursor = MDB_cursor {
                mc_next: std::ptr::null_mut::<MDB_cursor>(),
                mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                mc_dbi: 0,
                mc_db: std::ptr::null_mut::<MDB_db>(),
                mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                mc_ki: [0; 32],
            };
            let mut mcx: MDB_xcursor = MDB_xcursor {
                mx_cursor: MDB_cursor {
                    mc_next: std::ptr::null_mut::<MDB_cursor>(),
                    mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                    mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                    mc_txn: std::ptr::null_mut::<MDB_txn>(),
                    mc_dbi: 0,
                    mc_db: std::ptr::null_mut::<MDB_db>(),
                    mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                    mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                    mc_snum: 0,
                    mc_top: 0,
                    mc_flags: 0,
                    mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                    mc_ki: [0; 32],
                },
                mx_db: MDB_db {
                    md_pad: 0,
                    md_flags: 0,
                    md_depth: 0,
                    md_branch_pages: 0,
                    md_leaf_pages: 0,
                    md_overflow_pages: 0,
                    md_entries: 0,
                    md_root: 0,
                },
                mx_dbx: MDB_dbx {
                    md_name: MDB_val {
                        mv_size: 0,
                        mv_data: std::ptr::null_mut::<std::ffi::c_void>(),
                    },
                    md_cmp: None,
                    md_dcmp: None,
                    md_rel: None,
                    md_relctx: std::ptr::null_mut::<std::ffi::c_void>(),
                },
                mx_dbflag: 0,
            };
            if *((*(*mc).mc_txn).mt_dbiseqs).offset((*mc).mc_dbi as isize)
                != *((*(*(*mc).mc_txn).mt_env).me_dbiseqs).offset((*mc).mc_dbi as isize)
            {
                return -(30780 as std::ffi::c_int);
            }
            mdb_cursor_init(&mut mc2, (*mc).mc_txn, 1 as std::ffi::c_int as MDB_dbi, &mut mcx);
            rc = mdb_page_search(&mut mc2, &mut (*(*mc).mc_dbx).md_name, 1 as std::ffi::c_int);
            if rc != 0 {
                return rc;
            }
            *(*mc).mc_dbflag =
                (*(*mc).mc_dbflag as std::ffi::c_int | 0x1 as std::ffi::c_int) as std::ffi::c_uchar;
        }
        (*mc).mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
        if (*mc).mc_snum != 0 {
            loop {
                rc = mdb_page_touch(mc);
                if !(rc == 0 && {
                    (*mc).mc_top = ((*mc).mc_top).wrapping_add(1);
                    ((*mc).mc_top as std::ffi::c_int) < (*mc).mc_snum as std::ffi::c_int
                }) {
                    break;
                }
            }
            (*mc).mc_top =
                ((*mc).mc_snum as std::ffi::c_int - 1 as std::ffi::c_int) as std::ffi::c_ushort;
        }
        rc
    }
}
unsafe extern "C" fn _mdb_cursor_put(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: std::ffi::c_uint,
) -> std::ffi::c_int {
    #[allow(unpredictable_function_pointer_comparisons)]
    unsafe {
        let mut i: std::ffi::c_uint = 0;
        let mut offset: std::ffi::c_uint = 0;
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut ksize: std::ffi::c_uint = 0;
        let mut xflags: std::ffi::c_int = 0;
        let mut new_dupdata: std::ffi::c_int = 0;
        let mut ecount: mdb_size_t = 0;
        let mut current_block: u64;
        let mut env: *mut MDB_env = std::ptr::null_mut::<MDB_env>();
        let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut fp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut sub_root: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut fp_flags: uint16_t = 0;
        let mut xdata: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut rdata: *mut MDB_val = std::ptr::null_mut::<MDB_val>();
        let mut dkey: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut olddata: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut dummy: MDB_db = MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        };
        let mut do_sub: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut insert_key: std::ffi::c_int = 0;
        let mut insert_data: std::ffi::c_int = 0;
        let mut mcount: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
        let mut dcount: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
        let mut nospill: std::ffi::c_uint = 0;
        let mut nsize: size_t = 0;
        let mut rc: std::ffi::c_int = 0;
        let mut rc2: std::ffi::c_int = 0;
        let mut nflags: std::ffi::c_uint = 0;
        if mc.is_null() || key.is_null() {
            return 22 as std::ffi::c_int;
        }
        env = (*(*mc).mc_txn).mt_env;
        if flags & 0x80000 as std::ffi::c_int as std::ffi::c_uint != 0 {
            dcount = (*data.offset(1 as std::ffi::c_int as isize)).mv_size as std::ffi::c_uint;
            (*data.offset(1 as std::ffi::c_int as isize)).mv_size = 0 as std::ffi::c_int as size_t;
            if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x10 as std::ffi::c_int
                != 0x10 as std::ffi::c_int
            {
                return -(30784 as std::ffi::c_int);
            }
        }
        nospill = flags & 0x8000 as std::ffi::c_int as std::ffi::c_uint;
        flags &= !(0x8000 as std::ffi::c_int) as std::ffi::c_uint;
        if (*(*mc).mc_txn).mt_flags
            & (0x20000 as std::ffi::c_int
                | (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int))
                as std::ffi::c_uint
            != 0
        {
            return if (*(*mc).mc_txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0
            {
                13 as std::ffi::c_int
            } else {
                -(30782 as std::ffi::c_int)
            };
        }
        if ((*key).mv_size).wrapping_sub(1 as std::ffi::c_int as size_t)
            >= (if 0 as std::ffi::c_int != 0 {
                0 as std::ffi::c_int
            } else {
                511 as std::ffi::c_int
            }) as size_t
        {
            return -(30781 as std::ffi::c_int);
        }
        if (*data).mv_size
            > (if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x4 as std::ffi::c_int != 0 {
                (if 0 as std::ffi::c_int != 0 {
                    0 as std::ffi::c_int
                } else {
                    511 as std::ffi::c_int
                }) as std::ffi::c_ulong
            } else {
                0xffffffff as std::ffi::c_ulong
            })
        {
            return -(30781 as std::ffi::c_int);
        }
        dkey.mv_size = 0 as std::ffi::c_int as size_t;
        if flags & 0x40 as std::ffi::c_int as std::ffi::c_uint != 0 {
            if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0 {
                return 22 as std::ffi::c_int;
            }
            rc = 0 as std::ffi::c_int;
        } else if (*(*mc).mc_db).md_root == !(0 as std::ffi::c_int as pgno_t) {
            (*mc).mc_snum = 0 as std::ffi::c_int as std::ffi::c_ushort;
            (*mc).mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
            (*mc).mc_flags &= !(0x1 as std::ffi::c_int) as std::ffi::c_uint;
            rc = -(30779 as std::ffi::c_int) + 10 as std::ffi::c_int;
        } else {
            let mut exact: std::ffi::c_int = 0 as std::ffi::c_int;
            let mut d2: MDB_val =
                MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
            if flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                let mut k2: MDB_val =
                    MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
                rc = mdb_cursor_last(mc, &mut k2, &mut d2);
                if rc == 0 as std::ffi::c_int {
                    rc = ((*(*mc).mc_dbx).md_cmp).expect("non-null function pointer")(key, &mut k2);
                    if rc > 0 as std::ffi::c_int {
                        rc = -(30798 as std::ffi::c_int);
                        (*mc).mc_ki[(*mc).mc_top as usize] =
                            ((*mc).mc_ki[(*mc).mc_top as usize]).wrapping_add(1);
                        (*mc).mc_ki[(*mc).mc_top as usize];
                    } else {
                        rc = -(30799 as std::ffi::c_int);
                    }
                }
            } else {
                rc = mdb_cursor_set(mc, key, &mut d2, MDB_SET, &mut exact);
            }
            if flags & 0x10 as std::ffi::c_int as std::ffi::c_uint != 0
                && rc == 0 as std::ffi::c_int
            {
                *data = d2;
                return -(30799 as std::ffi::c_int);
            }
            if rc != 0 && rc != -(30798 as std::ffi::c_int) {
                return rc;
            }
        }
        if (*mc).mc_flags & 0x8 as std::ffi::c_int as std::ffi::c_uint != 0 {
            (*mc).mc_flags ^= 0x8 as std::ffi::c_int as std::ffi::c_uint;
        }
        if nospill == 0 {
            if flags & 0x80000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                rdata = &mut xdata;
                xdata.mv_size = (*data).mv_size * dcount as size_t;
            } else {
                rdata = data;
            }
            rc2 = mdb_page_spill(mc, key, rdata);
            if rc2 != 0 {
                return rc2;
            }
        }
        if rc == -(30779 as std::ffi::c_int) + 10 as std::ffi::c_int {
            let mut np: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
            rc2 =
                mdb_page_new(mc, 0x2 as std::ffi::c_int as uint32_t, 1 as std::ffi::c_int, &mut np);
            if rc2 != 0 {
                return rc2;
            }
            mdb_cursor_push(mc, np);
            (*(*mc).mc_db).md_root = (*np).mp_p.p_pgno;
            (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_add(1);
            (*(*mc).mc_db).md_depth;
            *(*mc).mc_dbflag =
                (*(*mc).mc_dbflag as std::ffi::c_int | 0x1 as std::ffi::c_int) as std::ffi::c_uchar;
            if (*(*mc).mc_db).md_flags as std::ffi::c_int
                & (0x4 as std::ffi::c_int | 0x10 as std::ffi::c_int)
                == 0x10 as std::ffi::c_int
            {
                let fresh30 = &mut (*(np as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags;
                *fresh30 = (*fresh30 as std::ffi::c_int | 0x20 as std::ffi::c_int) as uint16_t;
            }
            (*mc).mc_flags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
        } else {
            rc2 = mdb_cursor_touch(mc);
            if rc2 != 0 {
                return rc2;
            }
        }
        insert_data = rc;
        insert_key = insert_data;
        if insert_key != 0 {
            if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x4 as std::ffi::c_int != 0
                && (8 as std::ffi::c_ulong)
                    .wrapping_add((*key).mv_size)
                    .wrapping_add((*data).mv_size)
                    > (*env).me_nodemax as std::ffi::c_ulong
            {
                fp_flags = (0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int) as uint16_t;
                fp = (*env).me_pbuf as *mut MDB_page;
                (*fp).mp_pad = (*data).mv_size as uint16_t;
                let fresh31 = &mut (*(fp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper;
                *fresh31 = (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ) as indx_t;
                (*(fp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower = *fresh31;
                olddata.mv_size = 16 as std::ffi::c_ulong as std::ffi::c_uint as size_t;
                current_block = 13191470234520143498;
            } else {
                current_block = 17353026871531185123;
            }
        } else if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
            .mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            ptr = std::ptr::null_mut::<std::ffi::c_char>();
            ksize = (*(*mc).mc_db).md_pad;
            if (*key).mv_size != ksize as size_t {
                return -(30781 as std::ffi::c_int);
            }
            ptr = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_char)
                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                .offset(
                    ((*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint).wrapping_mul(ksize)
                        as isize,
                );
            memcpy(ptr as *mut std::ffi::c_void, (*key).mv_data, ksize as std::ffi::c_ulong);
            current_block = 5842863371754816627;
        } else {
            current_block = 5486385170289821549;
        }
        loop {
            match current_block {
                17353026871531185123 => {
                    rdata = data;
                    current_block = 11041480184543863428;
                }
                13191470234520143498 => {
                    if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x10 as std::ffi::c_int != 0 {
                        fp_flags =
                            (fp_flags as std::ffi::c_int | 0x20 as std::ffi::c_int) as uint16_t;
                        dummy.md_pad = (*fp).mp_pad as uint32_t;
                        dummy.md_flags = 0x10 as std::ffi::c_int as uint16_t;
                        if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x20 as std::ffi::c_int != 0
                        {
                            dummy.md_flags = (dummy.md_flags as std::ffi::c_int
                                | 0x8 as std::ffi::c_int)
                                as uint16_t;
                        }
                    } else {
                        dummy.md_pad = 0 as std::ffi::c_int as uint32_t;
                        dummy.md_flags = 0 as std::ffi::c_int as uint16_t;
                    }
                    dummy.md_depth = 1 as std::ffi::c_int as uint16_t;
                    dummy.md_branch_pages = 0 as std::ffi::c_int as pgno_t;
                    dummy.md_leaf_pages = 1 as std::ffi::c_int as pgno_t;
                    dummy.md_overflow_pages = 0 as std::ffi::c_int as pgno_t;
                    dummy.md_entries = (((*(fp as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_lower as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int)
                        as mdb_size_t;
                    xdata.mv_size = ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong;
                    xdata.mv_data = &mut dummy as *mut MDB_db as *mut std::ffi::c_void;
                    rc = mdb_page_alloc(mc, 1 as std::ffi::c_int, &mut mp);
                    if rc != 0 {
                        return rc;
                    }
                    offset = ((*env).me_psize as size_t).wrapping_sub(olddata.mv_size)
                        as std::ffi::c_uint;
                    flags |= (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
                    dummy.md_root = (*mp).mp_p.p_pgno;
                    sub_root = mp;
                    current_block = 10601179871800211547;
                }
                5842863371754816627 => {
                    if (*mc).mc_top as std::ffi::c_int != 0
                        && (*mc).mc_ki[(*mc).mc_top as usize] == 0
                    {
                        let mut dtop: std::ffi::c_ushort =
                            1 as std::ffi::c_int as std::ffi::c_ushort;
                        (*mc).mc_top = ((*mc).mc_top).wrapping_sub(1);
                        (*mc).mc_top;
                        while (*mc).mc_top as std::ffi::c_int != 0
                            && (*mc).mc_ki[(*mc).mc_top as usize] == 0
                        {
                            (*mc).mc_top = ((*mc).mc_top).wrapping_sub(1);
                            (*mc).mc_top;
                            dtop = dtop.wrapping_add(1);
                        }
                        if (*mc).mc_ki[(*mc).mc_top as usize] != 0 {
                            rc2 = mdb_update_key(mc, key);
                        } else {
                            rc2 = 0 as std::ffi::c_int;
                        }
                        (*mc).mc_top = ((*mc).mc_top as std::ffi::c_int + dtop as std::ffi::c_int)
                            as std::ffi::c_ushort;
                        if rc2 != 0 {
                            return rc2;
                        }
                    }
                    return 0 as std::ffi::c_int;
                }
                _ => {
                    leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_char)
                        .offset(
                            *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                                as *mut MDB_page2))
                                .mp2_ptrs)
                                .as_mut_ptr()
                                .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                as std::ffi::c_int as isize,
                        )
                        .offset(
                            (if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    olddata.mv_size = ((*leaf).mn_lo as std::ffi::c_uint
                        | ((*leaf).mn_hi as std::ffi::c_uint) << 16 as std::ffi::c_int)
                        as size_t;
                    olddata.mv_data = ((*leaf).mn_data)
                        .as_mut_ptr()
                        .offset((*leaf).mn_ksize as std::ffi::c_int as isize)
                        as *mut std::ffi::c_void;
                    if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x4 as std::ffi::c_int
                        == 0x4 as std::ffi::c_int
                    {
                        i = 0;
                        offset = 0 as std::ffi::c_int as std::ffi::c_uint;
                        xdata.mv_data = (*env).me_pbuf;
                        fp = xdata.mv_data as *mut MDB_page;
                        mp = fp;
                        (*mp).mp_p.p_pgno = (*(*mc).mc_pg[(*mc).mc_top as usize]).mp_p.p_pgno;
                        if (*leaf).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int
                            != 0x4 as std::ffi::c_int
                        {
                            let mut dcmp: Option<MDB_cmp_func> = None;
                            if flags == 0x40 as std::ffi::c_int as std::ffi::c_uint {
                                current_block = 1849955825383499200;
                            } else {
                                dcmp = (*(*mc).mc_dbx).md_dcmp;
                                if (0xffffffff as std::ffi::c_uint as std::ffi::c_ulong)
                                    < 18446744073709551615 as std::ffi::c_ulong
                                    && dcmp == Some(mdb_cmp_int as MDB_cmp_func)
                                    && olddata.mv_size
                                        == ::core::mem::size_of::<mdb_size_t>() as std::ffi::c_ulong
                                {
                                    dcmp = Some(mdb_cmp_cint as MDB_cmp_func);
                                }
                                if dcmp.expect("non-null function pointer")(data, &mut olddata) == 0
                                {
                                    if flags
                                        & (0x20 as std::ffi::c_int | 0x40000 as std::ffi::c_int)
                                            as std::ffi::c_uint
                                        != 0
                                    {
                                        return -(30799 as std::ffi::c_int);
                                    }
                                    current_block = 1849955825383499200;
                                } else {
                                    dkey.mv_size = olddata.mv_size;
                                    dkey.mv_data = memcpy(
                                        fp.offset(1 as std::ffi::c_int as isize)
                                            as *mut std::ffi::c_void,
                                        olddata.mv_data,
                                        olddata.mv_size,
                                    );
                                    (*(fp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags =
                                        (0x2 as std::ffi::c_int
                                            | 0x10 as std::ffi::c_int
                                            | 0x40 as std::ffi::c_int)
                                            as uint16_t;
                                    (*(fp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower =
                                        (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                            if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            },
                                        ) as indx_t;
                                    xdata.mv_size = (16 as std::ffi::c_ulong as std::ffi::c_uint
                                        as size_t)
                                        .wrapping_add(dkey.mv_size)
                                        .wrapping_add((*data).mv_size);
                                    if (*(*mc).mc_db).md_flags as std::ffi::c_int
                                        & 0x10 as std::ffi::c_int
                                        != 0
                                    {
                                        let fresh32 = &mut (*(fp as *mut std::ffi::c_void
                                            as *mut MDB_page2))
                                            .mp2_flags;
                                        *fresh32 = (*fresh32 as std::ffi::c_int
                                            | 0x20 as std::ffi::c_int)
                                            as uint16_t;
                                        (*fp).mp_pad = (*data).mv_size as uint16_t;
                                        xdata.mv_size = (xdata.mv_size).wrapping_add(
                                            2 as std::ffi::c_int as size_t * (*data).mv_size,
                                        );
                                    } else {
                                        xdata.mv_size =
                                            (xdata.mv_size as std::ffi::c_ulong).wrapping_add(
                                                (2 as std::ffi::c_int as std::ffi::c_ulong)
                                                    .wrapping_mul(
                                                        (::core::mem::size_of::<indx_t>()
                                                            as std::ffi::c_ulong)
                                                            .wrapping_add(8 as std::ffi::c_ulong),
                                                    )
                                                    .wrapping_add(
                                                        dkey.mv_size
                                                            & 1 as std::ffi::c_int as size_t,
                                                    )
                                                    .wrapping_add(
                                                        (*data).mv_size
                                                            & 1 as std::ffi::c_int as size_t,
                                                    ),
                                            ) as size_t
                                                as size_t;
                                    }
                                    (*(fp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper =
                                        (xdata.mv_size).wrapping_sub(
                                            (if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            })
                                                as size_t,
                                        ) as indx_t;
                                    olddata.mv_size = xdata.mv_size;
                                    current_block = 248631179418912492;
                                }
                            }
                        } else if (*leaf).mn_flags as std::ffi::c_int & 0x2 as std::ffi::c_int != 0
                        {
                            flags |= (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                as std::ffi::c_uint;
                            current_block = 15483849737692682886;
                        } else {
                            fp = olddata.mv_data as *mut MDB_page;
                            match flags {
                                64 => {
                                    current_block = 3293589076453740040;
                                }
                                _ => {
                                    if (*(*mc).mc_db).md_flags as std::ffi::c_int
                                        & 0x10 as std::ffi::c_int
                                        == 0
                                    {
                                        offset = ((8 as std::ffi::c_ulong)
                                            .wrapping_add(::core::mem::size_of::<indx_t>()
                                                as std::ffi::c_ulong)
                                            .wrapping_add((*data).mv_size)
                                            .wrapping_add(
                                                1 as std::ffi::c_uint as std::ffi::c_ulong,
                                            )
                                            & -(2 as std::ffi::c_int) as std::ffi::c_ulong)
                                            as std::ffi::c_uint;
                                        current_block = 6897179874198677617;
                                    } else {
                                        offset = (*fp).mp_pad as std::ffi::c_uint;
                                        if (((*(fp as *mut std::ffi::c_void as *mut MDB_page2))
                                            .mp2_upper
                                            as std::ffi::c_int
                                            - (*(fp as *mut std::ffi::c_void as *mut MDB_page2))
                                                .mp2_lower
                                                as std::ffi::c_int)
                                            as indx_t
                                            as std::ffi::c_uint)
                                            < offset
                                        {
                                            offset = offset.wrapping_mul(
                                                4 as std::ffi::c_int as std::ffi::c_uint,
                                            );
                                            current_block = 6897179874198677617;
                                        } else {
                                            current_block = 3293589076453740040;
                                        }
                                    }
                                    match current_block {
                                        3293589076453740040 => {}
                                        _ => {
                                            xdata.mv_size =
                                                (olddata.mv_size).wrapping_add(offset as size_t);
                                            current_block = 248631179418912492;
                                        }
                                    }
                                }
                            }
                            match current_block {
                                248631179418912492 => {}
                                _ => {
                                    let fresh33 = &mut (*(fp as *mut std::ffi::c_void
                                        as *mut MDB_page2))
                                        .mp2_flags;
                                    *fresh33 = (*fresh33 as std::ffi::c_int
                                        | 0x10 as std::ffi::c_int)
                                        as uint16_t;
                                    let mut s: *mut std::ffi::c_ushort =
                                        std::ptr::null_mut::<std::ffi::c_ushort>();
                                    let mut d: *mut std::ffi::c_ushort =
                                        std::ptr::null_mut::<std::ffi::c_ushort>();
                                    s = &mut (*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                        .mp2_p
                                        as *mut [uint16_t; 4]
                                        as *mut std::ffi::c_ushort;
                                    d = &mut (*(fp as *mut std::ffi::c_void as *mut MDB_page2))
                                        .mp2_p
                                        as *mut [uint16_t; 4]
                                        as *mut std::ffi::c_ushort;
                                    let fresh34 = s;
                                    s = s.offset(1);
                                    let fresh35 = d;
                                    d = d.offset(1);
                                    *fresh35 = *fresh34;
                                    let fresh36 = s;
                                    s = s.offset(1);
                                    let fresh37 = d;
                                    d = d.offset(1);
                                    *fresh37 = *fresh36;
                                    let fresh38 = s;
                                    s = s.offset(1);
                                    let fresh39 = d;
                                    d = d.offset(1);
                                    *fresh39 = *fresh38;
                                    *d = *s;
                                    (*(*mc).mc_xcursor).mx_cursor.mc_pg
                                        [0 as std::ffi::c_int as usize] = fp;
                                    flags |= 0x4 as std::ffi::c_int as std::ffi::c_uint;
                                    current_block = 15483849737692682886;
                                }
                            }
                        }
                        match current_block {
                            15483849737692682886 => {}
                            1849955825383499200 => {}
                            _ => {
                                fp_flags = std::ptr::read_unaligned(
                                    (fp as *mut u8)
                                        .offset(offset_of!(MDB_page2, mp2_flags) as isize)
                                        as *mut u16,
                                );
                                if (8 as std::ffi::c_ulong)
                                    .wrapping_add((*leaf).mn_ksize as std::ffi::c_ulong)
                                    .wrapping_add(xdata.mv_size)
                                    > (*env).me_nodemax as std::ffi::c_ulong
                                {
                                    fp_flags = (fp_flags as std::ffi::c_int
                                        & !(0x40 as std::ffi::c_int))
                                        as uint16_t;
                                    current_block = 13191470234520143498;
                                    continue;
                                } else {
                                    current_block = 10601179871800211547;
                                }
                            }
                        }
                    } else {
                        current_block = 1849955825383499200;
                    }
                    match current_block {
                        15483849737692682886 => {}
                        10601179871800211547 => {}
                        _ => {
                            if ((*leaf).mn_flags as std::ffi::c_uint ^ flags)
                                & 0x2 as std::ffi::c_int as std::ffi::c_uint
                                != 0
                            {
                                return -(30784 as std::ffi::c_int);
                            }
                            if (*leaf).mn_flags as std::ffi::c_int & 0x1 as std::ffi::c_int
                                == 0x1 as std::ffi::c_int
                            {
                                let mut omp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
                                let mut pg: pgno_t = 0;
                                let mut level: std::ffi::c_int = 0;
                                let mut ovpages: std::ffi::c_int = 0;
                                let mut dpages: std::ffi::c_int = (((16 as std::ffi::c_ulong
                                    as std::ffi::c_uint)
                                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                                    as size_t)
                                    .wrapping_add((*data).mv_size)
                                    / (*env).me_psize as size_t)
                                    .wrapping_add(1 as std::ffi::c_int as size_t)
                                    as std::ffi::c_int;
                                memcpy(
                                    &mut pg as *mut pgno_t as *mut std::ffi::c_void,
                                    olddata.mv_data,
                                    ::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong,
                                );
                                rc2 = mdb_page_get(mc, pg, &mut omp, &mut level);
                                if rc2 != 0 as std::ffi::c_int {
                                    return rc2;
                                }
                                ovpages = (*omp).mp_pb.pb_pages as std::ffi::c_int;
                                if ovpages >= dpages {
                                    if (*omp).mp_flags as std::ffi::c_int & 0x10 as std::ffi::c_int
                                        == 0
                                        && (level != 0
                                            || (*env).me_flags
                                                & 0x80000 as std::ffi::c_int as uint32_t
                                                != 0)
                                    {
                                        rc = mdb_page_unspill((*mc).mc_txn, omp, &mut omp);
                                        if rc != 0 {
                                            return rc;
                                        }
                                        level = 0 as std::ffi::c_int;
                                    }
                                    if (*omp).mp_flags as std::ffi::c_int & 0x10 as std::ffi::c_int
                                        != 0
                                    {
                                        if level > 1 as std::ffi::c_int {
                                            let mut sz: size_t =
                                                (*env).me_psize as size_t * ovpages as size_t;
                                            let mut off: size_t = 0;
                                            let mut np_0: *mut MDB_page = mdb_page_malloc(
                                                (*mc).mc_txn,
                                                ovpages as std::ffi::c_uint,
                                            );
                                            let mut id2: MDB_ID2 = MDB_ID2 {
                                                mid: 0,
                                                mptr: std::ptr::null_mut::<std::ffi::c_void>(),
                                            };
                                            if np_0.is_null() {
                                                return 12 as std::ffi::c_int;
                                            }
                                            id2.mid = pg;
                                            id2.mptr = np_0 as *mut std::ffi::c_void;
                                            rc2 = mdb_mid2l_insert(
                                                (*(*mc).mc_txn).mt_u.dirty_list,
                                                &mut id2,
                                            );
                                            if rc2 == 0 as std::ffi::c_int {
                                            } else {
                                                mdb_assert_fail(
                                                    (*(*mc).mc_txn).mt_env,
                                                    b"rc2 == 0\0" as *const u8
                                                        as *const std::ffi::c_char,
                                                    (*::core::mem::transmute::<
                                                        &[u8; 16],
                                                        &[std::ffi::c_char; 16],
                                                    >(
                                                        b"_mdb_cursor_put\0"
                                                    ))
                                                    .as_ptr(),
                                                    b"mdb.c\0" as *const u8
                                                        as *const std::ffi::c_char,
                                                    7991 as std::ffi::c_int,
                                                );
                                            };
                                            if flags
                                                & 0x10000 as std::ffi::c_int as std::ffi::c_uint
                                                == 0
                                            {
                                                off = (16 as std::ffi::c_ulong as std::ffi::c_uint
                                                    as size_t)
                                                    .wrapping_add((*data).mv_size)
                                                    & -(::core::mem::size_of::<size_t>()
                                                        as std::ffi::c_ulong
                                                        as std::ffi::c_int)
                                                        as size_t;
                                                memcpy(
                                                    (np_0 as *mut std::ffi::c_char)
                                                        .offset(off as isize)
                                                        as *mut size_t
                                                        as *mut std::ffi::c_void,
                                                    (omp as *mut std::ffi::c_char)
                                                        .offset(off as isize)
                                                        as *mut size_t
                                                        as *const std::ffi::c_void,
                                                    sz.wrapping_sub(off),
                                                );
                                                sz = 16 as std::ffi::c_ulong as std::ffi::c_uint
                                                    as size_t;
                                            }
                                            memcpy(
                                                np_0 as *mut std::ffi::c_void,
                                                omp as *const std::ffi::c_void,
                                                sz,
                                            );
                                            omp = np_0;
                                        }
                                        (*leaf).mn_lo = ((*data).mv_size
                                            & 0xffff as std::ffi::c_int as size_t)
                                            as std::ffi::c_ushort;
                                        (*leaf).mn_hi = ((*data).mv_size >> 16 as std::ffi::c_int)
                                            as std::ffi::c_ushort;
                                        if flags & 0x10000 as std::ffi::c_int as std::ffi::c_uint
                                            == 0x10000 as std::ffi::c_int as std::ffi::c_uint
                                        {
                                            (*data).mv_data = (omp as *mut std::ffi::c_char).offset(
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                                    as isize,
                                            )
                                                as *mut std::ffi::c_void;
                                        } else {
                                            memcpy(
                                                (omp as *mut std::ffi::c_char).offset(
                                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                                        as isize,
                                                )
                                                    as *mut std::ffi::c_void,
                                                (*data).mv_data,
                                                (*data).mv_size,
                                            );
                                        }
                                        return 0 as std::ffi::c_int;
                                    }
                                }
                                rc2 = mdb_ovpage_free(mc, omp);
                                if rc2 != 0 as std::ffi::c_int {
                                    return rc2;
                                }
                            } else if (*data).mv_size == olddata.mv_size {
                                if flags & 0x10000 as std::ffi::c_int as std::ffi::c_uint
                                    == 0x10000 as std::ffi::c_int as std::ffi::c_uint
                                {
                                    (*data).mv_data = olddata.mv_data;
                                    current_block = 10257223768985283691;
                                    break;
                                } else if (*mc).mc_flags
                                    & 0x4 as std::ffi::c_int as std::ffi::c_uint
                                    == 0
                                {
                                    memcpy(olddata.mv_data, (*data).mv_data, (*data).mv_size);
                                    current_block = 10257223768985283691;
                                    break;
                                } else if (*key).mv_size
                                    == std::ptr::read_unaligned(
                                        (leaf as *mut u8)
                                            .offset(offset_of!(MDB_node, mn_ksize) as isize)
                                            as *mut u16,
                                    ) as size_t
                                {
                                    memcpy(
                                        ((*leaf).mn_data).as_mut_ptr() as *mut std::ffi::c_void,
                                        (*key).mv_data,
                                        (*key).mv_size,
                                    );
                                    current_block = 5842863371754816627;
                                    continue;
                                }
                            }
                            mdb_node_del(mc, 0 as std::ffi::c_int);
                            current_block = 17353026871531185123;
                            continue;
                        }
                    }
                }
            }
            if current_block == 10601179871800211547 {
                if mp != fp {
                    (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags =
                        (fp_flags as std::ffi::c_int | 0x10 as std::ffi::c_int) as uint16_t;
                    std::ptr::write_unaligned(
                        (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_pad) as isize) as *mut u16,
                        std::ptr::read_unaligned(
                            (fp as *mut u8).offset(offset_of!(MDB_page2, mp2_pad) as isize)
                                as *mut u16,
                        ),
                    );
                    std::ptr::write_unaligned(
                        (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_lower) as isize)
                            as *mut indx_t,
                        std::ptr::read_unaligned(
                            (fp as *mut u8).offset(offset_of!(MDB_page2, mp2_lower) as isize)
                                as *mut indx_t,
                        ),
                    );
                    std::ptr::write_unaligned(
                        (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_upper) as isize)
                            as *mut indx_t,
                        (std::ptr::read_unaligned(
                            (fp as *mut u8).offset(offset_of!(MDB_page2, mp2_upper) as isize)
                                as *mut indx_t,
                        ) as std::ffi::c_uint)
                            .wrapping_add(offset) as indx_t,
                    );
                    if fp_flags as std::ffi::c_int & 0x20 as std::ffi::c_int != 0 {
                        memcpy(
                            (mp as *mut std::ffi::c_char)
                                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                                as *mut std::ffi::c_void,
                            (fp as *mut std::ffi::c_char)
                                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                                as *mut std::ffi::c_void,
                            (((*(fp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                                as std::ffi::c_uint)
                                .wrapping_sub(
                                    (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                        if 0 as std::ffi::c_int != 0 {
                                            16 as std::ffi::c_ulong as std::ffi::c_uint
                                        } else {
                                            0 as std::ffi::c_int as std::ffi::c_uint
                                        },
                                    ),
                                )
                                >> 1 as std::ffi::c_int)
                                .wrapping_mul((*fp).mp_pad as std::ffi::c_uint)
                                as std::ffi::c_ulong,
                        );
                    } else {
                        memcpy(
                            (mp as *mut std::ffi::c_char)
                                .offset(
                                    (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper
                                        as std::ffi::c_int
                                        as isize,
                                )
                                .offset(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as isize,
                                ) as *mut std::ffi::c_void,
                            (fp as *mut std::ffi::c_char)
                                .offset(std::ptr::read_unaligned(
                                    (fp as *mut u8)
                                        .offset(offset_of!(MDB_page2, mp2_upper) as isize)
                                        as *mut indx_t,
                                ) as std::ffi::c_int
                                    as isize)
                                .offset(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as isize,
                                ) as *const std::ffi::c_void,
                            (olddata.mv_size)
                                .wrapping_sub(std::ptr::read_unaligned(
                                    (fp as *mut u8)
                                        .offset(offset_of!(MDB_page2, mp2_upper) as isize)
                                        as *mut indx_t,
                                ) as size_t)
                                .wrapping_sub(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as size_t,
                                ),
                        );
                        memcpy(
                            (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize) as *mut std::ffi::c_void,
                            (fp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize) as *const std::ffi::c_void,
                            (((std::ptr::read_unaligned(
                                (fp as *mut u8).offset(offset_of!(MDB_page2, mp2_lower) as isize) as *mut indx_t
                            ) as std::ffi::c_uint)
                                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                    if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    },
                                ))
                                >> 1 as std::ffi::c_int) as std::ffi::c_ulong)
                                .wrapping_mul(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong),
                        );
                        i = 0 as std::ffi::c_int as std::ffi::c_uint;
                        while i
                            < (std::ptr::read_unaligned(
                                (fp as *mut u8).offset(offset_of!(MDB_page2, mp2_lower) as isize)
                                    as *mut indx_t,
                            ) as std::ffi::c_uint)
                                .wrapping_sub(
                                    (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                        if 0 as std::ffi::c_int != 0 {
                                            16 as std::ffi::c_ulong as std::ffi::c_uint
                                        } else {
                                            0 as std::ffi::c_int as std::ffi::c_uint
                                        },
                                    ),
                                )
                                >> 1 as std::ffi::c_int
                        {
                            let fresh40 = &mut (*((*mp).mp_ptrs).as_mut_ptr().offset(i as isize));
                            *fresh40 = (*fresh40 as std::ffi::c_uint).wrapping_add(offset) as indx_t
                                as indx_t;
                            i = i.wrapping_add(1);
                        }
                    }
                }
                rdata = &mut xdata;
                flags |= 0x4 as std::ffi::c_int as std::ffi::c_uint;
                do_sub = 1 as std::ffi::c_int;
                if insert_key == 0 {
                    mdb_node_del(mc, 0 as std::ffi::c_int);
                }
                current_block = 11041480184543863428;
            }
            if current_block == 11041480184543863428 {
                nflags = flags
                    & (0x4 as std::ffi::c_int
                        | 0x2 as std::ffi::c_int
                        | 0x10000 as std::ffi::c_int
                        | 0x20000 as std::ffi::c_int) as std::ffi::c_uint;
                nsize = if std::ptr::read_unaligned(
                    (*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2,
                )
                .mp2_flags as std::ffi::c_int
                    & 0x20 as std::ffi::c_int
                    == 0x20 as std::ffi::c_int
                {
                    (*key).mv_size
                } else {
                    mdb_leaf_size(env, key, rdata)
                };
                if ((std::ptr::read_unaligned::<MDB_page2>(
                    (*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2,
                )
                .mp2_upper as std::ffi::c_int
                    - std::ptr::read_unaligned(
                        (*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                            as *mut MDB_page2,
                    )
                    .mp2_lower as std::ffi::c_int) as indx_t as size_t)
                    < nsize
                {
                    if flags & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint
                        == 0x4 as std::ffi::c_int as std::ffi::c_uint
                    {
                        nflags &= !(0x20000 as std::ffi::c_int) as std::ffi::c_uint;
                    }
                    if insert_key == 0 {
                        nflags |= 0x40000 as std::ffi::c_int as std::ffi::c_uint;
                    }
                    rc = mdb_page_split(mc, key, rdata, !(0 as std::ffi::c_int as pgno_t), nflags);
                } else {
                    rc = mdb_node_add(
                        mc,
                        (*mc).mc_ki[(*mc).mc_top as usize],
                        key,
                        rdata,
                        0 as std::ffi::c_int as pgno_t,
                        nflags,
                    );
                    if rc == 0 as std::ffi::c_int {
                        let mut m2: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                        let mut m3: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                        let mut dbi: MDB_dbi = (*mc).mc_dbi;
                        let mut i_0: std::ffi::c_uint = (*mc).mc_top as std::ffi::c_uint;
                        let mut mp_0: *mut MDB_page = (*mc).mc_pg[i_0 as usize];
                        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
                        while !m2.is_null() {
                            if (*mc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                            } else {
                                m3 = m2;
                            }
                            if !(m3 == mc
                                || ((*m3).mc_snum as std::ffi::c_int)
                                    < (*mc).mc_snum as std::ffi::c_int
                                || (*m3).mc_pg[i_0 as usize] != mp_0)
                            {
                                if (*m3).mc_ki[i_0 as usize] as std::ffi::c_int
                                    >= (*mc).mc_ki[i_0 as usize] as std::ffi::c_int
                                    && insert_key != 0
                                {
                                    (*m3).mc_ki[i_0 as usize] =
                                        ((*m3).mc_ki[i_0 as usize]).wrapping_add(1);
                                    (*m3).mc_ki[i_0 as usize];
                                }
                                let mut xr_pg: *mut MDB_page = mp_0;
                                let mut xr_node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                                if !(!(!((*m3).mc_xcursor).is_null()
                                    && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                        & 0x1 as std::ffi::c_int as std::ffi::c_uint
                                        != 0)
                                    || (*m3).mc_ki[i_0 as usize] as std::ffi::c_uint
                                        >= ((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2))
                                            .mp2_lower
                                            as std::ffi::c_uint)
                                            .wrapping_sub(
                                                (16 as std::ffi::c_ulong as std::ffi::c_uint)
                                                    .wrapping_sub(if 0 as std::ffi::c_int != 0 {
                                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                                    } else {
                                                        0 as std::ffi::c_int as std::ffi::c_uint
                                                    }),
                                            )
                                            >> 1 as std::ffi::c_int)
                                {
                                    xr_node = (xr_pg as *mut std::ffi::c_char)
                                        .offset(
                                            *((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2))
                                                .mp2_ptrs)
                                                .as_mut_ptr()
                                                .offset((*m3).mc_ki[i_0 as usize] as isize)
                                                as std::ffi::c_int
                                                as isize,
                                        )
                                        .offset(
                                            (if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            }) as isize,
                                        )
                                        as *mut MDB_node;
                                    if (*xr_node).mn_flags as std::ffi::c_int
                                        & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                        == 0x4 as std::ffi::c_int
                                    {
                                        (*(*m3).mc_xcursor).mx_cursor.mc_pg
                                            [0 as std::ffi::c_int as usize] = ((*xr_node).mn_data)
                                            .as_mut_ptr()
                                            .offset((*xr_node).mn_ksize as std::ffi::c_int as isize)
                                            as *mut std::ffi::c_void
                                            as *mut MDB_page;
                                    }
                                }
                            }
                            m2 = (*m2).mc_next;
                        }
                    }
                }
                if rc != 0 as std::ffi::c_int {
                    current_block = 12092145755723610434;
                    break;
                }
                if do_sub != 0 {
                    xflags = 0;
                    new_dupdata = 0;
                    ecount = 0;
                    current_block = 15483849737692682886;
                } else {
                    current_block = 7301633501560609215;
                }
            }
            if current_block == 15483849737692682886 {
                xdata.mv_size = 0 as std::ffi::c_int as size_t;
                xdata.mv_data =
                    b"\0" as *const u8 as *const std::ffi::c_char as *mut std::ffi::c_void;
                leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_char)
                    .offset(
                        *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                            as *mut MDB_page2))
                            .mp2_ptrs)
                            .as_mut_ptr()
                            .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                            as std::ffi::c_int as isize,
                    )
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                if flags
                    & (0x40 as std::ffi::c_int | 0x40000 as std::ffi::c_int) as std::ffi::c_uint
                    == 0x40 as std::ffi::c_int as std::ffi::c_uint
                {
                    xflags = 0x40 as std::ffi::c_int | 0x8000 as std::ffi::c_int;
                } else {
                    mdb_xcursor_init1(mc, leaf);
                    xflags = if flags & 0x20 as std::ffi::c_int as std::ffi::c_uint != 0 {
                        0x10 as std::ffi::c_int | 0x8000 as std::ffi::c_int
                    } else {
                        0x8000 as std::ffi::c_int
                    };
                }
                if !sub_root.is_null() {
                    (*(*mc).mc_xcursor).mx_cursor.mc_pg[0 as std::ffi::c_int as usize] = sub_root;
                }
                new_dupdata = dkey.mv_size as std::ffi::c_int;
                if dkey.mv_size != 0 {
                    rc = _mdb_cursor_put(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        &mut dkey,
                        &mut xdata,
                        xflags as std::ffi::c_uint,
                    );
                    if rc != 0 {
                        current_block = 3910923109603533376;
                        break;
                    }
                    dkey.mv_size = 0 as std::ffi::c_int as size_t;
                }
                if (*leaf).mn_flags as std::ffi::c_int & 0x2 as std::ffi::c_int == 0
                    || !sub_root.is_null()
                {
                    let mut m2_0: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                    let mut mx: *mut MDB_xcursor = (*mc).mc_xcursor;
                    let mut i_1: std::ffi::c_uint = (*mc).mc_top as std::ffi::c_uint;
                    let mut mp_1: *mut MDB_page = (*mc).mc_pg[i_1 as usize];
                    m2_0 = *((*(*mc).mc_txn).mt_cursors).offset((*mc).mc_dbi as isize);
                    while !m2_0.is_null() {
                        if !(m2_0 == mc
                            || ((*m2_0).mc_snum as std::ffi::c_int)
                                < (*mc).mc_snum as std::ffi::c_int)
                            && ((*m2_0).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0)
                            && (*m2_0).mc_pg[i_1 as usize] == mp_1
                        {
                            if (*m2_0).mc_ki[i_1 as usize] as std::ffi::c_int
                                == (*mc).mc_ki[i_1 as usize] as std::ffi::c_int
                            {
                                mdb_xcursor_init2(m2_0, mx, new_dupdata);
                            } else if insert_key == 0 {
                                let mut xr_pg_0: *mut MDB_page = mp_1;
                                let mut xr_node_0: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                                if !(!(!((*m2_0).mc_xcursor).is_null()
                                    && (*(*m2_0).mc_xcursor).mx_cursor.mc_flags
                                        & 0x1 as std::ffi::c_int as std::ffi::c_uint
                                        != 0)
                                    || (*m2_0).mc_ki[i_1 as usize] as std::ffi::c_uint
                                        >= ((*(xr_pg_0 as *mut std::ffi::c_void as *mut MDB_page2))
                                            .mp2_lower
                                            as std::ffi::c_uint)
                                            .wrapping_sub(
                                                (16 as std::ffi::c_ulong as std::ffi::c_uint)
                                                    .wrapping_sub(if 0 as std::ffi::c_int != 0 {
                                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                                    } else {
                                                        0 as std::ffi::c_int as std::ffi::c_uint
                                                    }),
                                            )
                                            >> 1 as std::ffi::c_int)
                                {
                                    xr_node_0 = (xr_pg_0 as *mut std::ffi::c_char)
                                        .offset(
                                            *((*(xr_pg_0 as *mut std::ffi::c_void
                                                as *mut MDB_page2))
                                                .mp2_ptrs)
                                                .as_mut_ptr()
                                                .offset((*m2_0).mc_ki[i_1 as usize] as isize)
                                                as std::ffi::c_int
                                                as isize,
                                        )
                                        .offset(
                                            (if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            }) as isize,
                                        )
                                        as *mut MDB_node;
                                    if (*xr_node_0).mn_flags as std::ffi::c_int
                                        & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                        == 0x4 as std::ffi::c_int
                                    {
                                        (*(*m2_0).mc_xcursor).mx_cursor.mc_pg
                                            [0 as std::ffi::c_int as usize] =
                                            ((*xr_node_0).mn_data).as_mut_ptr().offset(
                                                (*xr_node_0).mn_ksize as std::ffi::c_int as isize,
                                            )
                                                as *mut std::ffi::c_void
                                                as *mut MDB_page;
                                    }
                                }
                            }
                        }
                        m2_0 = (*m2_0).mc_next;
                    }
                }
                ecount = (*(*mc).mc_xcursor).mx_db.md_entries;
                if flags & 0x40000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    xflags |= 0x20000 as std::ffi::c_int;
                }
                rc = _mdb_cursor_put(
                    &mut (*(*mc).mc_xcursor).mx_cursor,
                    data,
                    &mut xdata,
                    xflags as std::ffi::c_uint,
                );
                if flags & 0x2 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    let mut db: *mut std::ffi::c_void = ((*leaf).mn_data)
                        .as_mut_ptr()
                        .offset((*leaf).mn_ksize as std::ffi::c_int as isize)
                        as *mut std::ffi::c_void;
                    memcpy(
                        db,
                        &mut (*(*mc).mc_xcursor).mx_db as *mut MDB_db as *const std::ffi::c_void,
                        ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong,
                    );
                }
                insert_data =
                    ((*(*mc).mc_xcursor).mx_db.md_entries).wrapping_sub(ecount) as std::ffi::c_int;
            }
            if insert_data != 0 {
                (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_add(1);
                (*(*mc).mc_db).md_entries;
            }
            if insert_key != 0 {
                if rc != 0 {
                    current_block = 3910923109603533376;
                    break;
                }
                (*mc).mc_flags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
            }
            if flags & 0x80000 as std::ffi::c_int as std::ffi::c_uint == 0 {
                current_block = 5726862278832168375;
                break;
            }
            if rc != 0 {
                current_block = 5726862278832168375;
                break;
            }
            mcount = mcount.wrapping_add(1);
            (*data.offset(1 as std::ffi::c_int as isize)).mv_size = mcount as size_t;
            if mcount >= dcount {
                current_block = 5726862278832168375;
                break;
            }
            let fresh41 = &mut (*data.offset(0 as std::ffi::c_int as isize)).mv_data;
            *fresh41 = ((*data.offset(0 as std::ffi::c_int as isize)).mv_data
                as *mut std::ffi::c_char)
                .offset((*data.offset(0 as std::ffi::c_int as isize)).mv_size as isize)
                as *mut std::ffi::c_void;
            insert_data = 0 as std::ffi::c_int;
            insert_key = insert_data;
            current_block = 5486385170289821549;
        }
        match current_block {
            5726862278832168375 => return rc,
            3910923109603533376 => {
                if rc == -(30799 as std::ffi::c_int) {
                    rc = -(30779 as std::ffi::c_int);
                }
            }
            10257223768985283691 => return 0 as std::ffi::c_int,
            _ => {}
        }
        (*(*mc).mc_txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cursor_put(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = _mdb_cursor_put(mc, key, data, flags);
        rc
    }
}
unsafe extern "C" fn _mdb_cursor_del(
    mut mc: *mut MDB_cursor,
    mut flags: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut rc: std::ffi::c_int = 0;
        if (*(*mc).mc_txn).mt_flags
            & (0x20000 as std::ffi::c_int
                | (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int))
                as std::ffi::c_uint
            != 0
        {
            return if (*(*mc).mc_txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0
            {
                13 as std::ffi::c_int
            } else {
                -(30782 as std::ffi::c_int)
            };
        }
        if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0 {
            return 22 as std::ffi::c_int;
        }
        if (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint
            >= ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                .mp2_lower as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int
        {
            return -(30798 as std::ffi::c_int);
        }
        if flags & 0x8000 as std::ffi::c_int as std::ffi::c_uint == 0 && {
            rc = mdb_page_spill(
                mc,
                std::ptr::null_mut::<MDB_val>(),
                std::ptr::null_mut::<MDB_val>(),
            );
            rc != 0
        } {
            return rc;
        }
        rc = mdb_cursor_touch(mc);
        if rc != 0 {
            return rc;
        }
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            != 0x2 as std::ffi::c_int
        {
            return -(30796 as std::ffi::c_int);
        }
        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            != 0x20 as std::ffi::c_int
        {
            leaf = (mp as *mut std::ffi::c_char)
                .offset(
                    *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                        as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            if (*leaf).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int
                == 0x4 as std::ffi::c_int
            {
                if flags & 0x20 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(
                        ((*(*mc).mc_xcursor).mx_db.md_entries)
                            .wrapping_sub(1 as std::ffi::c_int as mdb_size_t),
                    );
                    (*(*mc).mc_xcursor).mx_cursor.mc_flags &=
                        !(0x1 as std::ffi::c_int) as std::ffi::c_uint;
                } else {
                    if (*leaf).mn_flags as std::ffi::c_int & 0x2 as std::ffi::c_int
                        != 0x2 as std::ffi::c_int
                    {
                        (*(*mc).mc_xcursor).mx_cursor.mc_pg[0 as std::ffi::c_int as usize] =
                            ((*leaf).mn_data)
                                .as_mut_ptr()
                                .offset((*leaf).mn_ksize as std::ffi::c_int as isize)
                                as *mut std::ffi::c_void
                                as *mut MDB_page;
                    }
                    rc = _mdb_cursor_del(
                        &mut (*(*mc).mc_xcursor).mx_cursor,
                        0x8000 as std::ffi::c_int as std::ffi::c_uint,
                    );
                    if rc != 0 {
                        return rc;
                    }
                    if (*(*mc).mc_xcursor).mx_db.md_entries != 0 {
                        if (*leaf).mn_flags as std::ffi::c_int & 0x2 as std::ffi::c_int != 0 {
                            let mut db: *mut std::ffi::c_void = ((*leaf).mn_data)
                                .as_mut_ptr()
                                .offset((*leaf).mn_ksize as std::ffi::c_int as isize)
                                as *mut std::ffi::c_void;
                            memcpy(
                                db,
                                &mut (*(*mc).mc_xcursor).mx_db as *mut MDB_db
                                    as *const std::ffi::c_void,
                                ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong,
                            );
                        } else {
                            let mut m2: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                            mdb_node_shrink(mp, (*mc).mc_ki[(*mc).mc_top as usize]);
                            leaf = (mp as *mut std::ffi::c_char)
                                .offset(
                                    *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                        .as_mut_ptr()
                                        .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                                        as std::ffi::c_int
                                        as isize,
                                )
                                .offset(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            (*(*mc).mc_xcursor).mx_cursor.mc_pg[0 as std::ffi::c_int as usize] =
                                ((*leaf).mn_data)
                                    .as_mut_ptr()
                                    .offset((*leaf).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void
                                    as *mut MDB_page;
                            m2 = *((*(*mc).mc_txn).mt_cursors).offset((*mc).mc_dbi as isize);
                            while !m2.is_null() {
                                if !(m2 == mc
                                    || ((*m2).mc_snum as std::ffi::c_int)
                                        < (*mc).mc_snum as std::ffi::c_int)
                                    && ((*m2).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint
                                        != 0)
                                    && (*m2).mc_pg[(*mc).mc_top as usize] == mp
                                {
                                    let mut xr_pg: *mut MDB_page = mp;
                                    let mut xr_node: *mut MDB_node =
                                        std::ptr::null_mut::<MDB_node>();
                                    if !(!(!((*m2).mc_xcursor).is_null()
                                        && (*(*m2).mc_xcursor).mx_cursor.mc_flags
                                            & 0x1 as std::ffi::c_int as std::ffi::c_uint
                                            != 0)
                                        || (*m2).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint
                                            >= ((*(xr_pg as *mut std::ffi::c_void
                                                as *mut MDB_page2))
                                                .mp2_lower
                                                as std::ffi::c_uint)
                                                .wrapping_sub(
                                                    (16 as std::ffi::c_ulong as std::ffi::c_uint)
                                                        .wrapping_sub(
                                                            if 0 as std::ffi::c_int != 0 {
                                                                16 as std::ffi::c_ulong
                                                                    as std::ffi::c_uint
                                                            } else {
                                                                0 as std::ffi::c_int
                                                                    as std::ffi::c_uint
                                                            },
                                                        ),
                                                )
                                                >> 1 as std::ffi::c_int)
                                    {
                                        xr_node = (xr_pg as *mut std::ffi::c_char)
                                            .offset(
                                                *((*(xr_pg as *mut std::ffi::c_void
                                                    as *mut MDB_page2))
                                                    .mp2_ptrs)
                                                    .as_mut_ptr()
                                                    .offset(
                                                        (*m2).mc_ki[(*mc).mc_top as usize] as isize,
                                                    )
                                                    as std::ffi::c_int
                                                    as isize,
                                            )
                                            .offset(
                                                (if 0 as std::ffi::c_int != 0 {
                                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                                } else {
                                                    0 as std::ffi::c_int as std::ffi::c_uint
                                                })
                                                    as isize,
                                            )
                                            as *mut MDB_node;
                                        if (*xr_node).mn_flags as std::ffi::c_int
                                            & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                            == 0x4 as std::ffi::c_int
                                        {
                                            (*(*m2).mc_xcursor).mx_cursor.mc_pg
                                                [0 as std::ffi::c_int as usize] =
                                                ((*xr_node).mn_data).as_mut_ptr().offset(
                                                    (*xr_node).mn_ksize as std::ffi::c_int as isize,
                                                )
                                                    as *mut std::ffi::c_void
                                                    as *mut MDB_page;
                                        }
                                    }
                                }
                                m2 = (*m2).mc_next;
                            }
                        }
                        (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(1);
                        (*(*mc).mc_db).md_entries;
                        return rc;
                    } else {
                        (*(*mc).mc_xcursor).mx_cursor.mc_flags &=
                            !(0x1 as std::ffi::c_int) as std::ffi::c_uint;
                    }
                }
                if (*leaf).mn_flags as std::ffi::c_int & 0x2 as std::ffi::c_int != 0 {
                    rc = mdb_drop0(&mut (*(*mc).mc_xcursor).mx_cursor, 0 as std::ffi::c_int);
                    if rc != 0 {
                        current_block = 16844394940769847043;
                    } else {
                        current_block = 3689906465960840878;
                    }
                } else {
                    current_block = 3689906465960840878;
                }
            } else if ((*leaf).mn_flags as std::ffi::c_uint ^ flags)
                & 0x2 as std::ffi::c_int as std::ffi::c_uint
                != 0
            {
                rc = -(30784 as std::ffi::c_int);
                current_block = 16844394940769847043;
            } else {
                current_block = 3689906465960840878;
            }
            if current_block == 3689906465960840878 {
                if (*leaf).mn_flags as std::ffi::c_int & 0x1 as std::ffi::c_int
                    == 0x1 as std::ffi::c_int
                {
                    let mut omp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
                    let mut pg: pgno_t = 0;
                    memcpy(
                        &mut pg as *mut pgno_t as *mut std::ffi::c_void,
                        ((*leaf).mn_data)
                            .as_mut_ptr()
                            .offset((*leaf).mn_ksize as std::ffi::c_int as isize)
                            as *mut std::ffi::c_void,
                        ::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong,
                    );
                    rc = mdb_page_get(mc, pg, &mut omp, std::ptr::null_mut::<std::ffi::c_int>());
                    if rc != 0 || {
                        rc = mdb_ovpage_free(mc, omp);
                        rc != 0
                    } {
                        current_block = 16844394940769847043;
                    } else {
                        current_block = 2385173818992883626;
                    }
                } else {
                    current_block = 2385173818992883626;
                }
            }
            match current_block {
                2385173818992883626 => {}
                _ => {
                    (*(*mc).mc_txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                    return rc;
                }
            }
        }
        mdb_cursor_del0(mc)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cursor_del(
    mut mc: *mut MDB_cursor,
    mut flags: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe { _mdb_cursor_del(mc, flags) }
}
unsafe extern "C" fn mdb_page_new(
    mut mc: *mut MDB_cursor,
    mut flags: uint32_t,
    mut num: std::ffi::c_int,
    mut mp: *mut *mut MDB_page,
) -> std::ffi::c_int {
    unsafe {
        let mut np: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut rc: std::ffi::c_int = 0;
        rc = mdb_page_alloc(mc, num, &mut np);
        if rc != 0 {
            return rc;
        }
        (*np).mp_flags = (flags | 0x10 as std::ffi::c_int as uint32_t) as uint16_t;
        (*np).mp_pb.pb.pb_lower = (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
            if 0 as std::ffi::c_int != 0 {
                16 as std::ffi::c_ulong as std::ffi::c_uint
            } else {
                0 as std::ffi::c_int as std::ffi::c_uint
            },
        ) as indx_t;
        (*np).mp_pb.pb.pb_upper =
            ((*(*(*mc).mc_txn).mt_env).me_psize).wrapping_sub(if 0 as std::ffi::c_int != 0 {
                16 as std::ffi::c_ulong as std::ffi::c_uint
            } else {
                0 as std::ffi::c_int as std::ffi::c_uint
            }) as indx_t;
        if (*(np as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x1 as std::ffi::c_int
            == 0x1 as std::ffi::c_int
        {
            (*(*mc).mc_db).md_branch_pages = ((*(*mc).mc_db).md_branch_pages).wrapping_add(1);
            (*(*mc).mc_db).md_branch_pages;
        } else if (*(np as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            == 0x2 as std::ffi::c_int
        {
            (*(*mc).mc_db).md_leaf_pages = ((*(*mc).mc_db).md_leaf_pages).wrapping_add(1);
            (*(*mc).mc_db).md_leaf_pages;
        } else if (*(np as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x4 as std::ffi::c_int
            == 0x4 as std::ffi::c_int
        {
            (*(*mc).mc_db).md_overflow_pages =
                ((*(*mc).mc_db).md_overflow_pages).wrapping_add(num as pgno_t);
            (*np).mp_pb.pb_pages = num as uint32_t;
        }
        *mp = np;
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_leaf_size(
    mut env: *mut MDB_env,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> size_t {
    unsafe {
        let mut sz: size_t = 0;
        sz = (8 as std::ffi::c_ulong).wrapping_add((*key).mv_size).wrapping_add((*data).mv_size);
        if sz > (*env).me_nodemax as size_t {
            sz = (sz as std::ffi::c_ulong).wrapping_sub(
                ((*data).mv_size)
                    .wrapping_sub(::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong),
            ) as size_t as size_t;
        }
        sz.wrapping_add(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
            .wrapping_add(1 as std::ffi::c_uint as std::ffi::c_ulong)
            & -(2 as std::ffi::c_int) as std::ffi::c_ulong
    }
}
unsafe extern "C" fn mdb_branch_size(mut _env: *mut MDB_env, mut key: *mut MDB_val) -> size_t {
    unsafe {
        let mut sz: size_t = 0;
        sz = (8 as std::ffi::c_ulong).wrapping_add(if key.is_null() {
            0 as std::ffi::c_int as size_t
        } else {
            (*key).mv_size
        });
        // if sz > (*env).me_nodemax as size_t {
        //     /* put on overflow page */
        //     /* not implemented */
        //     /* sz -= key->size - sizeof(pgno_t); */
        // }
        sz.wrapping_add(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
    }
}
unsafe extern "C" fn mdb_node_add(
    mut mc: *mut MDB_cursor,
    mut indx: indx_t,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut pgno: pgno_t,
    mut flags: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut i: std::ffi::c_uint = 0;
        let mut node_size: size_t = 8 as std::ffi::c_ulong;
        let mut room: ssize_t = 0;
        let mut ofs: indx_t = 0;
        let mut node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
        let mut ofp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut ndata: *mut std::ffi::c_void = std::ptr::null_mut::<std::ffi::c_void>();
        if std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2).mp2_upper
            as std::ffi::c_int
            >= std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2).mp2_lower
                as std::ffi::c_int
        {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"MP_UPPER(mp) >= MP_LOWER(mp)\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 13], &[std::ffi::c_char; 13]>(b"mdb_node_add\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                8408 as std::ffi::c_int,
            );
        };
        if std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2).mp2_flags
            as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            let mut ksize: std::ffi::c_int = (*(*mc).mc_db).md_pad as std::ffi::c_int;
            let mut dif: std::ffi::c_int = 0;
            let mut ptr: *mut std::ffi::c_char = (mp as *mut std::ffi::c_char)
                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                .offset((indx as std::ffi::c_int * ksize) as isize);
            dif = (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int)
                .wrapping_sub(indx as std::ffi::c_uint) as std::ffi::c_int;
            if dif > 0 as std::ffi::c_int {
                memmove(
                    ptr.offset(ksize as isize) as *mut std::ffi::c_void,
                    ptr as *const std::ffi::c_void,
                    (dif * ksize) as std::ffi::c_ulong,
                );
            }
            memcpy(ptr as *mut std::ffi::c_void, (*key).mv_data, ksize as std::ffi::c_ulong);
            let fresh42 = &mut (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower;
            *fresh42 = (*fresh42 as std::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
                as indx_t as indx_t;
            let fresh43 = &mut (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper;
            *fresh43 = (*fresh43 as std::ffi::c_ulong).wrapping_sub(
                (ksize as std::ffi::c_ulong)
                    .wrapping_sub(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong),
            ) as indx_t as indx_t;
            return 0 as std::ffi::c_int;
        }
        room = (std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2).mp2_upper
            as std::ffi::c_int
            - std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2).mp2_lower
                as std::ffi::c_int) as indx_t as ssize_t
            - ::core::mem::size_of::<indx_t>() as std::ffi::c_ulong as ssize_t;
        if !key.is_null() {
            node_size = node_size.wrapping_add((*key).mv_size);
        }
        if std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2).mp2_flags
            as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            == 0x2 as std::ffi::c_int
        {
            if !key.is_null() && !data.is_null() {
            } else {
                mdb_assert_fail(
                    (*(*mc).mc_txn).mt_env,
                    b"key && data\0" as *const u8 as *const std::ffi::c_char,
                    (*::core::mem::transmute::<&[u8; 13], &[std::ffi::c_char; 13]>(
                        b"mdb_node_add\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                    8436 as std::ffi::c_int,
                );
            };
            if flags & 0x1 as std::ffi::c_int as std::ffi::c_uint
                == 0x1 as std::ffi::c_int as std::ffi::c_uint
            {
                node_size = (node_size as std::ffi::c_ulong)
                    .wrapping_add(::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong)
                    as size_t as size_t;
                current_block = 11057878835866523405;
            } else if node_size.wrapping_add((*data).mv_size)
                > (*(*(*mc).mc_txn).mt_env).me_nodemax as size_t
            {
                let mut ovpages: std::ffi::c_int = (((16 as std::ffi::c_ulong as std::ffi::c_uint)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                    as size_t)
                    .wrapping_add((*data).mv_size)
                    / (*(*(*mc).mc_txn).mt_env).me_psize as size_t)
                    .wrapping_add(1 as std::ffi::c_int as size_t)
                    as std::ffi::c_int;
                let mut rc: std::ffi::c_int = 0;
                node_size = node_size
                    .wrapping_add(::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong)
                    .wrapping_add(1 as std::ffi::c_uint as std::ffi::c_ulong)
                    & -(2 as std::ffi::c_int) as std::ffi::c_ulong;
                if node_size as ssize_t > room {
                    current_block = 15594603006322722090;
                } else {
                    rc = mdb_page_new(mc, 0x4 as std::ffi::c_int as uint32_t, ovpages, &mut ofp);
                    if rc != 0 {
                        return rc;
                    }
                    flags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
                    current_block = 12048851432440741752;
                }
            } else {
                node_size = node_size.wrapping_add((*data).mv_size);
                current_block = 11057878835866523405;
            }
        } else {
            current_block = 11057878835866523405;
        }
        if current_block == 11057878835866523405 {
            node_size = node_size.wrapping_add(1 as std::ffi::c_uint as size_t)
                & -(2 as std::ffi::c_int) as size_t;
            if node_size as ssize_t > room {
                current_block = 15594603006322722090;
            } else {
                current_block = 12048851432440741752;
            }
        }
        match current_block {
            15594603006322722090 => {
                (*(*mc).mc_txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                -(30786 as std::ffi::c_int)
            }
            _ => {
                i = ((std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2))
                    .mp2_lower as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int;
                while i > indx as std::ffi::c_uint {
                    std::ptr::write_unaligned(
                        ((mp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize)
                            as *mut u16)
                            .offset(i as isize),
                        std::ptr::read_unaligned(
                            ((mp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize)
                                as *mut u16)
                                .offset(i.wrapping_sub(1 as std::ffi::c_uint) as isize),
                        ),
                    );
                    i = i.wrapping_sub(1);
                }
                ofs = ((std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2))
                    .mp2_upper as size_t)
                    .wrapping_sub(node_size) as indx_t;
                if ofs as std::ffi::c_ulong
                    >= ((std::ptr::read_unaligned(mp as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_lower as std::ffi::c_ulong)
                        .wrapping_add(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
                {
                } else {
                    mdb_assert_fail(
                        (*(*mc).mc_txn).mt_env,
                        b"ofs >= MP_LOWER(mp) + sizeof(indx_t)\0" as *const u8
                            as *const std::ffi::c_char,
                        (*::core::mem::transmute::<&[u8; 13], &[std::ffi::c_char; 13]>(
                            b"mdb_node_add\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                        8469 as std::ffi::c_int,
                    );
                };
                std::ptr::write_unaligned(
                    ((mp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize) as *mut u16)
                        .offset(indx as isize),
                    ofs,
                );
                std::ptr::write_unaligned(
                    (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_upper) as isize) as *mut u16,
                    ofs,
                );
                std::ptr::write_unaligned(
                    (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_lower) as isize) as *mut u16,
                    std::ptr::read_unaligned(mp as *mut MDB_page2)
                        .mp2_lower
                        .wrapping_add(::core::mem::size_of::<indx_t>() as u16),
                );
                node = (mp as *mut u8)
                    .offset(std::ptr::read_unaligned(
                        ((mp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize)
                            as *mut u16)
                            .offset(indx as isize),
                    ) as isize)
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                std::ptr::write_unaligned(
                    (node as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize) as *mut u16,
                    (if key.is_null() { 0 as std::ffi::c_int as size_t } else { (*key).mv_size })
                        as std::ffi::c_ushort,
                );

                std::ptr::write_unaligned(
                    (node as *mut u8).offset(offset_of!(MDB_node, mn_flags) as isize) as *mut u16,
                    flags as std::ffi::c_ushort,
                );

                if std::ptr::read_unaligned(
                    (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_flags) as isize) as *mut u16,
                ) as std::ffi::c_int
                    & 0x2 as std::ffi::c_int
                    == 0x2 as std::ffi::c_int
                {
                    std::ptr::write_unaligned(
                        (node as *mut u8).offset(offset_of!(MDB_node, mn_lo) as isize) as *mut u16,
                        ((*data).mv_size & 0xffff as std::ffi::c_int as size_t)
                            as std::ffi::c_ushort,
                    );
                    std::ptr::write_unaligned(
                        (node as *mut u8).offset(offset_of!(MDB_node, mn_hi) as isize) as *mut u16,
                        ((*data).mv_size >> 16 as std::ffi::c_int) as std::ffi::c_ushort,
                    );
                } else {
                    std::ptr::write_unaligned(
                        (node as *mut u8).offset(offset_of!(MDB_node, mn_lo) as isize) as *mut u16,
                        (pgno & 0xffff as std::ffi::c_int as pgno_t) as std::ffi::c_ushort,
                    );
                    std::ptr::write_unaligned(
                        (node as *mut u8).offset(offset_of!(MDB_node, mn_hi) as isize) as *mut u16,
                        (pgno >> 16 as std::ffi::c_int) as std::ffi::c_ushort,
                    );
                    if if -(1 as std::ffi::c_int) as pgno_t
                        > 0xffffffff as std::ffi::c_uint as pgno_t
                    {
                        32 as std::ffi::c_int
                    } else {
                        0 as std::ffi::c_int
                    } != 0
                    {
                        (*node).mn_flags = (pgno
                            >> (if -(1 as std::ffi::c_int) as pgno_t
                                > 0xffffffff as std::ffi::c_uint as pgno_t
                            {
                                32 as std::ffi::c_int
                            } else {
                                0 as std::ffi::c_int
                            })) as std::ffi::c_ushort;
                    }
                }
                if !key.is_null() {
                    memcpy(
                        ((*node).mn_data).as_mut_ptr() as *mut std::ffi::c_void,
                        (*key).mv_data,
                        (*key).mv_size,
                    );
                }
                if std::ptr::read_unaligned(
                    (mp as *mut u8).offset(offset_of!(MDB_page2, mp2_flags) as isize) as *mut u16,
                ) as std::ffi::c_int
                    & 0x2 as std::ffi::c_int
                    == 0x2 as std::ffi::c_int
                {
                    ndata = ((*node).mn_data).as_mut_ptr().offset(std::ptr::read_unaligned(
                        (node as *mut u8).offset(offset_of!(MDB_node, mn_ksize) as isize)
                            as *mut u16,
                    ) as isize) as *mut std::ffi::c_void;
                    if ofp.is_null() {
                        if flags & 0x1 as std::ffi::c_int as std::ffi::c_uint
                            == 0x1 as std::ffi::c_int as std::ffi::c_uint
                        {
                            memcpy(
                                ndata,
                                (*data).mv_data,
                                ::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong,
                            );
                        } else if flags & 0x10000 as std::ffi::c_int as std::ffi::c_uint
                            == 0x10000 as std::ffi::c_int as std::ffi::c_uint
                        {
                            (*data).mv_data = ndata;
                        } else {
                            memcpy(ndata, (*data).mv_data, (*data).mv_size);
                        }
                    } else {
                        memcpy(
                            ndata,
                            &mut (*ofp).mp_p.p_pgno as *mut pgno_t as *const std::ffi::c_void,
                            ::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong,
                        );
                        ndata = (ofp as *mut std::ffi::c_char)
                            .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                            as *mut std::ffi::c_void;
                        if flags & 0x10000 as std::ffi::c_int as std::ffi::c_uint
                            == 0x10000 as std::ffi::c_int as std::ffi::c_uint
                        {
                            (*data).mv_data = ndata;
                        } else {
                            memcpy(ndata, (*data).mv_data, (*data).mv_size);
                        }
                    }
                }
                0 as std::ffi::c_int
            }
        }
    }
}
unsafe extern "C" fn mdb_node_del(mut mc: *mut MDB_cursor, mut ksize: std::ffi::c_int) {
    unsafe {
        let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
        let mut indx: indx_t = (*mc).mc_ki[(*mc).mc_top as usize];
        let mut sz: std::ffi::c_uint = 0;
        let mut i: indx_t = 0;
        let mut j: indx_t = 0;
        let mut numkeys: indx_t = 0;
        let mut ptr: indx_t = 0;
        let mut node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut base: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        numkeys = (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
            as std::ffi::c_uint)
            .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                },
            ))
            >> 1 as std::ffi::c_int) as indx_t;
        if (indx as std::ffi::c_int) < numkeys as std::ffi::c_int {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"indx < numkeys\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 13], &[std::ffi::c_char; 13]>(b"mdb_node_del\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                8534 as std::ffi::c_int,
            );
        };
        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            let mut x: std::ffi::c_int =
                numkeys as std::ffi::c_int - 1 as std::ffi::c_int - indx as std::ffi::c_int;
            base = (mp as *mut std::ffi::c_char)
                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                .offset((indx as std::ffi::c_int * ksize) as isize);
            if x != 0 {
                memmove(
                    base as *mut std::ffi::c_void,
                    base.offset(ksize as isize) as *const std::ffi::c_void,
                    (x * ksize) as std::ffi::c_ulong,
                );
            }
            let fresh45 = &mut (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower;
            *fresh45 = (*fresh45 as std::ffi::c_ulong)
                .wrapping_sub(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
                as indx_t as indx_t;
            let fresh46 = &mut (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper;
            *fresh46 = (*fresh46 as std::ffi::c_ulong).wrapping_add(
                (ksize as std::ffi::c_ulong)
                    .wrapping_sub(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong),
            ) as indx_t as indx_t;
            return;
        }
        node = (mp as *mut std::ffi::c_char)
            .offset(
                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(indx as isize) as std::ffi::c_int as isize,
            )
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        sz = (8 as std::ffi::c_ulong).wrapping_add((*node).mn_ksize as std::ffi::c_ulong)
            as std::ffi::c_uint;
        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            == 0x2 as std::ffi::c_int
        {
            if (*node).mn_flags as std::ffi::c_int & 0x1 as std::ffi::c_int
                == 0x1 as std::ffi::c_int
            {
                sz = (sz as std::ffi::c_ulong)
                    .wrapping_add(::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong)
                    as std::ffi::c_uint as std::ffi::c_uint;
            } else {
                sz = sz.wrapping_add(
                    (*node).mn_lo as std::ffi::c_uint
                        | ((*node).mn_hi as std::ffi::c_uint) << 16 as std::ffi::c_int,
                );
            }
        }
        sz = sz.wrapping_add(1 as std::ffi::c_uint) & -(2 as std::ffi::c_int) as std::ffi::c_uint;
        ptr = *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
            .as_mut_ptr()
            .offset(indx as isize);
        j = 0 as std::ffi::c_int as indx_t;
        i = j;
        while (i as std::ffi::c_int) < numkeys as std::ffi::c_int {
            if i as std::ffi::c_int != indx as std::ffi::c_int {
                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(j as isize) = *((*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                    .mp2_ptrs)
                    .as_mut_ptr()
                    .offset(i as isize);
                if (*((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(i as isize) as std::ffi::c_int)
                    < ptr as std::ffi::c_int
                {
                    let fresh47 = &mut (*((*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_ptrs)
                        .as_mut_ptr()
                        .offset(j as isize));
                    *fresh47 = (*fresh47 as std::ffi::c_uint).wrapping_add(sz) as indx_t as indx_t;
                }
                j = j.wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
        base = (mp as *mut std::ffi::c_char)
            .offset(
                (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper as std::ffi::c_int
                    as isize,
            )
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            );
        memmove(
            base.offset(sz as isize) as *mut std::ffi::c_void,
            base as *const std::ffi::c_void,
            (ptr as std::ffi::c_int
                - (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper as std::ffi::c_int)
                as std::ffi::c_ulong,
        );
        let fresh48 = &mut (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower;
        *fresh48 = (*fresh48 as std::ffi::c_ulong)
            .wrapping_sub(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
            as indx_t as indx_t;
        let fresh49 = &mut (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper;
        *fresh49 = (*fresh49 as std::ffi::c_uint).wrapping_add(sz) as indx_t as indx_t;
    }
}
unsafe extern "C" fn mdb_node_shrink(mut mp: *mut MDB_page, mut indx: indx_t) {
    unsafe {
        let mut node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut sp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut xp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut base: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut delta: indx_t = 0;
        let mut nsize: indx_t = 0;
        let mut len: indx_t = 0;
        let mut ptr: indx_t = 0;
        let mut i: std::ffi::c_int = 0;
        node = (mp as *mut std::ffi::c_char)
            .offset(
                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(indx as isize) as std::ffi::c_int as isize,
            )
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        sp = ((*node).mn_data).as_mut_ptr().offset((*node).mn_ksize as std::ffi::c_int as isize)
            as *mut std::ffi::c_void as *mut MDB_page;
        delta = ((*(sp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper as std::ffi::c_int
            - (*(sp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_int)
            as indx_t;
        nsize = ((*node).mn_lo as std::ffi::c_uint
            | ((*node).mn_hi as std::ffi::c_uint) << 16 as std::ffi::c_int)
            .wrapping_sub(delta as std::ffi::c_uint) as indx_t;
        if (*(sp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            len = nsize;
            if nsize as std::ffi::c_int & 1 as std::ffi::c_int != 0 {
                return;
            }
        } else {
            xp = (sp as *mut std::ffi::c_char).offset(delta as std::ffi::c_int as isize)
                as *mut MDB_page;
            i = (((*(sp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int) as std::ffi::c_int;
            loop {
                i -= 1;
                if i < 0 as std::ffi::c_int {
                    break;
                }
                *((*(xp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(i as isize) =
                    (*((*(sp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(i as isize) as std::ffi::c_int
                        - delta as std::ffi::c_int) as indx_t;
            }
            len = 16 as std::ffi::c_ulong as std::ffi::c_uint as indx_t;
        }
        (*(sp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper =
            (*(sp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower;
        let mut s: *mut std::ffi::c_ushort = std::ptr::null_mut::<std::ffi::c_ushort>();
        let mut d: *mut std::ffi::c_ushort = std::ptr::null_mut::<std::ffi::c_ushort>();
        s = &mut (*mp).mp_p.p_pgno as *mut pgno_t as *mut std::ffi::c_ushort;
        d = &mut (*(sp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_p as *mut [uint16_t; 4]
            as *mut std::ffi::c_ushort;
        let fresh50 = s;
        s = s.offset(1);
        let fresh51 = d;
        d = d.offset(1);
        *fresh51 = *fresh50;
        let fresh52 = s;
        s = s.offset(1);
        let fresh53 = d;
        d = d.offset(1);
        *fresh53 = *fresh52;
        let fresh54 = s;
        s = s.offset(1);
        let fresh55 = d;
        d = d.offset(1);
        *fresh55 = *fresh54;
        *d = *s;
        (*node).mn_lo =
            (nsize as std::ffi::c_int & 0xffff as std::ffi::c_int) as std::ffi::c_ushort;
        (*node).mn_hi = (nsize as std::ffi::c_int >> 16 as std::ffi::c_int) as std::ffi::c_ushort;
        base = (mp as *mut std::ffi::c_char)
            .offset((*mp).mp_pb.pb.pb_upper as std::ffi::c_int as isize)
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            );
        memmove(
            base.offset(delta as std::ffi::c_int as isize) as *mut std::ffi::c_void,
            base as *const std::ffi::c_void,
            (sp as *mut std::ffi::c_char).offset(len as std::ffi::c_int as isize).offset_from(base)
                as std::ffi::c_long as std::ffi::c_ulong,
        );
        ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
        i = (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_uint)
            .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                },
            ))
            >> 1 as std::ffi::c_int) as std::ffi::c_int;
        loop {
            i -= 1;
            if i < 0 as std::ffi::c_int {
                break;
            }
            if *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as std::ffi::c_int
                <= ptr as std::ffi::c_int
            {
                let fresh56 = &mut (*((*mp).mp_ptrs).as_mut_ptr().offset(i as isize));
                *fresh56 = (*fresh56 as std::ffi::c_int + delta as std::ffi::c_int) as indx_t;
            }
        }
        (*mp).mp_pb.pb.pb_upper =
            ((*mp).mp_pb.pb.pb_upper as std::ffi::c_int + delta as std::ffi::c_int) as indx_t;
    }
}
unsafe extern "C" fn mdb_xcursor_init0(mut mc: *mut MDB_cursor) {
    unsafe {
        let mut mx: *mut MDB_xcursor = (*mc).mc_xcursor;
        (*mx).mx_cursor.mc_xcursor = std::ptr::null_mut::<MDB_xcursor>();
        (*mx).mx_cursor.mc_txn = (*mc).mc_txn;
        (*mx).mx_cursor.mc_db = &mut (*mx).mx_db;
        (*mx).mx_cursor.mc_dbx = &mut (*mx).mx_dbx;
        (*mx).mx_cursor.mc_dbi = (*mc).mc_dbi;
        (*mx).mx_cursor.mc_dbflag = &mut (*mx).mx_dbflag;
        (*mx).mx_cursor.mc_snum = 0 as std::ffi::c_int as std::ffi::c_ushort;
        (*mx).mx_cursor.mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
        (*mx).mx_cursor.mc_flags = 0x4 as std::ffi::c_int as std::ffi::c_uint
            | (*mc).mc_flags
                & (0x20000 as std::ffi::c_int | 0x80000 as std::ffi::c_int) as std::ffi::c_uint;
        (*mx).mx_dbx.md_name.mv_size = 0 as std::ffi::c_int as size_t;
        (*mx).mx_dbx.md_name.mv_data = std::ptr::null_mut::<std::ffi::c_void>();
        (*mx).mx_dbx.md_cmp = (*(*mc).mc_dbx).md_dcmp;
        (*mx).mx_dbx.md_dcmp = None;
        (*mx).mx_dbx.md_rel = (*(*mc).mc_dbx).md_rel;
    }
}
unsafe extern "C" fn mdb_xcursor_init1(mut mc: *mut MDB_cursor, mut node: *mut MDB_node) {
    #[allow(unpredictable_function_pointer_comparisons)]
    unsafe {
        let mut mx: *mut MDB_xcursor = (*mc).mc_xcursor;
        (*mx).mx_cursor.mc_flags &= (0x4 as std::ffi::c_int
            | 0x20000 as std::ffi::c_int
            | 0x80000 as std::ffi::c_int) as std::ffi::c_uint;
        if (*node).mn_flags as std::ffi::c_int & 0x2 as std::ffi::c_int != 0 {
            memcpy(
                &mut (*mx).mx_db as *mut MDB_db as *mut std::ffi::c_void,
                ((*node).mn_data).as_mut_ptr().offset((*node).mn_ksize as std::ffi::c_int as isize)
                    as *mut std::ffi::c_void,
                ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong,
            );
            (*mx).mx_cursor.mc_pg[0 as std::ffi::c_int as usize] = std::ptr::null_mut::<MDB_page>();
            (*mx).mx_cursor.mc_snum = 0 as std::ffi::c_int as std::ffi::c_ushort;
            (*mx).mx_cursor.mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
        } else {
            let mut fp: *mut MDB_page =
                ((*node).mn_data).as_mut_ptr().offset((*node).mn_ksize as std::ffi::c_int as isize)
                    as *mut std::ffi::c_void as *mut MDB_page;
            (*mx).mx_db.md_pad = 0 as std::ffi::c_int as uint32_t;
            (*mx).mx_db.md_flags = 0 as std::ffi::c_int as uint16_t;
            (*mx).mx_db.md_depth = 1 as std::ffi::c_int as uint16_t;
            (*mx).mx_db.md_branch_pages = 0 as std::ffi::c_int as pgno_t;
            (*mx).mx_db.md_leaf_pages = 1 as std::ffi::c_int as pgno_t;
            (*mx).mx_db.md_overflow_pages = 0 as std::ffi::c_int as pgno_t;
            (*mx).mx_db.md_entries = ((std::ptr::read_unaligned::<MDB_page2>(
                fp as *mut std::ffi::c_void as *mut MDB_page2,
            )
            .mp2_lower as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int) as mdb_size_t;

            let mut s: *mut std::ffi::c_ushort = std::ptr::null_mut::<std::ffi::c_ushort>();
            let mut d: *mut std::ffi::c_ushort = std::ptr::null_mut::<std::ffi::c_ushort>();
            s = &mut std::ptr::read_unaligned::<MDB_page2>(
                fp as *mut std::ffi::c_void as *mut MDB_page2,
            )
            .mp2_p as *mut [uint16_t; 4] as *mut std::ffi::c_ushort;
            d = &mut (*mx).mx_db.md_root as *mut pgno_t as *mut std::ffi::c_ushort;
            let fresh57 = s;
            s = s.offset(1);
            let fresh58 = d;
            d = d.offset(1);
            *fresh58 = *fresh57;
            let fresh59 = s;
            s = s.offset(1);
            let fresh60 = d;
            d = d.offset(1);
            *fresh60 = *fresh59;
            let fresh61 = s;
            s = s.offset(1);
            let fresh62 = d;
            d = d.offset(1);
            *fresh62 = *fresh61;
            *d = *s;
            (*mx).mx_cursor.mc_snum = 1 as std::ffi::c_int as std::ffi::c_ushort;
            (*mx).mx_cursor.mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
            (*mx).mx_cursor.mc_flags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
            (*mx).mx_cursor.mc_pg[0 as std::ffi::c_int as usize] = fp;
            (*mx).mx_cursor.mc_ki[0 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as indx_t;
            if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x10 as std::ffi::c_int != 0 {
                (*mx).mx_db.md_flags = 0x10 as std::ffi::c_int as uint16_t;
                (*mx).mx_db.md_pad = (*fp).mp_pad as uint32_t;
                if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x20 as std::ffi::c_int != 0 {
                    (*mx).mx_db.md_flags = ((*mx).mx_db.md_flags as std::ffi::c_int
                        | 0x8 as std::ffi::c_int)
                        as uint16_t;
                }
            }
        }
        (*mx).mx_dbflag = (0x8 as std::ffi::c_int
            | 0x10 as std::ffi::c_int
            | 0x20 as std::ffi::c_int) as std::ffi::c_uchar;
        if (0xffffffff as std::ffi::c_uint as std::ffi::c_ulong)
            < 18446744073709551615 as std::ffi::c_ulong
            && (*mx).mx_dbx.md_cmp == Some(mdb_cmp_int as MDB_cmp_func)
            && (*mx).mx_db.md_pad as std::ffi::c_ulong
                == ::core::mem::size_of::<mdb_size_t>() as std::ffi::c_ulong
        {
            (*mx).mx_dbx.md_cmp = Some(mdb_cmp_cint as MDB_cmp_func);
        }
    }
}
unsafe extern "C" fn mdb_xcursor_init2(
    mut mc: *mut MDB_cursor,
    mut src_mx: *mut MDB_xcursor,
    mut new_dupdata: std::ffi::c_int,
) {
    unsafe {
        let mut mx: *mut MDB_xcursor = (*mc).mc_xcursor;
        if new_dupdata != 0 {
            (*mx).mx_cursor.mc_snum = 1 as std::ffi::c_int as std::ffi::c_ushort;
            (*mx).mx_cursor.mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
            (*mx).mx_cursor.mc_flags |= 0x1 as std::ffi::c_int as std::ffi::c_uint;
            (*mx).mx_cursor.mc_ki[0 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as indx_t;
            (*mx).mx_dbflag = (0x8 as std::ffi::c_int
                | 0x10 as std::ffi::c_int
                | 0x20 as std::ffi::c_int) as std::ffi::c_uchar;
            (*mx).mx_dbx.md_cmp = (*src_mx).mx_dbx.md_cmp;
        } else if (*mx).mx_cursor.mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0 {
            return;
        }
        (*mx).mx_db = (*src_mx).mx_db;
        (*mx).mx_cursor.mc_pg[0 as std::ffi::c_int as usize] =
            (*src_mx).mx_cursor.mc_pg[0 as std::ffi::c_int as usize];
    }
}
unsafe extern "C" fn mdb_cursor_init(
    mut mc: *mut MDB_cursor,
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut mx: *mut MDB_xcursor,
) {
    unsafe {
        (*mc).mc_next = std::ptr::null_mut::<MDB_cursor>();
        (*mc).mc_backup = std::ptr::null_mut::<MDB_cursor>();
        (*mc).mc_dbi = dbi;
        (*mc).mc_txn = txn;
        (*mc).mc_db = &mut *((*txn).mt_dbs).offset(dbi as isize) as *mut MDB_db;
        (*mc).mc_dbx = &mut *((*txn).mt_dbxs).offset(dbi as isize) as *mut MDB_dbx;
        (*mc).mc_dbflag = &mut *((*txn).mt_dbflags).offset(dbi as isize) as *mut std::ffi::c_uchar;
        (*mc).mc_snum = 0 as std::ffi::c_int as std::ffi::c_ushort;
        (*mc).mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
        (*mc).mc_pg[0 as std::ffi::c_int as usize] = std::ptr::null_mut::<MDB_page>();
        (*mc).mc_ki[0 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as indx_t;
        (*mc).mc_flags = (*txn).mt_flags
            & (0x20000 as std::ffi::c_int | 0x80000 as std::ffi::c_int) as std::ffi::c_uint;
        if (*((*txn).mt_dbs).offset(dbi as isize)).md_flags as std::ffi::c_int
            & 0x4 as std::ffi::c_int
            != 0
        {
            if !mx.is_null() {
            } else {
                mdb_assert_fail(
                    (*txn).mt_env,
                    b"mx != NULL\0" as *const u8 as *const std::ffi::c_char,
                    (*::core::mem::transmute::<&[u8; 16], &[std::ffi::c_char; 16]>(
                        b"mdb_cursor_init\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                    8745 as std::ffi::c_int,
                );
            };
            (*mc).mc_xcursor = mx;
            mdb_xcursor_init0(mc);
        } else {
            (*mc).mc_xcursor = std::ptr::null_mut::<MDB_xcursor>();
        }
        if *(*mc).mc_dbflag as std::ffi::c_int & 0x2 as std::ffi::c_int != 0 {
            mdb_page_search(mc, std::ptr::null_mut::<MDB_val>(), 2 as std::ffi::c_int);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cursor_open(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut ret: *mut *mut MDB_cursor,
) -> std::ffi::c_int {
    unsafe {
        let mut mc: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut size: size_t = ::core::mem::size_of::<MDB_cursor>() as std::ffi::c_ulong;
        if ret.is_null()
            || !(!txn.is_null()
                && dbi < (*txn).mt_numdbs
                && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                    & 0x8 as std::ffi::c_int
                    != 0)
        {
            return 22 as std::ffi::c_int;
        }
        if (*txn).mt_flags
            & (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int)
                as std::ffi::c_uint
            != 0
        {
            return -(30782 as std::ffi::c_int);
        }
        if dbi == 0 as std::ffi::c_int as MDB_dbi
            && ((*txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint
                != 0x20000 as std::ffi::c_int as std::ffi::c_uint)
        {
            return 22 as std::ffi::c_int;
        }
        if (*((*txn).mt_dbs).offset(dbi as isize)).md_flags as std::ffi::c_int
            & 0x4 as std::ffi::c_int
            != 0
        {
            size = (size as std::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<MDB_xcursor>() as std::ffi::c_ulong)
                as size_t as size_t;
        }
        mc = malloc(size) as *mut MDB_cursor;
        if !mc.is_null() {
            mdb_cursor_init(
                mc,
                txn,
                dbi,
                mc.offset(1 as std::ffi::c_int as isize) as *mut MDB_xcursor,
            );
            if !((*txn).mt_cursors).is_null() {
                (*mc).mc_next = *((*txn).mt_cursors).offset(dbi as isize);
                let fresh63 = &mut (*((*txn).mt_cursors).offset(dbi as isize));
                *fresh63 = mc;
                (*mc).mc_flags |= 0x40 as std::ffi::c_int as std::ffi::c_uint;
            }
        } else {
            return 12 as std::ffi::c_int;
        }
        *ret = mc;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cursor_renew(
    mut txn: *mut MDB_txn,
    mut mc: *mut MDB_cursor,
) -> std::ffi::c_int {
    unsafe {
        if mc.is_null()
            || !(!txn.is_null()
                && (*mc).mc_dbi < (*txn).mt_numdbs
                && *((*txn).mt_dbflags).offset((*mc).mc_dbi as isize) as std::ffi::c_int
                    & 0x8 as std::ffi::c_int
                    != 0)
        {
            return 22 as std::ffi::c_int;
        }
        if (*mc).mc_flags & 0x40 as std::ffi::c_int as std::ffi::c_uint != 0
            || !((*txn).mt_cursors).is_null()
        {
            return 22 as std::ffi::c_int;
        }
        if (*txn).mt_flags
            & (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int)
                as std::ffi::c_uint
            != 0
        {
            return -(30782 as std::ffi::c_int);
        }
        mdb_cursor_init(mc, txn, (*mc).mc_dbi, (*mc).mc_xcursor);
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cursor_count(
    mut mc: *mut MDB_cursor,
    mut countp: *mut mdb_size_t,
) -> std::ffi::c_int {
    unsafe {
        let mut leaf: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        if mc.is_null() || countp.is_null() {
            return 22 as std::ffi::c_int;
        }
        if ((*mc).mc_xcursor).is_null() {
            return -(30784 as std::ffi::c_int);
        }
        if (*(*mc).mc_txn).mt_flags
            & (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int)
                as std::ffi::c_uint
            != 0
        {
            return -(30782 as std::ffi::c_int);
        }
        if (*mc).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0 {
            return 22 as std::ffi::c_int;
        }
        if (*mc).mc_snum == 0 {
            return -(30798 as std::ffi::c_int);
        }
        if (*mc).mc_flags & 0x2 as std::ffi::c_int as std::ffi::c_uint != 0 {
            if (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint
                >= ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_lower as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int
            {
                return -(30798 as std::ffi::c_int);
            }
            (*mc).mc_flags ^= 0x2 as std::ffi::c_int as std::ffi::c_uint;
        }
        leaf = ((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_char)
            .offset(
                *((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_ptrs)
                    .as_mut_ptr()
                    .offset((*mc).mc_ki[(*mc).mc_top as usize] as isize)
                    as std::ffi::c_int as isize,
            )
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        if (*leaf).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int != 0x4 as std::ffi::c_int {
            *countp = 1 as std::ffi::c_int as mdb_size_t;
        } else {
            if (*(*mc).mc_xcursor).mx_cursor.mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint
                == 0
            {
                return 22 as std::ffi::c_int;
            }
            *countp = (*(*mc).mc_xcursor).mx_db.md_entries;
        }
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cursor_close(mut mc: *mut MDB_cursor) {
    unsafe {
        mc.is_null();
        if !mc.is_null() && ((*mc).mc_backup).is_null() {
            if (*mc).mc_flags & 0x40 as std::ffi::c_int as std::ffi::c_uint != 0
                && !((*(*mc).mc_txn).mt_cursors).is_null()
            {
                let mut prev: *mut *mut MDB_cursor = &mut *((*(*mc).mc_txn).mt_cursors)
                    .offset((*mc).mc_dbi as isize)
                    as *mut *mut MDB_cursor;
                while !(*prev).is_null() && *prev != mc {
                    prev = &mut (**prev).mc_next;
                }
                if *prev == mc {
                    *prev = (*mc).mc_next;
                }
            }
            free(mc as *mut std::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cursor_txn(mut mc: *mut MDB_cursor) -> *mut MDB_txn {
    unsafe {
        if mc.is_null() {
            return std::ptr::null_mut::<MDB_txn>();
        }
        (*mc).mc_txn
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_cursor_dbi(mut mc: *mut MDB_cursor) -> MDB_dbi {
    unsafe { (*mc).mc_dbi }
}
unsafe extern "C" fn mdb_update_key(
    mut mc: *mut MDB_cursor,
    mut key: *mut MDB_val,
) -> std::ffi::c_int {
    unsafe {
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut base: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut len: size_t = 0;
        let mut delta: std::ffi::c_int = 0;
        let mut ksize: std::ffi::c_int = 0;
        let mut oksize: std::ffi::c_int = 0;
        let mut ptr: indx_t = 0;
        let mut i: indx_t = 0;
        let mut numkeys: indx_t = 0;
        let mut indx: indx_t = 0;
        indx = (*mc).mc_ki[(*mc).mc_top as usize];
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        node = (mp as *mut std::ffi::c_char)
            .offset(
                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                    .as_mut_ptr()
                    .offset(indx as isize) as std::ffi::c_int as isize,
            )
            .offset(
                (if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                }) as isize,
            ) as *mut MDB_node;
        ptr = *((*mp).mp_ptrs).as_mut_ptr().offset(indx as isize);
        ksize = (((*key).mv_size).wrapping_add(1 as std::ffi::c_uint as size_t)
            & -(2 as std::ffi::c_int) as size_t) as std::ffi::c_int;
        oksize = (((*node).mn_ksize as std::ffi::c_uint).wrapping_add(1 as std::ffi::c_uint)
            & -(2 as std::ffi::c_int) as std::ffi::c_uint) as std::ffi::c_int;
        delta = ksize - oksize;
        if delta != 0 {
            if delta > 0 as std::ffi::c_int
                && (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_upper
                    as std::ffi::c_int
                    - (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                        as std::ffi::c_int) as indx_t as std::ffi::c_int)
                    < delta
            {
                let mut pgno: pgno_t = 0;
                pgno = (*node).mn_lo as pgno_t
                    | ((*node).mn_hi as pgno_t) << 16 as std::ffi::c_int
                    | (if (if -(1 as std::ffi::c_int) as pgno_t
                        > 0xffffffff as std::ffi::c_uint as pgno_t
                    {
                        32 as std::ffi::c_int
                    } else {
                        0 as std::ffi::c_int
                    }) != 0
                    {
                        ((*node).mn_flags as pgno_t)
                            << (if -(1 as std::ffi::c_int) as pgno_t
                                > 0xffffffff as std::ffi::c_uint as pgno_t
                            {
                                32 as std::ffi::c_int
                            } else {
                                0 as std::ffi::c_int
                            })
                    } else {
                        0 as std::ffi::c_int as pgno_t
                    });
                mdb_node_del(mc, 0 as std::ffi::c_int);
                return mdb_page_split(
                    mc,
                    key,
                    std::ptr::null_mut::<MDB_val>(),
                    pgno,
                    0x40000 as std::ffi::c_int as std::ffi::c_uint,
                );
            }
            numkeys = (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int) as indx_t;
            i = 0 as std::ffi::c_int as indx_t;
            while (i as std::ffi::c_int) < numkeys as std::ffi::c_int {
                if *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) as std::ffi::c_int
                    <= ptr as std::ffi::c_int
                {
                    let fresh64 = &mut (*((*mp).mp_ptrs).as_mut_ptr().offset(i as isize));
                    *fresh64 = (*fresh64 as std::ffi::c_int - delta) as indx_t;
                }
                i = i.wrapping_add(1);
            }
            base = (mp as *mut std::ffi::c_char)
                .offset((*mp).mp_pb.pb.pb_upper as std::ffi::c_int as isize)
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                );
            len = ((ptr as std::ffi::c_int - (*mp).mp_pb.pb.pb_upper as std::ffi::c_int)
                as std::ffi::c_ulong)
                .wrapping_add(8 as std::ffi::c_ulong);
            memmove(
                base.offset(-(delta as isize)) as *mut std::ffi::c_void,
                base as *const std::ffi::c_void,
                len,
            );
            (*mp).mp_pb.pb.pb_upper =
                ((*mp).mp_pb.pb.pb_upper as std::ffi::c_int - delta) as indx_t;
            node = (mp as *mut std::ffi::c_char)
                .offset(
                    *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                        .as_mut_ptr()
                        .offset(indx as isize) as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node;
        }
        if (*node).mn_ksize as size_t != (*key).mv_size {
            (*node).mn_ksize = (*key).mv_size as std::ffi::c_ushort;
        }
        if (*key).mv_size != 0 {
            memcpy(
                ((*node).mn_data).as_mut_ptr() as *mut std::ffi::c_void,
                (*key).mv_data,
                (*key).mv_size,
            );
        }
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_node_move(
    mut csrc: *mut MDB_cursor,
    mut cdst: *mut MDB_cursor,
    mut fromleft: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut srcnode: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut key: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut data: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut srcpg: pgno_t = 0;
        let mut mn: MDB_cursor = MDB_cursor {
            mc_next: std::ptr::null_mut::<MDB_cursor>(),
            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
            mc_txn: std::ptr::null_mut::<MDB_txn>(),
            mc_dbi: 0,
            mc_db: std::ptr::null_mut::<MDB_db>(),
            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
            mc_ki: [0; 32],
        };
        let mut rc: std::ffi::c_int = 0;
        let mut flags: std::ffi::c_ushort = 0;
        rc = mdb_page_touch(csrc);
        if rc != 0 || {
            rc = mdb_page_touch(cdst);
            rc != 0
        } {
            return rc;
        }
        if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
            .mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
            key.mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_char)
                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                .offset(((*csrc).mc_ki[(*csrc).mc_top as usize] as size_t * key.mv_size) as isize)
                as *mut std::ffi::c_void;
            data.mv_size = 0 as std::ffi::c_int as size_t;
            data.mv_data = std::ptr::null_mut::<std::ffi::c_void>();
            srcpg = 0 as std::ffi::c_int as pgno_t;
            flags = 0 as std::ffi::c_int as std::ffi::c_ushort;
        } else {
            srcnode = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_char)
                .offset(
                    *((*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_void
                        as *mut MDB_page2))
                        .mp2_ptrs)
                        .as_mut_ptr()
                        .offset((*csrc).mc_ki[(*csrc).mc_top as usize] as isize)
                        as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            if srcnode as size_t & 1 as std::ffi::c_int as size_t == 0 {
            } else {
                mdb_assert_fail(
                    (*(*csrc).mc_txn).mt_env,
                    b"!((size_t)srcnode & 1)\0" as *const u8 as *const std::ffi::c_char,
                    (*::core::mem::transmute::<&[u8; 14], &[std::ffi::c_char; 14]>(
                        b"mdb_node_move\0",
                    ))
                    .as_ptr(),
                    b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                    9003 as std::ffi::c_int,
                );
            };
            srcpg = (*srcnode).mn_lo as pgno_t
                | ((*srcnode).mn_hi as pgno_t) << 16 as std::ffi::c_int
                | (if (if -(1 as std::ffi::c_int) as pgno_t
                    > 0xffffffff as std::ffi::c_uint as pgno_t
                {
                    32 as std::ffi::c_int
                } else {
                    0 as std::ffi::c_int
                }) != 0
                {
                    ((*srcnode).mn_flags as pgno_t)
                        << (if -(1 as std::ffi::c_int) as pgno_t
                            > 0xffffffff as std::ffi::c_uint as pgno_t
                        {
                            32 as std::ffi::c_int
                        } else {
                            0 as std::ffi::c_int
                        })
                } else {
                    0 as std::ffi::c_int as pgno_t
                });
            flags = (*srcnode).mn_flags;
            if (*csrc).mc_ki[(*csrc).mc_top as usize] as std::ffi::c_int == 0 as std::ffi::c_int
                && (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_flags as std::ffi::c_int
                    & 0x1 as std::ffi::c_int
                    == 0x1 as std::ffi::c_int
            {
                let mut snum: std::ffi::c_uint = (*csrc).mc_snum as std::ffi::c_uint;
                let mut s2: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                rc = mdb_page_search_lowest(csrc);
                if rc != 0 {
                    return rc;
                }
                if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_flags as std::ffi::c_int
                    & 0x20 as std::ffi::c_int
                    == 0x20 as std::ffi::c_int
                {
                    key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
                    key.mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_char)
                        .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                        .offset((0 as std::ffi::c_int as size_t * key.mv_size) as isize)
                        as *mut std::ffi::c_void;
                } else {
                    s2 = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_char)
                        .offset(
                            *((*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_void
                                as *mut MDB_page2))
                                .mp2_ptrs)
                                .as_mut_ptr()
                                .offset(0 as std::ffi::c_int as isize)
                                as std::ffi::c_int as isize,
                        )
                        .offset(
                            (if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    key.mv_size = (*s2).mn_ksize as size_t;
                    key.mv_data = ((*s2).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                }
                let fresh65 = snum;
                snum = snum.wrapping_sub(1);
                (*csrc).mc_snum = fresh65 as std::ffi::c_ushort;
                (*csrc).mc_top = snum as std::ffi::c_ushort;
            } else {
                key.mv_size = (*srcnode).mn_ksize as size_t;
                key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
            }
            data.mv_size = ((*srcnode).mn_lo as std::ffi::c_uint
                | ((*srcnode).mn_hi as std::ffi::c_uint) << 16 as std::ffi::c_int)
                as size_t;
            data.mv_data = ((*srcnode).mn_data)
                .as_mut_ptr()
                .offset((*srcnode).mn_ksize as std::ffi::c_int as isize)
                as *mut std::ffi::c_void;
        }
        mn.mc_xcursor = std::ptr::null_mut::<MDB_xcursor>();
        if (*((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
            .mp2_flags as std::ffi::c_int
            & 0x1 as std::ffi::c_int
            == 0x1 as std::ffi::c_int
            && (*cdst).mc_ki[(*cdst).mc_top as usize] as std::ffi::c_int == 0 as std::ffi::c_int
        {
            let mut snum_0: std::ffi::c_uint = (*cdst).mc_snum as std::ffi::c_uint;
            let mut s2_0: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
            let mut bkey: MDB_val =
                MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
            mdb_cursor_copy(cdst, &mut mn);
            rc = mdb_page_search_lowest(&mut mn);
            if rc != 0 {
                return rc;
            }
            if (*(mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                .mp2_flags as std::ffi::c_int
                & 0x20 as std::ffi::c_int
                == 0x20 as std::ffi::c_int
            {
                bkey.mv_size = (*mn.mc_db).md_pad as size_t;
                bkey.mv_data = (mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_char)
                    .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                    .offset((0 as std::ffi::c_int as size_t * bkey.mv_size) as isize)
                    as *mut std::ffi::c_void;
            } else {
                s2_0 = (mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_char)
                    .offset(
                        *((*(mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_void
                            as *mut MDB_page2))
                            .mp2_ptrs)
                            .as_mut_ptr()
                            .offset(0 as std::ffi::c_int as isize)
                            as std::ffi::c_int as isize,
                    )
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                bkey.mv_size = (*s2_0).mn_ksize as size_t;
                bkey.mv_data = ((*s2_0).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
            }
            let fresh66 = snum_0;
            snum_0 = snum_0.wrapping_sub(1);
            mn.mc_snum = fresh66 as std::ffi::c_ushort;
            mn.mc_top = snum_0 as std::ffi::c_ushort;
            mn.mc_ki[snum_0 as usize] = 0 as std::ffi::c_int as indx_t;
            rc = mdb_update_key(&mut mn, &mut bkey);
            if rc != 0 {
                return rc;
            }
        }
        rc = mdb_node_add(
            cdst,
            (*cdst).mc_ki[(*cdst).mc_top as usize],
            &mut key,
            &mut data,
            srcpg,
            flags as std::ffi::c_uint,
        );
        if rc != 0 as std::ffi::c_int {
            return rc;
        }
        mdb_node_del(csrc, key.mv_size as std::ffi::c_int);
        let mut m2: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut m3: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut dbi: MDB_dbi = (*csrc).mc_dbi;
        let mut mpd: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut mps: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        mps = (*csrc).mc_pg[(*csrc).mc_top as usize];
        if fromleft != 0 {
            mpd = (*cdst).mc_pg[(*csrc).mc_top as usize];
            m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
            while !m2.is_null() {
                if (*csrc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                } else {
                    m3 = m2;
                }
                if !((*m3).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0
                    || ((*m3).mc_top as std::ffi::c_int) < (*csrc).mc_top as std::ffi::c_int)
                {
                    if m3 != cdst
                        && (*m3).mc_pg[(*csrc).mc_top as usize] == mpd
                        && (*m3).mc_ki[(*csrc).mc_top as usize] as std::ffi::c_int
                            >= (*cdst).mc_ki[(*csrc).mc_top as usize] as std::ffi::c_int
                    {
                        (*m3).mc_ki[(*csrc).mc_top as usize] =
                            ((*m3).mc_ki[(*csrc).mc_top as usize]).wrapping_add(1);
                        (*m3).mc_ki[(*csrc).mc_top as usize];
                    }
                    if m3 != csrc
                        && (*m3).mc_pg[(*csrc).mc_top as usize] == mps
                        && (*m3).mc_ki[(*csrc).mc_top as usize] as std::ffi::c_int
                            == (*csrc).mc_ki[(*csrc).mc_top as usize] as std::ffi::c_int
                    {
                        (*m3).mc_pg[(*csrc).mc_top as usize] =
                            (*cdst).mc_pg[(*cdst).mc_top as usize];
                        (*m3).mc_ki[(*csrc).mc_top as usize] =
                            (*cdst).mc_ki[(*cdst).mc_top as usize];
                        (*m3).mc_ki
                            [((*csrc).mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as usize] =
                            ((*m3).mc_ki[((*csrc).mc_top as std::ffi::c_int - 1 as std::ffi::c_int)
                                as usize])
                                .wrapping_add(1);
                        (*m3).mc_ki
                            [((*csrc).mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as usize];
                    }
                    if (*(mps as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                        as std::ffi::c_int
                        & 0x2 as std::ffi::c_int
                        == 0x2 as std::ffi::c_int
                    {
                        let mut xr_pg: *mut MDB_page = (*m3).mc_pg[(*csrc).mc_top as usize];
                        let mut xr_node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                        if !(!(!((*m3).mc_xcursor).is_null()
                            && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                & 0x1 as std::ffi::c_int as std::ffi::c_uint
                                != 0)
                            || (*m3).mc_ki[(*csrc).mc_top as usize] as std::ffi::c_uint
                                >= ((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                                    as std::ffi::c_uint)
                                    .wrapping_sub(
                                        (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                            if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            },
                                        ),
                                    )
                                    >> 1 as std::ffi::c_int)
                        {
                            xr_node = (xr_pg as *mut std::ffi::c_char)
                                .offset(
                                    *((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2))
                                        .mp2_ptrs)
                                        .as_mut_ptr()
                                        .offset((*m3).mc_ki[(*csrc).mc_top as usize] as isize)
                                        as std::ffi::c_int
                                        as isize,
                                )
                                .offset(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            if (*xr_node).mn_flags as std::ffi::c_int
                                & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                == 0x4 as std::ffi::c_int
                            {
                                (*(*m3).mc_xcursor).mx_cursor.mc_pg
                                    [0 as std::ffi::c_int as usize] = ((*xr_node).mn_data)
                                    .as_mut_ptr()
                                    .offset((*xr_node).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void
                                    as *mut MDB_page;
                            }
                        }
                    }
                }
                m2 = (*m2).mc_next;
            }
        } else {
            m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
            while !m2.is_null() {
                if (*csrc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                } else {
                    m3 = m2;
                }
                if (m3 != csrc)
                    && !((*m3).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0
                        || ((*m3).mc_top as std::ffi::c_int) < (*csrc).mc_top as std::ffi::c_int)
                    && (*m3).mc_pg[(*csrc).mc_top as usize] == mps
                {
                    if (*m3).mc_ki[(*csrc).mc_top as usize] == 0 {
                        (*m3).mc_pg[(*csrc).mc_top as usize] =
                            (*cdst).mc_pg[(*cdst).mc_top as usize];
                        (*m3).mc_ki[(*csrc).mc_top as usize] =
                            (*cdst).mc_ki[(*cdst).mc_top as usize];
                        (*m3).mc_ki
                            [((*csrc).mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as usize] =
                            ((*m3).mc_ki[((*csrc).mc_top as std::ffi::c_int - 1 as std::ffi::c_int)
                                as usize])
                                .wrapping_sub(1);
                        (*m3).mc_ki
                            [((*csrc).mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as usize];
                    } else {
                        (*m3).mc_ki[(*csrc).mc_top as usize] =
                            ((*m3).mc_ki[(*csrc).mc_top as usize]).wrapping_sub(1);
                        (*m3).mc_ki[(*csrc).mc_top as usize];
                    }
                    if (*(mps as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                        as std::ffi::c_int
                        & 0x2 as std::ffi::c_int
                        == 0x2 as std::ffi::c_int
                    {
                        let mut xr_pg_0: *mut MDB_page = (*m3).mc_pg[(*csrc).mc_top as usize];
                        let mut xr_node_0: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                        if !(!(!((*m3).mc_xcursor).is_null()
                            && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                & 0x1 as std::ffi::c_int as std::ffi::c_uint
                                != 0)
                            || (*m3).mc_ki[(*csrc).mc_top as usize] as std::ffi::c_uint
                                >= ((*(xr_pg_0 as *mut std::ffi::c_void as *mut MDB_page2))
                                    .mp2_lower
                                    as std::ffi::c_uint)
                                    .wrapping_sub(
                                        (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                            if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            },
                                        ),
                                    )
                                    >> 1 as std::ffi::c_int)
                        {
                            xr_node_0 = (xr_pg_0 as *mut std::ffi::c_char)
                                .offset(
                                    *((*(xr_pg_0 as *mut std::ffi::c_void as *mut MDB_page2))
                                        .mp2_ptrs)
                                        .as_mut_ptr()
                                        .offset((*m3).mc_ki[(*csrc).mc_top as usize] as isize)
                                        as std::ffi::c_int
                                        as isize,
                                )
                                .offset(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            if (*xr_node_0).mn_flags as std::ffi::c_int
                                & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                == 0x4 as std::ffi::c_int
                            {
                                (*(*m3).mc_xcursor).mx_cursor.mc_pg
                                    [0 as std::ffi::c_int as usize] = ((*xr_node_0).mn_data)
                                    .as_mut_ptr()
                                    .offset((*xr_node_0).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void
                                    as *mut MDB_page;
                            }
                        }
                    }
                }
                m2 = (*m2).mc_next;
            }
        }
        if (*csrc).mc_ki[(*csrc).mc_top as usize] as std::ffi::c_int == 0 as std::ffi::c_int {
            if (*csrc).mc_ki[((*csrc).mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as usize]
                as std::ffi::c_int
                != 0 as std::ffi::c_int
            {
                if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_flags as std::ffi::c_int
                    & 0x20 as std::ffi::c_int
                    == 0x20 as std::ffi::c_int
                {
                    key.mv_data = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_char)
                        .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                        .offset((0 as std::ffi::c_int as size_t * key.mv_size) as isize)
                        as *mut std::ffi::c_void;
                } else {
                    srcnode = ((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_char)
                        .offset(
                            *((*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_void
                                as *mut MDB_page2))
                                .mp2_ptrs)
                                .as_mut_ptr()
                                .offset(0 as std::ffi::c_int as isize)
                                as std::ffi::c_int as isize,
                        )
                        .offset(
                            (if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    key.mv_size = (*srcnode).mn_ksize as size_t;
                    key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                }
                mdb_cursor_copy(csrc, &mut mn);
                mn.mc_snum = (mn.mc_snum).wrapping_sub(1);
                mn.mc_snum;
                mn.mc_top = (mn.mc_top).wrapping_sub(1);
                mn.mc_top;
                let mut dummy: MDB_cursor = MDB_cursor {
                    mc_next: std::ptr::null_mut::<MDB_cursor>(),
                    mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                    mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                    mc_txn: std::ptr::null_mut::<MDB_txn>(),
                    mc_dbi: 0,
                    mc_db: std::ptr::null_mut::<MDB_db>(),
                    mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                    mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                    mc_snum: 0,
                    mc_top: 0,
                    mc_flags: 0,
                    mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                    mc_ki: [0; 32],
                };
                let mut tracked: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                let mut tp: *mut *mut MDB_cursor = &mut *((*mn.mc_txn).mt_cursors)
                    .offset(mn.mc_dbi as isize)
                    as *mut *mut MDB_cursor;
                if mn.mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    dummy.mc_flags = 0x1 as std::ffi::c_int as std::ffi::c_uint;
                    dummy.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                    tracked = &mut dummy;
                } else {
                    tracked = &mut mn;
                }
                (*tracked).mc_next = *tp;
                *tp = tracked;
                rc = mdb_update_key(&mut mn, &mut key);
                *tp = (*tracked).mc_next;
                if rc != 0 {
                    return rc;
                }
            }
            if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_void
                as *mut MDB_page2))
                .mp2_flags as std::ffi::c_int
                & 0x1 as std::ffi::c_int
                == 0x1 as std::ffi::c_int
            {
                let mut nullkey: MDB_val =
                    MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
                let mut ix: indx_t = (*csrc).mc_ki[(*csrc).mc_top as usize];
                nullkey.mv_size = 0 as std::ffi::c_int as size_t;
                (*csrc).mc_ki[(*csrc).mc_top as usize] = 0 as std::ffi::c_int as indx_t;
                rc = mdb_update_key(csrc, &mut nullkey);
                (*csrc).mc_ki[(*csrc).mc_top as usize] = ix;
                if rc == 0 as std::ffi::c_int {
                } else {
                    mdb_assert_fail(
                        (*(*csrc).mc_txn).mt_env,
                        b"rc == MDB_SUCCESS\0" as *const u8 as *const std::ffi::c_char,
                        (*::core::mem::transmute::<&[u8; 14], &[std::ffi::c_char; 14]>(
                            b"mdb_node_move\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                        9160 as std::ffi::c_int,
                    );
                };
            }
        }
        if (*cdst).mc_ki[(*cdst).mc_top as usize] as std::ffi::c_int == 0 as std::ffi::c_int {
            if (*cdst).mc_ki[((*cdst).mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as usize]
                as std::ffi::c_int
                != 0 as std::ffi::c_int
            {
                if (*((*csrc).mc_pg[(*csrc).mc_top as usize] as *mut std::ffi::c_void
                    as *mut MDB_page2))
                    .mp2_flags as std::ffi::c_int
                    & 0x20 as std::ffi::c_int
                    == 0x20 as std::ffi::c_int
                {
                    key.mv_data = ((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut std::ffi::c_char)
                        .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                        .offset((0 as std::ffi::c_int as size_t * key.mv_size) as isize)
                        as *mut std::ffi::c_void;
                } else {
                    srcnode = ((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut std::ffi::c_char)
                        .offset(
                            *((*((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut std::ffi::c_void
                                as *mut MDB_page2))
                                .mp2_ptrs)
                                .as_mut_ptr()
                                .offset(0 as std::ffi::c_int as isize)
                                as std::ffi::c_int as isize,
                        )
                        .offset(
                            (if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            }) as isize,
                        ) as *mut MDB_node;
                    key.mv_size = (*srcnode).mn_ksize as size_t;
                    key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                }
                mdb_cursor_copy(cdst, &mut mn);
                mn.mc_snum = (mn.mc_snum).wrapping_sub(1);
                mn.mc_snum;
                mn.mc_top = (mn.mc_top).wrapping_sub(1);
                mn.mc_top;
                let mut dummy_0: MDB_cursor = MDB_cursor {
                    mc_next: std::ptr::null_mut::<MDB_cursor>(),
                    mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                    mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                    mc_txn: std::ptr::null_mut::<MDB_txn>(),
                    mc_dbi: 0,
                    mc_db: std::ptr::null_mut::<MDB_db>(),
                    mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                    mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                    mc_snum: 0,
                    mc_top: 0,
                    mc_flags: 0,
                    mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                    mc_ki: [0; 32],
                };
                let mut tracked_0: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                let mut tp_0: *mut *mut MDB_cursor = &mut *((*mn.mc_txn).mt_cursors)
                    .offset(mn.mc_dbi as isize)
                    as *mut *mut MDB_cursor;
                if mn.mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    dummy_0.mc_flags = 0x1 as std::ffi::c_int as std::ffi::c_uint;
                    dummy_0.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                    tracked_0 = &mut dummy_0;
                } else {
                    tracked_0 = &mut mn;
                }
                (*tracked_0).mc_next = *tp_0;
                *tp_0 = tracked_0;
                rc = mdb_update_key(&mut mn, &mut key);
                *tp_0 = (*tracked_0).mc_next;
                if rc != 0 {
                    return rc;
                }
            }
            if (*((*cdst).mc_pg[(*cdst).mc_top as usize] as *mut std::ffi::c_void
                as *mut MDB_page2))
                .mp2_flags as std::ffi::c_int
                & 0x1 as std::ffi::c_int
                == 0x1 as std::ffi::c_int
            {
                let mut nullkey_0: MDB_val =
                    MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
                let mut ix_0: indx_t = (*cdst).mc_ki[(*cdst).mc_top as usize];
                nullkey_0.mv_size = 0 as std::ffi::c_int as size_t;
                (*cdst).mc_ki[(*cdst).mc_top as usize] = 0 as std::ffi::c_int as indx_t;
                rc = mdb_update_key(cdst, &mut nullkey_0);
                (*cdst).mc_ki[(*cdst).mc_top as usize] = ix_0;
                if rc == 0 as std::ffi::c_int {
                } else {
                    mdb_assert_fail(
                        (*(*cdst).mc_txn).mt_env,
                        b"rc == MDB_SUCCESS\0" as *const u8 as *const std::ffi::c_char,
                        (*::core::mem::transmute::<&[u8; 14], &[std::ffi::c_char; 14]>(
                            b"mdb_node_move\0",
                        ))
                        .as_ptr(),
                        b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                        9191 as std::ffi::c_int,
                    );
                };
            }
        }
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_page_merge(
    mut csrc: *mut MDB_cursor,
    mut cdst: *mut MDB_cursor,
) -> std::ffi::c_int {
    unsafe {
        let mut psrc: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut pdst: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut srcnode: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut key: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut data: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut nkeys: std::ffi::c_uint = 0;
        let mut rc: std::ffi::c_int = 0;
        let mut i: indx_t = 0;
        let mut j: indx_t = 0;
        psrc = (*csrc).mc_pg[(*csrc).mc_top as usize];
        pdst = (*cdst).mc_pg[(*cdst).mc_top as usize];
        if (*csrc).mc_snum as std::ffi::c_int > 1 as std::ffi::c_int {
        } else {
            mdb_assert_fail(
                (*(*csrc).mc_txn).mt_env,
                b"csrc->mc_snum > 1\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 15], &[std::ffi::c_char; 15]>(
                    b"mdb_page_merge\0",
                ))
                .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                9221 as std::ffi::c_int,
            );
        };
        if (*cdst).mc_snum as std::ffi::c_int > 1 as std::ffi::c_int {
        } else {
            mdb_assert_fail(
                (*(*csrc).mc_txn).mt_env,
                b"cdst->mc_snum > 1\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 15], &[std::ffi::c_char; 15]>(
                    b"mdb_page_merge\0",
                ))
                .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                9222 as std::ffi::c_int,
            );
        };
        rc = mdb_page_touch(cdst);
        if rc != 0 {
            return rc;
        }
        pdst = (*cdst).mc_pg[(*cdst).mc_top as usize];
        nkeys = ((*(pdst as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
            as std::ffi::c_uint)
            .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                },
            ))
            >> 1 as std::ffi::c_int;
        j = nkeys as indx_t;
        if (*(psrc as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x20 as std::ffi::c_int
            == 0x20 as std::ffi::c_int
        {
            key.mv_size = (*(*csrc).mc_db).md_pad as size_t;
            key.mv_data = (psrc as *mut std::ffi::c_char)
                .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                as *mut std::ffi::c_void;
            i = 0 as std::ffi::c_int as indx_t;
            while (i as std::ffi::c_uint)
                < ((*(psrc as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                    as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int
            {
                rc = mdb_node_add(
                    cdst,
                    j,
                    &mut key,
                    std::ptr::null_mut::<MDB_val>(),
                    0 as std::ffi::c_int as pgno_t,
                    0 as std::ffi::c_int as std::ffi::c_uint,
                );
                if rc != 0 as std::ffi::c_int {
                    return rc;
                }
                key.mv_data = (key.mv_data as *mut std::ffi::c_char).offset(key.mv_size as isize)
                    as *mut std::ffi::c_void;
                i = i.wrapping_add(1);
                j = j.wrapping_add(1);
            }
        } else {
            i = 0 as std::ffi::c_int as indx_t;
            while (i as std::ffi::c_uint)
                < ((*(psrc as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                    as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int
            {
                srcnode = (psrc as *mut std::ffi::c_char)
                    .offset(
                        *((*(psrc as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                            .as_mut_ptr()
                            .offset(i as isize) as std::ffi::c_int as isize,
                    )
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                if i as std::ffi::c_int == 0 as std::ffi::c_int
                    && (*(psrc as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                        as std::ffi::c_int
                        & 0x1 as std::ffi::c_int
                        == 0x1 as std::ffi::c_int
                {
                    let mut mn: MDB_cursor = MDB_cursor {
                        mc_next: std::ptr::null_mut::<MDB_cursor>(),
                        mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                        mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                        mc_txn: std::ptr::null_mut::<MDB_txn>(),
                        mc_dbi: 0,
                        mc_db: std::ptr::null_mut::<MDB_db>(),
                        mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                        mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                        mc_snum: 0,
                        mc_top: 0,
                        mc_flags: 0,
                        mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                        mc_ki: [0; 32],
                    };
                    let mut s2: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                    mdb_cursor_copy(csrc, &mut mn);
                    mn.mc_xcursor = std::ptr::null_mut::<MDB_xcursor>();
                    rc = mdb_page_search_lowest(&mut mn);
                    if rc != 0 {
                        return rc;
                    }
                    if (*(mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_flags as std::ffi::c_int
                        & 0x20 as std::ffi::c_int
                        == 0x20 as std::ffi::c_int
                    {
                        key.mv_size = (*mn.mc_db).md_pad as size_t;
                        key.mv_data = (mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_char)
                            .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                            .offset((0 as std::ffi::c_int as size_t * key.mv_size) as isize)
                            as *mut std::ffi::c_void;
                    } else {
                        s2 = (mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_char)
                            .offset(
                                *((*(mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_void
                                    as *mut MDB_page2))
                                    .mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset(0 as std::ffi::c_int as isize)
                                    as std::ffi::c_int as isize,
                            )
                            .offset(
                                (if 0 as std::ffi::c_int != 0 {
                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                } else {
                                    0 as std::ffi::c_int as std::ffi::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        key.mv_size = (*s2).mn_ksize as size_t;
                        key.mv_data = ((*s2).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                    }
                } else {
                    key.mv_size = (*srcnode).mn_ksize as size_t;
                    key.mv_data = ((*srcnode).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                }
                data.mv_size = ((*srcnode).mn_lo as std::ffi::c_uint
                    | ((*srcnode).mn_hi as std::ffi::c_uint) << 16 as std::ffi::c_int)
                    as size_t;
                data.mv_data = ((*srcnode).mn_data)
                    .as_mut_ptr()
                    .offset((*srcnode).mn_ksize as std::ffi::c_int as isize)
                    as *mut std::ffi::c_void;
                rc = mdb_node_add(
                    cdst,
                    j,
                    &mut key,
                    &mut data,
                    (*srcnode).mn_lo as pgno_t
                        | ((*srcnode).mn_hi as pgno_t) << 16 as std::ffi::c_int
                        | (if (if -(1 as std::ffi::c_int) as pgno_t
                            > 0xffffffff as std::ffi::c_uint as pgno_t
                        {
                            32 as std::ffi::c_int
                        } else {
                            0 as std::ffi::c_int
                        }) != 0
                        {
                            ((*srcnode).mn_flags as pgno_t)
                                << (if -(1 as std::ffi::c_int) as pgno_t
                                    > 0xffffffff as std::ffi::c_uint as pgno_t
                                {
                                    32 as std::ffi::c_int
                                } else {
                                    0 as std::ffi::c_int
                                })
                        } else {
                            0 as std::ffi::c_int as pgno_t
                        }),
                    (*srcnode).mn_flags as std::ffi::c_uint,
                );
                if rc != 0 as std::ffi::c_int {
                    return rc;
                }
                i = i.wrapping_add(1);
                j = j.wrapping_add(1);
            }
        }
        (*csrc).mc_top = ((*csrc).mc_top).wrapping_sub(1);
        (*csrc).mc_top;
        mdb_node_del(csrc, 0 as std::ffi::c_int);
        if (*csrc).mc_ki[(*csrc).mc_top as usize] as std::ffi::c_int == 0 as std::ffi::c_int {
            key.mv_size = 0 as std::ffi::c_int as size_t;
            rc = mdb_update_key(csrc, &mut key);
            if rc != 0 {
                (*csrc).mc_top = ((*csrc).mc_top).wrapping_add(1);
                (*csrc).mc_top;
                return rc;
            }
        }
        (*csrc).mc_top = ((*csrc).mc_top).wrapping_add(1);
        (*csrc).mc_top;
        psrc = (*csrc).mc_pg[(*csrc).mc_top as usize];
        rc = mdb_page_loose(csrc, psrc);
        if rc != 0 {
            return rc;
        }
        if (*(psrc as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
            & 0x2 as std::ffi::c_int
            == 0x2 as std::ffi::c_int
        {
            (*(*csrc).mc_db).md_leaf_pages = ((*(*csrc).mc_db).md_leaf_pages).wrapping_sub(1);
            (*(*csrc).mc_db).md_leaf_pages;
        } else {
            (*(*csrc).mc_db).md_branch_pages = ((*(*csrc).mc_db).md_branch_pages).wrapping_sub(1);
            (*(*csrc).mc_db).md_branch_pages;
        }
        let mut m2: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut m3: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut dbi: MDB_dbi = (*csrc).mc_dbi;
        let mut top: std::ffi::c_uint = (*csrc).mc_top as std::ffi::c_uint;
        m2 = *((*(*csrc).mc_txn).mt_cursors).offset(dbi as isize);
        while !m2.is_null() {
            if (*csrc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
            } else {
                m3 = m2;
            }
            if (m3 != csrc)
                && (((*m3).mc_snum as std::ffi::c_int) >= (*csrc).mc_snum as std::ffi::c_int)
            {
                if (*m3).mc_pg[top as usize] == psrc {
                    (*m3).mc_pg[top as usize] = pdst;
                    (*m3).mc_ki[top as usize] = ((*m3).mc_ki[top as usize] as std::ffi::c_uint)
                        .wrapping_add(nkeys)
                        as indx_t as indx_t;
                    (*m3).mc_ki
                        [top.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as usize] =
                        (*cdst).mc_ki
                            [top.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as usize];
                } else if (*m3).mc_pg
                    [top.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as usize]
                    == (*csrc).mc_pg
                        [top.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as usize]
                    && (*m3).mc_ki
                        [top.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as usize]
                        as std::ffi::c_int
                        > (*csrc).mc_ki
                            [top.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as usize]
                            as std::ffi::c_int
                {
                    (*m3).mc_ki
                        [top.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as usize] =
                        ((*m3).mc_ki
                            [top.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as usize])
                            .wrapping_sub(1);
                    (*m3).mc_ki
                        [top.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as usize];
                }
                if (*(psrc as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                    & 0x2 as std::ffi::c_int
                    == 0x2 as std::ffi::c_int
                {
                    let mut xr_pg: *mut MDB_page = (*m3).mc_pg[top as usize];
                    let mut xr_node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                    if !(!(!((*m3).mc_xcursor).is_null()
                        && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                            & 0x1 as std::ffi::c_int as std::ffi::c_uint
                            != 0)
                        || (*m3).mc_ki[top as usize] as std::ffi::c_uint
                            >= ((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                                as std::ffi::c_uint)
                                .wrapping_sub(
                                    (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                        if 0 as std::ffi::c_int != 0 {
                                            16 as std::ffi::c_ulong as std::ffi::c_uint
                                        } else {
                                            0 as std::ffi::c_int as std::ffi::c_uint
                                        },
                                    ),
                                )
                                >> 1 as std::ffi::c_int)
                    {
                        xr_node = (xr_pg as *mut std::ffi::c_char)
                            .offset(
                                *((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset((*m3).mc_ki[top as usize] as isize)
                                    as std::ffi::c_int as isize,
                            )
                            .offset(
                                (if 0 as std::ffi::c_int != 0 {
                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                } else {
                                    0 as std::ffi::c_int as std::ffi::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if (*xr_node).mn_flags as std::ffi::c_int
                            & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                            == 0x4 as std::ffi::c_int
                        {
                            (*(*m3).mc_xcursor).mx_cursor.mc_pg[0 as std::ffi::c_int as usize] =
                                ((*xr_node).mn_data)
                                    .as_mut_ptr()
                                    .offset((*xr_node).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void
                                    as *mut MDB_page;
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
        let mut snum: std::ffi::c_uint = (*cdst).mc_snum as std::ffi::c_uint;
        let mut depth: uint16_t = (*(*cdst).mc_db).md_depth;
        mdb_cursor_pop(cdst);
        rc = mdb_rebalance(cdst);
        if depth as std::ffi::c_int != (*(*cdst).mc_db).md_depth as std::ffi::c_int {
            snum = snum.wrapping_add(
                ((*(*cdst).mc_db).md_depth as std::ffi::c_int - depth as std::ffi::c_int)
                    as std::ffi::c_uint,
            );
        }
        (*cdst).mc_snum = snum as std::ffi::c_ushort;
        (*cdst).mc_top =
            snum.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_ushort;
        rc
    }
}
unsafe extern "C" fn mdb_cursor_copy(mut csrc: *const MDB_cursor, mut cdst: *mut MDB_cursor) {
    unsafe {
        let mut i: std::ffi::c_uint = 0;
        (*cdst).mc_txn = (*csrc).mc_txn;
        (*cdst).mc_dbi = (*csrc).mc_dbi;
        (*cdst).mc_db = (*csrc).mc_db;
        (*cdst).mc_dbx = (*csrc).mc_dbx;
        (*cdst).mc_snum = (*csrc).mc_snum;
        (*cdst).mc_top = (*csrc).mc_top;
        (*cdst).mc_flags = (*csrc).mc_flags;
        i = 0 as std::ffi::c_int as std::ffi::c_uint;
        while i < (*csrc).mc_snum as std::ffi::c_uint {
            (*cdst).mc_pg[i as usize] = (*csrc).mc_pg[i as usize];
            (*cdst).mc_ki[i as usize] = (*csrc).mc_ki[i as usize];
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn mdb_rebalance(mut mc: *mut MDB_cursor) -> std::ffi::c_int {
    unsafe {
        let mut node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut rc: std::ffi::c_int = 0;
        let mut fromleft: std::ffi::c_int = 0;
        let mut ptop: std::ffi::c_uint = 0;
        let mut minkeys: std::ffi::c_uint = 0;
        let mut thresh: std::ffi::c_uint = 0;
        let mut mn: MDB_cursor = MDB_cursor {
            mc_next: std::ptr::null_mut::<MDB_cursor>(),
            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
            mc_txn: std::ptr::null_mut::<MDB_txn>(),
            mc_dbi: 0,
            mc_db: std::ptr::null_mut::<MDB_db>(),
            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
            mc_ki: [0; 32],
        };
        let mut oldki: indx_t = 0;
        if (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
            .mp2_flags as std::ffi::c_int
            & 0x1 as std::ffi::c_int
            == 0x1 as std::ffi::c_int
        {
            minkeys = 2 as std::ffi::c_int as std::ffi::c_uint;
            thresh = 1 as std::ffi::c_int as std::ffi::c_uint;
        } else {
            minkeys = 1 as std::ffi::c_int as std::ffi::c_uint;
            thresh = 250 as std::ffi::c_int as std::ffi::c_uint;
        }
        if 1000 as std::ffi::c_long
            * ((*(*(*mc).mc_txn).mt_env).me_psize)
                .wrapping_sub(16 as std::ffi::c_ulong as std::ffi::c_uint)
                .wrapping_sub(
                    ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                        as *mut MDB_page2))
                        .mp2_upper as std::ffi::c_int
                        - (*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void
                            as *mut MDB_page2))
                            .mp2_lower as std::ffi::c_int) as indx_t
                        as std::ffi::c_uint,
                ) as std::ffi::c_long
            / ((*(*(*mc).mc_txn).mt_env).me_psize)
                .wrapping_sub(16 as std::ffi::c_ulong as std::ffi::c_uint)
                as std::ffi::c_long
            >= thresh as std::ffi::c_long
            && ((*((*mc).mc_pg[(*mc).mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                .mp2_lower as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int
                >= minkeys
        {
            return 0 as std::ffi::c_int;
        }
        if ((*mc).mc_snum as std::ffi::c_int) < 2 as std::ffi::c_int {
            let mut mp: *mut MDB_page = (*mc).mc_pg[0 as std::ffi::c_int as usize];
            if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                & 0x40 as std::ffi::c_int
                == 0x40 as std::ffi::c_int
            {
                return 0 as std::ffi::c_int;
            }
            if ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int
                == 0 as std::ffi::c_int as std::ffi::c_uint
            {
                (*(*mc).mc_db).md_root = !(0 as std::ffi::c_int as pgno_t);
                (*(*mc).mc_db).md_depth = 0 as std::ffi::c_int as uint16_t;
                (*(*mc).mc_db).md_leaf_pages = 0 as std::ffi::c_int as pgno_t;
                rc = mdb_midl_append(&mut (*(*mc).mc_txn).mt_free_pgs, (*mp).mp_p.p_pgno);
                if rc != 0 {
                    return rc;
                }
                (*mc).mc_snum = 0 as std::ffi::c_int as std::ffi::c_ushort;
                (*mc).mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
                (*mc).mc_flags &= !(0x1 as std::ffi::c_int) as std::ffi::c_uint;
                let mut m2: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                let mut m3: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                let mut dbi: MDB_dbi = (*mc).mc_dbi;
                m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
                while !m2.is_null() {
                    if (*mc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                        m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                    } else {
                        m3 = m2;
                    }
                    if !((*m3).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint == 0
                        || ((*m3).mc_snum as std::ffi::c_int) < (*mc).mc_snum as std::ffi::c_int)
                        && (*m3).mc_pg[0 as std::ffi::c_int as usize] == mp
                    {
                        (*m3).mc_snum = 0 as std::ffi::c_int as std::ffi::c_ushort;
                        (*m3).mc_top = 0 as std::ffi::c_int as std::ffi::c_ushort;
                        (*m3).mc_flags &= !(0x1 as std::ffi::c_int) as std::ffi::c_uint;
                    }
                    m2 = (*m2).mc_next;
                }
            } else if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                as std::ffi::c_int
                & 0x1 as std::ffi::c_int
                == 0x1 as std::ffi::c_int
                && ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                    as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int
                    == 1 as std::ffi::c_int as std::ffi::c_uint
            {
                let mut i: std::ffi::c_int = 0;
                rc = mdb_midl_append(&mut (*(*mc).mc_txn).mt_free_pgs, (*mp).mp_p.p_pgno);
                if rc != 0 {
                    return rc;
                }
                (*(*mc).mc_db).md_root = (*((mp as *mut std::ffi::c_char)
                    .offset(std::ptr::read_unaligned(
                        ((mp as *mut u8).offset(offset_of!(MDB_page2, mp2_ptrs) as isize)
                            as *mut u16)
                            .offset(0 as isize),
                    ) as isize)
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node))
                    .mn_lo as pgno_t
                    | ((*((mp as *mut std::ffi::c_char)
                        .offset(
                            *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                .as_mut_ptr()
                                .offset(0 as std::ffi::c_int as isize)
                                as std::ffi::c_int as isize,
                        )
                        .offset(
                            (if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            }) as isize,
                        ) as *mut MDB_node))
                        .mn_hi as pgno_t)
                        << 16 as std::ffi::c_int
                    | (if (if -(1 as std::ffi::c_int) as pgno_t
                        > 0xffffffff as std::ffi::c_uint as pgno_t
                    {
                        32 as std::ffi::c_int
                    } else {
                        0 as std::ffi::c_int
                    }) != 0
                    {
                        ((*((mp as *mut std::ffi::c_char)
                            .offset(
                                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset(0 as std::ffi::c_int as isize)
                                    as std::ffi::c_int as isize,
                            )
                            .offset(
                                (if 0 as std::ffi::c_int != 0 {
                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                } else {
                                    0 as std::ffi::c_int as std::ffi::c_uint
                                }) as isize,
                            ) as *mut MDB_node))
                            .mn_flags as pgno_t)
                            << (if -(1 as std::ffi::c_int) as pgno_t
                                > 0xffffffff as std::ffi::c_uint as pgno_t
                            {
                                32 as std::ffi::c_int
                            } else {
                                0 as std::ffi::c_int
                            })
                    } else {
                        0 as std::ffi::c_int as pgno_t
                    });
                rc = mdb_page_get(
                    mc,
                    (*(*mc).mc_db).md_root,
                    &mut *((*mc).mc_pg).as_mut_ptr().offset(0 as std::ffi::c_int as isize),
                    std::ptr::null_mut::<std::ffi::c_int>(),
                );
                if rc != 0 {
                    return rc;
                }
                (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_sub(1);
                (*(*mc).mc_db).md_depth;
                (*(*mc).mc_db).md_branch_pages = ((*(*mc).mc_db).md_branch_pages).wrapping_sub(1);
                (*(*mc).mc_db).md_branch_pages;
                (*mc).mc_ki[0 as std::ffi::c_int as usize] =
                    (*mc).mc_ki[1 as std::ffi::c_int as usize];
                i = 1 as std::ffi::c_int;
                while i < (*(*mc).mc_db).md_depth as std::ffi::c_int {
                    (*mc).mc_pg[i as usize] = (*mc).mc_pg[(i + 1 as std::ffi::c_int) as usize];
                    (*mc).mc_ki[i as usize] = (*mc).mc_ki[(i + 1 as std::ffi::c_int) as usize];
                    i += 1;
                }
                let mut m2_0: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                let mut m3_0: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                let mut dbi_0: MDB_dbi = (*mc).mc_dbi;
                m2_0 = *((*(*mc).mc_txn).mt_cursors).offset(dbi_0 as isize);
                while !m2_0.is_null() {
                    if (*mc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                        m3_0 = &mut (*(*m2_0).mc_xcursor).mx_cursor;
                    } else {
                        m3_0 = m2_0;
                    }
                    if (m3_0 != mc)
                        && ((*m3_0).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0)
                        && (*m3_0).mc_pg[0 as std::ffi::c_int as usize] == mp
                    {
                        i = 0 as std::ffi::c_int;
                        while i < (*(*mc).mc_db).md_depth as std::ffi::c_int {
                            (*m3_0).mc_pg[i as usize] =
                                (*m3_0).mc_pg[(i + 1 as std::ffi::c_int) as usize];
                            (*m3_0).mc_ki[i as usize] =
                                (*m3_0).mc_ki[(i + 1 as std::ffi::c_int) as usize];
                            i += 1;
                        }
                        (*m3_0).mc_snum = ((*m3_0).mc_snum).wrapping_sub(1);
                        (*m3_0).mc_snum;
                        (*m3_0).mc_top = ((*m3_0).mc_top).wrapping_sub(1);
                        (*m3_0).mc_top;
                    }
                    m2_0 = (*m2_0).mc_next;
                }
            }
            return 0 as std::ffi::c_int;
        }
        ptop = ((*mc).mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as std::ffi::c_uint;
        if ((*((*mc).mc_pg[ptop as usize] as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
            as std::ffi::c_uint)
            .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                },
            ))
            >> 1 as std::ffi::c_int
            > 1 as std::ffi::c_int as std::ffi::c_uint
        {
        } else {
            mdb_assert_fail(
                (*(*mc).mc_txn).mt_env,
                b"NUMKEYS(mc->mc_pg[ptop]) > 1\0" as *const u8 as *const std::ffi::c_char,
                (*::core::mem::transmute::<&[u8; 14], &[std::ffi::c_char; 14]>(b"mdb_rebalance\0"))
                    .as_ptr(),
                b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                9486 as std::ffi::c_int,
            );
        };
        mdb_cursor_copy(mc, &mut mn);
        mn.mc_xcursor = std::ptr::null_mut::<MDB_xcursor>();
        oldki = (*mc).mc_ki[(*mc).mc_top as usize];
        if (*mc).mc_ki[ptop as usize] as std::ffi::c_int == 0 as std::ffi::c_int {
            mn.mc_ki[ptop as usize] = (mn.mc_ki[ptop as usize]).wrapping_add(1);
            mn.mc_ki[ptop as usize];
            node = ((*mc).mc_pg[ptop as usize] as *mut std::ffi::c_char)
                .offset(
                    *((*((*mc).mc_pg[ptop as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_ptrs)
                        .as_mut_ptr()
                        .offset(mn.mc_ki[ptop as usize] as isize)
                        as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            rc = mdb_page_get(
                mc,
                (*node).mn_lo as pgno_t
                    | ((*node).mn_hi as pgno_t) << 16 as std::ffi::c_int
                    | (if (if -(1 as std::ffi::c_int) as pgno_t
                        > 0xffffffff as std::ffi::c_uint as pgno_t
                    {
                        32 as std::ffi::c_int
                    } else {
                        0 as std::ffi::c_int
                    }) != 0
                    {
                        ((*node).mn_flags as pgno_t)
                            << (if -(1 as std::ffi::c_int) as pgno_t
                                > 0xffffffff as std::ffi::c_uint as pgno_t
                            {
                                32 as std::ffi::c_int
                            } else {
                                0 as std::ffi::c_int
                            })
                    } else {
                        0 as std::ffi::c_int as pgno_t
                    }),
                &mut *(mn.mc_pg).as_mut_ptr().offset(mn.mc_top as isize),
                std::ptr::null_mut::<std::ffi::c_int>(),
            );
            if rc != 0 {
                return rc;
            }
            mn.mc_ki[mn.mc_top as usize] = 0 as std::ffi::c_int as indx_t;
            (*mc).mc_ki[(*mc).mc_top as usize] = (((*((*mc).mc_pg[(*mc).mc_top as usize]
                as *mut std::ffi::c_void
                as *mut MDB_page2))
                .mp2_lower as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int) as indx_t;
            fromleft = 0 as std::ffi::c_int;
        } else {
            mn.mc_ki[ptop as usize] = (mn.mc_ki[ptop as usize]).wrapping_sub(1);
            mn.mc_ki[ptop as usize];
            node = ((*mc).mc_pg[ptop as usize] as *mut std::ffi::c_char)
                .offset(
                    *((*((*mc).mc_pg[ptop as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_ptrs)
                        .as_mut_ptr()
                        .offset(mn.mc_ki[ptop as usize] as isize)
                        as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            rc = mdb_page_get(
                mc,
                (*node).mn_lo as pgno_t
                    | ((*node).mn_hi as pgno_t) << 16 as std::ffi::c_int
                    | (if (if -(1 as std::ffi::c_int) as pgno_t
                        > 0xffffffff as std::ffi::c_uint as pgno_t
                    {
                        32 as std::ffi::c_int
                    } else {
                        0 as std::ffi::c_int
                    }) != 0
                    {
                        ((*node).mn_flags as pgno_t)
                            << (if -(1 as std::ffi::c_int) as pgno_t
                                > 0xffffffff as std::ffi::c_uint as pgno_t
                            {
                                32 as std::ffi::c_int
                            } else {
                                0 as std::ffi::c_int
                            })
                    } else {
                        0 as std::ffi::c_int as pgno_t
                    }),
                &mut *(mn.mc_pg).as_mut_ptr().offset(mn.mc_top as isize),
                std::ptr::null_mut::<std::ffi::c_int>(),
            );
            if rc != 0 {
                return rc;
            }
            mn.mc_ki[mn.mc_top as usize] =
                (((*(mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                    .mp2_lower as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int)
                    .wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint)
                    as indx_t;
            (*mc).mc_ki[(*mc).mc_top as usize] = 0 as std::ffi::c_int as indx_t;
            fromleft = 1 as std::ffi::c_int;
        }
        if 1000 as std::ffi::c_long
            * ((*(*(*mc).mc_txn).mt_env).me_psize)
                .wrapping_sub(16 as std::ffi::c_ulong as std::ffi::c_uint)
                .wrapping_sub(
                    ((*(mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_upper as std::ffi::c_int
                        - (*(mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_void
                            as *mut MDB_page2))
                            .mp2_lower as std::ffi::c_int) as indx_t
                        as std::ffi::c_uint,
                ) as std::ffi::c_long
            / ((*(*(*mc).mc_txn).mt_env).me_psize)
                .wrapping_sub(16 as std::ffi::c_ulong as std::ffi::c_uint)
                as std::ffi::c_long
            >= thresh as std::ffi::c_long
            && ((*(mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                .mp2_lower as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int
                > minkeys
        {
            rc = mdb_node_move(&mut mn, mc, fromleft);
            if fromleft != 0 {
                oldki = oldki.wrapping_add(1);
            }
        } else {
            if fromleft == 0 {
                rc = mdb_page_merge(&mut mn, mc);
            } else {
                oldki = (oldki as std::ffi::c_uint).wrapping_add(
                    ((*(mn.mc_pg[mn.mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_lower as std::ffi::c_uint)
                        .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                            if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            },
                        ))
                        >> 1 as std::ffi::c_int,
                ) as indx_t as indx_t;
                mn.mc_ki[mn.mc_top as usize] = (mn.mc_ki[mn.mc_top as usize] as std::ffi::c_int
                    + ((*mc).mc_ki[mn.mc_top as usize] as std::ffi::c_int + 1 as std::ffi::c_int))
                    as indx_t;
                let mut dummy: MDB_cursor = MDB_cursor {
                    mc_next: std::ptr::null_mut::<MDB_cursor>(),
                    mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                    mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                    mc_txn: std::ptr::null_mut::<MDB_txn>(),
                    mc_dbi: 0,
                    mc_db: std::ptr::null_mut::<MDB_db>(),
                    mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                    mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                    mc_snum: 0,
                    mc_top: 0,
                    mc_flags: 0,
                    mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                    mc_ki: [0; 32],
                };
                let mut tracked: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                let mut tp: *mut *mut MDB_cursor = &mut *((*mn.mc_txn).mt_cursors)
                    .offset(mn.mc_dbi as isize)
                    as *mut *mut MDB_cursor;
                if mn.mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    dummy.mc_flags = 0x1 as std::ffi::c_int as std::ffi::c_uint;
                    dummy.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                    tracked = &mut dummy;
                } else {
                    tracked = &mut mn;
                }
                (*tracked).mc_next = *tp;
                *tp = tracked;
                rc = mdb_page_merge(mc, &mut mn);
                *tp = (*tracked).mc_next;
                mdb_cursor_copy(&mut mn, mc);
            }
            (*mc).mc_flags &= !(0x2 as std::ffi::c_int) as std::ffi::c_uint;
        }
        (*mc).mc_ki[(*mc).mc_top as usize] = oldki;
        rc
    }
}
unsafe extern "C" fn mdb_cursor_del0(mut mc: *mut MDB_cursor) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut rc: std::ffi::c_int = 0;
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut ki: indx_t = 0;
        let mut nkeys: std::ffi::c_uint = 0;
        let mut m2: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut m3: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut dbi: MDB_dbi = (*mc).mc_dbi;
        ki = (*mc).mc_ki[(*mc).mc_top as usize];
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        mdb_node_del(mc, (*(*mc).mc_db).md_pad as std::ffi::c_int);
        (*(*mc).mc_db).md_entries = ((*(*mc).mc_db).md_entries).wrapping_sub(1);
        (*(*mc).mc_db).md_entries;
        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
        while !m2.is_null() {
            m3 = if (*mc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                &mut (*(*m2).mc_xcursor).mx_cursor
            } else {
                m2
            };
            if ((*m2).mc_flags & (*m3).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0)
                && !(m3 == mc
                    || ((*m3).mc_snum as std::ffi::c_int) < (*mc).mc_snum as std::ffi::c_int)
                && (*m3).mc_pg[(*mc).mc_top as usize] == mp
            {
                if (*m3).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int == ki as std::ffi::c_int {
                    (*m3).mc_flags |= 0x8 as std::ffi::c_int as std::ffi::c_uint;
                    if (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x4 as std::ffi::c_int != 0 {
                        (*(*m3).mc_xcursor).mx_cursor.mc_flags &=
                            !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
                    }
                } else {
                    if (*m3).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int > ki as std::ffi::c_int
                    {
                        (*m3).mc_ki[(*mc).mc_top as usize] =
                            ((*m3).mc_ki[(*mc).mc_top as usize]).wrapping_sub(1);
                        (*m3).mc_ki[(*mc).mc_top as usize];
                    }
                    let mut xr_pg: *mut MDB_page = mp;
                    let mut xr_node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
                    if !(!(!((*m3).mc_xcursor).is_null()
                        && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                            & 0x1 as std::ffi::c_int as std::ffi::c_uint
                            != 0)
                        || (*m3).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint
                            >= ((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                                as std::ffi::c_uint)
                                .wrapping_sub(
                                    (16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                                        if 0 as std::ffi::c_int != 0 {
                                            16 as std::ffi::c_ulong as std::ffi::c_uint
                                        } else {
                                            0 as std::ffi::c_int as std::ffi::c_uint
                                        },
                                    ),
                                )
                                >> 1 as std::ffi::c_int)
                    {
                        xr_node = (xr_pg as *mut std::ffi::c_char)
                            .offset(
                                *((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset((*m3).mc_ki[(*mc).mc_top as usize] as isize)
                                    as std::ffi::c_int as isize,
                            )
                            .offset(
                                (if 0 as std::ffi::c_int != 0 {
                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                } else {
                                    0 as std::ffi::c_int as std::ffi::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if (*xr_node).mn_flags as std::ffi::c_int
                            & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                            == 0x4 as std::ffi::c_int
                        {
                            (*(*m3).mc_xcursor).mx_cursor.mc_pg[0 as std::ffi::c_int as usize] =
                                ((*xr_node).mn_data)
                                    .as_mut_ptr()
                                    .offset((*xr_node).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void
                                    as *mut MDB_page;
                        }
                    }
                }
            }
            m2 = (*m2).mc_next;
        }
        rc = mdb_rebalance(mc);
        if rc == 0 {
            if (*mc).mc_snum == 0 {
                (*mc).mc_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                return rc;
            }
            mp = (*mc).mc_pg[(*mc).mc_top as usize];
            nkeys = ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower
                as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int;
            m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
            loop {
                if !(rc == 0 && !m2.is_null()) {
                    current_block = 13281731871476506071;
                    break;
                }
                m3 = if (*mc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                    &mut (*(*m2).mc_xcursor).mx_cursor
                } else {
                    m2
                };
                if ((*m2).mc_flags & (*m3).mc_flags & 0x1 as std::ffi::c_int as std::ffi::c_uint
                    != 0)
                    && (((*m3).mc_snum as std::ffi::c_int) >= (*mc).mc_snum as std::ffi::c_int)
                    && (*m3).mc_pg[(*mc).mc_top as usize] == mp
                    && (*m3).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int
                        >= (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int
                {
                    if (*m3).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint >= nkeys {
                        rc = mdb_cursor_sibling(m3, 1 as std::ffi::c_int);
                        if rc == -(30798 as std::ffi::c_int) {
                            (*m3).mc_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                            rc = 0 as std::ffi::c_int;
                            current_block = 2569451025026770673;
                        } else {
                            if rc != 0 {
                                current_block = 5357753983577566164;
                                break;
                            }
                            current_block = 18435049525520518667;
                        }
                    } else {
                        current_block = 18435049525520518667;
                    }
                    match current_block {
                        2569451025026770673 => {}
                        _ => {
                            if !((*m3).mc_xcursor).is_null()
                                && (*m3).mc_flags & 0x2 as std::ffi::c_int as std::ffi::c_uint == 0
                            {
                                let mut node: *mut MDB_node =
                                    ((*m3).mc_pg[(*m3).mc_top as usize] as *mut std::ffi::c_char)
                                        .offset(
                                            *((*((*m3).mc_pg[(*m3).mc_top as usize]
                                                as *mut std::ffi::c_void
                                                as *mut MDB_page2))
                                                .mp2_ptrs)
                                                .as_mut_ptr()
                                                .offset((*m3).mc_ki[(*m3).mc_top as usize] as isize)
                                                as std::ffi::c_int
                                                as isize,
                                        )
                                        .offset(
                                            (if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            }) as isize,
                                        ) as *mut MDB_node;
                                if (*node).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int != 0
                                {
                                    if (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                        & 0x1 as std::ffi::c_int as std::ffi::c_uint
                                        != 0
                                    {
                                        if (*node).mn_flags as std::ffi::c_int
                                            & 0x2 as std::ffi::c_int
                                            == 0
                                        {
                                            (*(*m3).mc_xcursor).mx_cursor.mc_pg
                                                [0 as std::ffi::c_int as usize] =
                                                ((*node).mn_data).as_mut_ptr().offset(
                                                    (*node).mn_ksize as std::ffi::c_int as isize,
                                                )
                                                    as *mut std::ffi::c_void
                                                    as *mut MDB_page;
                                        }
                                    } else {
                                        mdb_xcursor_init1(m3, node);
                                        rc = mdb_cursor_first(
                                            &mut (*(*m3).mc_xcursor).mx_cursor,
                                            std::ptr::null_mut::<MDB_val>(),
                                            std::ptr::null_mut::<MDB_val>(),
                                        );
                                        if rc != 0 {
                                            current_block = 5357753983577566164;
                                            break;
                                        }
                                    }
                                }
                                (*(*m3).mc_xcursor).mx_cursor.mc_flags |=
                                    0x8 as std::ffi::c_int as std::ffi::c_uint;
                            }
                        }
                    }
                }
                m2 = (*m2).mc_next;
            }
            match current_block {
                5357753983577566164 => {}
                _ => {
                    (*mc).mc_flags |= 0x8 as std::ffi::c_int as std::ffi::c_uint;
                }
            }
        }
        if rc != 0 {
            (*(*mc).mc_txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
        }
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_del(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
) -> std::ffi::c_int {
    unsafe {
        if key.is_null()
            || !(!txn.is_null()
                && dbi < (*txn).mt_numdbs
                && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                    & 0x10 as std::ffi::c_int
                    != 0)
        {
            return 22 as std::ffi::c_int;
        }
        if (*txn).mt_flags
            & (0x20000 as std::ffi::c_int
                | (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int))
                as std::ffi::c_uint
            != 0
        {
            return if (*txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                13 as std::ffi::c_int
            } else {
                -(30782 as std::ffi::c_int)
            };
        }
        if (*((*txn).mt_dbs).offset(dbi as isize)).md_flags as std::ffi::c_int
            & 0x4 as std::ffi::c_int
            != 0x4 as std::ffi::c_int
        {
            data = std::ptr::null_mut::<MDB_val>();
        }
        mdb_del0(txn, dbi, key, data, 0 as std::ffi::c_int as std::ffi::c_uint)
    }
}
unsafe extern "C" fn mdb_del0(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        let mut mc: MDB_cursor = MDB_cursor {
            mc_next: std::ptr::null_mut::<MDB_cursor>(),
            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
            mc_txn: std::ptr::null_mut::<MDB_txn>(),
            mc_dbi: 0,
            mc_db: std::ptr::null_mut::<MDB_db>(),
            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
            mc_ki: [0; 32],
        };
        let mut mx: MDB_xcursor = MDB_xcursor {
            mx_cursor: MDB_cursor {
                mc_next: std::ptr::null_mut::<MDB_cursor>(),
                mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                mc_dbi: 0,
                mc_db: std::ptr::null_mut::<MDB_db>(),
                mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                mc_ki: [0; 32],
            },
            mx_db: MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            },
            mx_dbx: MDB_dbx {
                md_name: MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() },
                md_cmp: None,
                md_dcmp: None,
                md_rel: None,
                md_relctx: std::ptr::null_mut::<std::ffi::c_void>(),
            },
            mx_dbflag: 0,
        };
        let mut op: MDB_cursor_op = MDB_FIRST;
        let mut rdata: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut xdata: *mut MDB_val = std::ptr::null_mut::<MDB_val>();
        let mut rc: std::ffi::c_int = 0;
        let mut exact: std::ffi::c_int = 0 as std::ffi::c_int;
        mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
        if !data.is_null() {
            op = MDB_GET_BOTH;
            rdata = *data;
            xdata = &mut rdata;
        } else {
            op = MDB_SET;
            xdata = std::ptr::null_mut::<MDB_val>();
            flags |= 0x20 as std::ffi::c_int as std::ffi::c_uint;
        }
        rc = mdb_cursor_set(&mut mc, key, xdata, op, &mut exact);
        if rc == 0 as std::ffi::c_int {
            mc.mc_next = *((*txn).mt_cursors).offset(dbi as isize);
            let fresh67 = &mut (*((*txn).mt_cursors).offset(dbi as isize));
            *fresh67 = &mut mc;
            rc = _mdb_cursor_del(&mut mc, flags);
            let fresh68 = &mut (*((*txn).mt_cursors).offset(dbi as isize));
            *fresh68 = mc.mc_next;
        }
        rc
    }
}
unsafe extern "C" fn mdb_page_split(
    mut mc: *mut MDB_cursor,
    mut newkey: *mut MDB_val,
    mut newdata: *mut MDB_val,
    mut newpgno: pgno_t,
    mut nflags: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut flags: std::ffi::c_uint = 0;
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut new_root: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut did_split: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut newindx: indx_t = 0;
        let mut pgno: pgno_t = 0 as std::ffi::c_int as pgno_t;
        let mut i: std::ffi::c_int = 0;
        let mut j: std::ffi::c_int = 0;
        let mut split_indx: std::ffi::c_int = 0;
        let mut nkeys: std::ffi::c_int = 0;
        let mut pmax: std::ffi::c_int = 0;
        let mut env: *mut MDB_env = (*(*mc).mc_txn).mt_env;
        let mut node: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut sepkey: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut rkey: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut xdata: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut rdata: *mut MDB_val = &mut xdata;
        let mut copy: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut rp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut pp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut ptop: std::ffi::c_int = 0;
        let mut mn: MDB_cursor = MDB_cursor {
            mc_next: std::ptr::null_mut::<MDB_cursor>(),
            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
            mc_txn: std::ptr::null_mut::<MDB_txn>(),
            mc_dbi: 0,
            mc_db: std::ptr::null_mut::<MDB_db>(),
            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
            mc_ki: [0; 32],
        };
        mp = (*mc).mc_pg[(*mc).mc_top as usize];
        newindx = (*mc).mc_ki[(*mc).mc_top as usize];
        nkeys = (((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_uint)
            .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                if 0 as std::ffi::c_int != 0 {
                    16 as std::ffi::c_ulong as std::ffi::c_uint
                } else {
                    0 as std::ffi::c_int as std::ffi::c_uint
                },
            ))
            >> 1 as std::ffi::c_int) as std::ffi::c_int;
        rc = mdb_page_new(mc, (*mp).mp_flags as uint32_t, 1 as std::ffi::c_int, &mut rp);
        if rc != 0 {
            return rc;
        }
        (*rp).mp_pad = (*mp).mp_pad;
        if ((*mc).mc_top as std::ffi::c_int) < 1 as std::ffi::c_int {
            rc =
                mdb_page_new(mc, 0x1 as std::ffi::c_int as uint32_t, 1 as std::ffi::c_int, &mut pp);
            if rc != 0 {
                current_block = 4951901818287548553;
            } else {
                i = (*mc).mc_snum as std::ffi::c_int;
                while i > 0 as std::ffi::c_int {
                    (*mc).mc_pg[i as usize] = (*mc).mc_pg[(i - 1 as std::ffi::c_int) as usize];
                    (*mc).mc_ki[i as usize] = (*mc).mc_ki[(i - 1 as std::ffi::c_int) as usize];
                    i -= 1;
                }
                (*mc).mc_pg[0 as std::ffi::c_int as usize] = pp;
                (*mc).mc_ki[0 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as indx_t;
                (*(*mc).mc_db).md_root = (*pp).mp_p.p_pgno;
                let fresh69 = (*(*mc).mc_db).md_depth;
                (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_add(1);
                new_root = fresh69 as std::ffi::c_int;
                rc = mdb_node_add(
                    mc,
                    0 as std::ffi::c_int as indx_t,
                    std::ptr::null_mut::<MDB_val>(),
                    std::ptr::null_mut::<MDB_val>(),
                    (*mp).mp_p.p_pgno,
                    0 as std::ffi::c_int as std::ffi::c_uint,
                );
                if rc != 0 as std::ffi::c_int {
                    (*mc).mc_pg[0 as std::ffi::c_int as usize] =
                        (*mc).mc_pg[1 as std::ffi::c_int as usize];
                    (*mc).mc_ki[0 as std::ffi::c_int as usize] =
                        (*mc).mc_ki[1 as std::ffi::c_int as usize];
                    (*(*mc).mc_db).md_root = (*mp).mp_p.p_pgno;
                    (*(*mc).mc_db).md_depth = ((*(*mc).mc_db).md_depth).wrapping_sub(1);
                    (*(*mc).mc_db).md_depth;
                    current_block = 4951901818287548553;
                } else {
                    (*mc).mc_snum = ((*mc).mc_snum).wrapping_add(1);
                    (*mc).mc_snum;
                    (*mc).mc_top = ((*mc).mc_top).wrapping_add(1);
                    (*mc).mc_top;
                    ptop = 0 as std::ffi::c_int;
                    current_block = 15925075030174552612;
                }
            }
        } else {
            ptop = (*mc).mc_top as std::ffi::c_int - 1 as std::ffi::c_int;
            current_block = 15925075030174552612;
        }
        if current_block == 15925075030174552612 {
            mdb_cursor_copy(mc, &mut mn);
            mn.mc_xcursor = std::ptr::null_mut::<MDB_xcursor>();
            mn.mc_pg[mn.mc_top as usize] = rp;
            mn.mc_ki[ptop as usize] =
                ((*mc).mc_ki[ptop as usize] as std::ffi::c_int + 1 as std::ffi::c_int) as indx_t;
            if nflags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                mn.mc_ki[mn.mc_top as usize] = 0 as std::ffi::c_int as indx_t;
                sepkey = *newkey;
                split_indx = newindx as std::ffi::c_int;
                nkeys = 0 as std::ffi::c_int;
                current_block = 4804377075063615140;
            } else {
                split_indx = (nkeys + 1 as std::ffi::c_int) / 2 as std::ffi::c_int;
                if (*(rp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                    & 0x20 as std::ffi::c_int
                    == 0x20 as std::ffi::c_int
                {
                    let mut split: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
                    let mut ins: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
                    let mut x: std::ffi::c_int = 0;
                    let mut lsize: std::ffi::c_uint = 0;
                    let mut rsize: std::ffi::c_uint = 0;
                    let mut ksize: std::ffi::c_uint = 0;
                    x = (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int - split_indx;
                    ksize = (*(*mc).mc_db).md_pad;
                    split = (mp as *mut std::ffi::c_char)
                        .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                        .offset((split_indx as std::ffi::c_uint).wrapping_mul(ksize) as isize);
                    rsize = ((nkeys - split_indx) as std::ffi::c_uint).wrapping_mul(ksize);
                    lsize = ((nkeys - split_indx) as std::ffi::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
                        as std::ffi::c_uint;
                    (*mp).mp_pb.pb.pb_lower = ((*mp).mp_pb.pb.pb_lower as std::ffi::c_uint)
                        .wrapping_sub(lsize) as indx_t
                        as indx_t;
                    (*rp).mp_pb.pb.pb_lower = ((*rp).mp_pb.pb.pb_lower as std::ffi::c_uint)
                        .wrapping_add(lsize) as indx_t
                        as indx_t;
                    (*mp).mp_pb.pb.pb_upper = ((*mp).mp_pb.pb.pb_upper as std::ffi::c_uint)
                        .wrapping_add(rsize.wrapping_sub(lsize))
                        as indx_t as indx_t;
                    (*rp).mp_pb.pb.pb_upper = ((*rp).mp_pb.pb.pb_upper as std::ffi::c_uint)
                        .wrapping_sub(rsize.wrapping_sub(lsize))
                        as indx_t as indx_t;
                    sepkey.mv_size = ksize as size_t;
                    if newindx as std::ffi::c_int == split_indx {
                        sepkey.mv_data = (*newkey).mv_data;
                    } else {
                        sepkey.mv_data = split as *mut std::ffi::c_void;
                    }
                    if x < 0 as std::ffi::c_int {
                        ins = (mp as *mut std::ffi::c_char)
                            .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                            .offset(
                                ((*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint)
                                    .wrapping_mul(ksize) as isize,
                            );
                        memcpy(
                            ((*rp).mp_ptrs).as_mut_ptr() as *mut std::ffi::c_void,
                            split as *const std::ffi::c_void,
                            rsize as std::ffi::c_ulong,
                        );
                        sepkey.mv_data = ((*rp).mp_ptrs).as_mut_ptr() as *mut std::ffi::c_void;
                        memmove(
                            ins.offset(ksize as isize) as *mut std::ffi::c_void,
                            ins as *const std::ffi::c_void,
                            ((split_indx - (*mc).mc_ki[(*mc).mc_top as usize] as std::ffi::c_int)
                                as std::ffi::c_uint)
                                .wrapping_mul(ksize)
                                as std::ffi::c_ulong,
                        );
                        memcpy(
                            ins as *mut std::ffi::c_void,
                            (*newkey).mv_data,
                            ksize as std::ffi::c_ulong,
                        );
                        (*mp).mp_pb.pb.pb_lower = ((*mp).mp_pb.pb.pb_lower as std::ffi::c_ulong)
                            .wrapping_add(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
                            as indx_t as indx_t;
                        (*mp).mp_pb.pb.pb_upper =
                            ((*mp).mp_pb.pb.pb_upper as std::ffi::c_ulong).wrapping_sub(
                                (ksize as std::ffi::c_ulong).wrapping_sub(::core::mem::size_of::<
                                    indx_t,
                                >(
                                )
                                    as std::ffi::c_ulong),
                            ) as indx_t as indx_t;
                    } else {
                        if x != 0 {
                            memcpy(
                                ((*rp).mp_ptrs).as_mut_ptr() as *mut std::ffi::c_void,
                                split as *const std::ffi::c_void,
                                (x as std::ffi::c_uint).wrapping_mul(ksize) as std::ffi::c_ulong,
                            );
                        }
                        ins = (rp as *mut std::ffi::c_char)
                            .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                            .offset((x as std::ffi::c_uint).wrapping_mul(ksize) as isize);
                        memcpy(
                            ins as *mut std::ffi::c_void,
                            (*newkey).mv_data,
                            ksize as std::ffi::c_ulong,
                        );
                        memcpy(
                            ins.offset(ksize as isize) as *mut std::ffi::c_void,
                            split.offset((x as std::ffi::c_uint).wrapping_mul(ksize) as isize)
                                as *const std::ffi::c_void,
                            rsize.wrapping_sub((x as std::ffi::c_uint).wrapping_mul(ksize))
                                as std::ffi::c_ulong,
                        );
                        (*rp).mp_pb.pb.pb_lower = ((*rp).mp_pb.pb.pb_lower as std::ffi::c_ulong)
                            .wrapping_add(::core::mem::size_of::<indx_t>() as std::ffi::c_ulong)
                            as indx_t as indx_t;
                        (*rp).mp_pb.pb.pb_upper =
                            ((*rp).mp_pb.pb.pb_upper as std::ffi::c_ulong).wrapping_sub(
                                (ksize as std::ffi::c_ulong).wrapping_sub(::core::mem::size_of::<
                                    indx_t,
                                >(
                                )
                                    as std::ffi::c_ulong),
                            ) as indx_t as indx_t;
                        (*mc).mc_ki[(*mc).mc_top as usize] = x as indx_t;
                    }
                    current_block = 4804377075063615140;
                } else {
                    let mut psize: std::ffi::c_int = 0;
                    let mut nsize: std::ffi::c_int = 0;
                    let mut k: std::ffi::c_int = 0;
                    let mut keythresh: std::ffi::c_int = 0;
                    pmax = ((*env).me_psize)
                        .wrapping_sub(16 as std::ffi::c_ulong as std::ffi::c_uint)
                        as std::ffi::c_int;
                    keythresh = ((*env).me_psize >> 7 as std::ffi::c_int) as std::ffi::c_int;
                    if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                        as std::ffi::c_int
                        & 0x2 as std::ffi::c_int
                        == 0x2 as std::ffi::c_int
                    {
                        nsize = mdb_leaf_size(env, newkey, newdata) as std::ffi::c_int;
                    } else {
                        nsize = mdb_branch_size(env, newkey) as std::ffi::c_int;
                    }
                    nsize = ((nsize as std::ffi::c_uint).wrapping_add(1 as std::ffi::c_uint)
                        & -(2 as std::ffi::c_int) as std::ffi::c_uint)
                        as std::ffi::c_int;
                    copy = mdb_page_malloc((*mc).mc_txn, 1 as std::ffi::c_int as std::ffi::c_uint);
                    if copy.is_null() {
                        rc = 12 as std::ffi::c_int;
                        current_block = 4951901818287548553;
                    } else {
                        (*copy).mp_p.p_pgno = (*mp).mp_p.p_pgno;
                        (*copy).mp_flags = (*mp).mp_flags;
                        (*copy).mp_pb.pb.pb_lower = (16 as std::ffi::c_ulong as std::ffi::c_uint)
                            .wrapping_sub(if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            }) as indx_t;
                        (*copy).mp_pb.pb.pb_upper =
                            ((*env).me_psize).wrapping_sub(if 0 as std::ffi::c_int != 0 {
                                16 as std::ffi::c_ulong as std::ffi::c_uint
                            } else {
                                0 as std::ffi::c_int as std::ffi::c_uint
                            }) as indx_t;
                        i = 0 as std::ffi::c_int;
                        j = 0 as std::ffi::c_int;
                        while i < nkeys {
                            if i == newindx as std::ffi::c_int {
                                let fresh70 = j;
                                j += 1;
                                *((*copy).mp_ptrs).as_mut_ptr().offset(fresh70 as isize) =
                                    0 as std::ffi::c_int as indx_t;
                            }
                            let fresh71 = j;
                            j += 1;
                            *((*copy).mp_ptrs).as_mut_ptr().offset(fresh71 as isize) =
                                *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize);
                            i += 1;
                        }
                        if nkeys < keythresh
                            || nsize > pmax / 16 as std::ffi::c_int
                            || newindx as std::ffi::c_int >= nkeys
                        {
                            psize = 0 as std::ffi::c_int;
                            if newindx as std::ffi::c_int <= split_indx
                                || newindx as std::ffi::c_int >= nkeys
                            {
                                i = 0 as std::ffi::c_int;
                                j = 1 as std::ffi::c_int;
                                k = if newindx as std::ffi::c_int >= nkeys {
                                    nkeys
                                } else {
                                    split_indx
                                        + 1 as std::ffi::c_int
                                        + ((*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                            .mp2_flags
                                            as std::ffi::c_int
                                            & 0x2 as std::ffi::c_int
                                            == 0x2 as std::ffi::c_int)
                                            as std::ffi::c_int
                                };
                            } else {
                                i = nkeys;
                                j = -(1 as std::ffi::c_int);
                                k = split_indx - 1 as std::ffi::c_int;
                            }
                            while i != k {
                                if i == newindx as std::ffi::c_int {
                                    psize += nsize;
                                    node = std::ptr::null_mut::<MDB_node>();
                                } else {
                                    node = (mp as *mut std::ffi::c_char)
                                        .offset(*((*copy).mp_ptrs).as_mut_ptr().offset(i as isize)
                                            as std::ffi::c_int
                                            as isize)
                                        .offset(
                                            (if 0 as std::ffi::c_int != 0 {
                                                16 as std::ffi::c_ulong as std::ffi::c_uint
                                            } else {
                                                0 as std::ffi::c_int as std::ffi::c_uint
                                            }) as isize,
                                        )
                                        as *mut MDB_node;
                                    psize = (psize as std::ffi::c_ulong).wrapping_add(
                                        (8 as std::ffi::c_ulong)
                                            .wrapping_add((*node).mn_ksize as std::ffi::c_ulong)
                                            .wrapping_add(::core::mem::size_of::<indx_t>()
                                                as std::ffi::c_ulong),
                                    ) as std::ffi::c_int
                                        as std::ffi::c_int;
                                    if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                                        as std::ffi::c_int
                                        & 0x2 as std::ffi::c_int
                                        == 0x2 as std::ffi::c_int
                                    {
                                        if (*node).mn_flags as std::ffi::c_int
                                            & 0x1 as std::ffi::c_int
                                            == 0x1 as std::ffi::c_int
                                        {
                                            psize = (psize as std::ffi::c_ulong)
                                                .wrapping_add(::core::mem::size_of::<pgno_t>()
                                                    as std::ffi::c_ulong)
                                                as std::ffi::c_int
                                                as std::ffi::c_int;
                                        } else {
                                            psize = (psize as std::ffi::c_uint).wrapping_add(
                                                (*node).mn_lo as std::ffi::c_uint
                                                    | ((*node).mn_hi as std::ffi::c_uint)
                                                        << 16 as std::ffi::c_int,
                                            )
                                                as std::ffi::c_int
                                                as std::ffi::c_int;
                                        }
                                    }
                                    psize = ((psize as std::ffi::c_uint)
                                        .wrapping_add(1 as std::ffi::c_uint)
                                        & -(2 as std::ffi::c_int) as std::ffi::c_uint)
                                        as std::ffi::c_int;
                                }
                                if psize > pmax || i == k - j {
                                    split_indx = i + (j < 0 as std::ffi::c_int) as std::ffi::c_int;
                                    break;
                                } else {
                                    i += j;
                                }
                            }
                        }
                        if split_indx == newindx as std::ffi::c_int {
                            sepkey.mv_size = (*newkey).mv_size;
                            sepkey.mv_data = (*newkey).mv_data;
                        } else {
                            node = (mp as *mut std::ffi::c_char)
                                .offset(*((*copy).mp_ptrs).as_mut_ptr().offset(split_indx as isize)
                                    as std::ffi::c_int
                                    as isize)
                                .offset(
                                    (if 0 as std::ffi::c_int != 0 {
                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                    } else {
                                        0 as std::ffi::c_int as std::ffi::c_uint
                                    }) as isize,
                                ) as *mut MDB_node;
                            sepkey.mv_size = (*node).mn_ksize as size_t;
                            sepkey.mv_data =
                                ((*node).mn_data).as_mut_ptr() as *mut std::ffi::c_void;
                        }
                        current_block = 4804377075063615140;
                    }
                }
            }
            match current_block {
                4951901818287548553 => {}
                _ => {
                    if (((*(mn.mc_pg[ptop as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_upper as std::ffi::c_int
                        - (*(mn.mc_pg[ptop as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                            .mp2_lower as std::ffi::c_int) as indx_t
                        as size_t)
                        < mdb_branch_size(env, &mut sepkey)
                    {
                        let mut snum: std::ffi::c_int = (*mc).mc_snum as std::ffi::c_int;
                        mn.mc_snum = (mn.mc_snum).wrapping_sub(1);
                        mn.mc_snum;
                        mn.mc_top = (mn.mc_top).wrapping_sub(1);
                        mn.mc_top;
                        did_split = 1 as std::ffi::c_int;
                        let mut dummy: MDB_cursor = MDB_cursor {
                            mc_next: std::ptr::null_mut::<MDB_cursor>(),
                            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                            mc_txn: std::ptr::null_mut::<MDB_txn>(),
                            mc_dbi: 0,
                            mc_db: std::ptr::null_mut::<MDB_db>(),
                            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                            mc_snum: 0,
                            mc_top: 0,
                            mc_flags: 0,
                            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                            mc_ki: [0; 32],
                        };
                        let mut tracked: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
                        let mut tp: *mut *mut MDB_cursor = &mut *((*mn.mc_txn).mt_cursors)
                            .offset(mn.mc_dbi as isize)
                            as *mut *mut MDB_cursor;
                        if mn.mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                            dummy.mc_flags = 0x1 as std::ffi::c_int as std::ffi::c_uint;
                            dummy.mc_xcursor = &mut mn as *mut MDB_cursor as *mut MDB_xcursor;
                            tracked = &mut dummy;
                        } else {
                            tracked = &mut mn;
                        }
                        (*tracked).mc_next = *tp;
                        *tp = tracked;
                        rc = mdb_page_split(
                            &mut mn,
                            &mut sepkey,
                            std::ptr::null_mut::<MDB_val>(),
                            (*rp).mp_p.p_pgno,
                            0 as std::ffi::c_int as std::ffi::c_uint,
                        );
                        *tp = (*tracked).mc_next;
                        if rc != 0 {
                            current_block = 4951901818287548553;
                        } else {
                            if (*mc).mc_snum as std::ffi::c_int > snum {
                                ptop += 1;
                            }
                            if mn.mc_pg[ptop as usize] != (*mc).mc_pg[ptop as usize]
                                && (*mc).mc_ki[ptop as usize] as std::ffi::c_uint
                                    >= ((*((*mc).mc_pg[ptop as usize] as *mut std::ffi::c_void
                                        as *mut MDB_page2))
                                        .mp2_lower
                                        as std::ffi::c_uint)
                                        .wrapping_sub(
                                            (16 as std::ffi::c_ulong as std::ffi::c_uint)
                                                .wrapping_sub(if 0 as std::ffi::c_int != 0 {
                                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                                } else {
                                                    0 as std::ffi::c_int as std::ffi::c_uint
                                                }),
                                        )
                                        >> 1 as std::ffi::c_int
                            {
                                i = 0 as std::ffi::c_int;
                                while i < ptop {
                                    (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                    (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                    i += 1;
                                }
                                (*mc).mc_pg[ptop as usize] = mn.mc_pg[ptop as usize];
                                if mn.mc_ki[ptop as usize] != 0 {
                                    (*mc).mc_ki[ptop as usize] = (mn.mc_ki[ptop as usize]
                                        as std::ffi::c_int
                                        - 1 as std::ffi::c_int)
                                        as indx_t;
                                } else {
                                    (*mc).mc_ki[ptop as usize] = mn.mc_ki[ptop as usize];
                                    rc = mdb_cursor_sibling(mc, 0 as std::ffi::c_int);
                                }
                            }
                            current_block = 5431927413890720344;
                        }
                    } else {
                        mn.mc_top = (mn.mc_top).wrapping_sub(1);
                        mn.mc_top;
                        rc = mdb_node_add(
                            &mut mn,
                            mn.mc_ki[ptop as usize],
                            &mut sepkey,
                            std::ptr::null_mut::<MDB_val>(),
                            (*rp).mp_p.p_pgno,
                            0 as std::ffi::c_int as std::ffi::c_uint,
                        );
                        mn.mc_top = (mn.mc_top).wrapping_add(1);
                        mn.mc_top;
                        current_block = 5431927413890720344;
                    }
                    match current_block {
                        4951901818287548553 => {}
                        _ => {
                            if rc != 0 as std::ffi::c_int {
                                if rc == -(30798 as std::ffi::c_int) {
                                    rc = -(30779 as std::ffi::c_int);
                                }
                            } else {
                                if nflags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                                    (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                    (*mc).mc_ki[(*mc).mc_top as usize] =
                                        0 as std::ffi::c_int as indx_t;
                                    rc = mdb_node_add(
                                        mc,
                                        0 as std::ffi::c_int as indx_t,
                                        newkey,
                                        newdata,
                                        newpgno,
                                        nflags,
                                    );
                                    if rc != 0 {
                                        current_block = 4951901818287548553;
                                    } else {
                                        i = 0 as std::ffi::c_int;
                                        while i < (*mc).mc_top as std::ffi::c_int {
                                            (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                            i += 1;
                                        }
                                        current_block = 6091595930016798176;
                                    }
                                } else if (*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                    .mp2_flags
                                    as std::ffi::c_int
                                    & 0x20 as std::ffi::c_int
                                    != 0x20 as std::ffi::c_int
                                {
                                    (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                    i = split_indx;
                                    j = 0 as std::ffi::c_int;
                                    loop {
                                        if i == newindx as std::ffi::c_int {
                                            rkey.mv_data = (*newkey).mv_data;
                                            rkey.mv_size = (*newkey).mv_size;
                                            if (*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                                .mp2_flags
                                                as std::ffi::c_int
                                                & 0x2 as std::ffi::c_int
                                                == 0x2 as std::ffi::c_int
                                            {
                                                rdata = newdata;
                                            } else {
                                                pgno = newpgno;
                                            }
                                            flags = nflags;
                                            (*mc).mc_ki[(*mc).mc_top as usize] = j as indx_t;
                                        } else {
                                            node = (mp as *mut std::ffi::c_char)
                                                .offset(
                                                    *((*copy).mp_ptrs)
                                                        .as_mut_ptr()
                                                        .offset(i as isize)
                                                        as std::ffi::c_int
                                                        as isize,
                                                )
                                                .offset(
                                                    (if 0 as std::ffi::c_int != 0 {
                                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                                    } else {
                                                        0 as std::ffi::c_int as std::ffi::c_uint
                                                    })
                                                        as isize,
                                                )
                                                as *mut MDB_node;
                                            rkey.mv_data = ((*node).mn_data).as_mut_ptr()
                                                as *mut std::ffi::c_void;
                                            rkey.mv_size = (*node).mn_ksize as size_t;
                                            if (*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                                .mp2_flags
                                                as std::ffi::c_int
                                                & 0x2 as std::ffi::c_int
                                                == 0x2 as std::ffi::c_int
                                            {
                                                xdata.mv_data =
                                                    ((*node).mn_data).as_mut_ptr().offset(
                                                        (*node).mn_ksize as std::ffi::c_int
                                                            as isize,
                                                    )
                                                        as *mut std::ffi::c_void;
                                                xdata.mv_size = ((*node).mn_lo as std::ffi::c_uint
                                                    | ((*node).mn_hi as std::ffi::c_uint)
                                                        << 16 as std::ffi::c_int)
                                                    as size_t;
                                                rdata = &mut xdata;
                                            } else {
                                                pgno = (*node).mn_lo as pgno_t
                                                    | ((*node).mn_hi as pgno_t)
                                                        << 16 as std::ffi::c_int
                                                    | (if (if -(1 as std::ffi::c_int) as pgno_t
                                                        > 0xffffffff as std::ffi::c_uint as pgno_t
                                                    {
                                                        32 as std::ffi::c_int
                                                    } else {
                                                        0 as std::ffi::c_int
                                                    }) != 0
                                                    {
                                                        ((*node).mn_flags as pgno_t)
                                                            << (if -(1 as std::ffi::c_int) as pgno_t
                                                                > 0xffffffff as std::ffi::c_uint
                                                                    as pgno_t
                                                            {
                                                                32 as std::ffi::c_int
                                                            } else {
                                                                0 as std::ffi::c_int
                                                            })
                                                    } else {
                                                        0 as std::ffi::c_int as pgno_t
                                                    });
                                            }
                                            flags = (*node).mn_flags as std::ffi::c_uint;
                                        }
                                        if ((*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                            .mp2_flags
                                            as std::ffi::c_int
                                            & 0x2 as std::ffi::c_int
                                            != 0x2 as std::ffi::c_int)
                                            && j == 0 as std::ffi::c_int
                                        {
                                            rkey.mv_size = 0 as std::ffi::c_int as size_t;
                                        }
                                        rc = mdb_node_add(
                                            mc,
                                            j as indx_t,
                                            &mut rkey,
                                            rdata,
                                            pgno,
                                            flags,
                                        );
                                        if rc != 0 {
                                            current_block = 4951901818287548553;
                                            break;
                                        }
                                        if i == nkeys {
                                            i = 0 as std::ffi::c_int;
                                            j = 0 as std::ffi::c_int;
                                            (*mc).mc_pg[(*mc).mc_top as usize] = copy;
                                        } else {
                                            i += 1;
                                            j += 1;
                                        }
                                        if i == split_indx {
                                            current_block = 13598848910332274892;
                                            break;
                                        }
                                    }
                                    match current_block {
                                        4951901818287548553 => {}
                                        _ => {
                                            nkeys = (((*(copy as *mut std::ffi::c_void
                                                as *mut MDB_page2))
                                                .mp2_lower
                                                as std::ffi::c_uint)
                                                .wrapping_sub(
                                                    (16 as std::ffi::c_ulong as std::ffi::c_uint)
                                                        .wrapping_sub(
                                                            if 0 as std::ffi::c_int != 0 {
                                                                16 as std::ffi::c_ulong
                                                                    as std::ffi::c_uint
                                                            } else {
                                                                0 as std::ffi::c_int
                                                                    as std::ffi::c_uint
                                                            },
                                                        ),
                                                )
                                                >> 1 as std::ffi::c_int)
                                                as std::ffi::c_int;
                                            i = 0 as std::ffi::c_int;
                                            while i < nkeys {
                                                *((*mp).mp_ptrs).as_mut_ptr().offset(i as isize) =
                                                    *((*copy).mp_ptrs)
                                                        .as_mut_ptr()
                                                        .offset(i as isize);
                                                i += 1;
                                            }
                                            (*mp).mp_pb.pb.pb_lower = (*copy).mp_pb.pb.pb_lower;
                                            (*mp).mp_pb.pb.pb_upper = (*copy).mp_pb.pb.pb_upper;
                                            memcpy(
                                                (mp as *mut std::ffi::c_char)
                                                    .offset(
                                                        *((*(mp as *mut std::ffi::c_void
                                                            as *mut MDB_page2))
                                                            .mp2_ptrs)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                (nkeys - 1 as std::ffi::c_int)
                                                                    as isize,
                                                            )
                                                            as std::ffi::c_int
                                                            as isize,
                                                    )
                                                    .offset(
                                                        (if 0 as std::ffi::c_int != 0 {
                                                            16 as std::ffi::c_ulong
                                                                as std::ffi::c_uint
                                                        } else {
                                                            0 as std::ffi::c_int as std::ffi::c_uint
                                                        })
                                                            as isize,
                                                    )
                                                    as *mut MDB_node
                                                    as *mut std::ffi::c_void,
                                                (copy as *mut std::ffi::c_char)
                                                    .offset(
                                                        *((*(copy as *mut std::ffi::c_void
                                                            as *mut MDB_page2))
                                                            .mp2_ptrs)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                (nkeys - 1 as std::ffi::c_int)
                                                                    as isize,
                                                            )
                                                            as std::ffi::c_int
                                                            as isize,
                                                    )
                                                    .offset(
                                                        (if 0 as std::ffi::c_int != 0 {
                                                            16 as std::ffi::c_ulong
                                                                as std::ffi::c_uint
                                                        } else {
                                                            0 as std::ffi::c_int as std::ffi::c_uint
                                                        })
                                                            as isize,
                                                    )
                                                    as *mut MDB_node
                                                    as *const std::ffi::c_void,
                                                ((*env).me_psize)
                                                    .wrapping_sub(
                                                        (*copy).mp_pb.pb.pb_upper
                                                            as std::ffi::c_uint,
                                                    )
                                                    .wrapping_sub(if 0 as std::ffi::c_int != 0 {
                                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                                    } else {
                                                        0 as std::ffi::c_int as std::ffi::c_uint
                                                    })
                                                    as std::ffi::c_ulong,
                                            );
                                            if (newindx as std::ffi::c_int) < split_indx {
                                                (*mc).mc_pg[(*mc).mc_top as usize] = mp;
                                            } else {
                                                (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                                (*mc).mc_ki[ptop as usize] =
                                                    ((*mc).mc_ki[ptop as usize]).wrapping_add(1);
                                                (*mc).mc_ki[ptop as usize];
                                                if mn.mc_pg[ptop as usize]
                                                    != (*mc).mc_pg[ptop as usize]
                                                    && (*mc).mc_ki[ptop as usize]
                                                        as std::ffi::c_uint
                                                        >= ((*((*mc).mc_pg[ptop as usize]
                                                            as *mut std::ffi::c_void
                                                            as *mut MDB_page2))
                                                            .mp2_lower
                                                            as std::ffi::c_uint)
                                                            .wrapping_sub(
                                                                (16 as std::ffi::c_ulong
                                                                    as std::ffi::c_uint)
                                                                    .wrapping_sub(
                                                                        if 0 as std::ffi::c_int != 0
                                                                        {
                                                                            16 as std::ffi::c_ulong
                                                                                as std::ffi::c_uint
                                                                        } else {
                                                                            0 as std::ffi::c_int
                                                                                as std::ffi::c_uint
                                                                        },
                                                                    ),
                                                            )
                                                            >> 1 as std::ffi::c_int
                                                {
                                                    i = 0 as std::ffi::c_int;
                                                    while i <= ptop {
                                                        (*mc).mc_pg[i as usize] =
                                                            mn.mc_pg[i as usize];
                                                        (*mc).mc_ki[i as usize] =
                                                            mn.mc_ki[i as usize];
                                                        i += 1;
                                                    }
                                                }
                                            }
                                            if nflags
                                                & 0x10000 as std::ffi::c_int as std::ffi::c_uint
                                                != 0
                                            {
                                                node = ((*mc).mc_pg[(*mc).mc_top as usize]
                                                    as *mut std::ffi::c_char)
                                                    .offset(
                                                        *((*((*mc).mc_pg[(*mc).mc_top as usize]
                                                            as *mut std::ffi::c_void
                                                            as *mut MDB_page2))
                                                            .mp2_ptrs)
                                                            .as_mut_ptr()
                                                            .offset(
                                                                (*mc).mc_ki[(*mc).mc_top as usize]
                                                                    as isize,
                                                            )
                                                            as std::ffi::c_int
                                                            as isize,
                                                    )
                                                    .offset(
                                                        (if 0 as std::ffi::c_int != 0 {
                                                            16 as std::ffi::c_ulong
                                                                as std::ffi::c_uint
                                                        } else {
                                                            0 as std::ffi::c_int as std::ffi::c_uint
                                                        })
                                                            as isize,
                                                    )
                                                    as *mut MDB_node;
                                                if (*node).mn_flags as std::ffi::c_int
                                                    & 0x1 as std::ffi::c_int
                                                    == 0
                                                {
                                                    (*newdata).mv_data =
                                                        ((*node).mn_data).as_mut_ptr().offset(
                                                            (*node).mn_ksize as std::ffi::c_int
                                                                as isize,
                                                        )
                                                            as *mut std::ffi::c_void;
                                                }
                                            }
                                            current_block = 6091595930016798176;
                                        }
                                    }
                                } else {
                                    if newindx as std::ffi::c_int >= split_indx {
                                        (*mc).mc_pg[(*mc).mc_top as usize] = rp;
                                        (*mc).mc_ki[ptop as usize] =
                                            ((*mc).mc_ki[ptop as usize]).wrapping_add(1);
                                        (*mc).mc_ki[ptop as usize];
                                        if mn.mc_pg[ptop as usize] != (*mc).mc_pg[ptop as usize]
                                            && (*mc).mc_ki[ptop as usize] as std::ffi::c_uint
                                                >= ((*((*mc).mc_pg[ptop as usize]
                                                    as *mut std::ffi::c_void
                                                    as *mut MDB_page2))
                                                    .mp2_lower
                                                    as std::ffi::c_uint)
                                                    .wrapping_sub(
                                                        (16 as std::ffi::c_ulong
                                                            as std::ffi::c_uint)
                                                            .wrapping_sub(
                                                                if 0 as std::ffi::c_int != 0 {
                                                                    16 as std::ffi::c_ulong
                                                                        as std::ffi::c_uint
                                                                } else {
                                                                    0 as std::ffi::c_int
                                                                        as std::ffi::c_uint
                                                                },
                                                            ),
                                                    )
                                                    >> 1 as std::ffi::c_int
                                        {
                                            i = 0 as std::ffi::c_int;
                                            while i <= ptop {
                                                (*mc).mc_pg[i as usize] = mn.mc_pg[i as usize];
                                                (*mc).mc_ki[i as usize] = mn.mc_ki[i as usize];
                                                i += 1;
                                            }
                                        }
                                    }
                                    current_block = 6091595930016798176;
                                }
                                match current_block {
                                    4951901818287548553 => {}
                                    _ => {
                                        let mut m2: *mut MDB_cursor =
                                            std::ptr::null_mut::<MDB_cursor>();
                                        let mut m3: *mut MDB_cursor =
                                            std::ptr::null_mut::<MDB_cursor>();
                                        let mut dbi: MDB_dbi = (*mc).mc_dbi;
                                        nkeys = (((*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                            .mp2_lower
                                            as std::ffi::c_uint)
                                            .wrapping_sub(
                                                (16 as std::ffi::c_ulong as std::ffi::c_uint)
                                                    .wrapping_sub(if 0 as std::ffi::c_int != 0 {
                                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                                    } else {
                                                        0 as std::ffi::c_int as std::ffi::c_uint
                                                    }),
                                            )
                                            >> 1 as std::ffi::c_int)
                                            as std::ffi::c_int;
                                        let mut current_block_285: u64;
                                        m2 = *((*(*mc).mc_txn).mt_cursors).offset(dbi as isize);
                                        while !m2.is_null() {
                                            if (*mc).mc_flags
                                                & 0x4 as std::ffi::c_int as std::ffi::c_uint
                                                != 0
                                            {
                                                m3 = &mut (*(*m2).mc_xcursor).mx_cursor;
                                            } else {
                                                m3 = m2;
                                            }
                                            if (m3 != mc)
                                                && ((*m2).mc_flags
                                                    & (*m3).mc_flags
                                                    & 0x1 as std::ffi::c_int as std::ffi::c_uint
                                                    != 0)
                                            {
                                                if new_root != 0 {
                                                    let mut k_0: std::ffi::c_int = 0;
                                                    if (*m3).mc_pg[0 as std::ffi::c_int as usize]
                                                        != mp
                                                    {
                                                        current_block_285 = 14303212209785889906;
                                                    } else {
                                                        k_0 = new_root;
                                                        while k_0 >= 0 as std::ffi::c_int {
                                                            (*m3).mc_ki[(k_0 + 1 as std::ffi::c_int)
                                                                as usize] =
                                                                (*m3).mc_ki[k_0 as usize];
                                                            (*m3).mc_pg[(k_0 + 1 as std::ffi::c_int)
                                                                as usize] =
                                                                (*m3).mc_pg[k_0 as usize];
                                                            k_0 -= 1;
                                                        }
                                                        if (*m3).mc_ki
                                                            [0 as std::ffi::c_int as usize]
                                                            as std::ffi::c_int
                                                            >= nkeys
                                                        {
                                                            (*m3).mc_ki
                                                                [0 as std::ffi::c_int as usize] =
                                                                1 as std::ffi::c_int as indx_t;
                                                        } else {
                                                            (*m3).mc_ki
                                                                [0 as std::ffi::c_int as usize] =
                                                                0 as std::ffi::c_int as indx_t;
                                                        }
                                                        (*m3).mc_pg
                                                            [0 as std::ffi::c_int as usize] = (*mc)
                                                            .mc_pg
                                                            [0 as std::ffi::c_int as usize];
                                                        (*m3).mc_snum =
                                                            ((*m3).mc_snum).wrapping_add(1);
                                                        (*m3).mc_snum;
                                                        (*m3).mc_top =
                                                            ((*m3).mc_top).wrapping_add(1);
                                                        (*m3).mc_top;
                                                        current_block_285 = 14723615986260991866;
                                                    }
                                                } else {
                                                    current_block_285 = 14723615986260991866;
                                                }
                                                match current_block_285 {
                                                    14303212209785889906 => {}
                                                    _ => {
                                                        if (*m3).mc_top as std::ffi::c_int
                                                            >= (*mc).mc_top as std::ffi::c_int
                                                            && (*m3).mc_pg[(*mc).mc_top as usize]
                                                                == mp
                                                        {
                                                            if (*m3).mc_ki[(*mc).mc_top as usize]
                                                                as std::ffi::c_int
                                                                >= newindx as std::ffi::c_int
                                                                && nflags
                                                                    & 0x40000 as std::ffi::c_int
                                                                        as std::ffi::c_uint
                                                                    == 0
                                                            {
                                                                (*m3).mc_ki
                                                                    [(*mc).mc_top as usize] =
                                                                    ((*m3).mc_ki
                                                                        [(*mc).mc_top as usize])
                                                                        .wrapping_add(1);
                                                                (*m3).mc_ki[(*mc).mc_top as usize];
                                                            }
                                                            if (*m3).mc_ki[(*mc).mc_top as usize]
                                                                as std::ffi::c_int
                                                                >= nkeys
                                                            {
                                                                (*m3).mc_pg
                                                                    [(*mc).mc_top as usize] = rp;
                                                                (*m3).mc_ki
                                                                    [(*mc).mc_top as usize] = ((*m3)
                                                                    .mc_ki
                                                                    [(*mc).mc_top as usize]
                                                                    as std::ffi::c_int
                                                                    - nkeys)
                                                                    as indx_t;
                                                                i = 0 as std::ffi::c_int;
                                                                while i
                                                                    < (*mc).mc_top
                                                                        as std::ffi::c_int
                                                                {
                                                                    (*m3).mc_ki[i as usize] =
                                                                        mn.mc_ki[i as usize];
                                                                    (*m3).mc_pg[i as usize] =
                                                                        mn.mc_pg[i as usize];
                                                                    i += 1;
                                                                }
                                                            }
                                                        } else if did_split == 0
                                                            && (*m3).mc_top as std::ffi::c_int
                                                                >= ptop
                                                            && (*m3).mc_pg[ptop as usize]
                                                                == (*mc).mc_pg[ptop as usize]
                                                            && (*m3).mc_ki[ptop as usize]
                                                                as std::ffi::c_int
                                                                >= (*mc).mc_ki[ptop as usize]
                                                                    as std::ffi::c_int
                                                        {
                                                            (*m3).mc_ki[ptop as usize] =
                                                                ((*m3).mc_ki[ptop as usize])
                                                                    .wrapping_add(1);
                                                            (*m3).mc_ki[ptop as usize];
                                                        }
                                                        if (*(mp as *mut std::ffi::c_void
                                                            as *mut MDB_page2))
                                                            .mp2_flags
                                                            as std::ffi::c_int
                                                            & 0x2 as std::ffi::c_int
                                                            == 0x2 as std::ffi::c_int
                                                        {
                                                            let mut xr_pg: *mut MDB_page =
                                                                (*m3).mc_pg[(*mc).mc_top as usize];
                                                            let mut xr_node: *mut MDB_node =
                                                                std::ptr::null_mut::<MDB_node>();
                                                            if !(!(!((*m3).mc_xcursor).is_null()
                                                                && (*(*m3).mc_xcursor).mx_cursor.mc_flags
                                                                    & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0)
                                                                || (*m3).mc_ki[(*mc).mc_top as usize] as std::ffi::c_uint
                                                                    >= ((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2))
                                                                        .mp2_lower as std::ffi::c_uint)
                                                                        .wrapping_sub(
                                                                            (16 as std::ffi::c_ulong as std::ffi::c_uint)
                                                                                .wrapping_sub(
                                                                                    if 0 as std::ffi::c_int != 0 {
                                                                                        16 as std::ffi::c_ulong as std::ffi::c_uint
                                                                                    } else {
                                                                                        0 as std::ffi::c_int as std::ffi::c_uint
                                                                                    },
                                                                                ),
                                                                        ) >> 1 as std::ffi::c_int)
                                                            {
                                                                xr_node = (xr_pg as *mut std::ffi::c_char)
                                                                    .offset(
                                                                        *((*(xr_pg as *mut std::ffi::c_void as *mut MDB_page2))
                                                                            .mp2_ptrs)
                                                                            .as_mut_ptr()
                                                                            .offset((*m3).mc_ki[(*mc).mc_top as usize] as isize)
                                                                            as std::ffi::c_int as isize,
                                                                    )
                                                                    .offset(
                                                                        (if 0 as std::ffi::c_int != 0 {
                                                                            16 as std::ffi::c_ulong as std::ffi::c_uint
                                                                        } else {
                                                                            0 as std::ffi::c_int as std::ffi::c_uint
                                                                        }) as isize,
                                                                    ) as *mut MDB_node;
                                                                if (*xr_node).mn_flags as std::ffi::c_int
                                                                    & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                                                                    == 0x4 as std::ffi::c_int
                                                                {
                                                                    (*(*m3).mc_xcursor)
                                                                        .mx_cursor
                                                                        .mc_pg[0 as std::ffi::c_int
                                                                        as usize] = ((*xr_node).mn_data)
                                                                        .as_mut_ptr()
                                                                        .offset((*xr_node).mn_ksize as std::ffi::c_int as isize)
                                                                        as *mut std::ffi::c_void as *mut MDB_page;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            m2 = (*m2).mc_next;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if !copy.is_null() {
            mdb_page_free(env, copy);
        }
        if rc != 0 {
            (*(*mc).mc_txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
        }
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_put(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut key: *mut MDB_val,
    mut data: *mut MDB_val,
    mut flags: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        let mut mc: MDB_cursor = MDB_cursor {
            mc_next: std::ptr::null_mut::<MDB_cursor>(),
            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
            mc_txn: std::ptr::null_mut::<MDB_txn>(),
            mc_dbi: 0,
            mc_db: std::ptr::null_mut::<MDB_db>(),
            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
            mc_ki: [0; 32],
        };
        let mut mx: MDB_xcursor = MDB_xcursor {
            mx_cursor: MDB_cursor {
                mc_next: std::ptr::null_mut::<MDB_cursor>(),
                mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                mc_dbi: 0,
                mc_db: std::ptr::null_mut::<MDB_db>(),
                mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                mc_ki: [0; 32],
            },
            mx_db: MDB_db {
                md_pad: 0,
                md_flags: 0,
                md_depth: 0,
                md_branch_pages: 0,
                md_leaf_pages: 0,
                md_overflow_pages: 0,
                md_entries: 0,
                md_root: 0,
            },
            mx_dbx: MDB_dbx {
                md_name: MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() },
                md_cmp: None,
                md_dcmp: None,
                md_rel: None,
                md_relctx: std::ptr::null_mut::<std::ffi::c_void>(),
            },
            mx_dbflag: 0,
        };
        let mut rc: std::ffi::c_int = 0;
        if key.is_null()
            || data.is_null()
            || !(!txn.is_null()
                && dbi < (*txn).mt_numdbs
                && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                    & 0x10 as std::ffi::c_int
                    != 0)
        {
            return 22 as std::ffi::c_int;
        }
        if flags
            & !(0x10 as std::ffi::c_int
                | 0x20 as std::ffi::c_int
                | 0x10000 as std::ffi::c_int
                | 0x20000 as std::ffi::c_int
                | 0x40000 as std::ffi::c_int) as std::ffi::c_uint
            != 0
        {
            return 22 as std::ffi::c_int;
        }
        if (*txn).mt_flags
            & (0x20000 as std::ffi::c_int
                | (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int))
                as std::ffi::c_uint
            != 0
        {
            return if (*txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                13 as std::ffi::c_int
            } else {
                -(30782 as std::ffi::c_int)
            };
        }
        mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
        mc.mc_next = *((*txn).mt_cursors).offset(dbi as isize);
        let fresh72 = &mut (*((*txn).mt_cursors).offset(dbi as isize));
        *fresh72 = &mut mc;
        rc = _mdb_cursor_put(&mut mc, key, data, flags);
        let fresh73 = &mut (*((*txn).mt_cursors).offset(dbi as isize));
        *fresh73 = mc.mc_next;
        rc
    }
}
#[cold]
unsafe extern "C" fn mdb_env_copythr(mut arg: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    unsafe {
        let mut my: *mut mdb_copy = arg as *mut mdb_copy;
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut toggle: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut rc: std::ffi::c_int = 0;
        let mut wsize: size_t = 0;
        let mut len: std::ffi::c_int = 0;
        let mut set: sigset_t = 0;
        set = 0 as std::ffi::c_int as sigset_t;
        set |= __sigbits(13 as std::ffi::c_int) as sigset_t;
        rc = pthread_sigmask(
            1 as std::ffi::c_int,
            &mut set as *mut sigset_t as *const sigset_t,
            std::ptr::null_mut::<sigset_t>(),
        );
        if rc != 0 as std::ffi::c_int {
            ::core::ptr::write_volatile(&mut (*my).mc_error as *mut std::ffi::c_int, rc);
        }
        pthread_mutex_lock(&mut (*my).mc_mutex);
        loop {
            while (*my).mc_new == 0 {
                pthread_cond_wait(&mut (*my).mc_cond, &mut (*my).mc_mutex);
            }
            if (*my).mc_new == 0 as std::ffi::c_int + 0x10 as std::ffi::c_int {
                break;
            }
            wsize = (*my).mc_wlen[toggle as usize];
            ptr = (*my).mc_wbuf[toggle as usize];
            loop {
                rc = 0 as std::ffi::c_int;
                while wsize > 0 as std::ffi::c_int as size_t && (*my).mc_error == 0 {
                    len = write((*my).mc_fd, ptr as *const std::ffi::c_void, wsize)
                        as std::ffi::c_int;
                    rc = (len >= 0 as std::ffi::c_int) as std::ffi::c_int;
                    if rc == 0 {
                        rc = *__error();
                        if rc == 32 as std::ffi::c_int {
                            let mut tmp: std::ffi::c_int = 0;
                            sigwait(&mut set, &mut tmp);
                        }
                        break;
                    } else if len > 0 as std::ffi::c_int {
                        rc = 0 as std::ffi::c_int;
                        ptr = ptr.offset(len as isize);
                        wsize = wsize.wrapping_sub(len as size_t);
                    } else {
                        rc = 5 as std::ffi::c_int;
                        break;
                    }
                }
                if rc != 0 {
                    ::core::ptr::write_volatile(&mut (*my).mc_error as *mut std::ffi::c_int, rc);
                }
                if (*my).mc_olen[toggle as usize] == 0 {
                    break;
                }
                wsize = (*my).mc_olen[toggle as usize];
                ptr = (*my).mc_over[toggle as usize];
                (*my).mc_olen[toggle as usize] = 0 as std::ffi::c_int as size_t;
            }
            (*my).mc_wlen[toggle as usize] = 0 as std::ffi::c_int as size_t;
            toggle ^= 1 as std::ffi::c_int;
            (*my).mc_new -= 1;
            (*my).mc_new;
            pthread_cond_signal(&mut (*my).mc_cond);
        }
        pthread_mutex_unlock(&mut (*my).mc_mutex);
        std::ptr::null_mut::<std::ffi::c_void>()
    }
}
#[cold]
unsafe extern "C" fn mdb_env_cthr_toggle(
    mut my: *mut mdb_copy,
    mut adjust: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        pthread_mutex_lock(&mut (*my).mc_mutex);
        (*my).mc_new += adjust;
        pthread_cond_signal(&mut (*my).mc_cond);
        while (*my).mc_new & 2 as std::ffi::c_int != 0 {
            pthread_cond_wait(&mut (*my).mc_cond, &mut (*my).mc_mutex);
        }
        pthread_mutex_unlock(&mut (*my).mc_mutex);
        (*my).mc_toggle ^= adjust & 1 as std::ffi::c_int;
        (*my).mc_wlen[(*my).mc_toggle as usize] = 0 as std::ffi::c_int as size_t;
        (*my).mc_error
    }
}
#[cold]
unsafe extern "C" fn mdb_env_cwalk(
    mut my: *mut mdb_copy,
    mut pg: *mut pgno_t,
    mut flags: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut mc: MDB_cursor = {
            MDB_cursor {
                mc_next: std::ptr::null_mut::<MDB_cursor>(),
                mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                mc_dbi: 0,
                mc_db: std::ptr::null_mut::<MDB_db>(),
                mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                mc_ki: [0; 32],
            }
        };
        let mut ni: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
        let mut mo: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut leaf: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut buf: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut rc: std::ffi::c_int = 0;
        let mut toggle: std::ffi::c_int = 0;
        let mut i: std::ffi::c_uint = 0;
        if *pg == !(0 as std::ffi::c_int as pgno_t) {
            return 0 as std::ffi::c_int;
        }
        mc.mc_snum = 1 as std::ffi::c_int as std::ffi::c_ushort;
        mc.mc_txn = (*my).mc_txn;
        mc.mc_flags = (*(*my).mc_txn).mt_flags
            & (0x20000 as std::ffi::c_int | 0x80000 as std::ffi::c_int) as std::ffi::c_uint;
        rc = mdb_page_get(
            &mut mc,
            *pg,
            &mut *(mc.mc_pg).as_mut_ptr().offset(0 as std::ffi::c_int as isize),
            std::ptr::null_mut::<std::ffi::c_int>(),
        );
        if rc != 0 {
            return rc;
        }
        rc = mdb_page_search_root(&mut mc, std::ptr::null_mut::<MDB_val>(), 4 as std::ffi::c_int);
        if rc != 0 {
            return rc;
        }
        ptr = malloc(((*(*my).mc_env).me_psize).wrapping_mul(mc.mc_snum as std::ffi::c_uint)
            as std::ffi::c_ulong) as *mut std::ffi::c_char;
        buf = ptr;
        if buf.is_null() {
            return 12 as std::ffi::c_int;
        }
        i = 0 as std::ffi::c_int as std::ffi::c_uint;
        while i < mc.mc_top as std::ffi::c_uint {
            mdb_page_copy(ptr as *mut MDB_page, mc.mc_pg[i as usize], (*(*my).mc_env).me_psize);
            mc.mc_pg[i as usize] = ptr as *mut MDB_page;
            ptr = ptr.offset((*(*my).mc_env).me_psize as isize);
            i = i.wrapping_add(1);
        }
        leaf = ptr as *mut MDB_page;
        toggle = (*my).mc_toggle;
        's_89: while mc.mc_snum as std::ffi::c_int > 0 as std::ffi::c_int {
            let mut n: std::ffi::c_uint = 0;
            mp = mc.mc_pg[mc.mc_top as usize];
            n = ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_lower as std::ffi::c_uint)
                .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                    if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    },
                ))
                >> 1 as std::ffi::c_int;
            if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                & 0x2 as std::ffi::c_int
                == 0x2 as std::ffi::c_int
            {
                if ((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                    & 0x20 as std::ffi::c_int
                    != 0x20 as std::ffi::c_int)
                    && flags & 0x4 as std::ffi::c_int == 0
                {
                    i = 0 as std::ffi::c_int as std::ffi::c_uint;
                    while i < n {
                        ni = (mp as *mut std::ffi::c_char)
                            .offset(
                                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset(i as isize)
                                    as std::ffi::c_int as isize,
                            )
                            .offset(
                                (if 0 as std::ffi::c_int != 0 {
                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                } else {
                                    0 as std::ffi::c_int as std::ffi::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if (*ni).mn_flags as std::ffi::c_int & 0x1 as std::ffi::c_int != 0 {
                            let mut omp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
                            let mut pg_0: pgno_t = 0;
                            if mp != leaf {
                                mc.mc_pg[mc.mc_top as usize] = leaf;
                                mdb_page_copy(leaf, mp, (*(*my).mc_env).me_psize);
                                mp = leaf;
                                ni = (mp as *mut std::ffi::c_char)
                                    .offset(
                                        *((*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                            .mp2_ptrs)
                                            .as_mut_ptr()
                                            .offset(i as isize)
                                            as std::ffi::c_int
                                            as isize,
                                    )
                                    .offset(
                                        (if 0 as std::ffi::c_int != 0 {
                                            16 as std::ffi::c_ulong as std::ffi::c_uint
                                        } else {
                                            0 as std::ffi::c_int as std::ffi::c_uint
                                        }) as isize,
                                    ) as *mut MDB_node;
                            }
                            memcpy(
                                &mut pg_0 as *mut pgno_t as *mut std::ffi::c_void,
                                ((*ni).mn_data)
                                    .as_mut_ptr()
                                    .offset((*ni).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void,
                                ::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong,
                            );
                            memcpy(
                                ((*ni).mn_data)
                                    .as_mut_ptr()
                                    .offset((*ni).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void,
                                &mut (*my).mc_next_pgno as *mut pgno_t as *const std::ffi::c_void,
                                ::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong,
                            );
                            rc = mdb_page_get(
                                &mut mc,
                                pg_0,
                                &mut omp,
                                std::ptr::null_mut::<std::ffi::c_int>(),
                            );
                            if rc != 0 {
                                break 's_89;
                            }
                            if (*my).mc_wlen[toggle as usize]
                                >= (1024 as std::ffi::c_int * 1024 as std::ffi::c_int) as size_t
                            {
                                rc = mdb_env_cthr_toggle(my, 1 as std::ffi::c_int);
                                if rc != 0 {
                                    break 's_89;
                                }
                                toggle = (*my).mc_toggle;
                            }
                            mo = ((*my).mc_wbuf[toggle as usize])
                                .offset((*my).mc_wlen[toggle as usize] as isize)
                                as *mut MDB_page;
                            memcpy(
                                mo as *mut std::ffi::c_void,
                                omp as *const std::ffi::c_void,
                                (*(*my).mc_env).me_psize as std::ffi::c_ulong,
                            );
                            (*mo).mp_p.p_pgno = (*my).mc_next_pgno;
                            (*my).mc_next_pgno =
                                ((*my).mc_next_pgno).wrapping_add((*omp).mp_pb.pb_pages as pgno_t);
                            (*my).mc_wlen[toggle as usize] = ((*my).mc_wlen[toggle as usize])
                                .wrapping_add((*(*my).mc_env).me_psize as size_t);
                            if (*omp).mp_pb.pb_pages > 1 as std::ffi::c_int as uint32_t {
                                (*my).mc_olen[toggle as usize] =
                                    ((*(*my).mc_env).me_psize).wrapping_mul(
                                        ((*omp).mp_pb.pb_pages)
                                            .wrapping_sub(1 as std::ffi::c_int as uint32_t),
                                    ) as size_t;
                                (*my).mc_over[toggle as usize] = (omp as *mut std::ffi::c_char)
                                    .offset((*(*my).mc_env).me_psize as isize);
                                rc = mdb_env_cthr_toggle(my, 1 as std::ffi::c_int);
                                if rc != 0 {
                                    break 's_89;
                                }
                                toggle = (*my).mc_toggle;
                            }
                        } else if (*ni).mn_flags as std::ffi::c_int & 0x2 as std::ffi::c_int != 0 {
                            let mut db: MDB_db = MDB_db {
                                md_pad: 0,
                                md_flags: 0,
                                md_depth: 0,
                                md_branch_pages: 0,
                                md_leaf_pages: 0,
                                md_overflow_pages: 0,
                                md_entries: 0,
                                md_root: 0,
                            };
                            if mp != leaf {
                                mc.mc_pg[mc.mc_top as usize] = leaf;
                                mdb_page_copy(leaf, mp, (*(*my).mc_env).me_psize);
                                mp = leaf;
                                ni = (mp as *mut std::ffi::c_char)
                                    .offset(
                                        *((*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                                            .mp2_ptrs)
                                            .as_mut_ptr()
                                            .offset(i as isize)
                                            as std::ffi::c_int
                                            as isize,
                                    )
                                    .offset(
                                        (if 0 as std::ffi::c_int != 0 {
                                            16 as std::ffi::c_ulong as std::ffi::c_uint
                                        } else {
                                            0 as std::ffi::c_int as std::ffi::c_uint
                                        }) as isize,
                                    ) as *mut MDB_node;
                            }
                            memcpy(
                                &mut db as *mut MDB_db as *mut std::ffi::c_void,
                                ((*ni).mn_data)
                                    .as_mut_ptr()
                                    .offset((*ni).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void,
                                ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong,
                            );
                            (*my).mc_toggle = toggle;
                            rc = mdb_env_cwalk(
                                my,
                                &mut db.md_root,
                                (*ni).mn_flags as std::ffi::c_int & 0x4 as std::ffi::c_int,
                            );
                            if rc != 0 {
                                break 's_89;
                            }
                            toggle = (*my).mc_toggle;
                            memcpy(
                                ((*ni).mn_data)
                                    .as_mut_ptr()
                                    .offset((*ni).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void,
                                &mut db as *mut MDB_db as *const std::ffi::c_void,
                                ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong,
                            );
                        }
                        i = i.wrapping_add(1);
                    }
                }
            } else {
                mc.mc_ki[mc.mc_top as usize] = (mc.mc_ki[mc.mc_top as usize]).wrapping_add(1);
                mc.mc_ki[mc.mc_top as usize];
                if (mc.mc_ki[mc.mc_top as usize] as std::ffi::c_uint) < n {
                    let mut pg_1: pgno_t = 0;
                    loop {
                        ni = (mp as *mut std::ffi::c_char)
                            .offset(
                                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset(mc.mc_ki[mc.mc_top as usize] as isize)
                                    as std::ffi::c_int as isize,
                            )
                            .offset(
                                (if 0 as std::ffi::c_int != 0 {
                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                } else {
                                    0 as std::ffi::c_int as std::ffi::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        pg_1 = (*ni).mn_lo as pgno_t
                            | ((*ni).mn_hi as pgno_t) << 16 as std::ffi::c_int
                            | (if (if -(1 as std::ffi::c_int) as pgno_t
                                > 0xffffffff as std::ffi::c_uint as pgno_t
                            {
                                32 as std::ffi::c_int
                            } else {
                                0 as std::ffi::c_int
                            }) != 0
                            {
                                ((*ni).mn_flags as pgno_t)
                                    << (if -(1 as std::ffi::c_int) as pgno_t
                                        > 0xffffffff as std::ffi::c_uint as pgno_t
                                    {
                                        32 as std::ffi::c_int
                                    } else {
                                        0 as std::ffi::c_int
                                    })
                            } else {
                                0 as std::ffi::c_int as pgno_t
                            });
                        rc = mdb_page_get(
                            &mut mc,
                            pg_1,
                            &mut mp,
                            std::ptr::null_mut::<std::ffi::c_int>(),
                        );
                        if rc != 0 {
                            break 's_89;
                        }
                        mc.mc_top = (mc.mc_top).wrapping_add(1);
                        mc.mc_top;
                        mc.mc_snum = (mc.mc_snum).wrapping_add(1);
                        mc.mc_snum;
                        mc.mc_ki[mc.mc_top as usize] = 0 as std::ffi::c_int as indx_t;
                        if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                            as std::ffi::c_int
                            & 0x1 as std::ffi::c_int
                            != 0x1 as std::ffi::c_int
                        {
                            break;
                        }
                        mdb_page_copy(mc.mc_pg[mc.mc_top as usize], mp, (*(*my).mc_env).me_psize);
                    }
                    mc.mc_pg[mc.mc_top as usize] = mp;
                    continue;
                }
            }
            if (*my).mc_wlen[toggle as usize]
                >= (1024 as std::ffi::c_int * 1024 as std::ffi::c_int) as size_t
            {
                rc = mdb_env_cthr_toggle(my, 1 as std::ffi::c_int);
                if rc != 0 {
                    break;
                }
                toggle = (*my).mc_toggle;
            }
            mo = ((*my).mc_wbuf[toggle as usize]).offset((*my).mc_wlen[toggle as usize] as isize)
                as *mut MDB_page;
            mdb_page_copy(mo, mp, (*(*my).mc_env).me_psize);
            let fresh74 = (*my).mc_next_pgno;
            (*my).mc_next_pgno = ((*my).mc_next_pgno).wrapping_add(1);
            (*mo).mp_p.p_pgno = fresh74;
            (*my).mc_wlen[toggle as usize] =
                ((*my).mc_wlen[toggle as usize]).wrapping_add((*(*my).mc_env).me_psize as size_t);
            if mc.mc_top != 0 {
                ni = (mc.mc_pg[(mc.mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as usize]
                    as *mut std::ffi::c_char)
                    .offset(
                        *((*(mc.mc_pg
                            [(mc.mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as usize]
                            as *mut std::ffi::c_void
                            as *mut MDB_page2))
                            .mp2_ptrs)
                            .as_mut_ptr()
                            .offset(
                                mc.mc_ki
                                    [(mc.mc_top as std::ffi::c_int - 1 as std::ffi::c_int) as usize]
                                    as isize,
                            ) as std::ffi::c_int as isize,
                    )
                    .offset(
                        (if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        }) as isize,
                    ) as *mut MDB_node;
                (*ni).mn_lo =
                    ((*mo).mp_p.p_pgno & 0xffff as std::ffi::c_int as pgno_t) as std::ffi::c_ushort;
                (*ni).mn_hi = ((*mo).mp_p.p_pgno >> 16 as std::ffi::c_int) as std::ffi::c_ushort;
                if if -(1 as std::ffi::c_int) as pgno_t > 0xffffffff as std::ffi::c_uint as pgno_t {
                    32 as std::ffi::c_int
                } else {
                    0 as std::ffi::c_int
                } != 0
                {
                    (*ni).mn_flags = ((*mo).mp_p.p_pgno
                        >> (if -(1 as std::ffi::c_int) as pgno_t
                            > 0xffffffff as std::ffi::c_uint as pgno_t
                        {
                            32 as std::ffi::c_int
                        } else {
                            0 as std::ffi::c_int
                        })) as std::ffi::c_ushort;
                }
                mdb_cursor_pop(&mut mc);
            } else {
                *pg = (*mo).mp_p.p_pgno;
                break;
            }
        }
        free(buf as *mut std::ffi::c_void);
        rc
    }
}
#[cold]
unsafe extern "C" fn mdb_env_copyfd1(
    mut env: *mut MDB_env,
    mut fd: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut mm: *mut MDB_meta = std::ptr::null_mut::<MDB_meta>();
        let mut mp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
        let mut my: mdb_copy = {
            mdb_copy {
                mc_env: std::ptr::null_mut::<MDB_env>(),
                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                mc_mutex: _opaque_pthread_mutex_t { __sig: 0, __opaque: [0; 56] },
                mc_cond: _opaque_pthread_cond_t { __sig: 0, __opaque: [0; 40] },
                mc_wbuf: [std::ptr::null_mut::<std::ffi::c_char>(); 2],
                mc_over: [std::ptr::null_mut::<std::ffi::c_char>(); 2],
                mc_wlen: [0; 2],
                mc_olen: [0; 2],
                mc_next_pgno: 0,
                mc_fd: 0,
                mc_toggle: 0,
                mc_new: 0,
                mc_error: 0,
            }
        };
        let mut txn: *mut MDB_txn = std::ptr::null_mut::<MDB_txn>();
        let mut thr: pthread_t = std::ptr::null_mut::<_opaque_pthread_t>();
        let mut root: pgno_t = 0;
        let mut new_root: pgno_t = 0;
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        rc = pthread_mutex_init(&mut my.mc_mutex, std::ptr::null::<pthread_mutexattr_t>());
        if rc != 0 as std::ffi::c_int {
            return rc;
        }
        rc = pthread_cond_init(&mut my.mc_cond, std::ptr::null::<pthread_condattr_t>());
        if rc == 0 as std::ffi::c_int {
            let mut p: *mut std::ffi::c_void = std::ptr::null_mut::<std::ffi::c_void>();
            rc = posix_memalign(
                &mut p,
                (*env).me_os_psize as size_t,
                (1024 as std::ffi::c_int * 1024 as std::ffi::c_int * 2 as std::ffi::c_int)
                    as size_t,
            );
            if rc == 0 as std::ffi::c_int {
                my.mc_wbuf[0 as std::ffi::c_int as usize] = p as *mut std::ffi::c_char;
                memset(
                    my.mc_wbuf[0 as std::ffi::c_int as usize] as *mut std::ffi::c_void,
                    0 as std::ffi::c_int,
                    (1024 as std::ffi::c_int * 1024 as std::ffi::c_int * 2 as std::ffi::c_int)
                        as std::ffi::c_ulong,
                );
                my.mc_wbuf[1 as std::ffi::c_int as usize] = (my.mc_wbuf
                    [0 as std::ffi::c_int as usize])
                    .offset((1024 as std::ffi::c_int * 1024 as std::ffi::c_int) as isize);
                my.mc_next_pgno = 2 as std::ffi::c_int as pgno_t;
                my.mc_env = env;
                my.mc_fd = fd;
                rc = pthread_create(
                    &mut thr,
                    std::ptr::null::<pthread_attr_t>(),
                    Some(
                        mdb_env_copythr
                            as unsafe extern "C" fn(*mut std::ffi::c_void) -> *mut std::ffi::c_void,
                    ),
                    &mut my as *mut mdb_copy as *mut std::ffi::c_void,
                );
                if rc == 0 {
                    rc = mdb_txn_begin(
                        env,
                        std::ptr::null_mut::<MDB_txn>(),
                        0x20000 as std::ffi::c_int as std::ffi::c_uint,
                        &mut txn,
                    );
                    if rc == 0 {
                        mp = my.mc_wbuf[0 as std::ffi::c_int as usize] as *mut MDB_page;
                        memset(
                            mp as *mut std::ffi::c_void,
                            0 as std::ffi::c_int,
                            (2 as std::ffi::c_int as std::ffi::c_uint).wrapping_mul((*env).me_psize)
                                as std::ffi::c_ulong,
                        );
                        (*mp).mp_p.p_pgno = 0 as std::ffi::c_int as pgno_t;
                        (*mp).mp_flags = 0x8 as std::ffi::c_int as uint16_t;
                        mm = (mp as *mut std::ffi::c_char)
                            .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                            as *mut std::ffi::c_void as *mut MDB_meta;
                        mdb_env_init_meta0(env, mm);
                        (*mm).mm_address =
                            (*(*env).me_metas[0 as std::ffi::c_int as usize]).mm_address;
                        mp = (my.mc_wbuf[0 as std::ffi::c_int as usize])
                            .offset((*env).me_psize as isize)
                            as *mut MDB_page;
                        (*mp).mp_p.p_pgno = 1 as std::ffi::c_int as pgno_t;
                        (*mp).mp_flags = 0x8 as std::ffi::c_int as uint16_t;
                        *((mp as *mut std::ffi::c_char)
                            .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                            as *mut std::ffi::c_void as *mut MDB_meta) = *mm;
                        mm = (mp as *mut std::ffi::c_char)
                            .offset(16 as std::ffi::c_ulong as std::ffi::c_uint as isize)
                            as *mut std::ffi::c_void as *mut MDB_meta;
                        new_root = (*((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize)).md_root;
                        root = new_root;
                        if root != !(0 as std::ffi::c_int as pgno_t) {
                            let mut freecount: MDB_ID = 0 as std::ffi::c_int as MDB_ID;
                            let mut mc: MDB_cursor = MDB_cursor {
                                mc_next: std::ptr::null_mut::<MDB_cursor>(),
                                mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                                mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                                mc_dbi: 0,
                                mc_db: std::ptr::null_mut::<MDB_db>(),
                                mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                                mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                                mc_snum: 0,
                                mc_top: 0,
                                mc_flags: 0,
                                mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                                mc_ki: [0; 32],
                            };
                            let mut key: MDB_val = MDB_val {
                                mv_size: 0,
                                mv_data: std::ptr::null_mut::<std::ffi::c_void>(),
                            };
                            let mut data: MDB_val = MDB_val {
                                mv_size: 0,
                                mv_data: std::ptr::null_mut::<std::ffi::c_void>(),
                            };
                            mdb_cursor_init(
                                &mut mc,
                                txn,
                                0 as std::ffi::c_int as MDB_dbi,
                                std::ptr::null_mut::<MDB_xcursor>(),
                            );
                            loop {
                                rc = mdb_cursor_get(&mut mc, &mut key, &mut data, MDB_NEXT);
                                if rc != 0 as std::ffi::c_int {
                                    break;
                                }
                                freecount = freecount.wrapping_add(*(data.mv_data as *mut MDB_ID));
                            }
                            if rc != -(30798 as std::ffi::c_int) {
                                current_block = 10260918000359197151;
                            } else {
                                freecount = freecount.wrapping_add(
                                    ((*((*txn).mt_dbs).offset(0 as std::ffi::c_int as isize))
                                        .md_branch_pages)
                                        .wrapping_add(
                                            (*((*txn).mt_dbs)
                                                .offset(0 as std::ffi::c_int as isize))
                                            .md_leaf_pages,
                                        )
                                        .wrapping_add(
                                            (*((*txn).mt_dbs)
                                                .offset(0 as std::ffi::c_int as isize))
                                            .md_overflow_pages,
                                        ),
                                );
                                new_root = ((*txn).mt_next_pgno)
                                    .wrapping_sub(1 as std::ffi::c_int as pgno_t)
                                    .wrapping_sub(freecount);
                                (*mm).mm_last_pg = new_root;
                                (*mm).mm_dbs[1 as std::ffi::c_int as usize] =
                                    *((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize);
                                (*mm).mm_dbs[1 as std::ffi::c_int as usize].md_root = new_root;
                                current_block = 652864300344834934;
                            }
                        } else {
                            (*mm).mm_dbs[1 as std::ffi::c_int as usize].md_flags =
                                (*((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize)).md_flags;
                            current_block = 652864300344834934;
                        }
                        match current_block {
                            10260918000359197151 => {}
                            _ => {
                                if root != !(0 as std::ffi::c_int as pgno_t)
                                    || (*mm).mm_dbs[1 as std::ffi::c_int as usize].md_flags
                                        as std::ffi::c_int
                                        != 0
                                {
                                    ::core::ptr::write_volatile(
                                        &mut (*mm).mm_txnid as *mut txnid_t,
                                        1 as std::ffi::c_int as txnid_t,
                                    );
                                }
                                my.mc_wlen[0 as std::ffi::c_int as usize] = ((*env).me_psize)
                                    .wrapping_mul(2 as std::ffi::c_int as std::ffi::c_uint)
                                    as size_t;
                                my.mc_txn = txn;
                                rc = mdb_env_cwalk(&mut my, &mut root, 0 as std::ffi::c_int);
                                if rc == 0 as std::ffi::c_int && root != new_root {
                                    rc = -(30784 as std::ffi::c_int);
                                }
                            }
                        }
                    }
                    if rc != 0 {
                        ::core::ptr::write_volatile(&mut my.mc_error as *mut std::ffi::c_int, rc);
                    }
                    mdb_env_cthr_toggle(&mut my, 1 as std::ffi::c_int | 0x10 as std::ffi::c_int);
                    rc = pthread_join(thr, std::ptr::null_mut::<*mut std::ffi::c_void>());
                    _mdb_txn_abort(txn);
                }
            }
            free(my.mc_wbuf[0 as std::ffi::c_int as usize] as *mut std::ffi::c_void);
            pthread_cond_destroy(&mut my.mc_cond);
        }
        pthread_mutex_destroy(&mut my.mc_mutex);
        if rc != 0 { rc } else { my.mc_error }
    }
}
#[cold]
unsafe extern "C" fn mdb_env_copyfd0(
    mut env: *mut MDB_env,
    mut fd: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut txn: *mut MDB_txn = std::ptr::null_mut::<MDB_txn>();
        let mut wmutex: mdb_mutexref_t = 0 as mdb_mutexref_t;
        let mut rc: std::ffi::c_int = 0;
        let mut wsize: mdb_size_t = 0;
        let mut w3: mdb_size_t = 0;
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut len: ssize_t = 0;
        let mut w2: size_t = 0;
        rc = mdb_txn_begin(
            env,
            std::ptr::null_mut::<MDB_txn>(),
            0x20000 as std::ffi::c_int as std::ffi::c_uint,
            &mut txn,
        );
        if rc != 0 {
            return rc;
        }
        if !((*env).me_txns).is_null() {
            mdb_txn_end(txn, MDB_END_RESET_TMP as std::ffi::c_int as std::ffi::c_uint);
            wmutex = ((*env).me_wmutex).as_mut_ptr();
            rc = mdb_sem_wait(wmutex);
            if rc != 0 && {
                rc = mdb_mutex_failed(env, wmutex, rc);
                rc != 0
            } {
                current_block = 256522870586370188;
            } else {
                rc = mdb_txn_renew0(txn);
                if rc != 0 {
                    let mut sb: sembuf = {
                        sembuf {
                            sem_num: 0 as std::ffi::c_int as std::ffi::c_ushort,
                            sem_op: 1 as std::ffi::c_int as std::ffi::c_short,
                            sem_flg: 0o10000 as std::ffi::c_int as std::ffi::c_short,
                        }
                    };
                    sb.sem_num = (*wmutex).semnum as std::ffi::c_ushort;
                    *(*wmutex).locked = 0 as std::ffi::c_int;
                    semop((*wmutex).semid, &mut sb, 1 as std::ffi::c_int as size_t);
                    current_block = 256522870586370188;
                } else {
                    current_block = 12349973810996921269;
                }
            }
        } else {
            current_block = 12349973810996921269;
        }
        if current_block == 12349973810996921269 {
            wsize = ((*env).me_psize).wrapping_mul(2 as std::ffi::c_int as std::ffi::c_uint)
                as mdb_size_t;
            ptr = (*env).me_map;
            w2 = wsize as _;
            while w2 > 0 as std::ffi::c_int as size_t {
                len = write(fd, ptr as *const std::ffi::c_void, w2);
                rc = (len >= 0 as std::ffi::c_int as ssize_t) as std::ffi::c_int;
                if rc == 0 {
                    rc = *__error();
                    break;
                } else if len > 0 as std::ffi::c_int as ssize_t {
                    rc = 0 as std::ffi::c_int;
                    ptr = ptr.offset(len as isize);
                    w2 = w2.wrapping_sub(len as size_t);
                } else {
                    rc = 5 as std::ffi::c_int;
                    break;
                }
            }
            if !wmutex.is_null() {
                let mut sb_0: sembuf = {
                    sembuf {
                        sem_num: 0 as std::ffi::c_int as std::ffi::c_ushort,
                        sem_op: 1 as std::ffi::c_int as std::ffi::c_short,
                        sem_flg: 0o10000 as std::ffi::c_int as std::ffi::c_short,
                    }
                };
                sb_0.sem_num = (*wmutex).semnum as std::ffi::c_ushort;
                *(*wmutex).locked = 0 as std::ffi::c_int;
                semop((*wmutex).semid, &mut sb_0, 1 as std::ffi::c_int as size_t);
            }
            if rc == 0 {
                w3 = ((*txn).mt_next_pgno * (*env).me_psize as pgno_t) as _;
                let mut fsize: mdb_size_t = 0 as std::ffi::c_int as mdb_size_t;
                rc = mdb_fsize((*env).me_fd, &mut fsize);
                if rc == 0 {
                    if w3 > fsize {
                        w3 = fsize;
                    }
                    wsize = w3.wrapping_sub(wsize);
                    while wsize > 0 as std::ffi::c_int as mdb_size_t {
                        if wsize
                            > (0x40000000 as std::ffi::c_uint
                                >> (::core::mem::size_of::<ssize_t>() as std::ffi::c_ulong
                                    == 4 as std::ffi::c_int as std::ffi::c_ulong)
                                    as std::ffi::c_int) as mdb_size_t
                        {
                            w2 = (0x40000000 as std::ffi::c_uint
                                >> (::core::mem::size_of::<ssize_t>() as std::ffi::c_ulong
                                    == 4 as std::ffi::c_int as std::ffi::c_ulong)
                                    as std::ffi::c_int) as size_t;
                        } else {
                            w2 = wsize as _;
                        }
                        len = write(fd, ptr as *const std::ffi::c_void, w2);
                        rc = (len >= 0 as std::ffi::c_int as ssize_t) as std::ffi::c_int;
                        if rc == 0 {
                            rc = *__error();
                            break;
                        } else if len > 0 as std::ffi::c_int as ssize_t {
                            rc = 0 as std::ffi::c_int;
                            ptr = ptr.offset(len as isize);
                            wsize = wsize.wrapping_sub(len as mdb_size_t);
                        } else {
                            rc = 5 as std::ffi::c_int;
                            break;
                        }
                    }
                }
            }
        }
        _mdb_txn_abort(txn);
        rc
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_copyfd2(
    mut env: *mut MDB_env,
    mut fd: std::ffi::c_int,
    mut flags: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        if flags & 0x1 as std::ffi::c_int as std::ffi::c_uint != 0 {
            mdb_env_copyfd1(env, fd)
        } else {
            mdb_env_copyfd0(env, fd)
        }
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_copyfd(
    mut env: *mut MDB_env,
    mut fd: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe { mdb_env_copyfd2(env, fd, 0 as std::ffi::c_int as std::ffi::c_uint) }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_copy2(
    mut env: *mut MDB_env,
    mut path: *const std::ffi::c_char,
    mut flags: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        let mut rc: std::ffi::c_int = 0;
        let mut fname: MDB_name =
            MDB_name { mn_len: 0, mn_alloced: 0, mn_val: std::ptr::null_mut::<mdb_nchar_t>() };
        let mut newfd: std::ffi::c_int = -(1 as std::ffi::c_int);
        rc = mdb_fname_init(
            path,
            (*env).me_flags | 0x400000 as std::ffi::c_int as uint32_t,
            &mut fname,
        );
        if rc == 0 as std::ffi::c_int {
            rc = mdb_fopen(
                env,
                &mut fname,
                MDB_O_COPY,
                0o666 as std::ffi::c_int as mdb_mode_t,
                &mut newfd,
            );
            if fname.mn_alloced != 0 {
                free(fname.mn_val as *mut std::ffi::c_void);
            }
        }
        if rc == 0 as std::ffi::c_int {
            rc = mdb_env_copyfd2(env, newfd, flags);
            if close(newfd) < 0 as std::ffi::c_int && rc == 0 as std::ffi::c_int {
                rc = *__error();
            }
        }
        rc
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_copy(
    mut env: *mut MDB_env,
    mut path: *const std::ffi::c_char,
) -> std::ffi::c_int {
    unsafe { mdb_env_copy2(env, path, 0 as std::ffi::c_int as std::ffi::c_uint) }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_set_flags(
    mut env: *mut MDB_env,
    mut flag: std::ffi::c_uint,
    mut onoff: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        if flag
            & !(0x10000 as std::ffi::c_int
                | 0x40000 as std::ffi::c_int
                | 0x100000 as std::ffi::c_int
                | 0x1000000 as std::ffi::c_int) as std::ffi::c_uint
            != 0
        {
            return 22 as std::ffi::c_int;
        }
        if onoff != 0 {
            (*env).me_flags |= flag;
        } else {
            (*env).me_flags &= !flag;
        }
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_get_flags(
    mut env: *mut MDB_env,
    mut arg: *mut std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        if env.is_null() || arg.is_null() {
            return 22 as std::ffi::c_int;
        }
        *arg = (*env).me_flags
            & (0x10000 as std::ffi::c_int
                | 0x40000 as std::ffi::c_int
                | 0x100000 as std::ffi::c_int
                | 0x1000000 as std::ffi::c_int
                | (0x1 as std::ffi::c_int
                    | 0x4000 as std::ffi::c_int
                    | 0x20000 as std::ffi::c_int
                    | 0x80000 as std::ffi::c_int
                    | 0x200000 as std::ffi::c_int
                    | 0x400000 as std::ffi::c_int
                    | 0x800000 as std::ffi::c_int
                    | 0x2000000 as std::ffi::c_int)) as uint32_t;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_set_userctx(
    mut env: *mut MDB_env,
    mut ctx: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    unsafe {
        if env.is_null() {
            return 22 as std::ffi::c_int;
        }
        (*env).me_userctx = ctx;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_get_userctx(mut env: *mut MDB_env) -> *mut std::ffi::c_void {
    unsafe {
        if !env.is_null() { (*env).me_userctx } else { std::ptr::null_mut::<std::ffi::c_void>() }
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_set_assert(
    mut env: *mut MDB_env,
    mut func: Option<MDB_assert_func>,
) -> std::ffi::c_int {
    unsafe {
        if env.is_null() {
            return 22 as std::ffi::c_int;
        }
        (*env).me_assert_func = func;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_get_path(
    mut env: *mut MDB_env,
    mut arg: *mut *const std::ffi::c_char,
) -> std::ffi::c_int {
    unsafe {
        if env.is_null() || arg.is_null() {
            return 22 as std::ffi::c_int;
        }
        *arg = (*env).me_path;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_get_fd(
    mut env: *mut MDB_env,
    mut arg: *mut mdb_filehandle_t,
) -> std::ffi::c_int {
    unsafe {
        if env.is_null() || arg.is_null() {
            return 22 as std::ffi::c_int;
        }
        *arg = (*env).me_fd;
        0 as std::ffi::c_int
    }
}
#[cold]
unsafe extern "C" fn mdb_stat0(
    mut env: *mut MDB_env,
    mut db: *mut MDB_db,
    mut arg: *mut MDB_stat,
) -> std::ffi::c_int {
    unsafe {
        (*arg).ms_psize = (*env).me_psize;
        (*arg).ms_depth = (*db).md_depth as std::ffi::c_uint;
        (*arg).ms_branch_pages = (*db).md_branch_pages as _;
        (*arg).ms_leaf_pages = (*db).md_leaf_pages as _;
        (*arg).ms_overflow_pages = (*db).md_overflow_pages as _;
        (*arg).ms_entries = (*db).md_entries;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_stat(
    mut env: *mut MDB_env,
    mut arg: *mut MDB_stat,
) -> std::ffi::c_int {
    unsafe {
        let mut meta: *mut MDB_meta = std::ptr::null_mut::<MDB_meta>();
        if env.is_null() || arg.is_null() {
            return 22 as std::ffi::c_int;
        }
        meta = mdb_env_pick_meta(env);
        mdb_stat0(
            env,
            &mut *((*meta).mm_dbs).as_mut_ptr().offset(1 as std::ffi::c_int as isize),
            arg,
        )
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_info(
    mut env: *mut MDB_env,
    mut arg: *mut MDB_envinfo,
) -> std::ffi::c_int {
    unsafe {
        let mut meta: *mut MDB_meta = std::ptr::null_mut::<MDB_meta>();
        if env.is_null() || arg.is_null() {
            return 22 as std::ffi::c_int;
        }
        meta = mdb_env_pick_meta(env);
        (*arg).me_mapaddr = (*meta).mm_address;
        (*arg).me_last_pgno = (*meta).mm_last_pg as _;
        (*arg).me_last_txnid = (*meta).mm_txnid as _;
        (*arg).me_mapsize = (*env).me_mapsize;
        (*arg).me_maxreaders = (*env).me_maxreaders;
        (*arg).me_numreaders = if !((*env).me_txns).is_null() {
            (*(*env).me_txns).mt1.mtb.mtb_numreaders
        } else {
            0 as std::ffi::c_int as std::ffi::c_uint
        };
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_default_cmp(mut txn: *mut MDB_txn, mut dbi: MDB_dbi) {
    unsafe {
        let mut f: uint16_t = (*((*txn).mt_dbs).offset(dbi as isize)).md_flags;
        let fresh75 = &mut (*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp;
        *fresh75 = if f as std::ffi::c_int & 0x2 as std::ffi::c_int != 0 {
            Some(mdb_cmp_memnr as MDB_cmp_func)
        } else if f as std::ffi::c_int & 0x8 as std::ffi::c_int != 0 {
            Some(mdb_cmp_cint as MDB_cmp_func)
        } else {
            Some(mdb_cmp_memn as MDB_cmp_func)
        };
        let fresh76 = &mut (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
        *fresh76 = if f as std::ffi::c_int & 0x4 as std::ffi::c_int == 0 {
            None
        } else if f as std::ffi::c_int & 0x20 as std::ffi::c_int != 0 {
            if f as std::ffi::c_int & 0x10 as std::ffi::c_int != 0 {
                Some(mdb_cmp_int as MDB_cmp_func)
            } else {
                Some(mdb_cmp_cint as MDB_cmp_func)
            }
        } else if f as std::ffi::c_int & 0x40 as std::ffi::c_int != 0 {
            Some(mdb_cmp_memnr as MDB_cmp_func)
        } else {
            Some(mdb_cmp_memn as MDB_cmp_func)
        };
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_dbi_open(
    mut txn: *mut MDB_txn,
    mut name: *const std::ffi::c_char,
    mut flags: std::ffi::c_uint,
    mut dbi: *mut MDB_dbi,
) -> std::ffi::c_int {
    unsafe {
        let mut key: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut data: MDB_val =
            MDB_val { mv_size: 0, mv_data: std::ptr::null_mut::<std::ffi::c_void>() };
        let mut i: MDB_dbi = 0;
        let mut mc: MDB_cursor = MDB_cursor {
            mc_next: std::ptr::null_mut::<MDB_cursor>(),
            mc_backup: std::ptr::null_mut::<MDB_cursor>(),
            mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
            mc_txn: std::ptr::null_mut::<MDB_txn>(),
            mc_dbi: 0,
            mc_db: std::ptr::null_mut::<MDB_db>(),
            mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
            mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
            mc_snum: 0,
            mc_top: 0,
            mc_flags: 0,
            mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
            mc_ki: [0; 32],
        };
        let mut dummy: MDB_db = MDB_db {
            md_pad: 0,
            md_flags: 0,
            md_depth: 0,
            md_branch_pages: 0,
            md_leaf_pages: 0,
            md_overflow_pages: 0,
            md_entries: 0,
            md_root: 0,
        };
        let mut rc: std::ffi::c_int = 0;
        let mut dbflag: std::ffi::c_int = 0;
        let mut exact: std::ffi::c_int = 0;
        let mut unused: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
        let mut seq: std::ffi::c_uint = 0;
        let mut namedup: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        let mut len: size_t = 0;
        if flags
            & !(0x2 as std::ffi::c_int
                | 0x4 as std::ffi::c_int
                | 0x8 as std::ffi::c_int
                | 0x10 as std::ffi::c_int
                | 0x20 as std::ffi::c_int
                | 0x40 as std::ffi::c_int
                | 0x40000 as std::ffi::c_int) as std::ffi::c_uint
            != 0
        {
            return 22 as std::ffi::c_int;
        }
        if (*txn).mt_flags
            & (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int)
                as std::ffi::c_uint
            != 0
        {
            return -(30782 as std::ffi::c_int);
        }
        if name.is_null() {
            *dbi = 1 as std::ffi::c_int as MDB_dbi;
            if flags
                & (0xffff as std::ffi::c_int & !(0x8000 as std::ffi::c_int)) as std::ffi::c_uint
                != 0
            {
                let mut f2: uint16_t = (flags
                    & (0xffff as std::ffi::c_int & !(0x8000 as std::ffi::c_int))
                        as std::ffi::c_uint) as uint16_t;
                if (*((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize)).md_flags
                    as std::ffi::c_int
                    | f2 as std::ffi::c_int
                    != (*((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize)).md_flags
                        as std::ffi::c_int
                {
                    let fresh77 =
                        &mut (*((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize)).md_flags;
                    *fresh77 = (*fresh77 as std::ffi::c_int | f2 as std::ffi::c_int) as uint16_t;
                    (*txn).mt_flags |= 0x4 as std::ffi::c_int as std::ffi::c_uint;
                }
            }
            mdb_default_cmp(txn, 1 as std::ffi::c_int as MDB_dbi);
            return 0 as std::ffi::c_int;
        }
        if ((*((*txn).mt_dbxs).offset(1 as std::ffi::c_int as isize)).md_cmp).is_none() {
            mdb_default_cmp(txn, 1 as std::ffi::c_int as MDB_dbi);
        }
        len = strlen(name);
        i = 2 as std::ffi::c_int as MDB_dbi;
        while i < (*txn).mt_numdbs {
            if (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_size == 0 {
                if unused == 0 {
                    unused = i;
                }
            } else if len == (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_size
                && strncmp(
                    name,
                    (*((*txn).mt_dbxs).offset(i as isize)).md_name.mv_data
                        as *const std::ffi::c_char,
                    len,
                ) == 0
            {
                *dbi = i;
                return 0 as std::ffi::c_int;
            }
            i = i.wrapping_add(1);
        }
        if unused == 0 && (*txn).mt_numdbs >= (*(*txn).mt_env).me_maxdbs {
            return -(30791 as std::ffi::c_int);
        }
        if (*((*txn).mt_dbs).offset(1 as std::ffi::c_int as isize)).md_flags as std::ffi::c_int
            & (0x4 as std::ffi::c_int | 0x8 as std::ffi::c_int)
            != 0
        {
            return if flags & 0x40000 as std::ffi::c_int as std::ffi::c_uint != 0 {
                -(30784 as std::ffi::c_int)
            } else {
                -(30798 as std::ffi::c_int)
            };
        }
        dbflag = 0x4 as std::ffi::c_int | 0x8 as std::ffi::c_int | 0x10 as std::ffi::c_int;
        exact = 0 as std::ffi::c_int;
        key.mv_size = len;
        key.mv_data = name as *mut std::ffi::c_void;
        mdb_cursor_init(
            &mut mc,
            txn,
            1 as std::ffi::c_int as MDB_dbi,
            std::ptr::null_mut::<MDB_xcursor>(),
        );
        rc = mdb_cursor_set(&mut mc, &mut key, &mut data, MDB_SET, &mut exact);
        if rc == 0 as std::ffi::c_int {
            let mut node: *mut MDB_node = (mc.mc_pg[mc.mc_top as usize] as *mut std::ffi::c_char)
                .offset(
                    *((*(mc.mc_pg[mc.mc_top as usize] as *mut std::ffi::c_void as *mut MDB_page2))
                        .mp2_ptrs)
                        .as_mut_ptr()
                        .offset(mc.mc_ki[mc.mc_top as usize] as isize)
                        as std::ffi::c_int as isize,
                )
                .offset(
                    (if 0 as std::ffi::c_int != 0 {
                        16 as std::ffi::c_ulong as std::ffi::c_uint
                    } else {
                        0 as std::ffi::c_int as std::ffi::c_uint
                    }) as isize,
                ) as *mut MDB_node;
            if (*node).mn_flags as std::ffi::c_int
                & (0x4 as std::ffi::c_int | 0x2 as std::ffi::c_int)
                != 0x2 as std::ffi::c_int
            {
                return -(30784 as std::ffi::c_int);
            }
        } else {
            if rc != -(30798 as std::ffi::c_int)
                || flags & 0x40000 as std::ffi::c_int as std::ffi::c_uint == 0
            {
                return rc;
            }
            if (*txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint
                == 0x20000 as std::ffi::c_int as std::ffi::c_uint
            {
                return 13 as std::ffi::c_int;
            }
        }
        namedup = strdup(name);
        if namedup.is_null() {
            return 12 as std::ffi::c_int;
        }
        if rc != 0 {
            data.mv_size = ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong;
            data.mv_data = &mut dummy as *mut MDB_db as *mut std::ffi::c_void;
            memset(
                &mut dummy as *mut MDB_db as *mut std::ffi::c_void,
                0 as std::ffi::c_int,
                ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong,
            );
            dummy.md_root = !(0 as std::ffi::c_int as pgno_t);
            dummy.md_flags = (flags
                & (0xffff as std::ffi::c_int & !(0x8000 as std::ffi::c_int)) as std::ffi::c_uint)
                as uint16_t;
            let mut dummy_0: MDB_cursor = MDB_cursor {
                mc_next: std::ptr::null_mut::<MDB_cursor>(),
                mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                mc_dbi: 0,
                mc_db: std::ptr::null_mut::<MDB_db>(),
                mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                mc_ki: [0; 32],
            };
            let mut tracked: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
            let mut tp: *mut *mut MDB_cursor =
                &mut *((*mc.mc_txn).mt_cursors).offset(mc.mc_dbi as isize) as *mut *mut MDB_cursor;
            if mc.mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0 {
                dummy_0.mc_flags = 0x1 as std::ffi::c_int as std::ffi::c_uint;
                dummy_0.mc_xcursor = &mut mc as *mut MDB_cursor as *mut MDB_xcursor;
                tracked = &mut dummy_0;
            } else {
                tracked = &mut mc;
            }
            (*tracked).mc_next = *tp;
            *tp = tracked;
            rc = _mdb_cursor_put(
                &mut mc,
                &mut key,
                &mut data,
                0x2 as std::ffi::c_int as std::ffi::c_uint,
            );
            *tp = (*tracked).mc_next;
            dbflag |= 0x1 as std::ffi::c_int;
        }
        if rc != 0 {
            free(namedup as *mut std::ffi::c_void);
        } else {
            let mut slot: std::ffi::c_uint = if unused != 0 { unused } else { (*txn).mt_numdbs };
            let fresh78 = &mut (*((*txn).mt_dbxs).offset(slot as isize)).md_name.mv_data;
            *fresh78 = namedup as *mut std::ffi::c_void;
            (*((*txn).mt_dbxs).offset(slot as isize)).md_name.mv_size = len;
            let fresh79 = &mut (*((*txn).mt_dbxs).offset(slot as isize)).md_rel;
            *fresh79 = None;
            *((*txn).mt_dbflags).offset(slot as isize) = dbflag as std::ffi::c_uchar;
            let fresh80 = &mut (*((*(*txn).mt_env).me_dbiseqs).offset(slot as isize));
            *fresh80 = (*fresh80).wrapping_add(1);
            seq = *fresh80;
            *((*txn).mt_dbiseqs).offset(slot as isize) = seq;
            memcpy(
                &mut *((*txn).mt_dbs).offset(slot as isize) as *mut MDB_db as *mut std::ffi::c_void,
                data.mv_data,
                ::core::mem::size_of::<MDB_db>() as std::ffi::c_ulong,
            );
            *dbi = slot;
            mdb_default_cmp(txn, slot);
            if unused == 0 {
                (*txn).mt_numdbs = ((*txn).mt_numdbs).wrapping_add(1);
                (*txn).mt_numdbs;
            }
        }
        rc
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_stat(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut arg: *mut MDB_stat,
) -> std::ffi::c_int {
    unsafe {
        if arg.is_null()
            || !(!txn.is_null()
                && dbi < (*txn).mt_numdbs
                && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                    & 0x8 as std::ffi::c_int
                    != 0)
        {
            return 22 as std::ffi::c_int;
        }
        if (*txn).mt_flags
            & (0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int | 0x10 as std::ffi::c_int)
                as std::ffi::c_uint
            != 0
        {
            return -(30782 as std::ffi::c_int);
        }
        if *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int & 0x2 as std::ffi::c_int
            != 0
        {
            let mut mc: MDB_cursor = MDB_cursor {
                mc_next: std::ptr::null_mut::<MDB_cursor>(),
                mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                mc_dbi: 0,
                mc_db: std::ptr::null_mut::<MDB_db>(),
                mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                mc_ki: [0; 32],
            };
            let mut mx: MDB_xcursor = MDB_xcursor {
                mx_cursor: MDB_cursor {
                    mc_next: std::ptr::null_mut::<MDB_cursor>(),
                    mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                    mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                    mc_txn: std::ptr::null_mut::<MDB_txn>(),
                    mc_dbi: 0,
                    mc_db: std::ptr::null_mut::<MDB_db>(),
                    mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                    mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                    mc_snum: 0,
                    mc_top: 0,
                    mc_flags: 0,
                    mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                    mc_ki: [0; 32],
                },
                mx_db: MDB_db {
                    md_pad: 0,
                    md_flags: 0,
                    md_depth: 0,
                    md_branch_pages: 0,
                    md_leaf_pages: 0,
                    md_overflow_pages: 0,
                    md_entries: 0,
                    md_root: 0,
                },
                mx_dbx: MDB_dbx {
                    md_name: MDB_val {
                        mv_size: 0,
                        mv_data: std::ptr::null_mut::<std::ffi::c_void>(),
                    },
                    md_cmp: None,
                    md_dcmp: None,
                    md_rel: None,
                    md_relctx: std::ptr::null_mut::<std::ffi::c_void>(),
                },
                mx_dbflag: 0,
            };
            mdb_cursor_init(&mut mc, txn, dbi, &mut mx);
        }
        mdb_stat0((*txn).mt_env, &mut *((*txn).mt_dbs).offset(dbi as isize), arg)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_dbi_close(mut env: *mut MDB_env, mut dbi: MDB_dbi) {
    unsafe {
        let mut ptr: *mut std::ffi::c_char = std::ptr::null_mut::<std::ffi::c_char>();
        if dbi < 2 as std::ffi::c_int as MDB_dbi || dbi >= (*env).me_maxdbs {
            return;
        }
        ptr = (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_data as *mut std::ffi::c_char;
        if !ptr.is_null() {
            let fresh81 = &mut (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_data;
            *fresh81 = std::ptr::null_mut::<std::ffi::c_void>();
            (*((*env).me_dbxs).offset(dbi as isize)).md_name.mv_size =
                0 as std::ffi::c_int as size_t;
            *((*env).me_dbflags).offset(dbi as isize) = 0 as std::ffi::c_int as uint16_t;
            let fresh82 = &mut (*((*env).me_dbiseqs).offset(dbi as isize));
            *fresh82 = (*fresh82).wrapping_add(1);
            free(ptr as *mut std::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_dbi_flags(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut flags: *mut std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        if !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                & 0x10 as std::ffi::c_int
                != 0)
        {
            return 22 as std::ffi::c_int;
        }
        *flags = ((*((*txn).mt_dbs).offset(dbi as isize)).md_flags as std::ffi::c_int
            & (0xffff as std::ffi::c_int & !(0x8000 as std::ffi::c_int)))
            as std::ffi::c_uint;
        0 as std::ffi::c_int
    }
}
unsafe extern "C" fn mdb_drop0(
    mut mc: *mut MDB_cursor,
    mut subs: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut rc: std::ffi::c_int = 0;
        rc = mdb_page_search(mc, std::ptr::null_mut::<MDB_val>(), 4 as std::ffi::c_int);
        if rc == 0 as std::ffi::c_int {
            let mut txn: *mut MDB_txn = (*mc).mc_txn;
            let mut ni: *mut MDB_node = std::ptr::null_mut::<MDB_node>();
            let mut mx: MDB_cursor = MDB_cursor {
                mc_next: std::ptr::null_mut::<MDB_cursor>(),
                mc_backup: std::ptr::null_mut::<MDB_cursor>(),
                mc_xcursor: std::ptr::null_mut::<MDB_xcursor>(),
                mc_txn: std::ptr::null_mut::<MDB_txn>(),
                mc_dbi: 0,
                mc_db: std::ptr::null_mut::<MDB_db>(),
                mc_dbx: std::ptr::null_mut::<MDB_dbx>(),
                mc_dbflag: std::ptr::null_mut::<std::ffi::c_uchar>(),
                mc_snum: 0,
                mc_top: 0,
                mc_flags: 0,
                mc_pg: [std::ptr::null_mut::<MDB_page>(); 32],
                mc_ki: [0; 32],
            };
            let mut i: std::ffi::c_uint = 0;
            if (*mc).mc_flags & 0x4 as std::ffi::c_int as std::ffi::c_uint != 0
                || subs == 0 && (*(*mc).mc_db).md_overflow_pages == 0
            {
                mdb_cursor_pop(mc);
            }
            mdb_cursor_copy(mc, &mut mx);
            's_29: loop {
                if (*mc).mc_snum as std::ffi::c_int <= 0 as std::ffi::c_int {
                    current_block = 14447253356787937536;
                    break;
                }
                let mut mp: *mut MDB_page = (*mc).mc_pg[(*mc).mc_top as usize];
                let mut n: std::ffi::c_uint = ((*(mp as *mut std::ffi::c_void as *mut MDB_page2))
                    .mp2_lower as std::ffi::c_uint)
                    .wrapping_sub((16 as std::ffi::c_ulong as std::ffi::c_uint).wrapping_sub(
                        if 0 as std::ffi::c_int != 0 {
                            16 as std::ffi::c_ulong as std::ffi::c_uint
                        } else {
                            0 as std::ffi::c_int as std::ffi::c_uint
                        },
                    ))
                    >> 1 as std::ffi::c_int;
                if (*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags as std::ffi::c_int
                    & 0x2 as std::ffi::c_int
                    == 0x2 as std::ffi::c_int
                {
                    i = 0 as std::ffi::c_int as std::ffi::c_uint;
                    while i < n {
                        ni = (mp as *mut std::ffi::c_char)
                            .offset(
                                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset(i as isize)
                                    as std::ffi::c_int as isize,
                            )
                            .offset(
                                (if 0 as std::ffi::c_int != 0 {
                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                } else {
                                    0 as std::ffi::c_int as std::ffi::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        if (*ni).mn_flags as std::ffi::c_int & 0x1 as std::ffi::c_int != 0 {
                            let mut omp: *mut MDB_page = std::ptr::null_mut::<MDB_page>();
                            let mut pg: pgno_t = 0;
                            memcpy(
                                &mut pg as *mut pgno_t as *mut std::ffi::c_void,
                                ((*ni).mn_data)
                                    .as_mut_ptr()
                                    .offset((*ni).mn_ksize as std::ffi::c_int as isize)
                                    as *mut std::ffi::c_void,
                                ::core::mem::size_of::<pgno_t>() as std::ffi::c_ulong,
                            );
                            rc = mdb_page_get(
                                mc,
                                pg,
                                &mut omp,
                                std::ptr::null_mut::<std::ffi::c_int>(),
                            );
                            if rc != 0 as std::ffi::c_int {
                                current_block = 10688690059288260727;
                                break 's_29;
                            }
                            if (*(omp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_flags
                                as std::ffi::c_int
                                & 0x4 as std::ffi::c_int
                                == 0x4 as std::ffi::c_int
                            {
                            } else {
                                mdb_assert_fail(
                                    (*(*mc).mc_txn).mt_env,
                                    b"IS_OVERFLOW(omp)\0" as *const u8 as *const std::ffi::c_char,
                                    (*::core::mem::transmute::<&[u8; 10], &[std::ffi::c_char; 10]>(
                                        b"mdb_drop0\0",
                                    ))
                                    .as_ptr(),
                                    b"mdb.c\0" as *const u8 as *const std::ffi::c_char,
                                    11096 as std::ffi::c_int,
                                );
                            };
                            rc = mdb_midl_append_range(
                                &mut (*txn).mt_free_pgs,
                                pg,
                                (*omp).mp_pb.pb_pages as usize,
                            );
                            if rc != 0 {
                                current_block = 10688690059288260727;
                                break 's_29;
                            }
                            (*(*mc).mc_db).md_overflow_pages = ((*(*mc).mc_db).md_overflow_pages)
                                .wrapping_sub((*omp).mp_pb.pb_pages as pgno_t);
                            if (*(*mc).mc_db).md_overflow_pages == 0 && subs == 0 {
                                break;
                            }
                        } else if subs != 0
                            && (*ni).mn_flags as std::ffi::c_int & 0x2 as std::ffi::c_int != 0
                        {
                            mdb_xcursor_init1(mc, ni);
                            rc =
                                mdb_drop0(&mut (*(*mc).mc_xcursor).mx_cursor, 0 as std::ffi::c_int);
                            if rc != 0 {
                                current_block = 10688690059288260727;
                                break 's_29;
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                    if subs == 0 && (*(*mc).mc_db).md_overflow_pages == 0 {
                        current_block = 335041022479935549;
                    } else {
                        current_block = 14434620278749266018;
                    }
                } else {
                    rc = mdb_midl_need(&mut (*txn).mt_free_pgs, n as usize);
                    if rc != 0 as std::ffi::c_int {
                        current_block = 10688690059288260727;
                        break;
                    }
                    i = 0 as std::ffi::c_int as std::ffi::c_uint;
                    while i < n {
                        let mut pg_0: pgno_t = 0;
                        ni = (mp as *mut std::ffi::c_char)
                            .offset(
                                *((*(mp as *mut std::ffi::c_void as *mut MDB_page2)).mp2_ptrs)
                                    .as_mut_ptr()
                                    .offset(i as isize)
                                    as std::ffi::c_int as isize,
                            )
                            .offset(
                                (if 0 as std::ffi::c_int != 0 {
                                    16 as std::ffi::c_ulong as std::ffi::c_uint
                                } else {
                                    0 as std::ffi::c_int as std::ffi::c_uint
                                }) as isize,
                            ) as *mut MDB_node;
                        pg_0 = (*ni).mn_lo as pgno_t
                            | ((*ni).mn_hi as pgno_t) << 16 as std::ffi::c_int
                            | (if (if -(1 as std::ffi::c_int) as pgno_t
                                > 0xffffffff as std::ffi::c_uint as pgno_t
                            {
                                32 as std::ffi::c_int
                            } else {
                                0 as std::ffi::c_int
                            }) != 0
                            {
                                ((*ni).mn_flags as pgno_t)
                                    << (if -(1 as std::ffi::c_int) as pgno_t
                                        > 0xffffffff as std::ffi::c_uint as pgno_t
                                    {
                                        32 as std::ffi::c_int
                                    } else {
                                        0 as std::ffi::c_int
                                    })
                            } else {
                                0 as std::ffi::c_int as pgno_t
                            });
                        mdb_midl_xappend(&mut (*txn).mt_free_pgs, pg_0);
                        i = i.wrapping_add(1);
                    }
                    current_block = 14434620278749266018;
                }
                if current_block == 14434620278749266018 {
                    if (*mc).mc_top == 0 {
                        current_block = 14447253356787937536;
                        break;
                    }
                    (*mc).mc_ki[(*mc).mc_top as usize] = i as indx_t;
                    rc = mdb_cursor_sibling(mc, 1 as std::ffi::c_int);
                    if rc == 0 {
                        continue;
                    }
                    if rc != -(30798 as std::ffi::c_int) {
                        current_block = 10688690059288260727;
                        break;
                    }
                }
                mdb_cursor_pop(mc);
                (*mc).mc_ki[0 as std::ffi::c_int as usize] = 0 as std::ffi::c_int as indx_t;
                i = 1 as std::ffi::c_int as std::ffi::c_uint;
                while i < (*mc).mc_snum as std::ffi::c_uint {
                    (*mc).mc_ki[i as usize] = 0 as std::ffi::c_int as indx_t;
                    (*mc).mc_pg[i as usize] = mx.mc_pg[i as usize];
                    i = i.wrapping_add(1);
                }
            }
            if current_block == 14447253356787937536 {
                rc = mdb_midl_append(&mut (*txn).mt_free_pgs, (*(*mc).mc_db).md_root);
            }
            if rc != 0 {
                (*txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
            }
        } else if rc == -(30798 as std::ffi::c_int) {
            rc = 0 as std::ffi::c_int;
        }
        (*mc).mc_flags &= !(0x1 as std::ffi::c_int) as std::ffi::c_uint;
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_drop(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut del: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut mc: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut m2: *mut MDB_cursor = std::ptr::null_mut::<MDB_cursor>();
        let mut rc: std::ffi::c_int = 0;
        if del as std::ffi::c_uint > 1 as std::ffi::c_int as std::ffi::c_uint
            || !(!txn.is_null()
                && dbi < (*txn).mt_numdbs
                && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                    & 0x10 as std::ffi::c_int
                    != 0)
        {
            return 22 as std::ffi::c_int;
        }
        if (*txn).mt_flags & 0x20000 as std::ffi::c_int as std::ffi::c_uint
            == 0x20000 as std::ffi::c_int as std::ffi::c_uint
        {
            return 13 as std::ffi::c_int;
        }
        if *((*txn).mt_dbiseqs).offset(dbi as isize)
            != *((*(*txn).mt_env).me_dbiseqs).offset(dbi as isize)
        {
            return -(30780 as std::ffi::c_int);
        }
        rc = mdb_cursor_open(txn, dbi, &mut mc);
        if rc != 0 {
            return rc;
        }
        rc = mdb_drop0(mc, (*(*mc).mc_db).md_flags as std::ffi::c_int & 0x4 as std::ffi::c_int);
        m2 = *((*txn).mt_cursors).offset(dbi as isize);
        while !m2.is_null() {
            (*m2).mc_flags &=
                !(0x1 as std::ffi::c_int | 0x2 as std::ffi::c_int) as std::ffi::c_uint;
            m2 = (*m2).mc_next;
        }
        if rc == 0 {
            if del != 0 && dbi >= 2 as std::ffi::c_int as MDB_dbi {
                rc = mdb_del0(
                    txn,
                    1 as std::ffi::c_int as MDB_dbi,
                    &mut (*(*mc).mc_dbx).md_name,
                    std::ptr::null_mut::<MDB_val>(),
                    0x2 as std::ffi::c_int as std::ffi::c_uint,
                );
                if rc == 0 {
                    *((*txn).mt_dbflags).offset(dbi as isize) =
                        0x2 as std::ffi::c_int as std::ffi::c_uchar;
                    mdb_dbi_close((*txn).mt_env, dbi);
                } else {
                    (*txn).mt_flags |= 0x2 as std::ffi::c_int as std::ffi::c_uint;
                }
            } else {
                let fresh84 = &mut (*((*txn).mt_dbflags).offset(dbi as isize));
                *fresh84 =
                    (*fresh84 as std::ffi::c_int | 0x1 as std::ffi::c_int) as std::ffi::c_uchar;
                (*((*txn).mt_dbs).offset(dbi as isize)).md_depth = 0 as std::ffi::c_int as uint16_t;
                (*((*txn).mt_dbs).offset(dbi as isize)).md_branch_pages =
                    0 as std::ffi::c_int as pgno_t;
                (*((*txn).mt_dbs).offset(dbi as isize)).md_leaf_pages =
                    0 as std::ffi::c_int as pgno_t;
                (*((*txn).mt_dbs).offset(dbi as isize)).md_overflow_pages =
                    0 as std::ffi::c_int as pgno_t;
                (*((*txn).mt_dbs).offset(dbi as isize)).md_entries =
                    0 as std::ffi::c_int as mdb_size_t;
                (*((*txn).mt_dbs).offset(dbi as isize)).md_root = !(0 as std::ffi::c_int as pgno_t);
                (*txn).mt_flags |= 0x4 as std::ffi::c_int as std::ffi::c_uint;
            }
        }
        mdb_cursor_close(mc);
        rc
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_set_compare(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut cmp: Option<MDB_cmp_func>,
) -> std::ffi::c_int {
    unsafe {
        if !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                & 0x10 as std::ffi::c_int
                != 0)
        {
            return 22 as std::ffi::c_int;
        }
        let fresh85 = &mut (*((*txn).mt_dbxs).offset(dbi as isize)).md_cmp;
        *fresh85 = cmp;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_set_dupsort(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut cmp: Option<MDB_cmp_func>,
) -> std::ffi::c_int {
    unsafe {
        if !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                & 0x10 as std::ffi::c_int
                != 0)
        {
            return 22 as std::ffi::c_int;
        }
        let fresh86 = &mut (*((*txn).mt_dbxs).offset(dbi as isize)).md_dcmp;
        *fresh86 = cmp;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_set_relfunc(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut rel: Option<MDB_rel_func>,
) -> std::ffi::c_int {
    unsafe {
        if !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                & 0x10 as std::ffi::c_int
                != 0)
        {
            return 22 as std::ffi::c_int;
        }
        let fresh87 = &mut (*((*txn).mt_dbxs).offset(dbi as isize)).md_rel;
        *fresh87 = rel;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_set_relctx(
    mut txn: *mut MDB_txn,
    mut dbi: MDB_dbi,
    mut ctx: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    unsafe {
        if !(!txn.is_null()
            && dbi < (*txn).mt_numdbs
            && *((*txn).mt_dbflags).offset(dbi as isize) as std::ffi::c_int
                & 0x10 as std::ffi::c_int
                != 0)
        {
            return 22 as std::ffi::c_int;
        }
        let fresh88 = &mut (*((*txn).mt_dbxs).offset(dbi as isize)).md_relctx;
        *fresh88 = ctx;
        0 as std::ffi::c_int
    }
}

#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_env_get_maxkeysize(mut _env: *mut MDB_env) -> std::ffi::c_int {
    // MDB_MAXKEYSIZE was disabled
    511
}

#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_reader_list(
    mut env: *mut MDB_env,
    mut func: Option<MDB_msg_func>,
    mut ctx: *mut std::ffi::c_void,
) -> std::ffi::c_int {
    unsafe {
        let mut i: std::ffi::c_uint = 0;
        let mut rdrs: std::ffi::c_uint = 0;
        let mut mr: *mut MDB_reader = std::ptr::null_mut::<MDB_reader>();
        let mut buf: [std::ffi::c_char; 64] = [0; 64];
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut first: std::ffi::c_int = 1 as std::ffi::c_int;
        if env.is_null() || func.is_none() {
            return -(1 as std::ffi::c_int);
        }
        if ((*env).me_txns).is_null() {
            return func.expect("non-null function pointer")(
                b"(no reader locks)\n\0" as *const u8 as *const std::ffi::c_char,
                ctx,
            );
        }
        rdrs = (*(*env).me_txns).mt1.mtb.mtb_numreaders;
        mr = ((*(*env).me_txns).mti_readers).as_mut_ptr();
        i = 0 as std::ffi::c_int as std::ffi::c_uint;
        while i < rdrs {
            if (*mr.offset(i as isize)).mru.mrx.mrb_pid != 0 {
                let mut txnid: txnid_t = (*mr.offset(i as isize)).mru.mrx.mrb_txnid;
                sprintf(
                    buf.as_mut_ptr(),
                    if txnid == -(1 as std::ffi::c_int) as txnid_t {
                        b"%10d %zx -\n\0" as *const u8 as *const std::ffi::c_char
                    } else {
                        b"%10d %zx %zu\n\0" as *const u8 as *const std::ffi::c_char
                    },
                    (*mr.offset(i as isize)).mru.mrx.mrb_pid,
                    (*mr.offset(i as isize)).mru.mrx.mrb_tid as size_t,
                    txnid,
                );
                if first != 0 {
                    first = 0 as std::ffi::c_int;
                    rc = func.expect("non-null function pointer")(
                        b"    pid     thread     txnid\n\0" as *const u8 as *const std::ffi::c_char,
                        ctx,
                    );
                    if rc < 0 as std::ffi::c_int {
                        break;
                    }
                }
                rc = func.expect("non-null function pointer")(buf.as_mut_ptr(), ctx);
                if rc < 0 as std::ffi::c_int {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
        if first != 0 {
            rc = func.expect("non-null function pointer")(
                b"(no active readers)\n\0" as *const u8 as *const std::ffi::c_char,
                ctx,
            );
        }
        rc
    }
}
#[cold]
unsafe extern "C" fn mdb_pid_insert(mut ids: *mut pid_t, mut pid: pid_t) -> std::ffi::c_int {
    unsafe {
        let mut base: std::ffi::c_uint = 0 as std::ffi::c_int as std::ffi::c_uint;
        let mut cursor: std::ffi::c_uint = 1 as std::ffi::c_int as std::ffi::c_uint;
        let mut val: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut n: std::ffi::c_uint =
            *ids.offset(0 as std::ffi::c_int as isize) as std::ffi::c_uint;
        while (0 as std::ffi::c_int as std::ffi::c_uint) < n {
            let mut pivot: std::ffi::c_uint = n >> 1 as std::ffi::c_int;
            cursor =
                base.wrapping_add(pivot).wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint);
            val = pid - *ids.offset(cursor as isize);
            if val < 0 as std::ffi::c_int {
                n = pivot;
            } else if val > 0 as std::ffi::c_int {
                base = cursor;
                n = n.wrapping_sub(pivot.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint));
            } else {
                return -(1 as std::ffi::c_int);
            }
        }
        if val > 0 as std::ffi::c_int {
            cursor = cursor.wrapping_add(1);
        }
        let fresh89 = &mut (*ids.offset(0 as std::ffi::c_int as isize));
        *fresh89 += 1;
        n = *ids.offset(0 as std::ffi::c_int as isize) as std::ffi::c_uint;
        while n > cursor {
            *ids.offset(n as isize) =
                *ids.offset(n.wrapping_sub(1 as std::ffi::c_int as std::ffi::c_uint) as isize);
            n = n.wrapping_sub(1);
        }
        *ids.offset(n as isize) = pid;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
#[cold]
pub unsafe extern "C" fn mdb_reader_check(
    mut env: *mut MDB_env,
    mut dead: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        if env.is_null() {
            return 22 as std::ffi::c_int;
        }
        if !dead.is_null() {
            *dead = 0 as std::ffi::c_int;
        }
        if !((*env).me_txns).is_null() {
            mdb_reader_check0(env, 0 as std::ffi::c_int, dead)
        } else {
            0 as std::ffi::c_int
        }
    }
}
#[cold]
unsafe extern "C" fn mdb_reader_check0(
    mut env: *mut MDB_env,
    mut rlocked: std::ffi::c_int,
    mut dead: *mut std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut rmutex: mdb_mutexref_t = if rlocked != 0 {
            std::ptr::null_mut::<mdb_mutex>()
        } else {
            ((*env).me_rmutex).as_mut_ptr()
        };
        let mut i: std::ffi::c_uint = 0;
        let mut j: std::ffi::c_uint = 0;
        let mut rdrs: std::ffi::c_uint = 0;
        let mut mr: *mut MDB_reader = std::ptr::null_mut::<MDB_reader>();
        let mut pids: *mut pid_t = std::ptr::null_mut::<pid_t>();
        let mut pid: pid_t = 0;
        let mut rc: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut count: std::ffi::c_int = 0 as std::ffi::c_int;
        rdrs = (*(*env).me_txns).mt1.mtb.mtb_numreaders;
        pids = malloc(
            (rdrs.wrapping_add(1 as std::ffi::c_int as std::ffi::c_uint) as std::ffi::c_ulong)
                .wrapping_mul(::core::mem::size_of::<pid_t>() as std::ffi::c_ulong),
        ) as *mut pid_t;
        if pids.is_null() {
            return 12 as std::ffi::c_int;
        }
        *pids.offset(0 as std::ffi::c_int as isize) = 0 as std::ffi::c_int;
        mr = ((*(*env).me_txns).mti_readers).as_mut_ptr();
        i = 0 as std::ffi::c_int as std::ffi::c_uint;
        while i < rdrs {
            pid = (*mr.offset(i as isize)).mru.mrx.mrb_pid;
            if pid != 0
                && pid != (*env).me_pid
                && mdb_pid_insert(pids, pid) == 0 as std::ffi::c_int
                && mdb_reader_pid(env, Pidcheck, pid) == 0
            {
                j = i;
                if !rmutex.is_null() {
                    rc = mdb_sem_wait(rmutex);
                    if rc != 0 as std::ffi::c_int {
                        rc = mdb_mutex_failed(env, rmutex, rc);
                        if rc != 0 {
                            break;
                        }
                        rdrs = 0 as std::ffi::c_int as std::ffi::c_uint;
                    } else if mdb_reader_pid(env, Pidcheck, pid) != 0 {
                        j = rdrs;
                    }
                }
                while j < rdrs {
                    if (*mr.offset(j as isize)).mru.mrx.mrb_pid == pid {
                        ::core::ptr::write_volatile(
                            &mut (*mr.offset(j as isize)).mru.mrx.mrb_pid as *mut pid_t,
                            0 as std::ffi::c_int,
                        );
                        count += 1;
                    }
                    j = j.wrapping_add(1);
                }
                if !rmutex.is_null() {
                    let mut sb: sembuf = {
                        sembuf {
                            sem_num: 0 as std::ffi::c_int as std::ffi::c_ushort,
                            sem_op: 1 as std::ffi::c_int as std::ffi::c_short,
                            sem_flg: 0o10000 as std::ffi::c_int as std::ffi::c_short,
                        }
                    };
                    sb.sem_num = (*rmutex).semnum as std::ffi::c_ushort;
                    *(*rmutex).locked = 0 as std::ffi::c_int;
                    semop((*rmutex).semid, &mut sb, 1 as std::ffi::c_int as size_t);
                }
            }
            i = i.wrapping_add(1);
        }
        free(pids as *mut std::ffi::c_void);
        if !dead.is_null() {
            *dead = count;
        }
        rc
    }
}
#[cold]
unsafe extern "C" fn mdb_mutex_failed(
    mut env: *mut MDB_env,
    mut mutex: mdb_mutexref_t,
    mut rc: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut rlocked: std::ffi::c_int = 0;
        let mut rc2: std::ffi::c_int = 0;
        let mut meta: *mut MDB_meta = std::ptr::null_mut::<MDB_meta>();
        if rc == -(30779 as std::ffi::c_int) + 11 as std::ffi::c_int {
            rc = 0 as std::ffi::c_int;
            rlocked = (mutex == ((*env).me_rmutex).as_mut_ptr()) as std::ffi::c_int;
            if rlocked == 0 {
                meta = mdb_env_pick_meta(env);
                ::core::ptr::write_volatile(
                    &mut (*(*env).me_txns).mt1.mtb.mtb_txnid as *mut txnid_t,
                    (*meta).mm_txnid,
                );
                if !((*env).me_txn).is_null() {
                    (*env).me_flags |= 0x80000000 as std::ffi::c_uint;
                    (*env).me_txn = std::ptr::null_mut::<MDB_txn>();
                    rc = -(30795 as std::ffi::c_int);
                }
            }
            rc2 = mdb_reader_check0(env, rlocked, std::ptr::null_mut::<std::ffi::c_int>());
            if rc2 == 0 as std::ffi::c_int {
                rc2 = 0 as std::ffi::c_int;
            }
            if rc != 0 || {
                rc = rc2;
                rc != 0
            } {
                let mut sb: sembuf = {
                    sembuf {
                        sem_num: 0 as std::ffi::c_int as std::ffi::c_ushort,
                        sem_op: 1 as std::ffi::c_int as std::ffi::c_short,
                        sem_flg: 0o10000 as std::ffi::c_int as std::ffi::c_short,
                    }
                };
                sb.sem_num = (*mutex).semnum as std::ffi::c_ushort;
                *(*mutex).locked = 0 as std::ffi::c_int;
                semop((*mutex).semid, &mut sb, 1 as std::ffi::c_int as size_t);
            }
        }
        rc
    }
}
