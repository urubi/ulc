//! # ACID-Like File Access and Manimpulation
//! This module procives an interface for interacting with files under ACID 
//! garantees provided that all concurrent file access is done through this
//! API. If not, only Atomicity and durrability are provided.
//! 
//! ## What to Expect
//! As of the time of this writing, this library creates temporary files 
//! in the same directory as the file being manipulated durring operation.
//! once the file handle is closed, all temporary files will be deleted.
//! 
//! (Needs AT_REPLACE functionality for shadow file)
//! 
//! ## Under the Hood
//! Atomicity is achived by creating a shadow file to which all uncommitted
//! data is written to. The file's inode is then linked to the directory
//! entry when a commit operation is issued. If the relinking succeeds,
//! the file will reflect its new state. If the operation fails, the file
//! will reflect it's old state.
//!
//! Consistancy is provided by atomicity if the file is taken as a database
//! with a single record. Lol.
//!
//! Isolation is provided by Linux's locking machinisms. This is the reason 
//! for requiring that concurrent file access be done through this API.
//!
//! Durrability garantees are provided by the undurling filesystem and 
//! storage medium.
//!


use ulc12;
use ulc22::results::StackErr;

use std::path::PathBuf;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
use std::io;


/// A refrence to an open file that is implements atomic file operations.

/// Atomic operations are implemented using shadow files that reside in the 
/// same directory, and file system linking.

/// Changes to the file are not written to disk until either `commit` is 
/// called or the file goes out of scope. Changes can be discarded by 
/// calling `forget`.

/// This object is designed to mimic the interfaces provided by the standard 
/// File object in `std::fs`: Write, Read, and Seek are implemented. 
/// However, it is not designed to be compatable with the structs specific
/// methods.

/// Example:
/// ```
/// let mut f = File::open("/tmp/atomic", 0o640).unwrap();
/// f.write_all("Welcome to the atomic age!".as_bytes());
/// f.close();
/// ```
    
pub struct File {
    orgin_path  : String,
    orgin_fd    : ulc12::File,
    shadow_path : String,
    shadow_fd   : ulc12::File,
    mode        : usize,
    valid       : bool,
    discard     : bool,
    wait        : bool
}

// Logic flow:
// open sets up File and calls load() which creats the shadow file and locks 
// the orgin
// commit saves by renaming shadow as orgin (might change once AT_REPLACE 
// gets implemented), closes both file discripters 

impl File {
    /// Attempts to return a handle to an atomic file object. `mode` are the 
    /// permissions with which the file should be opended if it deies not exist. 
    /// Files are always opened in RDWR mode with the cursor placed at the end
    /// of the file.
    pub fn open(pathname: &str, mode: usize, wait: bool) -> Result<File, StackErr> {
        use std::path::Path;
        
        let op: PathBuf;
        let mut pp: PathBuf;
        {
            let root = Path::new("/"); // This might not be necissary
            let parts = PathBuf::from(pathname);
            let parent = some!(parts.parent(), root);
            let name = some!(parts.file_name(), stacked_return!("No Filename in '{}'", pathname));
            let mut shadow_name = String::new();
            shadow_name.push_str(".");
            shadow_name.push_str(some!(name.to_str(), stacked_return!("Filename '{}' is not valid utf8", pathname)));
            shadow_name.push_str(".shadow");
            
            op = parts.clone();
            pp = PathBuf::from(parent);
            pp.push(shadow_name);
        }
        
        
        let mut f = File {
            orgin_path  : some!(op.to_str(), stacked_return!("orgin path not valid utf8")).to_string(),
            orgin_fd    : 0,
            shadow_path : some!(pp.to_str(), stacked_return!("shadow path not valid utf8")).to_string(), // err shouldn't occure
            shadow_fd   : 0,
            mode        : mode,
            valid       : false,
            discard     : false,
            wait        : wait
        };
        
        // open files and set up shadow file
        stacked_assert!(f.load(), "Could not set up files for access");
        
        Ok(f)
    }
    
    pub fn set_len(&mut self, length: u64) -> Result<(), StackErr> {
        if ulc12::fs::file::ftruncate(self.shadow_fd, length) != 0 {
            stacked_return!("Could not truncate file to {}", length);
        }
        Ok(())
    }
    
