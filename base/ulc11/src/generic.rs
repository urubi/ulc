use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::path::PathBuf;
use std::os::unix::OpenOptionsExt;
use ulc91::unsigned;
use ::ExecPath;
use ::Embed;

/* The module for the format-agnostic data embedder: GenericEmbed

    +---------------+
    |               |
    |               |
    |   Program     |
    |               |
    |               |
    |---------------|           +---------------+       +----------------------+
    |  Embeded Blob |    <-     |    AR Blob    |   <-  |  ulc91::archive blob |
    +---------------+           |---------------|       +----------------------+
                                |  ARB Length   |
                                |---------------|
                                |   MAGIC NO    |
                                +---------------+

*/


macro_rules! debug_try {
     // 1/2^128, taken from the middle to avoid confusion with ^
    ($e: expr) => (match $e {
        Ok(r) => r,
        Err(e) => panic!("))))))))))))))))))))))) error in {}:{}: {}", file!(), line!(), e.description())
    })
}

// -------------------------------------------------------------------------

const MARKER: [u8;18] = [   0x63, 0x29, 0x7a, 0x8a, 0x3f, 0x73, 0x03, 0xf8, 
                            0x2f, 0xfe, 0x9b, 0x65, 0x20, 0x08, 0x96, 0x8d, 
                            0x0e, 0x40];
                            
macro_rules! magic_marker {
     // 1/2^128, taken from the middle to avoid confusion with ^
    () => (&MARKER[1..17])
}

const NEG_OFFSET_MAGIC: i64 = -16;
const NEG_OFFSET_LEN: i64 = NEG_OFFSET_MAGIC-8;
const NEG_OFFSET_ALL: i64 = NEG_OFFSET_LEN;

// -------------------------------------------------------------------------

// generates embeded blob as specified above
fn gen_embed_blob(ar_blob: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];
    out.push_all(ar_blob);
    out.push_all(&unsigned::to_le_bytes(ar_blob.len() as u64)); // length as 8-byte LE int
    out.push_all(magic_marker!());
    out
}

// -------------------------------------------------------------------------

// This function finds the negative offset (relative to the end of the 
// file) of the embed blob and the blob's length. If a blob is not found,
// None is returned.
fn offset_and_length(e: &mut fs::File) -> io::Result<Option<(i64, u64)>> {
    let mut b: Vec<u8> = vec![];
    
    // read the last 32 bytes of the executable to compare to the marker.
    try!(e.seek(io::SeekFrom::End(NEG_OFFSET_MAGIC)));
    try!(e.read_to_end(&mut b));
    
    // if no magic marker (see top of file) is present, assume no data 
    // is embeded.
    if b != magic_marker!() {
        return Ok(Option::None);
    }
    
    // Read length of embeded data 
    let mut dl: u64;
    {
        let mut l: [u8; 8] = [0u8; 8];
        try!(e.seek(io::SeekFrom::End(NEG_OFFSET_LEN)));
        if let Ok(count) = e.read(&mut l) {
            if count != 8 {
                return Result::Err(io::Error::new(io::ErrorKind::Other, "Bad len read", Option::None)); 
            }
        }
        dl = unsigned::from_le_bytes(&l);
    }
    Ok(Some((NEG_OFFSET_ALL - (dl as i64), dl)))
}

// -------------------------------------------------------------------------


/// `GenericEmbed` deals with packing data into generic executables. 
/// Executables that make use of this embedder must be uneffected by 
/// arbitrary data appended to the end of the executable's file.
/// (e.g. elf binaries)
pub struct GenericEmbed {
    filename: PathBuf,  // Path to executable
}
impl GenericEmbed {
    /// Allocates a new GenericEmbed object. This function might fail if 
    /// an IO error occures.
    pub fn new(executable: ExecPath) -> io::Result<GenericEmbed> {
        let filename = match executable {
            ExecPath::File(p) => p,
            ExecPath::This => fs::read_link("/proc/self/exe").unwrap()
        };
        Result::Ok(GenericEmbed {filename: filename})
    }
}
impl Embed for GenericEmbed {
    fn load(&self) -> io::Result<Vec<u8>> {
        let mut b: Vec<u8>;
        let mut fd: fs::File;
        let off_len: (i64, u64);
        
        fd = try!(fs::File::open(&self.filename));
        
        // get data offset and length if available
        match try!(offset_and_length(&mut fd)) {
            Some(ol) => off_len = ol,
            None => return Result::Ok(vec![])
        }
        
        // read the data from the file and return
        b = vec![];
        try!(fd.seek(io::SeekFrom::End(off_len.0)));
        try!(fd.read_to_end(&mut b));
        b.truncate(off_len.1 as usize);
        Ok(b)
    }
    
    
    fn strip(&mut self) -> io::Result<()>{
        let mut fd: fs::File;
        
        fd = try!(fs::File::open(&self.filename));        
        
        // Linux doesn't like the binary being modified while the process is 
        // running (kernel doesn't load the entire elf into memory). As a work-
        // around, I'll unlink the old one and write a new one.
        // 
        // TODO more effecient way than copying file into memory, stream plz 
        // TODO failsafe way to do this: executable might delete itself without 
        // saving new exec if error occurs while writing new copy
    
        let mut buf: Vec<u8> = vec![];
        try!(fd.read_to_end(&mut buf));
        drop(fd);
        try!(fs::remove_file(&self.filename));
        fd = try!(fs::OpenOptions::new()
            .read(true).write(true).create(true).mode(0o700)
            .open(&self.filename));
        // copying old file to new
        try!(fd.write_all(&buf));
        
        
        // check if a blob is embeded in the exec, and remove by truncating
        match try!(offset_and_length(&mut fd)) {
            Some(ol) => try!(fd.set_len((try!(fd.metadata()).len() as i64 + ol.0) as u64)),
            None => ()
        }
        
        Ok(())
    }
    
    fn store(&mut self, data: &[u8]) -> io::Result<()> {
        let mut fd: fs::File;
        
        // removes old data if preset
        try!(self.strip());
        
        // wraps data in an understood format 
        let blob = gen_embed_blob(data);
        
        // append the blob to the executable
        fd = try!(fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&self.filename));
                    
        if try!(fd.write(&blob)) != blob.len() {
            Err(io::Error::new(io::ErrorKind::Other, "Bad blob write", Option::None))
        }
        else {
            Ok(())
        }
    }
}


#[test]
fn test_generic() {
    let mut p = GenericEmbed::new(ExecPath::This).unwrap();
    
    assert!(p.load().unwrap().len() == 0);
    assert!(p.store(&[1u8; 5]).is_ok());
    assert!(p.load().unwrap() == [1u8; 5]);
    match p.store(&[12u8; 12]) {
        Err(e) => panic!("{}", e.detail().unwrap()),
        _ => ()
    }
    assert!(p.store(&[12u8; 12]).is_ok());
    assert!(p.load().unwrap().len() == 12);
    assert!(p.load().unwrap()[11] == 12u8);
    assert!(p.strip().is_ok());
    assert!(p.load().unwrap().len() == 0);
}
