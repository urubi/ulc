pub fn type_to_bytes<T>(t: T) -> Vec<u8>{
    use std::mem;
    let t_ptr = unsafe { mem::transmute::<&T, *const u8>(&t) };
    let t_len: usize = mem::size_of::<T>();
    let mut out: Vec<u8> = vec![];
    
    for i in 0..t_len {
        out.push(unsafe {*t_ptr.offset(i as isize)});
    }
    out
}

pub fn hex_dump(buf: &[u8]) -> String {
    use std::char;
    
    let mut count = 0;
    let mut chars: String = String::new();;
    let mut out: String = String::new();
    
    
    out.push_str(&format!("Length: {}\n", buf.len()));
    
    for line in buf.chunks(16) {
        chars.clear();
        out.push_str(&format!("{:08x}:  ", count));
        for octet in line.chunks(8) {
            for byte in octet { 
                out.push_str(&format!("{:02x} ", byte)); 
                if *byte >= 0x20 && *byte < 0x7f {
                    chars.push(char::from_u32(*byte as u32).unwrap());
                }
                else {
                    chars.push('.');
                }
                count+=1;
            }
            out.push(' ');
        }
        out.push_str(&format!("|{}|", chars));
        out.push('\n');
    }    
    out
}
pub fn hd(buf: &[u8]) {
    println!("\n{}", hex_dump(buf));
}

pub fn phd(buf: &[u8]) -> ! {
    panic!("\n=== PANIC HEX DUMP ===\n{}\n=== END OF HEX DUMP ===\n", hex_dump(buf));
}



