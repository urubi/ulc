//! وحدة تخزين الكتل بالإسم
//! =====================
//! 
//! تعني هذه الوحدة بالتعامل البيانات المقرونة بإسم، في إصدارها الحالي، النوع 
//! الوحيد التي تدعمه هذه الوحدة هو `HashMap`. (قد أفك ارتباط الوحدة بهذا 
//! النوع - بتخصيص سمة وجعل دوال سائبة النوع - إن وجد داع.)

//! صيغة التخزين
//! ---------- 
//! تتكون صيغة التخزين من منطقتين: منطقة الفهرس الذي يحتوي على أسم الكتل 
//! ومواقعها في كتلة المخزن، ومنطقة المستودع لتخزين الكتل. كل الأرقام تستغل
//! ٨ بايتات ومخزنة من الخانة البايتية الصغرى للكبرى (Little Endian).

//! binary format not stable!


/*
last time:
    write better tests
    write better documentatio
    
    add support for appending blobs to large files: issues: need more control -> is appending nessary? ->
        Why not add stream: doesn't work because header size not known ->
        leave the fisrt byte as a pointer to the lookup blob THIS MIGHT ACTUALLY BE WORKABLE
    
    

*/


use std::collections::HashMap;
use ::blob;
use ::unsigned;

/// تنتج مصفوفة بايتات تمثل البيانات المخزنة في القاموس `ar`.
pub struct Archive {
    map: HashMap<String, Vec<u8>>
}

impl Archive {
    pub fn new(blob: Option<&[u8]>) -> Result<Archive, Vec<String>> {
        let mut map: HashMap<String, Vec<u8>> = HashMap::new();
        
        if let Some(blob) = blob {
            let doff = stacked_assert!(get_data_offset(blob), "could not get data offset");
            let lookup = stacked_assert!(get_lookup_table(blob), "could not get list of entries in archive");
            
            for (n, ol) in lookup.iter() {
                map.insert(n.clone(), stacked_slice!(   blob, 
                                                        doff + ol.0, 
                                                        doff + ol.0 + ol.1).to_vec());
            }
        }
        Ok(Archive {map: map})
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut key_blob: Vec<u8> = vec![];
        let mut value_blob: Vec<u8> = vec![];
        for (k, v) in self.map.iter() {
            let mut key_bloblet: Vec<u8> = vec![];
            key_bloblet.append(&mut unsigned::to_le_bytes::<u64>(value_blob.len() as u64));    // offset at which v will be inserted
            key_bloblet.append(&mut unsigned::to_le_bytes::<u64>(v.len() as u64));             // length of v
            key_bloblet.push_all(&mut k.as_bytes());                                           // name
            key_blob.append(&mut blob::packet(key_bloblet.as_ref()));
            value_blob.push_all(v);
        }
        blob::pack([key_blob, value_blob].as_ref())
    }
    
    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.map.get(key)
    }
    
    pub fn insert(&mut self, key: String, value: Vec<u8>) {
        self.map.insert(key, value);
    }
}


// Reading the lookup without loading the files.
// Takes the archive blob FROM THE BEGINING to atleast the last byte of the lookup header
// Does not touch or expect the data blo
fn get_lookup_table(blob: &[u8]) -> Result<HashMap<String, (u64, u64)>, Vec<String>> {
    let mut d: HashMap<String, (u64, u64)> = HashMap::new();
    let lookup_len: u64 = unsigned::from_le_bytes(stacked_slice!(blob, 0, 8));
    let lookup_blob = stacked_slice!(blob, 8, lookup_len+8); // "to" symantics not length
    let lookup_entries = ok!(blob::unpack(lookup_blob), stacked_return!("could not unpack lookup header entries"));
    for entry in lookup_entries {
        let offset = unsigned::from_le_bytes::<u64>(stacked_slice!(entry, 0, 8));
        let length = unsigned::from_le_bytes::<u64>(stacked_slice!(entry, 8, 16));
        let name = ok!(String::from_utf8((&entry[16..]).to_vec()), stacked_return!("invalid name for entry"));
        d.insert(name, (offset, length));
    }
    Ok(d)
}

