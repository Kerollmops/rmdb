#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

pub type size_t = usize;
pub type mdb_size_t = size_t;

use ::core::mem::size_of;
use libc::ENOMEM;

/// A generic unsigned ID number. These were entryIDs in back-bdb.
/// Preferably it should have the same size as a pointer.
pub type MDB_ID = usize;

/// An IDL is an ID List, a sorted array of IDs. The first
/// element of the array is a counter for how many actual
/// IDs are in the list. In the original back-bdb code, IDLs are
/// sorted in ascending order. For libmdb IDLs are sorted in
/// descending order.
pub type MDB_IDL = *mut MDB_ID;

// IDL sizes - likely should be even bigger
// limiting factors: sizeof(ID), thread stack size
/// DB_SIZE is 2^16, UM_SIZE is 2^17
const MDB_IDL_LOGN: i32 = 16;
const MDB_IDL_DB_SIZE: i32 = 1 << MDB_IDL_LOGN;
const MDB_IDL_UM_SIZE: i32 = 1 << (MDB_IDL_LOGN + 1);

const MDB_IDL_DB_MAX: i32 = MDB_IDL_DB_SIZE - 1;
const MDB_IDL_UM_MAX: i32 = MDB_IDL_UM_SIZE - 1;

/// An ID2 is an ID/pointer pair.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MDB_ID2 {
    /// The ID.
    pub mid: MDB_ID,
    /// The pointer.
    pub mptr: *mut std::ffi::c_void,
}

/// An ID2L is an ID2 List, a sorted array of ID2s.
///
/// The first element's \b mid member is a count of how many actual
/// elements are in the array. The \b mptr member of the first element is unused.
/// The array is sorted in ascending order by \b mid.
pub type MDB_ID2L = *mut MDB_ID2;

// /** Append ID to IDL. The IDL must be big enough. */
// #define mdb_midl_xappend(idl, id) do { \
// 		MDB_ID *xidl = (idl), xlen = ++(xidl[0]); \
// 		xidl[xlen] = (id); \
// 	} while (0)

/// Search for an ID in an IDL.
///
/// @param[in] ids	The IDL to search.
/// @param[in] id	The ID to search for.
/// @return	The index of the first ID greater than or equal to \b id.
pub unsafe fn mdb_midl_search(ids: MDB_IDL, id: MDB_ID) -> u32 {
    let slice = unsafe {
        let len = ids.read();
        std::slice::from_raw_parts(ids.offset(1), len)
    };

    // binary search of id in ids
    // if found, returns position of id
    // if not found, returns first position greater than id
    match slice.binary_search_by(|m| m.cmp(&id).reverse()) {
        Ok(pos) => pos as u32 + 1,
        Err(pos) => pos as u32 + 1,
    }
}

/// Allocate an IDL.
///
/// Allocates memory for an IDL of the given size.
/// @return	IDL on success, NULL on failure.
pub unsafe fn mdb_midl_alloc(mut num: u32) -> MDB_IDL {
    unsafe {
        let mut ids = libc::malloc((num as usize + 2).wrapping_mul(size_of::<MDB_ID>())) as MDB_IDL;
        if !ids.is_null() {
            let fresh0 = ids;
            ids = ids.offset(1);
            *fresh0 = num as MDB_ID;
            *ids = 0 as MDB_ID;
        }
        ids
    }
}

/// Free an IDL.
///
/// @param[in] ids	The IDL to free.
pub unsafe fn mdb_midl_free(mut ids: MDB_IDL) {
    if !ids.is_null() {
        unsafe { libc::free(ids.offset(-1) as *mut std::ffi::c_void) }
    }
}

/// Shrink an IDL.
///
/// Return the IDL to the default size if it has grown larger.
/// @param[in,out] idp	Address of the IDL to shrink.
pub unsafe fn mdb_midl_shrink(mut idp: *mut MDB_IDL) {
    unsafe {
        let mut ids: MDB_IDL = *idp;
        ids = ids.offset(-1);
        if *ids > (((1) << (MDB_IDL_LOGN + 1)) - 1) as MDB_ID && {
            ids = libc::realloc(
                ids as *mut std::ffi::c_void,
                ((((1) << (MDB_IDL_LOGN + 1)) - 1 + 2) as size_t).wrapping_mul(size_of::<MDB_ID>()),
            ) as MDB_IDL;
            !ids.is_null()
        } {
            let fresh1 = ids;
            ids = ids.offset(1);
            *fresh1 = (((1) << (MDB_IDL_LOGN + 1)) - 1) as MDB_ID;
            *idp = ids;
        }
    }
}

unsafe fn mdb_midl_grow(mut idp: *mut MDB_IDL, mut num: u32) -> i32 {
    unsafe {
        let mut idn: MDB_IDL = (*idp).offset(-1);
        idn = libc::realloc(
            idn as *mut std::ffi::c_void,
            (idn.read())
                .wrapping_add(num as usize)
                .wrapping_add(2)
                .wrapping_mul(size_of::<MDB_ID>()),
        ) as MDB_IDL;
        if idn.is_null() {
            return ENOMEM;
        }
        let fresh2 = idn;
        idn = idn.offset(1);
        *fresh2 = (*fresh2).wrapping_add(num as usize);
        *idp = idn;
        0
    }
}

