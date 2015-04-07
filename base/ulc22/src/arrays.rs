#[macro_export]
macro_rules! slice_at_most {
    ($array: expr, $to:expr) => ({
        let mut to: usize = $to;
        if to > $array.len() {
            to = $array.len();
        }
        &$array[..to]
    })
}

#[macro_export]
macro_rules! mut_slice_at_most {
    ($array: expr, $to:expr) => ({
        let mut to: usize = $to;
        if to > $array.len() {
            to = $array.len();
        }
        &mut $array[..to]
    })
}

#[test]
fn test_slice_at_most() {
    let a: [u8; 4] = [0, 1, 2, 3];
    assert!(slice_at_most!(a, 0).len() == 0);
    assert!(slice_at_most!(a, 3).len() == 3);
    assert!(slice_at_most!(a, 4).len() == 4);
    assert!(slice_at_most!(a, 5).len() == 4);
    
    let mut a: [u8; 4] = [0, 1, 2, 3];
    a[0] = 1;
    assert!(mut_slice_at_most!(a, 0).len() == 0);
    assert!(mut_slice_at_most!(a, 3).len() == 3);
    assert!(mut_slice_at_most!(a, 4).len() == 4);
    assert!(mut_slice_at_most!(a, 5).len() == 4);
}

/// ment for flow control (not option, result typed)
/// use return, break, panic!, Fn -> !, ...
#[macro_export]
macro_rules! slice_or_else {
    ($array: expr, $from:expr, $to:expr, $err: expr) => ({
        if  ($to as usize) <= $array.len() && 
            ($from as usize) <= ($to as usize) {
            &$array[($from as usize)..($to as usize)]
        }
        else {$err}
    })
}

#[macro_export]
macro_rules! mut_slice_or_else {
    ($array: expr, $from:expr, $to:expr, $err: expr) => ({
        if  ($to as usize) <= $array.len() && 
            ($from as usize) <= ($to as usize) {
            &mut $array[($from as usize)..($to as usize)]
        }
        else {$err}
    })
}

#[test]
fn test_slice_or_else() {
    let a: [u8; 4] = [0, 1, 2, 3];
    let b: [u8; 2] = [0, 1];
    let v: Vec<u8> = vec![0, 0, 0, 0, 0];
    
    assert!(slice_or_else!(a, 0, 0, panic!("Not yet")).len() == 0);
    assert!(slice_or_else!(a, 0, 3, panic!("not yet")).len() == 3);
    assert!(slice_or_else!(a, 0, 4, panic!("not yet")).len() == 4);
    assert!(slice_or_else!(a, 0, 5, &b[0..]).len() == 2);
    assert!(slice_or_else!(a, 1, 0, &b[0..]).len() == 2);
    assert!(slice_or_else!(v, 0, 3, panic!("not yet")).len() == 3);
    assert!(slice_or_else!(&v, 0, 3, panic!("not yet")).len() == 3);
    assert!(slice_or_else!(&v[0..], 0, 3, panic!("not yet")).len() == 3);
    assert!(slice_or_else!(&a, 0, 2, panic!("not yet")).len() == 2);
    assert!(slice_or_else!(&a[1..], 0, 2, panic!("not yet")).len() == 2);
    assert!(slice_or_else!((&a[1..]), 0, 2, panic!("not yet")).len() == 2);
    
    let mut a: [u8; 4] = [0, 1, 2, 3];
    {
        let b: &mut [u8] = mut_slice_or_else!(&mut a, 1, 3, panic!("should happen"));
        b[0] = 0;
    }
    assert!(a[1] == 0);
}

#[macro_export]
macro_rules! stacked_slice {
    ($array: expr, $from:expr, $to:expr) => ({
        slice_or_else!($array, $from, $to, {
            stacked_return!("Attempted to slice[{}..{}] an array of length {}", $from, $to, $array.len());
        })
    })
}
#[cfg(test)]
fn slc(array: &[u8], from: usize, to: usize) -> Result<&[u8], Vec<String>> {
    Ok(stacked_slice!(array, from, to))    
}
#[test]
fn test_stacked_slice() {
    let a: [u8; 4] = [0, 1, 2, 3];
    assert!(slc(&a, 0, 0).unwrap().len() == 0);
    assert!(slc(&a, 0, 3).unwrap().len() == 3);
    assert!(slc(&a, 0, 4).unwrap().len() == 4);
    assert!(slc(&a, 0, 5).is_err());
}
