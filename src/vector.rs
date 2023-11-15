#![allow(non_camel_case_types)]
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub elements: *mut *mut libc::c_void,
    pub size: size_t,
}
pub type vector_t = vector;
#[no_mangle]
pub unsafe extern "C" fn vector_set_size(
    vec: *mut vector_t,
    number: libc::c_int,
) {
    assert!(!vec.is_null());
    (*vec).size = number as size_t;
    (*vec)
        .elements = libc::realloc(
        (*vec).elements as *mut libc::c_void,
        (*vec).size
            * ::core::mem::size_of::<*mut libc::c_void>()
    ) as *mut *mut libc::c_void;
    libc::memset(
        (*vec).elements as *mut libc::c_void,
        0 as libc::c_int,
        (*vec).size
            * ::core::mem::size_of::<*mut libc::c_void>()
    );
}
#[no_mangle]
pub unsafe extern "C" fn vector_create(size: size_t) -> *mut vector_t {
    let vector: *mut vector_t = libc::calloc(
        1,
        ::core::mem::size_of::<vector_t>(),
    ) as *mut vector_t;
    if size != 0 {
        vector_set_size(vector, size as libc::c_int);
    }
    return vector;
}
#[no_mangle]
pub unsafe extern "C" fn vector_add(
    vec: *mut vector_t,
    element: *mut libc::c_void,
) {
    assert!(!vec.is_null());

    (*vec).size = (*vec).size + 1;
    (*vec).size;
    (*vec)
        .elements = libc::realloc(
        (*vec).elements as *mut libc::c_void,
        (*vec).size
            * ::core::mem::size_of::<*mut libc::c_void>())as *mut *mut libc::c_void;
    let ref mut fresh0 = *((*vec).elements)
        .offset(((*vec).size - 1) as isize);
    *fresh0 = element;
}
#[no_mangle]
pub unsafe extern "C" fn vector_remove_at_index(
    vec: *mut vector_t,
    index: size_t,
) {
    assert!(!vec.is_null());
    assert!(index < (*vec).size);
    if index < (*vec).size - 1 {
        libc::memmove(
            ((*vec).elements).offset(index as isize) as *mut libc::c_void,
            ((*vec).elements).offset(index as isize).offset(1 as libc::c_int as isize)
                as *const libc::c_void,
            ((*vec).size
                - index
                - 1)
                * ::core::mem::size_of::<*mut libc::c_void>(),
        );
    }
    let ref mut fresh1 = *((*vec).elements)
        .offset(((*vec).size - 1) as isize);
    *fresh1 = 0 as *mut libc::c_void;
    (*vec).size = (*vec).size - 1;
    (*vec).size;
}
#[no_mangle]
pub unsafe extern "C" fn vector_at(
    vec: *mut vector_t,
    index: size_t,
) -> *mut libc::c_void {
    assert!(!vec.is_null());
    assert!(index < (*vec).size);
    return *((*vec).elements).offset(index as isize);
}
#[no_mangle]
pub unsafe extern "C" fn vector_set(
    vec: *mut vector_t,
    index: size_t,
    new_value: *mut libc::c_void,
) {
    assert!(!vec.is_null());
    assert!(index < (*vec).size);
    let ref mut fresh2 = *((*vec).elements).offset(index as isize);
    *fresh2 = new_value;
}
#[no_mangle]
pub unsafe extern "C" fn vector_find(
    vec: *mut vector_t,
    element_to_match: *mut libc::c_void,
    match_elements: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
    >,
) -> *mut libc::c_void {
    assert!(!vec.is_null());
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < (*vec).size {
        let element: *mut libc::c_void = vector_at(vec, i);
        if match_elements.expect("non-null function pointer")(element, element_to_match)
        {
            return element;
        }
        i = i + 1;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn vector_size(vec: *mut vector_t) -> size_t {
    assert!(!vec.is_null());
    return (*vec).size;
}
#[no_mangle]
pub unsafe extern "C" fn vector_destroy(vec: *mut vector_t) {
    assert!(!vec.is_null());
    libc::free((*vec).elements as *mut libc::c_void);
    libc::free(vec as *mut libc::c_void);
}