/// Make room for num additional elements in an IDL.
///
/// @param[in,out] idp	Address of the IDL.
/// @param[in] num	Number of elements to make room for.
/// @return	0 on success, ENOMEM on failure.
pub unsafe fn mdb_midl_need(mut idp: *mut MDB_IDL, mut num: std::ffi::c_uint) -> std::ffi::c_int {
    unsafe {
        let mut ids: MDB_IDL = *idp;
        num = (num as std::ffi::c_ulong)
            .wrapping_add(*ids.offset(0 as std::ffi::c_int as isize) as std::ffi::c_ulong)
            as std::ffi::c_uint as std::ffi::c_uint;
        if num as MDB_ID > *ids.offset(-(1 as std::ffi::c_int) as isize) {
            num = num
                .wrapping_add(num.wrapping_div(4 as std::ffi::c_uint))
                .wrapping_add((256 as std::ffi::c_int + 2 as std::ffi::c_int) as std::ffi::c_uint)
                & -(256 as std::ffi::c_int) as std::ffi::c_uint;
            ids = libc::realloc(
                ids.offset(-(1 as std::ffi::c_int as isize)) as *mut std::ffi::c_void,
                (num as size_t).wrapping_mul(size_of::<MDB_ID>() as size_t),
            ) as MDB_IDL;
            if ids.is_null() {
                return ENOMEM;
            }
            let fresh3 = ids;
            ids = ids.offset(1);
            *fresh3 = num.wrapping_sub(2 as std::ffi::c_uint) as MDB_ID;
            *idp = ids;
        }
        0 as std::ffi::c_int
    }
}

/// Append an ID onto an IDL.
///
/// @param[in,out] idp	Address of the IDL to append to.
/// @param[in] id	The ID to append.
/// @return	0 on success, ENOMEM if the IDL is too large.
pub unsafe fn mdb_midl_append(mut idp: *mut MDB_IDL, mut id: MDB_ID) -> i32 {
    unsafe {
        let mut ids: MDB_IDL = *idp;
        if *ids.offset(0) >= *ids.offset(-1) {
            if mdb_midl_grow(idp, ((1) << (MDB_IDL_LOGN + 1)) - 1) != 0 {
                return ENOMEM;
            }
            ids = *idp;
        }
        let fresh4 = &mut (*ids.offset(0));
        *fresh4 = (*fresh4).wrapping_add(1);
        *ids.offset(*ids.offset(0) as isize) = id;
        0
    }
}

/// Append an IDL onto an IDL.
///
/// @param[in,out] idp	Address of the IDL to append to.
/// @param[in] app	The IDL to append.
/// @return	0 on success, ENOMEM if the IDL is too large.
pub unsafe fn mdb_midl_append_list(mut idp: *mut MDB_IDL, mut app: MDB_IDL) -> std::ffi::c_int {
    unsafe {
        let mut ids: MDB_IDL = *idp;
        if (*ids.offset(0 as std::ffi::c_int as isize))
            .wrapping_add(*app.offset(0 as std::ffi::c_int as isize))
            >= *ids.offset(-(1 as std::ffi::c_int) as isize)
        {
            if mdb_midl_grow(idp, *app.offset(0 as isize) as u32) != 0 {
                return ENOMEM;
            }
            ids = *idp;
        }
        std::ptr::copy_nonoverlapping(
            // &app[1]
            &mut *app.offset(1),
            // &ids[ids[0]+1]
            &mut *ids.offset((ids.read() + 1) as isize),
            // app[0] * sizeof(MDB_ID)
            app.read() as usize,
        );

        let fresh5 = &mut (*ids.offset(0 as std::ffi::c_int as isize));
        *fresh5 = (*fresh5 as std::ffi::c_ulong)
            .wrapping_add(*app.offset(0 as std::ffi::c_int as isize) as std::ffi::c_ulong)
            as MDB_ID;
        0 as std::ffi::c_int
    }
}

/// Append an ID range onto an IDL.
///
/// @param[in,out] idp	Address of the IDL to append to.
/// @param[in] id	The lowest ID to append.
/// @param[in] n		Number of IDs to append.
/// @return	0 on success, ENOMEM if the IDL is too large.
pub unsafe fn mdb_midl_append_range(
    mut idp: *mut MDB_IDL,
    mut id: MDB_ID,
    mut n: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        let mut ids: *mut MDB_ID = *idp;
        let mut len: MDB_ID = *ids.offset(0);
        if len.wrapping_add(n as MDB_ID) > *ids.offset(-(1 as std::ffi::c_int) as isize) {
            if mdb_midl_grow(idp, n | (((1) << (MDB_IDL_LOGN + 1)) - 1)) != 0 {
                return ENOMEM;
            }
            ids = *idp as *mut MDB_ID;
        }
        *ids.offset(0) = len.wrapping_add(n as MDB_ID);
        ids = ids.offset(len as isize);
        while n != 0 {
            let fresh6 = id;
            id = id.wrapping_add(1);
            let fresh7 = n;
            n = n.wrapping_sub(1);
            *ids.offset(fresh7 as isize) = fresh6;
        }
        0 as std::ffi::c_int
    }
}

