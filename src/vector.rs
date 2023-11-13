#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub elements: *mut *mut libc::c_void,
    pub size: size_t,
}
pub type vector_t = vector;
#[no_mangle]
pub unsafe extern "C" fn vector_set_size(
    mut vec: *mut vector_t,
    mut number: libc::c_int,
) {
    if !vec.is_null() {} else {
        __assert_fail(
            b"vec\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            14 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 38],
                &[libc::c_char; 38],
            >(b"void vector_set_size(vector_t *, int)\0"))
                .as_ptr(),
        );
    }
    'c_915: {
        if !vec.is_null() {} else {
            __assert_fail(
                b"vec\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                14 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 38],
                    &[libc::c_char; 38],
                >(b"void vector_set_size(vector_t *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    (*vec).size = number as size_t;
    (*vec)
        .elements = realloc(
        (*vec).elements as *mut libc::c_void,
        ((*vec).size)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *mut libc::c_void;
    memset(
        (*vec).elements as *mut libc::c_void,
        0 as libc::c_int,
        ((*vec).size)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn vector_create(mut size: size_t) -> *mut vector_t {
    let mut vector: *mut vector_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<vector_t>() as libc::c_ulong,
    ) as *mut vector_t;
    if size != 0 {
        vector_set_size(vector, size as libc::c_int);
    }
    return vector;
}
#[no_mangle]
pub unsafe extern "C" fn vector_add(
    mut vec: *mut vector_t,
    mut element: *mut libc::c_void,
) {
    if !vec.is_null() {} else {
        __assert_fail(
            b"vec\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            28 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void vector_add(vector_t *, void *)\0"))
                .as_ptr(),
        );
    }
    'c_1014: {
        if !vec.is_null() {} else {
            __assert_fail(
                b"vec\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                28 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void vector_add(vector_t *, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*vec).size = ((*vec).size).wrapping_add(1);
    (*vec).size;
    (*vec)
        .elements = realloc(
        (*vec).elements as *mut libc::c_void,
        ((*vec).size)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut *mut libc::c_void;
    let ref mut fresh0 = *((*vec).elements)
        .offset(((*vec).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    *fresh0 = element;
}
#[no_mangle]
pub unsafe extern "C" fn vector_remove_at_index(
    mut vec: *mut vector_t,
    mut index: size_t,
) {
    if !vec.is_null() {} else {
        __assert_fail(
            b"vec\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            35 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void vector_remove_at_index(vector_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1179: {
        if !vec.is_null() {} else {
            __assert_fail(
                b"vec\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                35 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void vector_remove_at_index(vector_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if index < (*vec).size {} else {
        __assert_fail(
            b"index < vec->size\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            36 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"void vector_remove_at_index(vector_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1135: {
        if index < (*vec).size {} else {
            __assert_fail(
                b"index < vec->size\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                36 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"void vector_remove_at_index(vector_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if index < ((*vec).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) {
        memmove(
            ((*vec).elements).offset(index as isize) as *mut libc::c_void,
            ((*vec).elements).offset(index as isize).offset(1 as libc::c_int as isize)
                as *const libc::c_void,
            ((*vec).size)
                .wrapping_sub(index)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
    }
    let ref mut fresh1 = *((*vec).elements)
        .offset(((*vec).size).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
    *fresh1 = 0 as *mut libc::c_void;
    (*vec).size = ((*vec).size).wrapping_sub(1);
    (*vec).size;
}
#[no_mangle]
pub unsafe extern "C" fn vector_at(
    mut vec: *mut vector_t,
    mut index: size_t,
) -> *mut libc::c_void {
    if !vec.is_null() {} else {
        __assert_fail(
            b"vec\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            44 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void *vector_at(vector_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1269: {
        if !vec.is_null() {} else {
            __assert_fail(
                b"vec\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                44 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void *vector_at(vector_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if index < (*vec).size {} else {
        __assert_fail(
            b"index < vec->size\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            45 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 36],
                &[libc::c_char; 36],
            >(b"void *vector_at(vector_t *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_1227: {
        if index < (*vec).size {} else {
            __assert_fail(
                b"index < vec->size\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                45 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 36],
                    &[libc::c_char; 36],
                >(b"void *vector_at(vector_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return *((*vec).elements).offset(index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn vector_set(
    mut vec: *mut vector_t,
    mut index: size_t,
    mut new_value: *mut libc::c_void,
) {
    if !vec.is_null() {} else {
        __assert_fail(
            b"vec\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            50 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void vector_set(vector_t *, size_t, void *)\0"))
                .as_ptr(),
        );
    }
    'c_1363: {
        if !vec.is_null() {} else {
            __assert_fail(
                b"vec\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                50 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void vector_set(vector_t *, size_t, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    if index < (*vec).size {} else {
        __assert_fail(
            b"index < vec->size\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            51 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 44],
                &[libc::c_char; 44],
            >(b"void vector_set(vector_t *, size_t, void *)\0"))
                .as_ptr(),
        );
    }
    'c_1320: {
        if index < (*vec).size {} else {
            __assert_fail(
                b"index < vec->size\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                51 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 44],
                    &[libc::c_char; 44],
                >(b"void vector_set(vector_t *, size_t, void *)\0"))
                    .as_ptr(),
            );
        }
    };
    let ref mut fresh2 = *((*vec).elements).offset(index as isize);
    *fresh2 = new_value;
}
#[no_mangle]
pub unsafe extern "C" fn vector_find(
    mut vec: *mut vector_t,
    mut element_to_match: *mut libc::c_void,
    mut match_elements: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
    >,
) -> *mut libc::c_void {
    if !vec.is_null() {} else {
        __assert_fail(
            b"vec\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            56 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[libc::c_char; 65],
            >(b"void *vector_find(vector_t *, void *, _Bool (*)(void *, void *))\0"))
                .as_ptr(),
        );
    }
    'c_1446: {
        if !vec.is_null() {} else {
            __assert_fail(
                b"vec\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                56 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[libc::c_char; 65],
                >(b"void *vector_find(vector_t *, void *, _Bool (*)(void *, void *))\0"))
                    .as_ptr(),
            );
        }
    };
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*vec).size {
        let mut element: *mut libc::c_void = vector_at(vec, i);
        if match_elements.expect("non-null function pointer")(element, element_to_match)
        {
            return element;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn vector_size(mut vec: *mut vector_t) -> size_t {
    if !vec.is_null() {} else {
        __assert_fail(
            b"vec\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            66 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 31],
                &[libc::c_char; 31],
            >(b"size_t vector_size(vector_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1489: {
        if !vec.is_null() {} else {
            __assert_fail(
                b"vec\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                66 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 31],
                    &[libc::c_char; 31],
                >(b"size_t vector_size(vector_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return (*vec).size;
}
#[no_mangle]
pub unsafe extern "C" fn vector_destroy(mut vec: *mut vector_t) {
    if !vec.is_null() {} else {
        __assert_fail(
            b"vec\0" as *const u8 as *const libc::c_char,
            b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                as *const libc::c_char,
            71 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[libc::c_char; 32],
            >(b"void vector_destroy(vector_t *)\0"))
                .as_ptr(),
        );
    }
    'c_1547: {
        if !vec.is_null() {} else {
            __assert_fail(
                b"vec\0" as *const u8 as *const libc::c_char,
                b"/home/alicia/workshop_start_point/lib/vector.c\0" as *const u8
                    as *const libc::c_char,
                71 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[libc::c_char; 32],
                >(b"void vector_destroy(vector_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    free((*vec).elements as *mut libc::c_void);
    free(vec as *mut libc::c_void);
}
