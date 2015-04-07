use errno::*;
use ::Dir;

pub fn readlink(pathname: &str) -> Result<String, isize> {
    use std::ffi::CString;
    
    const SIZE: usize = 4096;
    let mut buf: Vec<u8> = vec![0; SIZE];
    
    match CString::new(pathname) {
        Ok(path) => {
            let r = signed_syscall!(READLINK, path.as_ptr(), buf.as_mut_ptr(), SIZE);
            if r < 0 {
                Err(r)
            }
            else {
                buf.truncate(r as usize);
                if let Ok(s) = String::from_utf8(buf) {
                    Ok(s)
                }
                else {
                    tty_debug!("Non utf8 pathname returned by the kernel. Posible trouble ahead!\n");
                    Err(-EINVAL)
                }
            }
        },
        Err(_) => {
            Err(-ENOENT)
        }
    }
}

pub fn symlink(oldpath: &str, newpath: &str) -> isize {
    use std::ffi::CString;
    
    let op = CString::new(oldpath);
    let np = CString::new(newpath);
    
    if op.is_err() || np.is_err() {
        -EINVAL
    }
    else {
        signed_syscall!(SYMLINK, op.unwrap().as_ptr(), np.unwrap().as_ptr())
    }
}

pub fn link(oldpath: &str, newpath: &str) -> isize {
    use std::ffi::CString;
    
    let op = CString::new(oldpath);
    let np = CString::new(newpath);
    
    if op.is_err() || np.is_err() {
        -EINVAL
    }
    else {
        signed_syscall!(LINK, op.unwrap().as_ptr(), np.unwrap().as_ptr())
    }
}

pub fn linkat(olddirfd: Dir, oldpath: &str, newdirfd: Dir, newpath: &str, flags: usize) -> isize {
    use std::ffi::CString;
    
    let op = CString::new(oldpath);
    let np = CString::new(newpath);
    
    if op.is_err() || np.is_err() {
        -EINVAL
    }
    else {
        signed_syscall!(LINKAT, olddirfd, op.unwrap().as_ptr(), newdirfd, np.unwrap().as_ptr(), flags)
    }
}

pub fn unlink(pathname: &str) -> isize {
    use std::ffi::CString;
    
    let p = CString::new(pathname);
    
    if p.is_err() {
        -EINVAL
    }
    else {
        signed_syscall!(UNLINK, p.unwrap().as_ptr())
    }
}

pub fn rename(oldpath: &str, newpath: &str) -> isize {
    use std::ffi::CString;
    
    let op = CString::new(oldpath);
    let np = CString::new(newpath);
    
    if op.is_err() || np.is_err() {
        -EINVAL
    }
    else {
        signed_syscall!(RENAME, op.unwrap().as_ptr(), np.unwrap().as_ptr())
    }
}
