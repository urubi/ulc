extern crate ulc41;
extern crate ulc92;

use ulc41::parser::*;
use ulc92::character::{Character, CharSpec};

use std::rc::Rc;


pub fn alpha() -> Rc<Box<Parser<char>>> {
    let spec = vec![CharSpec::IRange('a' , 'z'), CharSpec::IRange('A' , 'Z')];
    Character::new("alpha", spec)
}
pub fn num() -> Rc<Box<Parser<char>>> {
    let spec = vec![CharSpec::IRange('0' , '9')];
    Character::new("num", spec)
}

pub fn vws() -> Rc<Box<Parser<char>>> {
    let spec = vec![CharSpec::Singleton('\t'), 
                    CharSpec::Singleton(' ')];
    Character::new("vws", spec)
}

pub fn ws() -> Rc<Box<Parser<char>>> {
    let spec = vec![CharSpec::Singleton('\t'), 
                    CharSpec::Singleton('\n'),
                    CharSpec::Singleton('\r'),
                    CharSpec::Singleton(' ')];
    Character::new("ws", spec)
}


fn main() {
    //read all input
    let _ = Repeat::new("statements*", And::new("statement", vec![
        Repeat::new("ws*", ws(), 0, None),
        Repeat::new("(def)", alpha(), 1, None),
        Repeat::new("vws*", vws(), 0, None),
        Match::new("=", vec!['=']),
        Repeat::new("vws*", vws(), 0, None),
        Repeat::new("posibilities+", And::new("posibility", vec![
            Repeat::new("(pos)", alpha(), 1, None),
            Repeat::new("vws+", vws(), 0, None)
        ]), 1, None), 
        Repeat::new("ws*", ws(), 0, None)
    ]), 1, None);
    
    
    
}
