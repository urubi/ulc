//! مكتبة ulc11: تَضمين البيانات في صُورة البَرَامج المُصَرَفَة
//! ==============================================


#![feature(path)]
#![feature(fs)]
#![feature(io)]
#![feature(std_misc)]
#![feature(collections)]

#![feature(core)]

extern crate ulc91;
extern crate ulc21;

use std::path::PathBuf;
use std::io;

/// This enum specifies the path of the executable to operate on. It can be
/// an explicit path, or `This` which denotes the current executing executable.
pub enum ExecPath {
    File(PathBuf),
    This
}
/// Operations on an Executable file.
pub trait Embed { //: io::Read + io::Write 
    /// Reads a blob embeded in the executable. Always returns an array 
    /// (empty if no data is embeded).
    fn load(&self) -> io::Result<Vec<u8>>;
    /// Strips the embeded blob from the executable if it exist.
    fn strip(&mut self) -> io::Result<()>;
    /// Embeds `data` into the executable. 
    fn store(&mut self, data: &[u8]) -> io::Result<()>;
}


/// Operations on an Executable file.
pub trait NewEmbed: io::Read + io::Write  {
    fn strip(&mut self) -> io::Result<()>;
}


pub mod c;
pub mod auto;
pub mod generic;


