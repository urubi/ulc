//#![feature(ip)]
#![feature(std_misc)] 
#![feature(thread_sleep)] 

fn main(){
    use std::time::duration::Duration;
    use std::net::{TcpListener, SocketAddr, Ipv4Addr};
    use std::io::Read;
    use std::thread;
    
    thread::spawn(move || {
        let s = TcpListener::bind("127.0.0.1:0").unwrap();
        println!("listening at {}", s.local_addr().unwrap());
        loop {
            let c = s.accept();
            if c.is_err() {
                println!("Err accepting, probably disconnected.");
                break;
            }
            let (mut stream, remote) = c.unwrap();
            if ! match remote { 
                SocketAddr::V4(a) => a.ip() != &Ipv4Addr::new(172, 0, 0, 1), 
                _ => panic!("Should never happen") } {
                    println!("Skipping, cause not from loopback");
                    continue;            
            }
            
            let mut input: String = String::new();
            println!("got the following from {}:", remote);
            assert!(stream.read_to_string(&mut input).is_ok());
            println!("{}", input);
            println!("bytes sent: {}", input.len());
            println!("------");
        }
    });
    
    thread::sleep(Duration::seconds(30));
}
