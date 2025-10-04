#![feature(extern_types)]

mod midl;

#[cfg(all(target_endian = "little", target_os = "macos", target_pointer_width = "64"))]
mod mdb_macos_64bit_m1;

use bitflags::bitflags;
#[cfg(all(target_endian = "little", target_os = "macos", target_pointer_width = "64"))]
pub use mdb_macos_64bit_m1::*;

bitflags! {
    /// mdb_env: Environment Flags
    pub struct EnvironmentFlags: u32 {
        /// mmap at a fixed address (experimental)
        const MDB_FIXEDMAP = 0x01;
        /// no environment directory
        const MDB_NOSUBDIR = 0x4000;
        /// don't fsync after commit
        const MDB_NOSYNC = 0x10000;
        /// read only
        const MDB_RDONLY = 0x20000;
        /// don't fsync metapage after commit
        const MDB_NOMETASYNC = 0x40000;
        /// use writable mmap
        const MDB_WRITEMAP = 0x80000;
        /// use asynchronous msync when #MDB_WRITEMAP is used
        const MDB_MAPASYNC = 0x100000;
        /// tie reader locktable slots to #MDB_txn objects instead of to threads
        const MDB_NOTLS = 0x200000;
        /// don't do any locking, caller must manage their own locks
        const MDB_NOLOCK = 0x400000;
        /// don't do readahead (no effect on Windows)
        const MDB_NORDAHEAD = 0x800000;
        /// don't initialize malloc'd memory before writing to datafile
        const MDB_NOMEMINIT = 0x1000000;
        /// use the previous snapshot rather than the latest one
        const MDB_PREVSNAPSHOT = 0x2000000;
    }
}

pub const MDB_FIXEDMAP: u32 = EnvironmentFlags::MDB_FIXEDMAP.bits();
pub const MDB_NOSUBDIR: u32 = EnvironmentFlags::MDB_NOSUBDIR.bits();
pub const MDB_NOSYNC: u32 = EnvironmentFlags::MDB_NOSYNC.bits();
pub const MDB_RDONLY: u32 = EnvironmentFlags::MDB_RDONLY.bits();
pub const MDB_NOMETASYNC: u32 = EnvironmentFlags::MDB_NOMETASYNC.bits();
pub const MDB_WRITEMAP: u32 = EnvironmentFlags::MDB_WRITEMAP.bits();
pub const MDB_MAPASYNC: u32 = EnvironmentFlags::MDB_MAPASYNC.bits();
pub const MDB_NOTLS: u32 = EnvironmentFlags::MDB_NOTLS.bits();
pub const MDB_NOLOCK: u32 = EnvironmentFlags::MDB_NOLOCK.bits();
pub const MDB_NORDAHEAD: u32 = EnvironmentFlags::MDB_NORDAHEAD.bits();
pub const MDB_NOMEMINIT: u32 = EnvironmentFlags::MDB_NOMEMINIT.bits();
pub const MDB_PREVSNAPSHOT: u32 = EnvironmentFlags::MDB_PREVSNAPSHOT.bits();

bitflags! {
    /// mdb_dbi_open: Database Flags
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    pub struct DatabaseFlags: u32 {
        /// use reverse string keys
        const MDB_REVERSEKEY = 0x02;
        /// use sorted duplicates
        const MDB_DUPSORT = 0x04;
        /// numeric keys in native byte order, either unsigned int or #mdb_size_t.
        /// (lmdb expects 32-bit int <= size_t <= 32/64-bit mdb_size_t.)
        /// The keys must all be of the same size.
        const MDB_INTEGERKEY = 0x08;
        /// with #MDB_DUPSORT, sorted dup items have fixed size
        const MDB_DUPFIXED = 0x10;
        /// with #MDB_DUPSORT, dups are #MDB_INTEGERKEY-style integers
        const MDB_INTEGERDUP = 0x20;
        /// with #MDB_DUPSORT, use reverse string dups
        const MDB_REVERSEDUP = 0x40;
        /// create DB if not already existing
        const MDB_CREATE = 0x40000;

        // Note: Can we hide them somehow when we will expose
        //       DatabaseFlags as an actual, idiomatic Rust bitflags?
        /// DB handle is valid, for me_dbflags
        const MDB_VALID = 0x8000;
        /// An internal flag to make sure we do not keep flags outside
        /// of a given range internally. It seems that it is only relevant
        /// to the MDB_CREATE flag.
        const PERSISTENT_FLAGS = 0xffff & !DatabaseFlags::MDB_VALID.bits();
    }
}

