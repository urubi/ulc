// this module is designed for a specific usecase: control a local tor instance
// to access the network and to provide services. Nothing biond this is 
// supported.


//  tor -f $HOME/.torrc --socksport 8880 --datadirectory /tmp/rmme \
//  --controlport 8881 \
//  --HashedControlPassword "$(tor --hash-password hehe | tail -n 1)"
//  setconf HiddenServiceDir=/tmp/hidden-n-bitten HiddenServicePort="80 127.0.0.1:8888" HiddenServiceDir=/tmp/hidden-n-bitten-2 HiddenServicePort="80 127.0.0.1:8889"

// getconf HiddenServiceOptions

// 
#![feature(convert, io)]
#[macro_use]
extern crate ulc22;
extern crate ulc31;
extern crate ulc32;

use std::net::*;
use std::io::Result;
use ulc32::Socks4;
/*
// envronment variables as understood by this moduleTOR_SKIP_LAUNCH
//const TOR_CONTROL_HOST              :&'static str ="TOR_CONTROL_HOST";
const TOR_CONTROL_PORT              :&'static str ="TOR_CONTROL_PORT";
const TOR_CONTROL_PASSWD            :&'static str ="TOR_CONTROL_PASSWD";
const TOR_CONTROL_COOKIE_AUTH_FILE  :&'static str ="TOR_CONTROL_COOKIE_AUTH_FILE";
//const TOR_SOCKS_HOST                :&'static str ="TOR_SOCKS_HOST";
const TOR_SOCKS_PORT                :&'static str ="TOR_SOCKS_PORT";
*/


pub mod control;



pub enum Auth {
    Cookie(String),
    Password(String),
    None
}


pub struct Service {
    name: String,
    hostname: String,
    private_key: String
}

impl Service {
    //pub fn from_files(name: &str, hostname: &str, private_key: &str)
    //
    //pub fn from_bytes()
    //pub fn to_bytes()
    //pub fn name()
    //pub fn hostname()
    //pub fn private_key()
}

// if environment vars are set, use environment, else use own instense ? XXX
// set as tor controler ?
// no local? or use local if necesary info is in enviroemnt
// make local vs private automatic (use local if necisay information is available AND initial testing works)
// respect environment variable that forces either local or private mode
// if facist firewall, user must depend on env tor

pub struct Tor {
    socks: Socks4,
    control: SocketAddrV4,
    control_auth: Auth
    //control_socket: TcpStream,
}

impl Tor {
    pub fn new(socks_port: u16, control_port: u16, control_auth: Auth) -> Result<Tor> {
        Ok(Tor {
            socks: ir!(Socks4::new(format!("127.0.0.1:{}", socks_port).as_ref()), "Tor Error", "Could not setup proxing"),
            control: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), control_port),
            control_auth: control_auth
        })
    }
    //pub fn start_instance() local tor instance for 
    pub fn connect(&self, address: &str) -> Result<TcpStream> {
        self.socks.connect(address)
    }
    /*
    pub fn bind(&self, service: &Service) -> Result<TcpListener> {
        // ....
    }
    pub fn gen_service(&self) -> Result<Service> {
        
    }*/
}
#[test]
fn it_works() {
    let t = Tor::new(9050, 9051, Auth::None).unwrap();
    t.connect("example.com:80").unwrap_or_else(|e| panic!("{}", e));
}



