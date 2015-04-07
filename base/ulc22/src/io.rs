#[macro_export]
macro_rules! ir_ret {
    ($description: expr, $($fargs:tt)*) => ({
        use std::io::{Error, ErrorKind};
        let io_err = Err(Error::new(ErrorKind::Other, $description, Some(format!($($fargs)*))));
        return io_err;
    });
    ($description: expr) => ({
        use std::io::{Error, ErrorKind};
        let io_err = Err(Error::new(ErrorKind::Other, $description, None));
        return io_err;
    })
}
#[macro_export]
macro_rules! ir {
    ($x: expr, $description: expr, $($fargs:tt)*) => ({
        match $x {
            Ok(r) => r,
            Err(e) => {
                use std::io::Error;
                let io_err = Err(Error::new(e.kind(), $description, Some(
                    format!("{}\n\t{}", format!($($fargs)*), e)
                )));
                return io_err;
            }
        }
    });
    ($x: expr, $description: expr) => ({
        match $x {
            Ok(r) => r,
            Err(e) => {
                use std::io::Error;
                let io_err = Err(Error::new(e.kind(), $description, Some(format!("{}", e))));
                return io_err;
            }
        }
    })
    
}