impl DatabaseFlags {
    pub fn is_valid(&self) -> bool {
        self.bits()
            & !(DatabaseFlags::MDB_REVERSEKEY
                | DatabaseFlags::MDB_DUPSORT
                | DatabaseFlags::MDB_INTEGERKEY
                | DatabaseFlags::MDB_DUPFIXED
                | DatabaseFlags::MDB_INTEGERDUP
                | DatabaseFlags::MDB_REVERSEDUP
                | DatabaseFlags::MDB_CREATE)
                .bits()
            == 0
    }
}

bitflags! {
    /// mdb_dbi_open: Database Flags
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    pub struct InternalDatabaseFlags: u16 {
        /// use reverse string keys
        const MDB_REVERSEKEY = DatabaseFlags::MDB_REVERSEKEY.bits() as u16;
        /// use sorted duplicates
        const MDB_DUPSORT = DatabaseFlags::MDB_DUPSORT.bits() as u16;
        /// numeric keys in native byte order, either unsigned int or #mdb_size_t.
        /// (lmdb expects 32-bit int <= size_t <= 32/64-bit mdb_size_t.)
        /// The keys must all be of the same size.
        const MDB_INTEGERKEY = DatabaseFlags::MDB_INTEGERKEY.bits() as u16;
        /// with #MDB_DUPSORT, sorted dup items have fixed size
        const MDB_DUPFIXED = DatabaseFlags::MDB_DUPFIXED.bits() as u16;
        /// with #MDB_DUPSORT, dups are #MDB_INTEGERKEY-style integers
        const MDB_INTEGERDUP = DatabaseFlags::MDB_INTEGERDUP.bits() as u16;
        /// with #MDB_DUPSORT, use reverse string dups
        const MDB_REVERSEDUP = DatabaseFlags::MDB_REVERSEDUP.bits() as u16;

        // Note: Can we hide them somehow when we will expose
        //       DatabaseFlags as an actual, idiomatic Rust bitflags?
        /// DB handle is valid, for me_dbflags
        const MDB_VALID = 0x8000;
    }
}

pub const MDB_REVERSEKEY: u32 = DatabaseFlags::MDB_REVERSEKEY.bits();
pub const MDB_DUPSORT: u32 = DatabaseFlags::MDB_DUPSORT.bits();
pub const MDB_INTEGERKEY: u32 = DatabaseFlags::MDB_INTEGERKEY.bits();
pub const MDB_DUPFIXED: u32 = DatabaseFlags::MDB_DUPFIXED.bits();
pub const MDB_INTEGERDUP: u32 = DatabaseFlags::MDB_INTEGERDUP.bits();
pub const MDB_REVERSEDUP: u32 = DatabaseFlags::MDB_REVERSEDUP.bits();
pub const MDB_CREATE: u32 = DatabaseFlags::MDB_CREATE.bits();

bitflags! {
    /// mdb_put: Write Flags
    pub struct WriteFlags: u32 {
        /// For put: Don't write if the key already exists. */
        const MDB_NOOVERWRITE = 0x10;
        /// Only for #MDB_DUPSORT<br>
        /// For put: don't write if the key and data pair already exist.<br>
        /// For mdb_cursor_del: remove all duplicate data items.
        const MDB_NODUPDATA = 0x20;
        /// For mdb_cursor_put: overwrite the current key/data pair */
        const MDB_CURRENT = 0x40;
        /// For put: Just reserve space for data, don't copy it. Return a
        /// pointer to the reserved space.
        const MDB_RESERVE = 0x10000;
        /// Data is being appended, don't split full pages. */
        const MDB_APPEND = 0x20000;
        /// Duplicate data is being appended, don't split full pages. */
        const MDB_APPENDDUP = 0x40000;
        /// Store multiple data items in one call. Only for #MDB_DUPFIXED. */
        const MDB_MULTIPLE = 0x80000;
    }
}

pub const MDB_NOOVERWRITE: u32 = WriteFlags::MDB_NOOVERWRITE.bits();
pub const MDB_NODUPDATA: u32 = WriteFlags::MDB_NODUPDATA.bits();
pub const MDB_CURRENT: u32 = WriteFlags::MDB_CURRENT.bits();
pub const MDB_RESERVE: u32 = WriteFlags::MDB_RESERVE.bits();
pub const MDB_APPEND: u32 = WriteFlags::MDB_APPEND.bits();
pub const MDB_APPENDDUP: u32 = WriteFlags::MDB_APPENDDUP.bits();
pub const MDB_MULTIPLE: u32 = WriteFlags::MDB_MULTIPLE.bits();

bitflags! {
    /// mdb_copy: Copy Flags
    pub struct CopyFlags: u32 {
        /// Compacting copy: Omit free space from copy, and renumber all
        /// pages sequentially.
        const MDB_CP_COMPACT = 0x01;
    }
}

