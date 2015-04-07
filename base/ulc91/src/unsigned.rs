use std::num::*;
use std::mem;

pub fn to_le_bytes<T: Int>(n: T) -> Vec<u8> {
    let mut num: u64 = u64::from(n).unwrap();
    let mut buf: Vec<u8> = Vec::with_capacity(mem::size_of::<T>());
    
    for _ in 0..mem::size_of::<T>() {
        buf.push((num & 0xff) as u8);
        num >>= 8;
    }
    
    buf
}

pub fn from_le_bytes<T: Int>(buf: &[u8]) -> T {
    let mut num: u64 = 0;
    let mut count = 0;
    
    assert!(buf.len() == mem::size_of::<T>());
    
    for i in buf.iter() {
        num |= (*i as u64) << count;
        count+=8;
    }
    
    T::from(num).unwrap()
}

pub fn to_be_bytes<T: Int>(n: T) -> Vec<u8> {
    let mut b = to_le_bytes(n);
    b.reverse();
    b
}
pub fn from_be_bytes<T: Int>(buf: &[u8]) -> T {
    let mut b = buf.to_vec(); // too busy, do effeciantly later TODO
    b.reverse();
    from_le_bytes(&b)
}

#[test]
fn test_unsigned() {
    macro_rules! test_le {
        ($val: expr, $t: ty) => (
            assert!($val == from_le_bytes::<$t>(&to_le_bytes::<$t>($val)));
        )
    }
    macro_rules! test_be {
        ($val: expr, $t: ty) => (
            assert!($val == from_le_bytes::<$t>(&to_le_bytes::<$t>($val)));
        )
    }
    test_le!(0, u64);
    test_le!(15, u8);
    test_le!(0xff, u8);
    test_le!(1312412354, u64);
    test_le!(1312412352352435324, u64);
    test_le!(1244556, u32);
    test_le!(12400, u16);
    
    let bs = to_le_bytes(0x1ffu16);
    println!("{:?}", bs);
    assert!(bs[0] == 0xff);
    assert!(bs[1] == 0x1);
    
    
    test_be!(0, u64);
    test_be!(1312412354, u64);
    test_be!(1312412352352435324, u64);
    test_be!(1244556, u32);
    test_be!(12400, u16);
 
    let bs = to_be_bytes(0x1ffu16);
    println!("{:?}", bs);
    assert!(bs[0] == 0x1);
    assert!(bs[1] == 0xff);   
}