/// Merge an IDL onto an IDL. The destination IDL must be big enough.
///
/// @param[in] idl      The IDL to merge into.
/// @param[in] merge    The IDL to merge.
pub unsafe fn mdb_midl_xmerge(mut idl: MDB_IDL, mut merge: MDB_IDL) {
    unsafe {
        let mut old_id: MDB_ID = 0;
        let mut merge_id: MDB_ID = 0;
        let mut i: MDB_ID = *merge.offset(0 as std::ffi::c_int as isize);
        let mut j: MDB_ID = *idl.offset(0 as std::ffi::c_int as isize);
        let mut k: MDB_ID = i.wrapping_add(j);
        let mut total: MDB_ID = k;
        *idl.offset(0 as std::ffi::c_int as isize) = -(1 as std::ffi::c_int) as MDB_ID;
        old_id = *idl.offset(j as isize);
        while i != 0 {
            let fresh8 = i;
            i = i.wrapping_sub(1);
            merge_id = *merge.offset(fresh8 as isize);
            while old_id < merge_id {
                let fresh9 = k;
                k = k.wrapping_sub(1);
                *idl.offset(fresh9 as isize) = old_id;
                j = j.wrapping_sub(1);
                old_id = *idl.offset(j as isize);
            }
            let fresh10 = k;
            k = k.wrapping_sub(1);
            *idl.offset(fresh10 as isize) = merge_id;
        }
        *idl.offset(0 as std::ffi::c_int as isize) = total;
    }
}

pub const SMALL: std::ffi::c_int = 8 as std::ffi::c_int;

/// Sort an IDL.
///
/// @param[in,out] ids	The IDL to sort.
pub unsafe fn mdb_midl_sort(mut ids: MDB_IDL) {
    let slice = unsafe {
        let len = ids.read();
        std::slice::from_raw_parts_mut(ids.offset(1), len)
    };

    slice.sort_unstable_by(|a, b| a.cmp(&b).reverse());
}

/// Search for an ID in an ID2L.
///
/// @param[in] ids	The ID2L to search.
/// @param[in] id	The ID to search for.
/// @return	The index of the first ID2 whose \b mid member is greater than or equal to \b id.
pub unsafe fn mdb_mid2l_search(ids: MDB_ID2L, id: MDB_ID) -> u32 {
    let slice = unsafe {
        let MDB_ID2 { mid: len, mptr: _ } = ids.read();
        std::slice::from_raw_parts(ids.offset(1), len)
    };

    // binary search of id in ids
    // if found, returns position of id
    // if not found, returns first position greater than id
    match slice.binary_search_by_key(&id, |m| m.mid) {
        Ok(pos) => pos as u32 + 1,
        Err(pos) => pos as u32 + 1,
    }
}

/// Insert an ID2 into a ID2L.
///
/// @param[in,out] ids	The ID2L to insert into.
/// @param[in] id	The ID2 to insert.
/// @return	0 on success, -1 if the ID was already present in the ID2L.
pub unsafe fn mdb_mid2l_insert(mut ids: MDB_ID2L, mut id: *mut MDB_ID2) -> std::ffi::c_int {
    unsafe {
        let mut x: std::ffi::c_uint = 0;
        let mut i: std::ffi::c_uint = 0;
        x = mdb_mid2l_search(ids, (*id).mid);
        if x < 1 {
            return -(2);
        }
        if x as MDB_ID <= (*ids.offset(0)).mid && (*ids.offset(x as isize)).mid == (*id).mid {
            return -(1);
        }
        if (*ids.offset(0)).mid >= (((1) << (MDB_IDL_LOGN + 1)) - 1) as MDB_ID {
            return -(2);
        } else {
            let fresh13 = &mut (*ids.offset(0)).mid;
            *fresh13 = (*fresh13).wrapping_add(1);
            i = (*ids.offset(0)).mid as std::ffi::c_uint;
            while i > x {
                *ids.offset(i as isize) = *ids.offset(i.wrapping_sub(1) as isize);
                i = i.wrapping_sub(1);
            }
            *ids.offset(x as isize) = *id;
        }
        0
    }
}

/// Append an ID2 into a ID2L.
///
/// @param[in,out] ids	The ID2L to append into.
/// @param[in] id	The ID2 to append.
/// @return	0 on success, -2 if the ID2L is too big.
pub unsafe fn mdb_mid2l_append(mut ids: MDB_ID2L, mut id: *mut MDB_ID2) -> std::ffi::c_int {
    unsafe {
        if (*ids.offset(0)).mid >= (((1) << (MDB_IDL_LOGN + 1)) - 1) as MDB_ID {
            return -(2);
        }
        let fresh14 = &mut (*ids.offset(0)).mid;
        *fresh14 = (*fresh14).wrapping_add(1);
        *ids.offset((*ids.offset(0)).mid as isize) = *id;
        0
    }
}