pub const MDB_CP_COMPACT: u32 = CopyFlags::MDB_CP_COMPACT.bits();

/// Cursor Get operations.
///
/// This is the set of all operations for retrieving data
/// using a cursor.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum MDB_cursor_op {
    /// Position at first key/data item.
    MDB_FIRST = 0,
    /// Position at first data item of current key. Only for #MDB_DUPSORT.
    MDB_FIRST_DUP,
    /// Position at key/data pair. Only for #MDB_DUPSORT.
    MDB_GET_BOTH,
    /// Position at key, nearest data. Only for #MDB_DUPSORT.
    MDB_GET_BOTH_RANGE,
    /// Return key/data at current cursor position.
    MDB_GET_CURRENT,
    /// Return up to a page of duplicate data items
    /// from current cursor position. Move cursor to prepare
    /// for #MDB_NEXT_MULTIPLE. Only for #MDB_DUPFIXED.
    MDB_GET_MULTIPLE,
    /// Position at last key/data item.
    MDB_LAST,
    /// Position at last data item of current key. Only for #MDB_DUPSORT.
    MDB_LAST_DUP,
    /// Position at next data item.
    MDB_NEXT,
    /// Position at next data item of current key. Only for #MDB_DUPSORT.
    MDB_NEXT_DUP,
    /// Return up to a page of duplicate data items
    /// from next cursor position. Move cursor to prepare
    /// for #MDB_NEXT_MULTIPLE. Only for #MDB_DUPFIXED.
    MDB_NEXT_MULTIPLE,
    /// Position at first data item of next key.
    MDB_NEXT_NODUP,
    /// Position at previous data item.
    MDB_PREV,
    /// Position at previous data item of current key. Only for #MDB_DUPSORT.
    MDB_PREV_DUP,
    /// Position at last data item of previous key.
    MDB_PREV_NODUP,
    /// Position at specified key.
    MDB_SET,
    /// Position at specified key, return key + data.
    MDB_SET_KEY,
    /// Position at first key greater than or equal to specified key..
    MDB_SET_RANGE,
    /// Position at previous page and return up to
    /// a page of duplicate data items. Only for #MDB_DUPFIXED.
    MDB_PREV_MULTIPLE,
}

pub const MDB_FIRST: MDB_cursor_op = MDB_cursor_op::MDB_FIRST;
pub const MDB_FIRST_DUP: MDB_cursor_op = MDB_cursor_op::MDB_FIRST_DUP;
pub const MDB_GET_BOTH: MDB_cursor_op = MDB_cursor_op::MDB_GET_BOTH;
pub const MDB_GET_BOTH_RANGE: MDB_cursor_op = MDB_cursor_op::MDB_GET_BOTH_RANGE;
pub const MDB_GET_CURRENT: MDB_cursor_op = MDB_cursor_op::MDB_GET_CURRENT;
pub const MDB_GET_MULTIPLE: MDB_cursor_op = MDB_cursor_op::MDB_GET_MULTIPLE;
pub const MDB_LAST: MDB_cursor_op = MDB_cursor_op::MDB_LAST;
pub const MDB_LAST_DUP: MDB_cursor_op = MDB_cursor_op::MDB_LAST_DUP;
pub const MDB_NEXT: MDB_cursor_op = MDB_cursor_op::MDB_NEXT;
pub const MDB_NEXT_DUP: MDB_cursor_op = MDB_cursor_op::MDB_NEXT_DUP;
pub const MDB_NEXT_MULTIPLE: MDB_cursor_op = MDB_cursor_op::MDB_NEXT_MULTIPLE;
pub const MDB_NEXT_NODUP: MDB_cursor_op = MDB_cursor_op::MDB_NEXT_NODUP;
pub const MDB_PREV: MDB_cursor_op = MDB_cursor_op::MDB_PREV;
pub const MDB_PREV_DUP: MDB_cursor_op = MDB_cursor_op::MDB_PREV_DUP;
pub const MDB_PREV_NODUP: MDB_cursor_op = MDB_cursor_op::MDB_PREV_NODUP;
pub const MDB_SET: MDB_cursor_op = MDB_cursor_op::MDB_SET;
pub const MDB_SET_KEY: MDB_cursor_op = MDB_cursor_op::MDB_SET_KEY;
pub const MDB_SET_RANGE: MDB_cursor_op = MDB_cursor_op::MDB_SET_RANGE;
pub const MDB_PREV_MULTIPLE: MDB_cursor_op = MDB_cursor_op::MDB_PREV_MULTIPLE;

