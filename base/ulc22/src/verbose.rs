#[macro_export]
macro_rules! tty_print {
    ($($fargs:tt)*) => ({
        use std::fs::OpenOptions;
        use std::io::Write;
        
        if let Ok(mut f) = OpenOptions::new().write(true).open("/dev/tty") {
            let _ = write!(&mut f, $($fargs)*);
        }
    })
}

#[macro_export]
macro_rules! tty_debug {
    ($($fargs:tt)*) => (
        tty_print!("{}:{}:{}: {}", "function!()", file!(), line!(), format!($($fargs)*));
    )
}
