extern crate ulc11;

use ulc11::ExecPath;
use ulc11::Embed;
use ulc11::auto::AutoEmbed;

fn main() {
    let mut p = AutoEmbed::new(ExecPath::This).unwrap();
    
    let mut d = p.load().unwrap();
    
    // if fresh run
    if d.len() == 0 {
        d.push(1u8);
    }
    
    if d[0] != 0xff {
        println!("Run count: {}", d[0]);
        d[0] += 1;
    }
    else {
        println!("Sudden amnesia!");
        d[0] = 1;
    }
        
    p.store(&d).unwrap();
}