    fn load(&mut self) -> Result<(), StackErr> {
        // orgin file is locked here
        use ulc12::fs::file;
        use ulc12::fs::stat;
        use ulc12::io;
        use ulc12::fs::flags::*;
        use ulc12::errno::*;
        
        self.valid = false;
        
        // opening both orgin and shadow files, and copying contents to shadow
        // linux specific: use another load for alternate systems
        
        self.orgin_fd = file::open(&self.orgin_path, O_RDWR|O_CREAT, self.mode);
        if self.orgin_fd < 0 {
            stacked_return!("Could not open '{}': Error #{}", &self.orgin_path, self.orgin_fd);
        }

        {        
            let w = if !self.wait {LOCK_NB} else {0};
            loop {
                let r = file::flock(self.orgin_fd, LOCK_EX|w);
                if r == -EINTR {continue;}
                else if r == 0 {break;}
                else {
                    stacked_return!("Could not lock '{}': #{}", &self.orgin_path, r);
                }
            }
        }
        
        self.shadow_fd = file::open(&self.shadow_path, O_RDWR|O_CREAT|O_TRUNC, self.mode);
        if self.shadow_fd < 0 {
            file::close(self.orgin_fd);
            stacked_return!("Could not open '{}': Error #{}", &self.shadow_path, self.shadow_fd);
        }
        
        let s = ok!(stat::fstat(self.orgin_fd), stacked_return!("could not stat '{}'", &self.orgin_path));
        
        if io::sendfile(self.shadow_fd, self.orgin_fd, None, s.st_size as usize) < 0 {
            file::close(self.orgin_fd);
            file::close(self.shadow_fd);
            stacked_return!("Could not sync shadow with orgin");
        }
        
        self.valid = true;
        
        Ok(())
    }
    /// Commits changes to disk. This operation can either succeed completely 
    /// or fail 
    pub fn commit(&mut self) -> Result<(), StackErr> {
        stacked_assert!(self.store(), "Could not commit changes");
        stacked_assert!(self.load(), "Could not reload orgin");
        Ok(())
    }
    
    fn store(&mut self) -> Result<(), StackErr> {
        // orgin file is unlocked her, then relocked
        use ulc12::fs::ln;
        use ulc12::fs::file;
        use ulc12::fs::flags::*;
        
        ifn!(self.valid, stacked_return!("Shadowing state is invalid. File object is unusable."));
        
        
        if file::fdatasync(self.shadow_fd) < 0 {
            stacked_return!("Could not write pending data to shadow");
        }
        
        file::close(self.shadow_fd);
        
        if !self.discard {
            if ln::rename(&self.shadow_path, &self.orgin_path) < 0 {
                stacked_return!("Could not sync orgin with shadow");
            }
        }
        else {
            if ln::unlink(&self.shadow_path) < 0 {
                stacked_return!("Could not unlink shadow");
            }            
        }
        
        if file::flock(self.orgin_fd, LOCK_UN) < 0 {
            self.valid = false;
            stacked_return!("Could not unlock orgin");
        }
        
        file::close(self.orgin_fd);
        Ok(())
    }
    
    pub fn close(mut self) -> Result<(), StackErr> {
        self.store()
    }
    
    pub fn forget(mut self) {
        self.discard = true;
    }
    
}

