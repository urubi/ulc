use std::default::Default;


// Element Count Requirement
// calculates the number of element cells of U required to overwrite with 
// T safely: 
// u64 would require 2 elements of u32 
// u32 would require 4 elements of u8
//
// count is the number of T elements
fn er<T, U>(count: usize) -> usize {
    use std::num::Float;
    use std::mem;
    
    (   count as f32 * 
        mem::size_of::<T>() as f32 /
        mem::size_of::<U>() as f32
    ).ceil() as usize
}
#[test]
fn test_er() {
    assert!(er::<u64, u8>(1) == 8);
    assert!(er::<u64, u32>(1) == 2);
    assert!(er::<f32, u8>(1) == 4);
    assert!(er::<u8, u64>(1) == 1);
    assert!(er::<u8, u64>(2) == 1);
    assert!(er::<u8, u64>(9) == 2);
}


// Stores a refrence to a raw boxed vector in the vector itself
// Copy is to avoid calling the deallocator on 
pub unsafe fn into_buffer<T: Default+Copy+Clone>(vector: Vec<T>) -> (*mut T, usize) {
    use std::mem;
    use std::ptr;
    
    let req_count = er::<usize, T>(2);
    
    let mut vector: Vec<T> = vec![Default::default(); req_count] + vector.as_slice();
    let vec_ptr = vector.as_mut_ptr();
    let vec_len = vector.len();
    let vec_cap = vector.capacity();
    
    mem::forget(vector);
    
    ptr::write((vec_ptr as *mut usize).offset(0), vec_len);
    ptr::write((vec_ptr as *mut usize).offset(1), vec_cap);
    
    
    (vec_ptr.offset(req_count as isize), vec_len - req_count)
}
// Safely frees the mess
pub unsafe fn free_buffer<T>(buf: *mut T) {
    let vec_ptr: *mut T = buf.offset(-(er::<usize, T>(2) as isize));
    let vec_len = *(vec_ptr as *mut usize).offset(0);
    let vec_cap = *(vec_ptr as *mut usize).offset(1);
    Vec::from_raw_parts(vec_ptr, vec_len, vec_cap);
}

#[test]
fn test_buffers() {
    let v: Vec<u32> = vec![1, 3, 7];
    unsafe {
        let temp = into_buffer(v);
        let p: *mut u32 = temp.0;
        assert!(temp.1 == 3);
        assert!(*(p.offset(0)) == 1);
        assert!(*(p.offset(0)) == 1);
        assert!(*(p.offset(1)) == 3);
        assert!(*(p.offset(2)) == 7);
        free_buffer(p);
    }
}

