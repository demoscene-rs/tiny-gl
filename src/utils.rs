use core::{ffi::c_void, mem::transmute};

pub unsafe fn call_ptr_0arg<R>(ptr: *const c_void) -> R {
    let ptr = transmute::<*const c_void, extern "system" fn()->R>(ptr);
    ptr()
}

pub unsafe fn call_ptr_1arg<R, A>(ptr: *const c_void, a: A) -> R {
    let ptr = transmute::<*const c_void, extern "system" fn(A)->R>(ptr);
    ptr(a)
}

pub unsafe fn call_ptr_2arg<R, A, B>(ptr: *const c_void, a: A, b: B) -> R {
    let ptr = transmute::<*const c_void, extern "system" fn(A, B)->R>(ptr);
    ptr(a, b)
}

pub unsafe fn call_ptr_3arg<R, A, B, C>(ptr: *const c_void, a: A, b: B, c: C) -> R {
    let ptr = transmute::<*const c_void, extern "system" fn(A, B, C)->R>(ptr);
    ptr(a, b, c)
}

pub unsafe fn call_ptr_4arg<R, A, B, C, D>(ptr: *const c_void, a: A, b: B, c: C, d: D) -> R {
    let ptr = transmute::<*const c_void, extern "system" fn(A, B, C, D)->R>(ptr);
    ptr(a, b, c, d)
}

pub unsafe fn call_ptr_5arg<R, A, B, C, D, E>(ptr: *const c_void, a: A, b: B, c: C, d: D, e: E) -> R {
    let ptr = transmute::<*const c_void, extern "system" fn(A, B, C, D, E)->R>(ptr);
    ptr(a, b, c, d, e)
}

pub unsafe fn call_ptr_6arg<R, A, B, C, D, E, F>(ptr: *const c_void, a: A, b: B, c: C, d: D, e: E, f: F) -> R {
    let ptr = transmute::<*const c_void, extern "system" fn(A, B, C, D, E, F)->R>(ptr);
    ptr(a, b, c, d, e, f)
}