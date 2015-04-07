//! وحدة تجميع الكتل 
//! ==============
//! 
//! تعني هذه الوحدة برص مجموعة كتل في كتلة واحدة جامعة.
//! packet and pack are geranteed to produce output that can be concatinated together (TODO add test)

use ::unsigned;
use std::convert::AsRef;


pub fn pack<T, U>(segments: U) -> Vec<u8> 
    where   T: AsRef<[u8]>,
            U: AsRef<[T]>
{
    let mut blob: Vec<u8> = vec![];
    for seg in segments.as_ref().iter() { 
        blob.push_all(&packet(seg.as_ref())); 
    }
    blob
}

pub fn packet(blob: &[u8]) -> Vec<u8> 
{
    let mut out: Vec<u8> = vec![];
    out.push_all(&unsigned::to_le_bytes(blob.len() as u64));
    out.push_all(blob);
    out
}

pub fn unpack(blob: &[u8]) -> Result<Vec<Vec<u8>>, ()> {
    let mut segments: Vec<Vec<u8>> = vec![];
    let mut cur = 0;
    
    macro_rules! consume {
        ($N:expr) => (
            if (cur+$N) > blob.len() {
                return Err(());
            }
            else {                
                cur+=$N;
                &blob[cur-$N..cur]
            }
        )
    }
    
    while cur != blob.len() {
        let len = unsigned::from_le_bytes::<u64>(&consume!(8));
        let mut v = Vec::new();
        v.push_all(consume!(len as usize));
        segments.push(v);
    }
    Ok(segments)
}

#[test]
fn test_blob_segments(){
    println!("- slices -");
    let m: &[&[u8]] = &[
        &[1, 2, 3],
        &[4],
        &[5],
        &[6],
        &[],
        &[7, 8, 9, 10],
        &[],
        &[],
        &[11]
    ];
    let n = unpack(&pack(m)).unwrap();
    assert!(m.len() == n.len());
    for i in m.iter().zip(n.iter()) {
        println!("comparing {:?} with {:?}", i.0, i.1);
        assert!(i.0 == i.1);
    }
    
    
    println!("- vectors -");
    let m: Vec<Vec<u8>> = vec![
        vec![1, 2, 3],
        vec![4],
        vec![5],
        vec![6],
        vec![],
        vec![7, 8, 9, 10],
        vec![],
        vec![],
        vec![11]
    ];
    let n = unpack(&pack(&m)).unwrap();
    assert!(m.len() == n.len());
    for i in m.iter().zip(n.iter()) {
        println!("comparing {:?} with {:?}", i.0, i.1);
        assert!(i.0 == i.1);
    }
    
    println!("- empty -");
    let m: &[&[u8]] = &[];
    let n = unpack(&pack(m)).unwrap();
    assert!(m.len() == n.len());
    for i in m.iter().zip(n.iter()) {
        println!("comparing {:?} with {:?}", i.0, i.1);
        assert!(i.0 == i.1);
    }
    
}

