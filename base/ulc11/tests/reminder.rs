#![feature(fs)]
#![feature(io)]
#[test]
fn reminder() {
    use std::fs::File;
    use std::io::Write;
    
    // maybe run the script from here once rust gets passable documentation
    
    write!(&mut File::create("/dev/tty").unwrap(), 
        "
        [1;93m****************************************************************[0m
        [1;93m**                                                            **[0m
        [1;93m**   Dont forget to run the test script './tests/script.sh'   **[0m
        [1;93m**                                                            **[0m
        [1;93m****************************************************************[0m
        \n").unwrap();
}

