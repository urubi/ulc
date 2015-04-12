
use std::rc::Rc;
use ulc41::tree;
use ulc41::parser::{Parser, And, Or, Perhaps, Repeat, Match};
use ulc41::parser::Error;
use ::character::{Character, CharSpec};

/// https://tools.ietf.org/html/rfc5234

fn prepare_input(input: &mut String) {
    // This function prepares a gramar specification for processing by doing
    // the following:
    // 
    //      1. Converting every character to lowercase. (rfc:2.1)
    //
    // TODO
}

fn anbf_parser() -> Rc<Box<Parser<char>>> {
    // Core Rules (rfc:B.1)
    let alpha   = Character::new("ALPHA",   vec![CharSpec::IRange('A', 'Z'), CharSpec::IRange('a', 'z')]);
    let bit     = Character::new("BIT",     vec![CharSpec::Singleton('0'), CharSpec::Singleton('1')]);
    let char_   = Character::new("CHAR",    vec![CharSpec::IRange('', '')]);
    let cr      = Character::new("CR",      vec![CharSpec::Singleton('\r')]);
    let lf      = Character::new("LF",      vec![CharSpec::Singleton('\n')]);
    let crlf    = And::new("CRLF",          vec![cr.clone(), lf.clone()]);
    let ctl     = Or::new("CTL",            vec![
                                                Character::new("", vec![CharSpec::IRange('\0', '')]),
                                                Character::new("", vec![CharSpec::Singleton('')])
                                            ]);
    let digit   = Character::new("DIGIT",   vec![CharSpec::IRange('0', '9')]);
    let dquote  = Character::new("DQUOTE",  vec![CharSpec::Singleton('"')]);
    let hexdig  = Or::new("HEXDIG",         vec![
                                                digit.clone(),
                                                Character::new("", vec![CharSpec::IRange('A', 'F')]), // Dumb anbf is case insensitive 
                                                Character::new("", vec![CharSpec::IRange('a', 'f')])
                                            ]);
    let htab    = Character::new("HTAB",    vec![CharSpec::Singleton('\t')]);
    let sp      = Character::new("SP",      vec![CharSpec::Singleton(' ')]);
    let wsp     = Or::new("WSP",            vec![sp.clone(), htab.clone()]);
    let lwsp    = Repeat::new("LWSP",       Or::new("", vec![
                                                wsp.clone(),
                                                And::new("", vec![
                                                    crlf.clone(),
                                                    wsp.clone()
                                                ])
                                            ]), 0, None);
    let vchar   = Character::new("VCHAR",    vec![CharSpec::IRange('!', '~')]);
    
    // ABNF Definition Rules (rfc:4); going down to top
    //let prose_val       = 
}
