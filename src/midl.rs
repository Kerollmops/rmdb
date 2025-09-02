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

use std::ops::{Index, IndexMut, Range};

use libc::ENOMEM;

use crate::MDB_val;

// IDL sizes - likely should be even bigger
// limiting factors: sizeof(ID), thread stack size
/// DB_SIZE is 2^16, UM_SIZE is 2^17
const MDB_IDL_LOGN: usize = 16;
const MDB_IDL_DB_SIZE: usize = 1 << MDB_IDL_LOGN;
const MDB_IDL_UM_SIZE: usize = 1 << (MDB_IDL_LOGN + 1);

const MDB_IDL_DB_MAX: usize = MDB_IDL_DB_SIZE - 1;
const MDB_IDL_UM_MAX: usize = MDB_IDL_UM_SIZE - 1;

/// A generic unsigned ID number. These were entryIDs in back-bdb.
/// Preferably it should have the same size as a pointer.
pub type MDB_ID = usize;

/// An IDL is an ID List, a sorted array of IDs. The first
/// element of the array is a counter for how many actual
/// IDs are in the list. In the original back-bdb code, IDLs are
/// sorted in ascending order. For libmdb IDLs are sorted in
/// descending order.
#[derive(Debug, Clone, Default)]
pub struct MDB_IDL {
    inner: Vec<MDB_ID>,
}

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

impl MDB_IDL {
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn retain(&mut self, f: impl FnMut(&MDB_ID) -> bool) {
        self.inner.retain(f);
    }

    pub fn pop(&mut self) -> Option<MDB_ID> {
        self.inner.pop()
    }

    pub fn extend(&mut self, iter: impl Iterator<Item = MDB_ID>) {
        self.inner.extend(iter);
    }

    pub fn size_of_with_len(&self) -> usize {
        (self.len() + 1) * size_of::<MDB_ID>()
    }

    pub fn copy_with_len_into_data(&self, data: MDB_val) {
        assert_eq!(data.mv_size, self.size_of_with_len() as _);
        unsafe {
            std::ptr::write_unaligned(data.mv_data as *mut MDB_ID, self.len());
            std::ptr::copy_nonoverlapping(
                self.inner.as_ptr() as *const u8,
                (data.mv_data as *mut MDB_ID).offset(1) as *mut u8,
                self.len() * size_of::<MDB_ID>(),
            );
        }
    }

    pub fn copy_range_with_len_into_buffer(&self, range: Range<usize>, buffer: &mut [usize]) {
        let slice: &[MDB_ID] = &self.inner[range];
        assert_eq!(buffer.len(), slice.len() + 1);
        if let Some((len, tail)) = buffer.split_first_mut() {
            *len = slice.len();
            tail.copy_from_slice(slice);
        }
    }
}

impl Index<usize> for MDB_IDL {
    type Output = MDB_ID;

    fn index(&self, index: usize) -> &Self::Output {
        // TODO remove this (-1) and the (+1) in the search functions
        &self.inner[index.checked_sub(1).unwrap()]
    }
}

impl IndexMut<usize> for MDB_IDL {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // TODO remove this (-1) and the (+1) in the search functions
        &mut self.inner[index.checked_sub(1).unwrap()]
    }
}

pub struct MDB_IDLRef<'a> {
    inner: &'a [MDB_ID],
}

impl<'a> MDB_IDLRef<'a> {
    pub unsafe fn from_pointer(ptr: *mut MDB_ID) -> Self {
        let len: usize = unsafe { ptr.read() };
        MDB_IDLRef { inner: unsafe { std::slice::from_raw_parts(ptr.offset(1), len) } }
    }
}

/// Search for an ID in an IDL.
///
/// @param[in] ids	The IDL to search.
/// @param[in] id	The ID to search for.
/// @return	The index of the first ID greater than or equal to \b id.
pub fn mdb_midl_search(ids: &MDB_IDL, id: MDB_ID) -> u32 {
    // binary search of id in ids
    // if found, returns position of id
    // if not found, returns first position greater than id
    match ids.inner.binary_search_by(|m| m.cmp(&id).reverse()) {
        Ok(pos) => pos as u32 + 1,
        Err(pos) => pos as u32 + 1,
    }
}

/// Allocate an IDL.
///
/// Allocates memory for an IDL of the given size.
/// @return	IDL on success, NULL on failure.
pub fn mdb_midl_alloc(num: usize) -> Option<MDB_IDL> {
    let mut vec = Vec::new();
    match vec.try_reserve(num) {
        Ok(_) => Some(MDB_IDL { inner: vec }),
        Err(_) => None,
    }
}

/// Free an IDL.
///
/// @param[in] ids	The IDL to free.
pub fn mdb_midl_free(_ids: MDB_IDL) {
    // drop(ids)
}

