use ::File;

pub fn write(fd: File, buf: &[u8]) -> isize {
    signed_syscall!(WRITE, fd, buf.as_ptr(), buf.len())
}

pub fn read(fd: File, buf: &mut[u8]) -> isize {
    signed_syscall!(READ, fd, buf.as_mut_ptr(), buf.len())
}

pub fn sendfile(out_fd: File, in_fd: File, offset: Option<&mut usize>, count: usize) -> isize {
    match offset {
        Some(a) => signed_syscall!(SENDFILE, out_fd, in_fd, a as *mut usize, count),
        None => signed_syscall!(SENDFILE, out_fd, in_fd, 0, count)
    }
}
