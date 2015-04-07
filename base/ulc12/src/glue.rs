#[macro_export]
macro_rules! signed_syscall {
    ($($t:tt)*) => (
        unsafe {syscall!($($t)*)} as isize
    )
}
