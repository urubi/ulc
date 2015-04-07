use std::net::TcpStream;
use std::io::Result;

const BUFFER_LIMIT:&'static str = "Buffer Limit Exceeded";
const PREMATURE_CLOSE:&'static str = "Remote closed the connection without sending enough data.";


// TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP
// read_until is not implemented for sockets, remove once rust devs implement it
pub trait Read {
    fn read_to_fill(&mut self, buf: &mut [u8]) -> Result<()>;
    fn read_until(&mut self, buf: &mut Vec<u8>, pat: &[u8], limit: usize) -> Result<()>;
}

// NOTE: update if TcpStream chages to support unblocking
// NOTE: update when TcpStream can check if stream is open
impl Read for TcpStream {
    fn read_to_fill(&mut self, buf: &mut [u8]) -> Result<()> {
        use std::io::Read;
        use std::io::{ErrorKind, Error};
        
        // stupid, but works. 
        let mut inb: [u8; 1] = [0];
        
        for i in buf.iter_mut() {
            loop {
                match self.read(&mut inb) {
                    Ok(c) => {
                        if c == 0 {
                            return Err(Error::new(ErrorKind::ConnectionAborted, 
                                        PREMATURE_CLOSE, None));
                        }
                        else {
                            *i = inb[0];
                            break;
                        }
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::Interrupted {
                            continue;
                        }
                        else {
                            return Err(e);
                        }
                    }
                };
            }
        }
        Ok(())
    }
    fn read_until(&mut self, buf: &mut Vec<u8>, pat: &[u8], limit: usize) -> Result<()> {
        use std::io::{ErrorKind, Error};
        use std::io::Read;
        
        let mut inb: [u8; 1] = [0];
        
        buf.clear();
        
        loop {
            if buf.len() > limit {
                return Err(Error::new(ErrorKind::Other, BUFFER_LIMIT, 
                       Some(format!("Maximum buffer size is set to {} bytes", 
                       limit))));
            }
            loop { match self.read(&mut inb) {
                Ok(c) => {
                    if c == 0 {
                        return Err(Error::new(ErrorKind::ConnectionAborted, 
                                    PREMATURE_CLOSE, None));
                    }
                    else {break;}
                }
                Err(e) => {
                    if e.kind() == ErrorKind::Interrupted {continue;}
                    else {return Err(e);}
                }
            }}
            buf.push(inb[0]);
            
            if buf.len() >= pat.len() && &buf[(buf.len()-pat.len())..] == pat {
                return Ok(())
            }
        }
    }
}


#[cfg(test)]
fn pew_pew(address: &'static str, list: Vec<&'static str>) {
    use std::net::*;
    use std::thread;
    use std::io::Write;
    use std::time::duration::Duration;
    
    thread::spawn(move|| {
        let s = TcpListener::bind(address).unwrap_or_else(|e| panic!(e));
        loop {
            let (mut stream, _) = s.accept().unwrap_or_else(|e| panic!(e));
            for i in list.iter() {
                stream.write(i.as_bytes()).unwrap_or_else(|e| panic!(e));
                stream.flush().unwrap_or_else(|e| panic!(e));
            }
        }
    });
    thread::sleep(Duration::milliseconds(100));
}

#[test]
fn test_read_to_fill() {
    use std::net::*;

    let addr = "127.0.0.1:8800";
    
    pew_pew(addr, vec![
        "hello!",
        "this is not the amount you're looking for",
    ]);
    
    let mut out: [u8; 200] = [0; 200];
    let mut c = TcpStream::connect(addr).unwrap();
    match c.read_to_fill(&mut out[..6]) {Err(e) => panic!("{}", e), _ => ()};
    assert!(&out[..6] == "hello!".as_bytes());
    match c.read_to_fill(&mut out) {
        Err(e) => assert!(e.description() == PREMATURE_CLOSE), Ok(_) => panic!("Should not get data")};
    
    let mut c = TcpStream::connect(addr).unwrap();
    match c.read_to_fill(&mut out[..10]) {Err(e) => panic!("{}", e), _ => ()};
    assert!(&out[..10] == "hello!this".as_bytes());
}

#[test]
fn test_read_until() {
    use std::net::*;
    use std::error::Error;
    let addr = "127.0.0.1:8801";
    
    pew_pew(addr, vec![
        "hello!\n",
        "this is not the amount you're looking for",
    ]);
    
    let mut reply: Vec<u8> = vec![];
    
    let mut c = TcpStream::connect(addr).unwrap();
    match c.read_until(&mut reply, "\n".as_bytes(), 10000) {Err(e) => panic!("{}", e), _ => ()};
    assert!(&reply[..6] == "hello!".as_bytes());
    
    match c.read_until(&mut reply, "h".as_bytes(), 10000) {Err(e) => panic!("{}", e), _ => ()};
    assert!(&reply[..2] == "th".as_bytes());
    
    match c.read_until(&mut reply, "s".as_bytes(), 10000) {Err(e) => panic!("{}", e), _ => ()};
    assert!(&reply[..2] == "is".as_bytes());
    
    match c.read_until(&mut reply, "\n".as_bytes(), 10000) {
        Err(e) => assert!(e.description() == PREMATURE_CLOSE), Ok(_) => panic!("Should not get data")};
    
    let mut c = TcpStream::connect(addr).unwrap();
    match c.read_until(&mut reply, "\n".as_bytes(), 1) {
        Err(e) => assert!(e.description() == BUFFER_LIMIT), Ok(_) => panic!("Should not get data")};
    
    
    
}

