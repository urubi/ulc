use errno::*;
use ::File;


// Stat is an uncomplete desaster. DO NOT USE
// Likely to be removed.

/* Do these later
pub const S_ISLNK(m):usize     = (((m) & S_IFMT) == S_IFLNK);
pub const S_ISREG(m):usize     = (((m) & S_IFMT) == S_IFREG);
pub const S_ISDIR(m):usize     = (((m) & S_IFMT) == S_IFDIR);
pub const S_ISCHR(m):usize     = (((m) & S_IFMT) == S_IFCHR);
pub const S_ISBLK(m):usize     = (((m) & S_IFMT) == S_IFBLK);
pub const S_ISFIFO(m):usize    = (((m) & S_IFMT) == S_IFIFO);
pub const S_ISSOCK(m):usize    = (((m) & S_IFMT) == S_IFSOCK);
*/

#[cfg(all(target_os="linux", target_arch="x86_64"))]
#[derive(Default, Debug)]
#[repr(C)]
struct IntStat {
    st_dev          :u64,
    st_ino          :u64,
    st_nlink        :u64,
    st_mode         :u32,
    st_uid          :u32,
    st_gid          :u32,
    __unused0       :u32,
    st_rdev         :u64,
    st_size         :u64,
    st_blksize      :u64,
    st_blocks       :u64,
    st_atime        :u64,
    st_atime_nsec   :u64,
    st_mtime        :u64,
    st_mtime_nsec   :u64,
    st_ctime        :u64,
    st_ctime_nsec   :u64,
    __unused1       :u32,
    __unused2       :u32,
    __unused3       :u32,
    __unused4       :u32,
    __unused5       :u32,
    __unused6       :u32        
}

// Arch independent struct
#[derive(Default, Debug)]
pub struct Stat {
    pub st_dev          :u64,
    pub st_ino          :u64,
    pub st_nlink        :u64,
    pub st_mode         :u32,
    pub st_uid          :u32,
    pub st_gid          :u32,        
    pub st_rdev         :u64,
    pub st_size         :u64,
    pub st_blksize      :u64,
    pub st_blocks       :u64,
    pub st_atime        :u64,
    pub st_atime_nsec   :u64,
    pub st_mtime        :u64,
    pub st_mtime_nsec   :u64,
    pub st_ctime        :u64,
    pub st_ctime_nsec   :u64,
}

impl Stat {
    pub fn isreg(&self)     -> bool { use ::fs::flags::*; (self.st_mode as usize & S_IFMT) == S_IFREG }
    pub fn isdir(&self)     -> bool { use ::fs::flags::*; (self.st_mode as usize & S_IFMT) == S_IFDIR }
    pub fn ischr(&self)     -> bool { use ::fs::flags::*; (self.st_mode as usize & S_IFMT) == S_IFCHR }
    pub fn isblk(&self)     -> bool { use ::fs::flags::*; (self.st_mode as usize & S_IFMT) == S_IFBLK }
    pub fn isfifo(&self)    -> bool { use ::fs::flags::*; (self.st_mode as usize & S_IFMT) == S_IFIFO }
    pub fn islnk(&self)     -> bool { use ::fs::flags::*; (self.st_mode as usize & S_IFMT) == S_IFLNK }
    pub fn issock(&self)    -> bool { use ::fs::flags::*; (self.st_mode as usize & S_IFMT) == S_IFSOCK }
}

pub fn stat(pathname: &str) -> Result<Stat, isize> {
    use std::ffi::CString;
    use std::default::Default;
    
    match CString::new(pathname) {
        Ok(path) => {
            let mut st: IntStat = Default::default();
            let r = signed_syscall!(STAT, path.as_ptr(), (&mut st) as *mut IntStat);
            if r < 0 {
                Err(r as isize)
            }
            else {
                Ok(Stat {
                    st_dev          : st.st_dev,
                    st_ino          : st.st_ino,
                    st_nlink        : st.st_nlink,
                    st_mode         : st.st_mode,
                    st_uid          : st.st_uid,
                    st_gid          : st.st_gid,
                    st_rdev         : st.st_rdev,
                    st_size         : st.st_size,
                    st_blksize      : st.st_blksize,
                    st_blocks       : st.st_blocks,
                    st_atime        : st.st_atime,
                    st_atime_nsec   : st.st_atime_nsec,
                    st_mtime        : st.st_mtime,
                    st_mtime_nsec   : st.st_mtime_nsec,
                    st_ctime        : st.st_ctime,
                    st_ctime_nsec   : st.st_ctime_nsec
                })
            }
        },
        Err(_) => {
            Err(-ENOENT as isize)
        }
    }    
}