impl Drop for File {
    fn drop(&mut self) {
        let _ = self.store();
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use ulc12::errno::*;
        use std::io::ErrorKind::*;
        
        ifn!(self.valid, return Err(io::Error::new(InvalidInput, "Shadowing state is invalid. File object is unusable.", None)));
        
        let r = ulc12::io::write(self.shadow_fd, buf);
        if r >= 0 {
            Ok(r as usize)
        }
        else if r == -EAGAIN    {Err(io::Error::new(WouldBlock, "Write would block", None))}
        else if r == -EBADF     {Err(io::Error::new(NotFound, "Bad file discriptor", None))}
        else if r == -EDQUOT    {Err(io::Error::new(PermissionDenied, "Exhausted user quota", None))}
        else if r == -EFAULT    {panic!("An attempt was made to access an address outside of this {}",
                                        "applications address space. Something VERY wrong happened")}
        else if r == -EFBIG     {Err(io::Error::new(PermissionDenied, "Write too big for file", None))}
        else if r == -EINTR     {Err(io::Error::new(Interrupted, "Write was interrupted", None))}
        else if r == -EINVAL    {Err(io::Error::new(InvalidInput, "Invalid input was specified", None))}
        else if r == -EIO       {Err(io::Error::new(Other, "Low level I/O error while modifing inode", None))}
        else if r == -ENOSPC    {Err(io::Error::new(PermissionDenied, "No space left on device", None))}
        else if r == -EPIPE     {Err(io::Error::new(BrokenPipe, "A write to a closed pipe or socket", None))}
        else                    {Err(io::Error::new(Other, "An unhandled I/O error occured", None))}
    }
    fn flush(&mut self) -> io::Result<()> {
        use ulc12::errno::*;
        use std::io::ErrorKind::*;

        ifn!(self.valid, return Err(io::Error::new(InvalidInput, "Shadowing state is invalid. File object is unusable.", None)));
        
        let r = ulc12::fs::file::fdatasync(self.shadow_fd);
        if r == 0 {
            Ok(())
        }
        else if r == -EBADF     {Err(io::Error::new(NotFound, "Bad file discriptor", None))}
        else if r == -EIO       {Err(io::Error::new(Other, "Low level I/O error while modifing inode", None))}
        else if r == -EINVAL    {Err(io::Error::new(InvalidInput, "Special file does not support syncing", None))}
        else                    {Err(io::Error::new(Other, "An unhandled I/O error occured", None))}
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use ulc12::errno::*;
        use std::io::ErrorKind::*;
        
        ifn!(self.valid, return Err(io::Error::new(InvalidInput, "Shadowing state is invalid. File object is unusable.", None)));
        
        let r = ulc12::io::read(self.shadow_fd, buf);
        if r >= 0 {
            Ok(r as usize)
        }
        else if r == -EAGAIN    {Err(io::Error::new(WouldBlock, "Reqd would block", None))}
        else if r == -EBADF     {Err(io::Error::new(NotFound, "Bad file discriptor", None))}
        else if r == -EFAULT    {panic!("An attempt was made to access an address outside of this {}",
                                        "applications address space. Something VERY wrong happened")}
        else if r == -EINTR     {Err(io::Error::new(Interrupted, "Read was interrupted", None))}
        else if r == -EINVAL    {Err(io::Error::new(InvalidInput, "Invalid input was specified", None))}
        else if r == -EIO       {Err(io::Error::new(Other, "Low level I/O error while modifing inode", None))}
        else if r == -ENOSPC    {Err(io::Error::new(PermissionDenied, "No space left on device", None))}
        else if r == -EISDIR    {Err(io::Error::new(InvalidInput, "Cannot read a directory", None))}
        else                    {Err(io::Error::new(Other, "An unhandled I/O error occured", None))}
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        use std::io::ErrorKind::*;
        use ulc12::fs::flags::*;
        use ulc12::fs::file::lseek;
        use ulc12::errno::*;
        // should you be using llseek? No, unless you're using x86. And even 
        // then, I think it should be handled by the system call wrapper lib
        // should I explicitly handle negative offsets ("-10 from end of file")?
        // Can lseek handle these cases? Will it's behavior be compatable with
        // io::Seek's specification? Dunno.
        
        ifn!(self.valid, return Err(io::Error::new(InvalidInput, "Shadowing state is invalid. File object is unusable.", None)));
        
        let whence: usize;
        let offset: isize;
        match pos {
            SeekFrom::Start(u) => {
                whence = SEEK_SET;
                offset = u as isize;
            }
            SeekFrom::End(i) => {
                whence = SEEK_END;
                offset = i as isize;
            }
            SeekFrom::Current(i) => {
                whence = SEEK_CUR;
                offset = i as isize;
            }
        }
        let r = lseek(self.shadow_fd, offset, whence);
        
        if r >= 0 {
            Ok(r as u64)
        }
        else if r == -EBADF     {Err(io::Error::new(NotFound, "Bad file discriptor", None))}
        else if r == -EINVAL    {Err(io::Error::new(InvalidInput, "Whence is not valid or the seek is imposible (out of bounds)", None))}
        else if r == -EOVERFLOW {Err(io::Error::new(Other, "Result cant be stored in the return type (>= 1<<64)", None))}
        else if r == -ESPIPE    {Err(io::Error::new(Other, "Cannot seek a pipe", None))}
        else if r == -ENXIO     {Err(io::Error::new(InvalidInput, "Out of bounds and seeking data/holes", None))} 
        else                    {Err(io::Error::new(Other, "An unhandled I/O error occured", None))}
        
    }
}



