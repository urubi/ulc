use errno::*;
use ::File;

pub fn open(pathname: &str, flags: usize, mode: usize) -> File {
    use std::ffi::CString;
    match CString::new(pathname) {
        Ok(path) => {
            signed_syscall!(OPEN, path.as_ptr(), flags, mode)
        },
        Err(_) => {
            -ENOENT
        }
    }
}

pub fn close(fd: File) -> isize {
    signed_syscall!(CLOSE, fd)
}


pub fn fsync(fd: File) -> isize {
    signed_syscall!(FSYNC, fd)
}

pub fn fdatasync(fd: File) -> isize {
    signed_syscall!(FDATASYNC, fd)
}

pub fn lseek(fd: File, offset: isize, whence: usize) -> isize {
    signed_syscall!(LSEEK, fd, offset, whence)
}

pub fn ftruncate(fd: File, length: u64) -> isize {
    signed_syscall!(FTRUNCATE, fd, length)
}

pub fn flock(fd: File, operation: usize) -> isize {
    signed_syscall!(FLOCK, fd, operation)
}

pub fn truncate(pathname: &str, length: u64) -> isize {
    use std::ffi::CString;
    match CString::new(pathname) {
        Ok(path) => {
            signed_syscall!(TRUNCATE, path.as_ptr(), length)
        },
        Err(_) => {
            -ENOENT
        }
    }
}


