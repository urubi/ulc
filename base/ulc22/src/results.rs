pub type StackErr = Vec<String>;


#[macro_export]
macro_rules! some {
    ($e: expr, $err: expr) => (
        match $e {
            Some(o) => o,
            None => $err
        }
    )
}



#[macro_export]
macro_rules! ok {
    ($e: expr, $err: expr) => (
        some!($e.ok(), $err)
    );
    ($e: expr, $err_var: ident, $err: expr) => (
        match $e {
            Ok(o) => o,
            Err($err_var) => $err
        }
    );
}

#[test]
fn test_ok() {
    let mut x: Result<bool, ()>;
    // sanity check
    x = Ok(true);
    assert!(ok!(x, false) == true);
    x = Err(());
    assert!(ok!(x, true) == true);
    
    assert!(ok!(Err(5), err, err) == 5);
    
    // flow control
    x = Err(());
    loop {
        assert!(ok!(x, break));
        panic!("could not break");
    }
    assert!((|| {
        assert!(ok!(x, return true));
        panic!("could not return");
    })());
    
    // blocks
    assert!(ok!(x, {false; true}) == true);
}

#[macro_export]
macro_rules! stacked_assert {
    ($e: expr, $($fargs:tt)*) => (
        match $e {
            Ok(o) => o,
            Err(mut e) => {
                e.push(format!("{}:{}:{}): {}", "function!()", file!(), line!(), format!($($fargs)*)));
                return Err(e);
            }
        }
    )
}
#[macro_export]
macro_rules! stacked_return {
    ($($fargs:tt)*) => ({
        stacked_assert!(Err(vec![]), $($fargs)*);
        unreachable!();
    })
}


// fast num syncing llz


#[test]
fn test_stacked() {    
    assert!((|| {
        let y: Result<(), StackErr> = Ok(());
        assert!(stacked_assert!(y, "Should not fail") == ());
        y
    })().is_ok());
    
    match (|| {
        let y: Result<(), StackErr> = (|| {stacked_return!("This is the root {}", "error");})();
        stacked_assert!(y, "This is a top level error");
        Ok(())
    })() {
        Ok(_) => panic!("Should be an error"),
        Err(x) => {
            println!("{:?}", x);
            assert!(x.len() == 2);
        }
    }
}

