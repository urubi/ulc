#![feature(env)]
extern crate ulc11;

use ulc11::ExecPath;
use ulc11::Embed;
use ulc11::auto::AutoEmbed;

use std::env::args;

#[allow(dead_code)]
fn main() {
    if args().count() == 2 {
        AutoEmbed::new(ExecPath::This).unwrap().strip().unwrap();
    }
    else {
        let mut p = AutoEmbed::new(ExecPath::This).unwrap();
        let mut d = p.load().unwrap();
        if d.len() == 0 {
            d.push(0u8);
        }
        
        println!("{}", d[0]);
        
        if d[0] != 0xff {
            d[0] += 1;
        }
        else {
            d[0] = 0;
        }
            
        p.store(&d).unwrap();
    }
}