/// Shrink an IDL.
///
/// Return the IDL to the default size if it has grown larger.
/// @param[in,out] idp	Address of the IDL to shrink.
pub fn mdb_midl_shrink(idp: &mut MDB_IDL) {
    if idp.inner.capacity() > MDB_IDL_UM_MAX {
        idp.inner.shrink_to(MDB_IDL_UM_MAX);
    }
}

/// Make room for num additional elements in an IDL.
///
/// @param[in,out] idp	Address of the IDL.
/// @param[in] num	Number of elements to make room for.
/// @return	0 on success, ENOMEM on failure.
pub fn mdb_midl_need(idp: &mut MDB_IDL, num: usize) -> i32 {
    match idp.inner.try_reserve(num) {
        Ok(_) => 0,
        Err(_) => ENOMEM,
    }
}

/// Append ID to IDL. The IDL must be big enough.
pub fn mdb_midl_xappend(idl: &mut MDB_IDL, id: MDB_ID) {
    // Note: It seems that it's simply an mdb_midl_append without the midl_need... but in a macro.
    // #define mdb_midl_xappend(idl, id) do { \
    // 		MDB_ID *xidl = (idl), xlen = ++(xidl[0]); \
    // 		xidl[xlen] = (id); \
    // 	} while (0)
    assert!(idl.capacity() - idl.len() >= 1);
    idl.inner.push(id);
}

/// Append an ID onto an IDL.
///
/// @param[in,out] idp	Address of the IDL to append to.
/// @param[in] id	The ID to append.
/// @return	0 on success, ENOMEM if the IDL is too large.
pub fn mdb_midl_append(idp: &mut MDB_IDL, id: MDB_ID) -> i32 {
    if mdb_midl_need(idp, 1) != 0 {
        return ENOMEM;
    }
    idp.inner.push(id);
    0
}

/// Append an IDL onto an IDL.
///
/// @param[in,out] idp	Address of the IDL to append to.
/// @param[in] app	The IDL to append.
/// @return	0 on success, ENOMEM if the IDL is too large.
pub fn mdb_midl_append_list(idp: &mut MDB_IDL, mut app: MDB_IDL) -> i32 {
    if mdb_midl_need(idp, app.inner.len()) != 0 {
        return ENOMEM;
    }
    idp.inner.append(&mut app.inner);
    0
}

/// Append an ID range onto an IDL.
///
/// @param[in,out] idp  Address of the IDL to append to.
/// @param[in] id       The lowest ID to append.
/// @param[in] n        Number of IDs to append.
/// @return 0 on success, ENOMEM if the IDL is too large.
pub fn mdb_midl_append_range(idp: &mut MDB_IDL, id: MDB_ID, n: usize) -> i32 {
    if mdb_midl_need(idp, n) != 0 {
        return ENOMEM;
    }
    idp.inner.extend((0..n).map(|x| id + x));
    0
}

/// Merge an IDL onto an IDL. The destination IDL must be big enough.
///
/// @param[in] idl      The IDL to merge into.
/// @param[in] merge    The IDL to merge.
pub fn mdb_midl_xmerge(idl: &mut MDB_IDL, mut merge: MDB_IDL) {
    // TODO Implement mdb_midl_xmerge
    //    MDB_ID old_id, merge_id, i = merge[0], j = idl[0], k = i+j, total = k;
    // idl[0] = (MDB_ID)-1;		/* delimiter for idl scan below */
    // old_id = idl[j];
    // while (i) {
    // 	merge_id = merge[i--];
    // 	for (; old_id < merge_id; old_id = idl[--j])
    // 		idl[k--] = old_id;
    // 	idl[k--] = merge_id;
    // }
    // idl[0] = total;

    if idl.inner.capacity() < idl.inner.len() + merge.inner.len() {
        panic!("Insufficient capacity when xmerging");
    }

    idl.inner.append(&mut merge.inner);
    mdb_midl_sort(idl);
}

/// Merge an IDL onto an IDL. The destination IDL must be big enough.
///
/// @param[in] idl      The IDL to merge into.
/// @param[in] merge    The IDL to merge.
pub fn mdb_midl_xmerge_ref(idl: &mut MDB_IDL, merge: MDB_IDLRef) {
    // TODO Implement mdb_midl_xmerge
    if idl.inner.capacity() < idl.inner.len() + merge.inner.len() {
        panic!("Insufficient capacity when xmerging");
    }

    idl.extend(merge.inner.iter().copied());
    mdb_midl_sort(idl);
}

pub const SMALL: i32 = 8;

/// Sort an IDL.
///
/// @param[in,out] ids	The IDL to sort.
pub fn mdb_midl_sort(ids: &mut MDB_IDL) {
    ids.inner.sort_unstable_by(|a, b| a.cmp(&b).reverse());
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
