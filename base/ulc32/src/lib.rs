// Standards
// Socks4:  http://www.openssh.com/txt/socks4.protocol
// Socks4a: http://www.openssh.com/txt/socks4a.protocol

// used string instead of SocketAddr because SocketAddr tries to resolve 
// addresses imediatly.
#![feature(convert)]
#![feature(collections)]
#![feature(io)]
#[macro_use]
extern crate ulc22;
extern crate ulc31;
extern crate ulc91;

use std::net::*;
use std::io::Result;

const CONNECT:u8 = 1;
const SE:&'static str = "Socks Error";


pub struct Socks4 {
    address: String  
}

impl Socks4 {
    pub fn new(address: &str) -> Result<Socks4> {
        Ok(Socks4 {
            address: address.to_string()
        })
    }
    pub fn connect(&self, address: &str) -> Result<TcpStream> {
        // is str a vailid ip:port string (TODO see if you can use std)
        // or at least do it properly
        use std::io::Write;
        use std::io::Read;
        use ulc31::address::parse_as_ipv4_address;
        use ulc31::address::parse_as_domain_address;
        use ulc31::address::ipv4;
        
        let include_domain: bool;           // include domain in request
        let mut domain_name: &str = "";     // domain to include
        
        // Check if connecting to raw address or requires name resolution
        let da = match parse_as_ipv4_address(address) {
            Some(a) => {
                include_domain = false;
                a
            },
            None => {
                include_domain = true;
                // pick out the port number from the address
                // return io error if no port was found or in ambiguis
                if let Some((domain, port)) = parse_as_domain_address(address) {
                    if port == None {
                        ir_ret!(SE,"No valid port specified in address '{}'", address);
                    }
                    domain_name = domain;
                    ipv4(0, 0, 0, 1, port.unwrap())
                    
                }
                else {
                    ir_ret!(SE,"Invalid address '{}'", address);
                }
            }
        };
        let da_address = da.ip().octets();
        let da_port = ulc91::unsigned::to_be_bytes(da.port());

        // build request
        let mut request: Vec<u8> = vec![
            4, 
            CONNECT, 
            da_port[0], da_port[1], 
            da_address[0], da_address[1], da_address[2], da_address[3], 
            0];
        // tack the domain if required
        if include_domain {
            request.push_all(domain_name.as_bytes());
            request.push(0);
        }
        
        // reply buffer
        let mut reply: [u8; 8] = [0;8];
        
        let mut s = ir!(TcpStream::connect(<String as AsRef<str>>::as_ref(&self.address)), 
            SE, "Could not connect to proxy at '{}'", self.address);
            
        ir!(s.write_all(&request), 
            SE, "could not complete initial request to proxy");
        
        let n = ir!(s.read(&mut reply), 
            SE, "Could not recieve reply from server");
        
        if n != 8 {
            ir_ret!(SE,"Partial response, Fix this later: {}", n);
        }
        if reply[1] != 90 {
            ir_ret!(SE,"Not ok {}", reply[1]);
        }
        
        Ok(s)
    }
}

#[cfg(test)]
fn test_socks4_for_address(a: &str, host_str: &str, succeeds: bool) {
    use std::io::Write;
    use std::io::Read;
    
    let s = Socks4::new("127.0.0.1:9050").unwrap();
    
    let mut c = match s.connect(a) {
        Ok(c) => {
            if !succeeds {panic!("This should have not succeeded.");}
            else {c}
        },
        Err(e) => {
            if succeeds {panic!("{}", e.to_string())}
            else {return;}
        }
    };
    
    c.write_all(format!("GET /index.html HTTP/1.1\r\nHost: {}\r\n\r\n", host_str).as_bytes()).unwrap();
    c.flush().unwrap();
    let mut reply: [u8; 2] = [0;2];
    assert!(c.read(&mut reply).unwrap() == 2);
}

#[test]
fn test_socks4() {
    test_socks4_for_address("93.184.216.34:80", "www.example.com", true);
    test_socks4_for_address("3g2upl4pq6kufc4m.onion:80", "3g2upl4pq6kufc4m.onion", true);
    test_socks4_for_address("3g2dsdasf.onion:80", "sdasfd", false);
    test_socks4_for_address("example.invalid:80", "example.invalid", false);
}