#[test]
fn test_get_lookup_table() {
    use std::option::Option;
    let mut ar = Archive::new(Option::None).unwrap();
    ar.insert("loli".to_string(), vec![1,2,3,4]);
    ar.insert("a-kun".to_string(), vec![1,2,3,4, 5, 6, 7]);
    let bytes = ar.to_bytes();
    get_lookup_table(bytes.as_ref()).unwrap();
    // Do this properly.. later
}


// reads the first 8 bytes from the archive blob and returns the offset
// at which the data blob starts (the blob that contains the named blobs) 
fn get_data_offset(blob: &[u8]) -> Result<u64, Vec<String>> {
    Ok(unsigned::from_le_bytes(stacked_slice!(blob, 0, 8)) + 16)
}



/// تنتج قاموس للبيانات الممثلة في المصفوفة `blob`.

#[test]
fn test_archive() {
    macro_rules! test_for_input {
        ($e: expr) => (
            {
                use std::option::Option;
                let data:&[(String, Vec<u8>)] = &$e;
                let mut ar = Archive::new(Option::None).unwrap();
                println!("stage 1");
                for &(ref name, ref content) in data.iter() {
                    ar.insert(name.clone(), content.clone());
                }
                
                println!("stage 2");
                for &(ref name, ref content) in data.iter() {
                    let x = ar.get(name).unwrap();
                    let y = content;
                    assert!(x == y);
                }
                
                println!("stage 3");
                let ar = Archive::new(Option::Some(ar.to_bytes().as_ref())).unwrap();
                
                
                println!("stage 4");
                for &(ref name, ref content) in data.iter() {
                    let x = ar.get(name).unwrap();
                    let y = content;
                    assert!(x == y);
                }
            }
        )
    }

    test_for_input!([
        ("File A".to_string(), vec![0x23, 0x3a, 0x00, 0x1f, 0x88]),
        ("File B".to_string(), vec![]),
        ("File C".to_string(), vec![0x00, 0x00]),
        ("File D".to_string(), vec![0x22, 0x23, 0x24, 0x25, 0x26]),
    ]);

    test_for_input!([]);    
}

pub mod file {
    //! read/Semi-write blob access without loading everything in memory
    //! only loads the lookup table. Ment for very large files, or blobs.
    //!
    //! supports read/overwrite, but blob lengths remain constant; cannot 
    //! shrink or grow.
    //!
    //! if shrinking, growing, or appending new blobs in an archive is 
    //! required, you'll need to use the normal functions (i.e. load 
    //! everything in an Archive object).
    use std::collections::HashMap;
    use std::io::Seek;
    use std::io::SeekFrom;
    use std::io::Read;
    use std::io::Write;
    use std::io;
    use unsigned;
    
    
    
    pub struct ArchiveHandle<T> {
        file: T,
        lookup: HashMap<String, (u64, u64)>,
        offset: u64
    }
    
    impl<T: Write + Read + Seek> ArchiveHandle<T> {
        /// read position must be set at the begining of the archive blob
        /// side effects:
        /// sets the position of the file at the begining of the data
        pub fn new(file: T) -> Result<ArchiveHandle<T>, Vec<String>> {
            // getting the length of the lookup table
            let mut file = file;
            let mut len_buf: [u8; 8] = [0; 8];
            if ok!(file.read(&mut len_buf), stacked_return!("Could not read data from file")) != 8 {
                stacked_return!("missing data from file");
            }
            
            // getting the lookup table itself
            let length = unsigned::from_le_bytes(&len_buf);
            let mut buf: Vec<u8> = vec![0; length+8];
            // reseting file position for get_lookup_table which expects the position to be at the begining of the blob (before length bytes)
            ok!(file.seek(SeekFrom::Current(-8)), stacked_return!("wierd error"));
            // reading blob
            if ok!(file.read(&mut buf), stacked_return!("Could not read data from file")) != buf.len() {
                stacked_return!("missing data from file");
            }
            
            let lookup = ok!(super::get_lookup_table(buf.as_ref()), stacked_return!("could not load lookup table"));
            
            // 
            let offset = ok!(file.seek(SeekFrom::Current(8)), stacked_return!("Could not set current position at begining of data"));
            
            Ok( ArchiveHandle {
                file: file,
                lookup: lookup,
                offset: offset
            })
        }
        