pub fn lstat(pathname: &str) -> Result<Stat, isize> {
    use std::ffi::CString;
    use std::default::Default;
    
    match CString::new(pathname) {
        Ok(path) => {
            let mut st: IntStat = Default::default();
            let r = signed_syscall!(LSTAT, path.as_ptr(), (&mut st) as *mut IntStat);
            if r < 0 {
                Err(r as isize)
            }
            else {
                Ok(Stat {
                    st_dev          : st.st_dev,
                    st_ino          : st.st_ino,
                    st_nlink        : st.st_nlink,
                    st_mode         : st.st_mode,
                    st_uid          : st.st_uid,
                    st_gid          : st.st_gid,
                    st_rdev         : st.st_rdev,
                    st_size         : st.st_size,
                    st_blksize      : st.st_blksize,
                    st_blocks       : st.st_blocks,
                    st_atime        : st.st_atime,
                    st_atime_nsec   : st.st_atime_nsec,
                    st_mtime        : st.st_mtime,
                    st_mtime_nsec   : st.st_mtime_nsec,
                    st_ctime        : st.st_ctime,
                    st_ctime_nsec   : st.st_ctime_nsec
                })
            }
        },
        Err(_) => {
            Err(-ENOENT as isize)
        }
    }    
}


pub fn fstat(fd: File) -> Result<Stat, isize> {
    use std::default::Default;
    
    let mut st: IntStat = Default::default();
    let r = signed_syscall!(FSTAT, fd, (&mut st) as *mut IntStat);
    if r < 0 {
        Err(r as isize)
    }
    else {
        Ok(Stat {
            st_dev          : st.st_dev,
            st_ino          : st.st_ino,
            st_nlink        : st.st_nlink,
            st_mode         : st.st_mode,
            st_uid          : st.st_uid,
            st_gid          : st.st_gid,
            st_rdev         : st.st_rdev,
            st_size         : st.st_size,
            st_blksize      : st.st_blksize,
            st_blocks       : st.st_blocks,
            st_atime        : st.st_atime,
            st_atime_nsec   : st.st_atime_nsec,
            st_mtime        : st.st_mtime,
            st_mtime_nsec   : st.st_mtime_nsec,
            st_ctime        : st.st_ctime,
            st_ctime_nsec   : st.st_ctime_nsec
        })
    }
}

#[test]
fn test_stat() {
    use super::flags::*;
    use super::file;
    use super::ln;
    use ::io;
    
    let op = "/tmp/test_stat_normal_file";
    let hp = "/tmp/test_stat_hardlink";
    let sp = "/tmp/test_stat_symlink";
    let phrase = "testing that i am cool";
    
    // cleanup
    ln::unlink(hp);
    ln::unlink(op);
    ln::unlink(sp);
    
    // setup
    let fd = file::open(op, O_WRONLY|O_CREAT, 0o620);
    assert!(fd > 0);
    assert!(io::write(fd, phrase.as_bytes()) == phrase.len() as isize);
    
    
    // test size
    {
        let s = stat(op);    
        assert!(s.is_ok());
        let s = s.unwrap();
        assert!(s.st_size == phrase.len() as u64);
    }
    // sanity test link field 
    {
        assert!(ln::link(op, hp) == 0);
        let s = stat(op);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert!(s.st_nlink == 2);
        assert!(s.isreg());
        assert!(!s.isdir());
    }
    // test symlink travrsal
    {    
        assert!(ln::symlink(op, sp) == 0);
        let s = stat(sp);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert!(s.st_nlink == 2); // didn;'t remove hardlink in prev blk
        assert!(s.st_size == phrase.len() as u64);
        assert!(!s.islnk());
        // stating symlink iteslf
        let s = lstat(sp);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert!(s.st_nlink == 1);
        assert!(s.islnk());
        assert!(ln::unlink(sp) == 0);
    }
    // fstat test
    {
        let s = fstat(fd);
        assert!(s.is_ok());
        let s = s.unwrap();
        assert!(s.st_nlink == 2);
        assert!(s.st_size == phrase.len() as u64);
    }
    ln::unlink(hp);
    ln::unlink(op);
    
    //tty_print!("{} {}", format!("{:?}", s).replace(", ", "\n"), phrase.len());
}