/// errors: Return Codes
///
/// BerkeleyDB uses -30800 to -30999, we'll go under them
#[repr(i32)]
pub enum ReturnCode {
    /// Successful result.
    MdbSuccess = 0,
    /// key/data pair already exists.
    MdbKeyexist = -30799,
    /// key/data pair not found (EOF).
    MdbNotfound = -30798,
    /// Requested page not found - this usually indicates corruption.
    MdbPageNotfound = -30797,
    /// Located page was wrong type.
    MdbCorrupted = -30796,
    /// Update of meta page failed or environment had fatal error.
    MdbPanic = -30795,
    /// Environment version mismatch.
    MdbVersionMismatch = -30794,
    /// File is not a valid LMDB file.
    MdbInvalid = -30793,
    /// Environment mapsize reached.
    MdbMapFull = -30792,
    /// Environment maxdbs reached.
    MdbDbsFull = -30791,
    /// Environment maxreaders reached.
    MdbReadersFull = -30790,
    /// Too many TLS keys in use - Windows only.
    MdbTlsFull = -30789,
    /// Txn has too many dirty pages.
    MdbTxnFull = -30788,
    /// Cursor stack too deep - internal error.
    MdbCursorFull = -30787,
    /// Page has not enough space - internal error.
    MdbPageFull = -30786,
    /// Database contents grew beyond environment mapsize.
    MdbMapResized = -30785,
    /// Operation and DB incompatible, or DB type changed. This can mean:
    /// - The operation expects an #MDB_DUPSORT / #MDB_DUPFIXED database.
    /// - Opening a named DB when the unnamed DB has #MDB_DUPSORT / #MDB_INTEGERKEY.
    /// - Accessing a data record as a database, or vice versa.
    /// - The database was dropped and recreated with different flags.
    MdbIncompatible = -30784,
    /// Invalid reuse of reader locktable slot
    MdbBadRslot = -30783,
    /// Transaction must abort, has a child, or is invalid
    MdbBadTxn = -30782,
    /// Unsupported size of key/DB name/data, or wrong DUPFIXED size
    MdbBadValsize = -30781,
    /// The specified DBI was changed unexpectedly
    MdbBadDbi = -30780,
    /// Unexpected problem - txn should abort
    MdbProblem = -30779,
}

pub const MDB_SUCCESS: i32 = ReturnCode::MdbSuccess as i32;
pub const MDB_KEYEXIST: i32 = ReturnCode::MdbKeyexist as i32;
pub const MDB_NOTFOUND: i32 = ReturnCode::MdbNotfound as i32;
pub const MDB_PAGE_NOTFOUND: i32 = ReturnCode::MdbPageNotfound as i32;
pub const MDB_CORRUPTED: i32 = ReturnCode::MdbCorrupted as i32;
pub const MDB_PANIC: i32 = ReturnCode::MdbPanic as i32;
pub const MDB_VERSION_MISMATCH: i32 = ReturnCode::MdbVersionMismatch as i32;
pub const MDB_INVALID: i32 = ReturnCode::MdbInvalid as i32;
pub const MDB_MAP_FULL: i32 = ReturnCode::MdbMapFull as i32;
pub const MDB_DBS_FULL: i32 = ReturnCode::MdbDbsFull as i32;
pub const MDB_READERS_FULL: i32 = ReturnCode::MdbReadersFull as i32;
pub const MDB_TLS_FULL: i32 = ReturnCode::MdbTlsFull as i32;
pub const MDB_TXN_FULL: i32 = ReturnCode::MdbTxnFull as i32;
pub const MDB_CURSOR_FULL: i32 = ReturnCode::MdbCursorFull as i32;
pub const MDB_PAGE_FULL: i32 = ReturnCode::MdbPageFull as i32;
pub const MDB_MAP_RESIZED: i32 = ReturnCode::MdbMapResized as i32;
pub const MDB_INCOMPATIBLE: i32 = ReturnCode::MdbIncompatible as i32;
pub const MDB_BAD_RSLOT: i32 = ReturnCode::MdbBadRslot as i32;
pub const MDB_BAD_TXN: i32 = ReturnCode::MdbBadTxn as i32;
pub const MDB_BAD_VALSIZE: i32 = ReturnCode::MdbBadValsize as i32;
pub const MDB_BAD_DBI: i32 = ReturnCode::MdbBadDbi as i32;
pub const MDB_PROBLEM: i32 = ReturnCode::MdbProblem as i32;
pub const MDB_LAST_ERRCODE: i32 = ReturnCode::MdbProblem as i32;

/// Handle for the DB used to track free pages.
const FREE_DBI: MDB_dbi = 0;
/// Handle for the default DB.
const MAIN_DBI: MDB_dbi = 1;
/// Number of DBs in metapage (free and main) - also hardcoded elsewhere
const CORE_DBS: u32 = 2;
/// Number of meta pages - also hardcoded elsewhere
const NUM_METAS: usize = 2;
