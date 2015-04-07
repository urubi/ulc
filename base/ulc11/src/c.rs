//extern crate libc;

use std::str;
use std::ffi;
use std::ptr;
use std::slice;
use std::path::PathBuf;

use ::ExecPath;
use ::Embed;
use ::auto::AutoEmbed;

use ::ulc21::object;
use ::ulc21::buffer;

#[no_mangle]
pub extern "C" fn ulc11_new(name: *const i8) -> *mut AutoEmbed {
    let ep: ExecPath;
    
    if name.is_null() {
        ep = ExecPath::This;
    }
    else {
        match unsafe {str::from_utf8(ffi::CStr::from_ptr(name).to_bytes())} {
            Ok(filename) => {ep = ExecPath::File(PathBuf::new(filename));},
            _ => {return ptr::null_mut();}       
        }
    }
    match AutoEmbed::new(ep) {
        Ok(e) => unsafe {object::into_obj(e)},
        _ => ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn ulc11_free(e: *mut AutoEmbed) {
    unsafe { object::free_obj(e) };
}

#[no_mangle]
pub extern "C" fn ulc11_data_strip(e: *mut AutoEmbed) -> bool{
    match unsafe {(*e).strip()} {
        Ok(_) => true,
        Err(_) => false
    }
}

#[no_mangle]
pub unsafe extern "C" fn ulc11_data_load(e: *mut AutoEmbed, length: *mut u64) -> *mut u8 {
    match (*e).load() {
        Ok(l) => {
            let (ptr, len) = buffer::into_buffer(l);
            *length = len as u64;
            return ptr;
        }
        _ => {
            return ptr::null_mut();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ulc11_data_free(data: *mut u8) {
    buffer::free_buffer(data);
}


#[no_mangle]
pub extern "C" fn ulc11_data_store(e: *mut AutoEmbed, data: *mut u8, length: u64) -> bool {
    match unsafe {(*e).store(slice::from_raw_parts(data, length as usize))} {
        Ok(_) => {true},
        Err(_) => {false}
    }
}
