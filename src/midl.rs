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
    fn realloc(__ptr: *mut std::ffi::c_void, __size: size_t) -> *mut std::ffi::c_void;
}
pub type size_t = usize;
pub type mdb_size_t = size_t;

use ::core::mem::size_of;

use crate::{MDB_ID, MDB_ID2, MDB_ID2L, MDB_IDL};

pub const MDB_IDL_LOGN: std::ffi::c_int = 16 as std::ffi::c_int;
pub const ENOMEM: std::ffi::c_int = 12 as std::ffi::c_int;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_search(mut ids: MDB_IDL, mut id: MDB_ID) -> std::ffi::c_uint {
    unsafe {
        let mut base: std::ffi::c_uint = 0 as std::ffi::c_uint;
        let mut cursor: std::ffi::c_uint = 1 as std::ffi::c_uint;
        let mut val: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut n: std::ffi::c_uint =
            *ids.offset(0 as std::ffi::c_int as isize) as std::ffi::c_uint;
        while (0 as std::ffi::c_uint) < n {
            let mut pivot: std::ffi::c_uint = n >> 1 as std::ffi::c_int;
            cursor = base.wrapping_add(pivot).wrapping_add(1 as std::ffi::c_uint);
            val = if *ids.offset(cursor as isize) < id {
                -(1 as std::ffi::c_int)
            } else {
                (*ids.offset(cursor as isize) > id) as std::ffi::c_int
            };
            if val < 0 as std::ffi::c_int {
                n = pivot;
            } else if val > 0 as std::ffi::c_int {
                base = cursor;
                n = n.wrapping_sub(pivot.wrapping_add(1 as std::ffi::c_uint));
            } else {
                return cursor;
            }
        }
        if val > 0 as std::ffi::c_int {
            cursor = cursor.wrapping_add(1);
        }
        cursor
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_alloc(mut num: std::ffi::c_int) -> MDB_IDL {
    unsafe {
        let mut ids: MDB_IDL = libc::malloc(
            ((num + 2 as std::ffi::c_int) as usize).wrapping_mul(size_of::<MDB_ID>() as usize),
        ) as MDB_IDL;
        if !ids.is_null() {
            let fresh0 = ids;
            ids = ids.offset(1);
            *fresh0 = num as MDB_ID;
            *ids = 0 as MDB_ID;
        }
        ids
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_free(mut ids: MDB_IDL) {
    unsafe {
        if !ids.is_null() {
            libc::free(ids.offset(-(1 as isize)) as *mut std::ffi::c_void);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_shrink(mut idp: *mut MDB_IDL) {
    unsafe {
        let mut ids: MDB_IDL = *idp;
        ids = ids.offset(-1);
        if *ids
            > (((1 as std::ffi::c_int) << (MDB_IDL_LOGN + 1 as std::ffi::c_int))
                - 1 as std::ffi::c_int) as MDB_ID
            && {
                ids = realloc(
                    ids as *mut std::ffi::c_void,
                    ((((1 as std::ffi::c_int) << (MDB_IDL_LOGN + 1 as std::ffi::c_int))
                        - 1 as std::ffi::c_int
                        + 2 as std::ffi::c_int) as size_t)
                        .wrapping_mul(size_of::<MDB_ID>()),
                ) as MDB_IDL;
                !ids.is_null()
            }
        {
            let fresh1 = ids;
            ids = ids.offset(1);
            *fresh1 = (((1 as std::ffi::c_int) << (MDB_IDL_LOGN + 1 as std::ffi::c_int))
                - 1 as std::ffi::c_int) as MDB_ID;
            *idp = ids;
        }
    }
}
unsafe extern "C" fn mdb_midl_grow(
    mut idp: *mut MDB_IDL,
    mut num: std::ffi::c_int,
) -> std::ffi::c_int {
    unsafe {
        let mut idn: MDB_IDL = (*idp).offset(-(1 as std::ffi::c_int as isize));
        idn = realloc(
            idn as *mut std::ffi::c_void,
            (*idn as usize)
                .wrapping_add(num as usize)
                .wrapping_add(2)
                .wrapping_mul(size_of::<MDB_ID>()),
        ) as MDB_IDL;
        if idn.is_null() {
            return ENOMEM;
        }
        let fresh2 = idn;
        idn = idn.offset(1);
        *fresh2 = (*fresh2 as std::ffi::c_ulong).wrapping_add(num as std::ffi::c_ulong) as MDB_ID
            as MDB_ID;
        *idp = idn;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_need(
    mut idp: *mut MDB_IDL,
    mut num: std::ffi::c_uint,
) -> std::ffi::c_int {
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
            ids = realloc(
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_append(mut idp: *mut MDB_IDL, mut id: MDB_ID) -> std::ffi::c_int {
    unsafe {
        let mut ids: MDB_IDL = *idp;
        if *ids.offset(0 as std::ffi::c_int as isize)
            >= *ids.offset(-(1 as std::ffi::c_int) as isize)
        {
            if mdb_midl_grow(
                idp,
                ((1 as std::ffi::c_int) << (MDB_IDL_LOGN + 1 as std::ffi::c_int))
                    - 1 as std::ffi::c_int,
            ) != 0
            {
                return ENOMEM;
            }
            ids = *idp;
        }
        let fresh4 = &mut (*ids.offset(0 as std::ffi::c_int as isize));
        *fresh4 = (*fresh4).wrapping_add(1);
        *ids.offset(*ids.offset(0 as std::ffi::c_int as isize) as isize) = id;
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_append_list(
    mut idp: *mut MDB_IDL,
    mut app: MDB_IDL,
) -> std::ffi::c_int {
    unsafe {
        let mut ids: MDB_IDL = *idp;
        if (*ids.offset(0 as std::ffi::c_int as isize))
            .wrapping_add(*app.offset(0 as std::ffi::c_int as isize))
            >= *ids.offset(-(1 as std::ffi::c_int) as isize)
        {
            if mdb_midl_grow(idp, *app.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int)
                != 0
            {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_append_range(
    mut idp: *mut MDB_IDL,
    mut id: MDB_ID,
    mut n: std::ffi::c_uint,
) -> std::ffi::c_int {
    unsafe {
        let mut ids: *mut MDB_ID = *idp;
        let mut len: MDB_ID = *ids.offset(0 as std::ffi::c_int as isize);
        if len.wrapping_add(n as MDB_ID) > *ids.offset(-(1 as std::ffi::c_int) as isize) {
            if mdb_midl_grow(
                idp,
                (n | (((1 as std::ffi::c_int) << (MDB_IDL_LOGN + 1 as std::ffi::c_int))
                    - 1 as std::ffi::c_int) as std::ffi::c_uint) as std::ffi::c_int,
            ) != 0
            {
                return ENOMEM;
            }
            ids = *idp as *mut MDB_ID;
        }
        *ids.offset(0 as std::ffi::c_int as isize) = len.wrapping_add(n as MDB_ID);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_xmerge(mut idl: MDB_IDL, mut merge: MDB_IDL) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_midl_sort(mut ids: MDB_IDL) {
    unsafe {
        let mut istack: [std::ffi::c_int; 64] = [0; 64];
        let mut i: std::ffi::c_int = 0;
        let mut j: std::ffi::c_int = 0;
        let mut k: std::ffi::c_int = 0;
        let mut l: std::ffi::c_int = 0;
        let mut ir: std::ffi::c_int = 0;
        let mut jstack: std::ffi::c_int = 0;
        let mut a: MDB_ID = 0;
        let mut itmp: MDB_ID = 0;
        ir = *ids.offset(0 as std::ffi::c_int as isize) as std::ffi::c_int;
        l = 1 as std::ffi::c_int;
        jstack = 0 as std::ffi::c_int;
        loop {
            if ir - l < SMALL {
                j = l + 1 as std::ffi::c_int;
                while j <= ir {
                    a = *ids.offset(j as isize);
                    i = j - 1 as std::ffi::c_int;
                    while i >= 1 as std::ffi::c_int {
                        if *ids.offset(i as isize) >= a {
                            break;
                        }
                        *ids.offset((i + 1 as std::ffi::c_int) as isize) = *ids.offset(i as isize);
                        i -= 1;
                    }
                    *ids.offset((i + 1 as std::ffi::c_int) as isize) = a;
                    j += 1;
                }
                if jstack == 0 as std::ffi::c_int {
                    break;
                }
                let fresh11 = jstack;
                jstack -= 1;
                ir = istack[fresh11 as usize];
                let fresh12 = jstack;
                jstack -= 1;
                l = istack[fresh12 as usize];
            } else {
                k = (l + ir) >> 1 as std::ffi::c_int;
                itmp = *ids.offset(k as isize);
                *ids.offset(k as isize) = *ids.offset((l + 1 as std::ffi::c_int) as isize);
                *ids.offset((l + 1 as std::ffi::c_int) as isize) = itmp;
                if *ids.offset(l as isize) < *ids.offset(ir as isize) {
                    itmp = *ids.offset(l as isize);
                    *ids.offset(l as isize) = *ids.offset(ir as isize);
                    *ids.offset(ir as isize) = itmp;
                }
                if *ids.offset((l + 1 as std::ffi::c_int) as isize) < *ids.offset(ir as isize) {
                    itmp = *ids.offset((l + 1 as std::ffi::c_int) as isize);
                    *ids.offset((l + 1 as std::ffi::c_int) as isize) = *ids.offset(ir as isize);
                    *ids.offset(ir as isize) = itmp;
                }
                if *ids.offset(l as isize) < *ids.offset((l + 1 as std::ffi::c_int) as isize) {
                    itmp = *ids.offset(l as isize);
                    *ids.offset(l as isize) = *ids.offset((l + 1 as std::ffi::c_int) as isize);
                    *ids.offset((l + 1 as std::ffi::c_int) as isize) = itmp;
                }
                i = l + 1 as std::ffi::c_int;
                j = ir;
                a = *ids.offset((l + 1 as std::ffi::c_int) as isize);
                loop {
                    loop {
                        i += 1;
                        if *ids.offset(i as isize) <= a {
                            break;
                        }
                    }
                    loop {
                        j -= 1;
                        if *ids.offset(j as isize) >= a {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    itmp = *ids.offset(i as isize);
                    *ids.offset(i as isize) = *ids.offset(j as isize);
                    *ids.offset(j as isize) = itmp;
                }
                *ids.offset((l + 1 as std::ffi::c_int) as isize) = *ids.offset(j as isize);
                *ids.offset(j as isize) = a;
                jstack += 2 as std::ffi::c_int;
                if ir - i + 1 as std::ffi::c_int >= j - l {
                    istack[jstack as usize] = ir;
                    istack[(jstack - 1 as std::ffi::c_int) as usize] = i;
                    ir = j - 1 as std::ffi::c_int;
                } else {
                    istack[jstack as usize] = j - 1 as std::ffi::c_int;
                    istack[(jstack - 1 as std::ffi::c_int) as usize] = l;
                    l = i;
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_mid2l_search(mut ids: MDB_ID2L, mut id: MDB_ID) -> std::ffi::c_uint {
    unsafe {
        let mut base: std::ffi::c_uint = 0 as std::ffi::c_uint;
        let mut cursor: std::ffi::c_uint = 1 as std::ffi::c_uint;
        let mut val: std::ffi::c_int = 0 as std::ffi::c_int;
        let mut n: std::ffi::c_uint =
            (*ids.offset(0 as std::ffi::c_int as isize)).mid as std::ffi::c_uint;
        while (0 as std::ffi::c_uint) < n {
            let mut pivot: std::ffi::c_uint = n >> 1 as std::ffi::c_int;
            cursor = base.wrapping_add(pivot).wrapping_add(1 as std::ffi::c_uint);
            val = if id < (*ids.offset(cursor as isize)).mid {
                -(1 as std::ffi::c_int)
            } else {
                (id > (*ids.offset(cursor as isize)).mid) as std::ffi::c_int
            };
            if val < 0 as std::ffi::c_int {
                n = pivot;
            } else if val > 0 as std::ffi::c_int {
                base = cursor;
                n = n.wrapping_sub(pivot.wrapping_add(1 as std::ffi::c_uint));
            } else {
                return cursor;
            }
        }
        if val > 0 as std::ffi::c_int {
            cursor = cursor.wrapping_add(1);
        }
        cursor
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_mid2l_insert(
    mut ids: MDB_ID2L,
    mut id: *mut MDB_ID2,
) -> std::ffi::c_int {
    unsafe {
        let mut x: std::ffi::c_uint = 0;
        let mut i: std::ffi::c_uint = 0;
        x = mdb_mid2l_search(ids, (*id).mid);
        if x < 1 as std::ffi::c_uint {
            return -(2 as std::ffi::c_int);
        }
        if x as MDB_ID <= (*ids.offset(0 as std::ffi::c_int as isize)).mid
            && (*ids.offset(x as isize)).mid == (*id).mid
        {
            return -(1 as std::ffi::c_int);
        }
        if (*ids.offset(0 as std::ffi::c_int as isize)).mid
            >= (((1 as std::ffi::c_int) << (MDB_IDL_LOGN + 1 as std::ffi::c_int))
                - 1 as std::ffi::c_int) as MDB_ID
        {
            return -(2 as std::ffi::c_int);
        } else {
            let fresh13 = &mut (*ids.offset(0 as std::ffi::c_int as isize)).mid;
            *fresh13 = (*fresh13).wrapping_add(1);
            i = (*ids.offset(0 as std::ffi::c_int as isize)).mid as std::ffi::c_uint;
            while i > x {
                *ids.offset(i as isize) =
                    *ids.offset(i.wrapping_sub(1 as std::ffi::c_uint) as isize);
                i = i.wrapping_sub(1);
            }
            *ids.offset(x as isize) = *id;
        }
        0 as std::ffi::c_int
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mdb_mid2l_append(
    mut ids: MDB_ID2L,
    mut id: *mut MDB_ID2,
) -> std::ffi::c_int {
    unsafe {
        if (*ids.offset(0 as std::ffi::c_int as isize)).mid
            >= (((1 as std::ffi::c_int) << (MDB_IDL_LOGN + 1 as std::ffi::c_int))
                - 1 as std::ffi::c_int) as MDB_ID
        {
            return -(2 as std::ffi::c_int);
        }
        let fresh14 = &mut (*ids.offset(0 as std::ffi::c_int as isize)).mid;
        *fresh14 = (*fresh14).wrapping_add(1);
        *ids.offset((*ids.offset(0 as std::ffi::c_int as isize)).mid as isize) = *id;
        0 as std::ffi::c_int
    }
}