        pub fn read(&mut self, name: &str, buf: &mut [u8], blob_offset: u64) -> io::Result<usize> {
            let offset: u64;
            let length: u64;
            
            match self.lookup.get(name) {
                Some(&(o, l)) => {offset=o; length=l;},
                None => return Ok(0)
            };
            
            if blob_offset > length {
                return Err(io::Error::new(
                    io::ErrorKind::Other, 
                    "Out of bonds", 
                    Some(format!("Requesting byte [{}] from a record which is only {} bytes long", blob_offset, length))));
            }
            
            try!(self.file.seek(SeekFrom::Start(self.offset+offset+blob_offset)));
            Ok(try!(self.file.read(mut_slice_at_most!(buf, (length-blob_offset) as usize))))
        }
        
        pub fn overwrite(&mut self, name: &str, buf: &[u8], blob_offset: u64) -> io::Result<()> {
            
            let offset: u64;
            let length: u64;
            
            match self.lookup.get(name) {
                Some(&(o, l)) => {offset=o; length=l;},
                None => return Err(io::Error::new(
                                io::ErrorKind::NotFound, 
                                "Record not found in archive", 
                                None))
            };
            
            if blob_offset > length || buf.len() > (length-blob_offset) as usize {
                return Err(io::Error::new(
                    io::ErrorKind::Other, 
                    "Out of bonds", 
                    Some(format!("Writing upto byte [{}] from a record which is only {} bytes long", blob_offset, length))));
            }
            
            
            try!(self.file.seek(SeekFrom::Start(self.offset+offset+blob_offset)));
            Ok(try!(self.file.write_all(buf)))
        }
        
        
    }

    #[test]
    fn test_archive_file() {
        use std::fs::File;
        use std::option::Option;
        use std::io::Write;
        use std::fs;
        
        let mut file = File::create("/tmp/test_archive_file").unwrap();
        let mut ar =  super::Archive::new(Option::None).unwrap();
        let a_kun = ("A-Kun", vec![0, 1, 2, 3]);
        let b_kun = ("B-Kun", vec![0xff, 0xff]);
        
        ar.insert(a_kun.0.to_string(), a_kun.1.clone());
        ar.insert(b_kun.0.to_string(), b_kun.1.clone());
        
        ok!(file.write_all(ar.to_bytes().as_ref()), panic!("could not write archive to file")); // does not check if all data was writen
        
        
        {
            let file = fs::OpenOptions::new().read(true).write(true).open("/tmp/test_archive_file").unwrap();
            let mut ah = ok!(ArchiveHandle::new(file), panic!("could not get archive handle"));
            
            let mut buf: Vec<u8> = vec![0; 60];
            
            let read_len = ah.read(a_kun.0, &mut buf, 0).unwrap();
            assert!(read_len == a_kun.1.len());
            //println!("Trying to {:?} != {:?} which both should be {} bytes long", &buf[0..read_len], a_kun.1, read_len);
            assert!(&buf[0..read_len] == a_kun.1);
            
            let read_len = ah.read(b_kun.0, &mut buf, 1).unwrap();
            assert!(read_len == b_kun.1.len()-1);
            //println!("Trying to {:?} != {:?} which both should be {} bytes long", &buf[0..read_len], b_kun.1, read_len);
            assert!(&buf[0..read_len] == &b_kun.1[1..]);
            
            assert!(ah.read(b_kun.0, &mut buf, 2).unwrap() == 0);
            assert!(ah.read(b_kun.0, &mut buf, 3).is_err());
            
            
            ah.overwrite(a_kun.0, a_kun.1.as_ref(), 0).unwrap();
            ah.overwrite(a_kun.0, &[3, 0xf, 0xf], 0).unwrap();
            ah.overwrite(a_kun.0, &[0, 0], 1).unwrap();
            ah.overwrite(a_kun.0, &[9], 0).unwrap();
            assert!(ah.overwrite(a_kun.0, &[3, 0, 0, 0, 0], 0).is_err());
            assert!(ah.overwrite(a_kun.0, &[1], 4).is_err());
            
            
        }
        
        let mut ar_bytes: Vec<u8> = vec![];
        fs::File::open("/tmp/test_archive_file").unwrap().read_to_end(&mut ar_bytes).unwrap();
        ar = super::Archive::new(Option::Some(ar_bytes.as_ref())).unwrap();
        assert!(ar.get(a_kun.0).unwrap() == &[9, 0, 0, 3]);
        assert!(ar.get(b_kun.0).unwrap() == &b_kun.1);
    }


}
