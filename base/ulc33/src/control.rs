use std::net::*;
use std::io::Result;

pub const TC:[(u16, &'static str); 17] = [
    (250, "OK"),
    (251, "Operation was unnecessary"),
    (451, "Resource exhausted"),
    (500, "Syntax error: protocol"),
    (510, "Unrecognized command"),
    (511, "Unimplemented command"),
    (512, "Syntax error in command argument"),
    (513, "Unrecognized command argument"),
    (514, "Authentication required"),
    (515, "Bad authentication"),
    (550, "Unspecified Tor error"),
    (551, "Internal error"),
    (552, "Unrecognized entity"),
    (553, "Invalid configuration value"),
    (554, "Invalid descriptor"),
    (555, "Unmanaged entity"),
    (650, "Asynchronous event notification")
];

pub fn errstr(code: u16) -> &'static str {
    for &(tcode, ref tdesc) in TC.iter() {
        if code == tcode {return tdesc;}
    }
    concat!("undefined error occured: Tor might be malfunctioning ",
            "or this library is outdated. Please file a bug report.")
}


pub enum Credential {
    Password(String),
    None
}

pub struct TorController {
    address: SocketAddrV4,
    socket: Option<TcpStream>
    credential: Option<String>
    // LOOK AT STEM documentation
    // TODO add authentication here
    // add admin functions
    // add hidden services function
    // ... This is the main Tor controlller module, move TORHS module own ulc: ulc34
    // this is ulc33
}
// Only passwords are support atm

impl TorController {
    pub fn new(address: SocketAddrV4, credential: Option<String>) -> TorController {
       TorController {
            address: address,
            credential: credential
            socket: None,
        } 
    }
    pub fn connect(&mut self) -> Result<()> {
        self.socket = Some(ir!(TcpStream::connect(self.address), "Could not connect to the tor instance."));
        
        // Authenticating
        
        let r = if let &Some(ref password) = self.credential {
            /quotee sting/
            ir!(self.raw_command(format!("Authenticate \"{}\"", password), "Error while sending authentication request")
        }
        else {
            ""
        }
        
        
        
        if r.len != 1 {
            ir_ret!("Expected one response, got {}.");
        }
        if r[0].0 != 250 {ir_ret!("Authentication failed");}
        
        Ok(())
    }
    pub fn disconnect(&mut self) {
        self.socket = None;
    }
    pub fn raw_command(&mut self, request: &[u8]) -> Result<Vec<(u16, Vec<u8>)>> {        
        use ulc31::io::Read;
        use std::io::Write;
        
        if let &mut Some(ref mut ts) = &mut self.socket {
            let mut buf: [u8; 200] = [0; 200];
            let mut out: Vec<(u16, Vec<u8>)> = vec![];
            
            
            // sending request
            ir!(ts.write(request), "Could not issue command");
            ir!(ts.write("\r\n".as_bytes()), "Could not issue command");
            let _ = ts.flush();
            
            loop {
                let mut reply_type: u8;
                let mut reply_status: u16;
                let mut reply_buf: Vec<u8> = vec![];
                
                // response code
                ir!(ts.read_to_fill(&mut buf[0..3]), "Missing data from reply: Missing status code");
                
                
                if let Ok(status) = (String::from_utf8_lossy(&buf[..3])).parse::<u16>() {
                    reply_status = status;
                }
                else {
                    ir_ret!("Invalid data recieved", "'{}', is not a valid reply status", String::from_utf8_lossy(&buf[..3]));
                }
                
                // checking reply type if  MidReplyLine, DataReplyLine, or EndReplyLine
                ir!(ts.read_to_fill(&mut buf[..1]), "Missing data from reply", 
                    "Missing MID/DATA reply line indicator (+/-)");
                
                if buf[0] == 0x2D || buf[0] ==  0x20 { // MidReplyLine/EndReplyLine
                    ir!(ts.read_until(&mut reply_buf, "\r\n".as_bytes(), 1<<16), "Missing data from reply",
                        "MidReplyLine does not terminate with expected sequence");
                    reply_type = buf[0];
                    let rl = reply_buf.len();
                    reply_buf.truncate(rl - 2);
                }
                else if buf[0] == 0x2B { // DataReplyLine
                    ir!(ts.read_until(&mut reply_buf, "\r\n.\r\n".as_bytes(), 1<<16), "Missing data from reply", 
                        "DataReplyLine does not terminate with expected sequence");
                    reply_type = buf[0];
                    let rl = reply_buf.len();
                    reply_buf.truncate(rl - 5);
                }
                else {
                    ir_ret!("Malformed reply", "reply line type is not one of thee valid characters: got '{}'", buf[0]);
                }
                
                out.push((reply_status, reply_buf));
                
                if out.len() > 2000 {
                    ir_ret!("Response list is too long (2000 entries).");
                }
                
                if reply_type == 0x20 {
                    return Ok(out);
                }
                
            }
        }
        else {
            ir_ret!("Not connected yet: call connect() first");
        }
    }
}

#[test]
fn test_tor_controller() {
    let mut tc = TorController::new(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 9051));
    tc.connect().unwrap_or_else(|e| {panic!("{}", e)});
    let r = tc.command("Authenticate \"hehe\"".as_bytes()).unwrap_or_else(|e| {panic!("{}", e)});
    assert!(r.len() == 1);
    assert!(r[0].0 == 250);
    assert!(r[0].1.len() == 2);
    assert!(r[0].1 == "OK".as_bytes());
    
    
    
}
