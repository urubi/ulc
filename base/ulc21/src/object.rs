use std::boxed::into_raw;
pub unsafe fn into_obj<T>(obj: T) -> *mut T {
    into_raw(Box::new(obj))
}
pub unsafe fn free_obj<T>(obj: *mut T) {
    Box::from_raw(obj);
}
