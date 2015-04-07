use std::net::{SocketAddrV4, Ipv4Addr};

fn validate_host_domain(s: &str) -> bool {
    //  Ignoring the following from relavent standards:
    //  1.  Max lengths of any kind
    //  2.  Disallowing domains that terminate with a period: Seems that 
    //      software nowadays allows it. Maybe superseeded by newer rfc?
    //  3.  Allowing single letter domains. (See Note)
    //
    //  Note:   rfc952 seems to have a contradiction between a statement
    //          regarding disallowing single letter domain names, and the
    //          grammar specification which allows it.
    //          
    //          This function will honer the statement and ignor the 
    //          grammer, as it seems widely accepted that single letter
    //          domains are not valid.
    //
    // No built-in regex
    const NONE:isize    = 0;
    const ALPHA:isize   = 1;
    const NUM:isize     = 2;
    const DOT:isize     = 4;
    const DASH:isize    = 8;
    
    if s.len() < 2 {return false;}
    
    let mut last = NONE;                // last character processed
    let mut segment_length:usize = 0;   // number of characters processed since
                                        // last "."
    
    for i in s.bytes() {
        segment_length += 1;
        let current = 
            if (i >= 0x41 && i <= 0x5A) || (i >= 0x61 && i <= 0x7A) {ALPHA}
            else if i >= 0x30 && i <= 0x39 {NUM}
            else if i == 0x2D {DASH}
            else if i == 0x2E {
                if segment_length <= 2 {
                    return false;
                }
                else {
                    segment_length = 0;
                }
                DOT
            }
            else {return false;};
            
        // First character must be alhpanumric [1]
        if last == NONE && !(current & (ALPHA|NUM) > 0) {return false;}
        
        // dots must be surrounded by alphnumric characters. A dot 
        // at the end of a domain is only required to have one 
        // alphanumric character before it.
        if current == DOT && !(last & (ALPHA|NUM) > 0) {return false;}
        if last == DOT && !(current & (ALPHA|NUM) > 0) {return false;}
        
        last = current;
    }
    
    // last character checks
    {
        // trailing dashs are not allowed.
        if last == DASH {return false;}   
        // last segment must be longer than 2 and not 0 (i.e. a period)
        if segment_length == 1 {return false;}
    }
    
    true
    // [1]: http://www.ietf.org/rfc/rfc1123.txt (section 2.1)
    // [2]: http://www.ietf.org/rfc/rfc952.txt (Grammar section: B: <hname>)
}

#[test]
fn test_validate_host_domain() {
    assert!(validate_host_domain("") == false);
    assert!(validate_host_domain("a") == false);
    assert!(validate_host_domain("ab") == true);
    assert!(validate_host_domain("ab-") == false);
    assert!(validate_host_domain("ab.") == true);
    assert!(validate_host_domain("ab.a.qqq") == false);
    assert!(validate_host_domain("ab.aa.") == true);
    assert!(validate_host_domain("ab.a----a.") == true);
    assert!(validate_host_domain("ab.---.ddddd") == false);
    assert!(validate_host_domain(".ab") == false);
    assert!(validate_host_domain("a b c") == false);
    assert!(validate_host_domain("a-b.") == true);
    assert!(validate_host_domain("a-b.b") == false);
    assert!(validate_host_domain("a-b.--.dsffds") == false);
    assert!(validate_host_domain("a-b.-.dsf") == false);
    assert!(validate_host_domain("-ab") == false);
    assert!(validate_host_domain("example.com") == true);
    assert!(validate_host_domain("our-cheese.factory.is.4years.old") == true);
    assert!(validate_host_domain("1234-5678-9.phonenumber") == true); // Hunch that num-only domains are not valid
    assert!(validate_host_domain("12.12.12.12") == true); // Ambigius but valid.
    assert!(validate_host_domain("ab.c.") == false);
}

/// Ambigius wraning: Accrding to relavent rfcs, "12.12.12.12" is a valid 
/// host domain as well as a valid IPv4 address. ALWAYS TRY TO VALIDATE 
/// an address as a literal IP before validating it as host domain.
pub fn parse_as_domain_address<'a>(s: &'a str) -> Option<(&'a str, Option<u16>)> {
    let ip_port: Vec<&str> = s.split(":").collect();
    if ip_port.len() == 2 {
        let port: u16 = ok!(ip_port[1].parse::<u16>(), return None);
        if validate_host_domain(ip_port[0]) {
            Some((ip_port[0], Some(port)))
        }
        else {
            None
        }
    }
    else if ip_port.len() == 1 {
        if validate_host_domain(s) {
            Some((s, None))
        }
        else {
            None
        }
    }
    else {
        None
    }
}
#[test]
fn test_parse_as_domain_address() {
    assert!(parse_as_domain_address("hello") == Some(("hello", None)));
    assert!(parse_as_domain_address("hello:234") == Some(("hello", Some(234))));
    assert!(parse_as_domain_address("hello:12233234") == None);
    assert!(parse_as_domain_address("hello.example.com:0") == Some(("hello.example.com", Some(0))));
    assert!(parse_as_domain_address("localhost:631") == Some(("localhost", Some(631))));
}



pub fn parse_as_ipv4_address(s: &str) -> Option<SocketAddrV4> {
    let ip_port: Vec<&str> = s.split(":").collect();
    if ip_port.len() != 2 {return None;}
    
    let port: u16 = ok!(ip_port[1].parse::<u16>(), return None);
    
    let ip: Vec<&str> = ip_port[0].split(".").collect();
    if ip.len() != 4 {return None;}
    let mut bytes: [u8; 4] = [0; 4];
    bytes[0] = ok!(ip[0].parse::<u8>(), return None);
    bytes[1] = ok!(ip[1].parse::<u8>(), return None);
    bytes[2] = ok!(ip[2].parse::<u8>(), return None);
    bytes[3] = ok!(ip[3].parse::<u8>(), return None);
    
    Some(ipv4(bytes[0], bytes[1], bytes[2], bytes[3], port))
}

pub fn ipv4(a: u8, b: u8, c: u8, d: u8, port: u16) -> SocketAddrV4 {
    SocketAddrV4::new(Ipv4Addr::new(a, b, c, d), port)
}

#[test]
fn test_parse_as_ipv4_address() {
    assert!(parse_as_ipv4_address("0.0.0.0:50")              == Some(ipv4(0,0,0,0,50)));
    assert!(parse_as_ipv4_address("255.0.255.0:30000")       == Some(ipv4(255,0,255,0,30000)));
    assert!(parse_as_ipv4_address("255.0.256.0:30000")       == None);
    assert!(parse_as_ipv4_address("This is an IP.. Honest!") == None);
}