#[cfg(test)]
fn shadow_data() -> String {
    use std::fs::File;
    
    // explicit path to make sure the file is where it's supposed to be.
    let mut out: Vec<u8> = vec![];
    let mut f = ok!(File::open("/tmp/.acid_test.shadow"), panic!(""));
    
    ok!(f.read_to_end(&mut out), panic!(""));
    ok!(String::from_utf8(out), panic!(""))
}

#[cfg(test)]
fn orgin_data() -> String {
    use std::fs::File;
    
    let mut out: Vec<u8> = vec![];
    let mut f = ok!(File::open("/tmp/acid_test"), panic!(""));
    
    ok!(f.read_to_end(&mut out), panic!(""));
    ok!(String::from_utf8(out), panic!(""))
}


#[cfg(test)]
fn test_for_pathname(pathname: &str) {
    use std::thread;
    use std::time::duration::Duration;
    
    ulc12::fs::ln::unlink(pathname);
    let mut f = ok!(File::open(pathname, 0o640, false), panic!(""));
    
    // testing staging
    assert!(f.write_all("Hello world".as_bytes()).is_ok());
    assert!(orgin_data() == "");
    assert!(shadow_data() == "Hello world");
    f.valid = false;
    assert!(f.write_all("Hello world".as_bytes()).is_err());
    f.valid = true;
    assert!(orgin_data() == "");
    assert!(shadow_data() == "Hello world");
    assert!(f.commit().is_ok());
    assert!(f.commit().is_ok());
    assert!(shadow_data() == orgin_data());
    
    // post commit writes
    assert!(shadow_data() == "Hello world");
    assert!(f.write_all("\n-|<".as_bytes()).is_ok());
    assert!(orgin_data() == "Hello world");
    assert!(shadow_data() == "Hello world\n-|<");
    
    // seek 
    assert!(f.seek(SeekFrom::Start(6)).is_ok());
    assert!(f.write_all("you ;)".as_bytes()).is_ok());
    assert!(orgin_data() == "Hello world");
    assert!(shadow_data() == "Hello you ;)-|<");
    assert!(f.close().is_ok());
    assert!(orgin_data() == "Hello you ;)-|<"); // shadow died
    
    // TODO better test Seek and lseek compat
    
    // close then reopen open && out of scope test
    {
        let mut f = ok!(File::open(pathname, 0o640, false), panic!());
        assert!(shadow_data() == orgin_data());
        assert!(orgin_data() == "Hello you ;)-|<");
        assert!(f.write_all("\\".as_bytes()).is_ok());
    }
    assert!(orgin_data() == "Hello you ;)-|<\\");
    
    // forget
    let mut f = ok!(File::open(pathname, 0o640, false), panic!());
    assert!(shadow_data() == orgin_data());
    assert!(f.write_all("blabla".as_bytes()).is_ok());
    assert!(shadow_data() == "Hello you ;)-|<\\blabla");
    assert!(orgin_data() == "Hello you ;)-|<\\");
    f.forget();
    assert!(orgin_data() == "Hello you ;)-|<\\");
    
    let mut f = ok!(File::open(pathname, 0o640, false), panic!(""));
    assert!(f.set_len(0).is_ok());
    assert!(f.close().is_ok());
    assert!(orgin_data() == "");
    
    // wait
    let pathname_string = pathname.to_string();
    thread::spawn(move || {
        tty_print!(".");
        let f = File::open(&pathname_string, 0o640, false);
        assert!(f.is_ok());
        thread::sleep(Duration::milliseconds(800));
    });
    thread::sleep(Duration::milliseconds(650));
    assert!(File::open(pathname, 0o640, false).is_err());
    assert!(File::open(pathname, 0o640, true).is_ok());
}

#[test]
fn it_works() {
    use std::env;
    use std::path::PathBuf;
    test_for_pathname("/tmp/acid_test");
    assert!(env::set_current_dir(&PathBuf::from("/tmp")).is_ok());
    test_for_pathname("acid_test");
    test_for_pathname("../tmp/acid_test");
    assert!(ulc12::fs::ln::unlink("acid_test") == 0);
    tty_print!("\n");
}

