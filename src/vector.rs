#![allow(non_camel_case_types)]
pub type size_t = usize;

/// # Safety
/// The pointer passed to vec must originate from vector_create.
#[no_mangle]
pub extern "C" fn vector_create(size: size_t) -> *mut Vec<*mut libc::c_void> {
    let vector: Vec<*mut libc::c_void> = vec![0 as *mut libc::c_void; size];
    let boxed_vec: *mut Vec<*mut libc::c_void> = Box::into_raw(Box::new(vector));
    boxed_vec
}
#[no_mangle]
pub unsafe extern "C" fn vector_add(
    vec: *mut Vec<*mut libc::c_void>,
    element: *mut libc::c_void,
) {
    assert!(!vec.is_null());
    let mut boxed_vec = unsafe{Box::from_raw(vec)};
    boxed_vec.push(element);
    std::mem::forget(boxed_vec);
}
#[no_mangle]
pub unsafe extern "C" fn vector_remove_at_index(
    vec: *mut Vec<*mut libc::c_void>,
    index: size_t,
) {
    assert!(!vec.is_null());

    let mut boxed_vec = unsafe{Box::from_raw(vec)};
    boxed_vec.remove(index);
    std::mem::forget(boxed_vec);
}
#[no_mangle]
pub unsafe extern "C" fn vector_at(
    vec: *mut Vec<*mut libc::c_void>,
    index: usize,
) -> *mut libc::c_void {
    assert!(!vec.is_null());
    let boxed_vec = unsafe{Box::from_raw(vec)};
    assert!(index < boxed_vec.len());
    let res = boxed_vec[index];
    std::mem::forget(boxed_vec);
    res
}
#[no_mangle]
pub extern "C" fn vector_set(
    vec: *mut Vec<*mut libc::c_void>,
    index: size_t,
    new_value: *mut libc::c_void,
) {
    assert!(!vec.is_null());

    let mut boxed_vec = unsafe{Box::from_raw(vec)};
    boxed_vec[index] = new_value;
    std::mem::forget(boxed_vec);
}

#[no_mangle]
pub extern "C" fn vector_find(
    vec: *mut Vec<*mut libc::c_void>,
    element_to_match: *mut libc::c_void,
    match_elements: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> bool,
    >,
) -> *mut libc::c_void {
    assert!(!vec.is_null());

    let boxed_vec = unsafe{Box::from_raw(vec)};
    let res = boxed_vec.iter().rfind(|&&element| unsafe{match_elements.expect("non-null function pointer")(element, element_to_match)} == true);
    let real_res = match res {
        Some(s) => *s,
        None => 0 as *mut libc::c_void,
    };
    std::mem::forget(boxed_vec);
    real_res
}

#[no_mangle]
pub extern "C" fn vector_size(vec: *mut Vec<*mut libc::c_void>) -> size_t {
    assert!(!vec.is_null());

    let boxed_vec = unsafe{Box::from_raw(vec)};
    let len = boxed_vec.len();
    std::mem::forget(boxed_vec);
    len
}

#[no_mangle]
pub unsafe extern "C" fn vector_destroy(vec: *mut Vec<*mut libc::c_void>) {
    assert!(!vec.is_null());
    unsafe{
        drop(Box::from_raw(vec));
    }
}
