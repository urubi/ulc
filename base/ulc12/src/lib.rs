#![allow(trivial_numeric_casts)]
#![allow(trivial_casts)]

#[macro_use]
extern crate syscall;
#[macro_use]
extern crate ulc22;

// This library should only provide 64bit linux api (including 32-bit 
// application). If the underlying system calls (or their 64-bit equivilants) 
// can't handle values >= 32<<1 , then -EOVERFLOW is returned.

// This library should remain as close to the original api as posible; reusing 
// names and signitures when posible. Only unsafe usages and macros can be 
// rustified. (macros are defined as methods on the types their applied to)




// TODO should I explicitly change everything to handle 64bit by default, and 
// handle 32bit either manually or fail if not posible?
// This will solve many, many headaches.
// Downside: wont be a thin system call library anymore.


#[macro_use]
mod glue;

pub type File = isize;
pub type Dir = File;

pub mod errno;
pub mod fs;
pub mod io;


#[test]
fn it_works() {
    // Sanity testing: no covrage garantees
    use errno::*;
    use fs::flags::*;
    use fs::file;
    
    let example = "\nRaw baby. Raw.\n".as_bytes();
    let example_len = example.len() as isize;
    {
    assert!(io::write(1, example) == example_len);
    assert!(io::write(50, example) == -EBADF);
    assert!(file::open("/asfsdgf/sdfsdg", O_RDONLY, 0) == -ENOENT);
    }
    {    
        let fd = file::open("/tmp/open_test", O_WRONLY|O_CREAT, 0o640);
        assert!(fd > 0);
        assert!(io::write(fd, example) == example_len);
        assert!(file::close(fd) == 0);
    }
    {
        let mut buf:Vec<u8> = vec![0;30];
        let fd = file::open("/tmp/open_test", O_RDONLY, 0);
        assert!(fd > 0);
        assert!(io::read(fd, buf.as_mut_slice()) == example_len);
        assert!(&buf[..example_len as usize] == example);
        assert!(file::close(fd) == 0);
    }
    {
        use std::fs::File;
        let read_fd = file::open("/tmp/open_test", O_RDONLY, 0);
        let write_fd = file::open("/tmp/open_test_2", O_WRONLY|O_CREAT|O_TRUNC, 0o640);
        let read_fd_len = File::open("/tmp/open_test").unwrap().metadata().unwrap().len();
        assert!(example_len == read_fd_len as isize);
        assert!(io::sendfile(write_fd, read_fd, None, read_fd_len as usize) == read_fd_len as isize);
        file::close(read_fd);
        file::close(write_fd);
        let write_fd = file::open("/tmp/open_test_2", O_RDONLY, 0);
        let mut buf:Vec<u8> = vec![0;30];
        assert!(io::read(write_fd, buf.as_mut_slice()) == example_len);
        file::close(write_fd);
    }
    {
        use fs::ln;
        use std::process::Command;
        Command::new("rm").arg("/tmp/open_test.ln").output().unwrap();
        Command::new("ln").arg("-s").arg("/tmp/open_test").arg("/tmp/open_test.ln").output().unwrap();
        assert!(ln::readlink("/tmp/open_test.ln").unwrap() == "/tmp/open_test");
        assert!(ln::readlink("/tmp/open_test") == Err(-EINVAL));
    }
    {
        use fs::ln;
        let r = ln::unlink("/tmp/klutplut");
        assert!(r == -ENOENT);
        let fd = file::open("/tmp/13333333", O_WRONLY|O_CREAT, 0o640);
        assert!(fd > 0);
        assert!(io::write(fd, &[55, 55, 55]) > 0);
        assert!(ln::rename("/tmp/13333333", "/tmp/122") == 0);
        assert!(ln::unlink("/tmp/122") == 0);
        assert!(file::close(fd) == 0);
    }
    {
        use fs::ln;
        use fs::file;
        use std::process::Command;
        Command::new("rm").arg("/tmp/open_test.hardlink").output().unwrap();
        
        let fd = file::open("/tmp/open_test", O_RDONLY, 0); assert!(fd > 0);
        let sympath = format!("/proc/self/fd/{}", fd);
        assert!(ln::linkat(0, &sympath, 0, "/tmp/open_test.hardlink", AT_SYMLINK_FOLLOW) == 0);
    }
    
}
