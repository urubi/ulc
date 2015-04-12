use std::rc::Rc;
use ::tree::{Node, Leaf, Branch};
use ::tree::NodeExt;






// Not
// matcher -> Result<Node, ParserError>


// To do implement Error trait
#[derive(Debug, PartialEq)]
pub struct Error {
    pub tag         : Rc<String>,
    pub desc        : String,
    pub offset      : usize,
}

pub mod text; 

pub trait Parser<T> { // replace parse with Matcher
    fn parse<'a>(&self, source: &'a[T]) -> Result<Node<'a, T>, Error>;
    fn parse_tag(&self) -> Rc<String>;
}





#[cfg(test)]
pub mod debug_parser {
    use std::fmt::Debug;
    use std::rc::Rc;
    use super::*;
    use ::tree::Node;
    use ::tree::Leaf;
    
    #[derive(Debug)]
    pub struct Sqeek {
        tag: Rc<String>,
    }
    impl Sqeek {
        pub fn new<T:Debug + 'static>(tag: &str) -> Rc<Box<Parser<T>>> {
            Rc::new(Box::new(Sqeek {
                tag: Rc::new(tag.to_string())
            }))
        }
    }
    impl<T:Debug> Parser<T> for Sqeek {    
        fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
        fn parse<'a>(&self, source: &'a[T]) -> Result<Node<'a, T>, Error> {
            tty_print!("\n~~~~~~~~~~~~ '{}' sqeeked: at '{:?}...'\n", *self.tag, &source[..3]);
            Ok(Leaf::new(<Self as Parser<T>>::parse_tag(self), &source[..0]))
        }
    }
}




pub struct Match<T> {
    slice: Vec<T>,
    tag: Rc<String>
}
impl<T: PartialEq + 'static> Match<T> {
    pub fn new(tag: &str, slice: Vec<T>) -> Rc<Box<Parser<T>>> {
        Rc::new(Box::new(Match {
            slice: slice,
            tag: Rc::new(tag.to_string())
        }))
    }
}
impl<T: PartialEq> Parser<T> for Match<T> {    
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Result<Node<'a, T>, Error> {
        let mlen = self.slice.len();
        if mlen == 0 || source.len() < mlen || self.slice != &source[..mlen] {
            Err(Error {
                tag: self.parse_tag(),
                desc: "Mismatch".to_string(),
                offset: 0
            })
        }
        else {
            Ok(Leaf::new(self.parse_tag(), &source[..mlen]))
        }
    }
}
#[test]
fn match_() {
    let z = Match::new("name token", uv!("Name"));
    let s: Vec<char> = "Name".chars().collect();
    assert!(z.parse(&s).is_ok());
}


/*
    Or: [Parser List]
    
*/
pub struct Or<T> {
    parsers: Vec<Rc<Box<Parser<T>>>>,
    tag: Rc<String>
}
impl<T: 'static> Or<T> {
    pub fn new(tag: &str, parsers: Vec<Rc<Box<Parser<T>>>>) -> Rc<Box<Parser<T>>> {
        Rc::new(Box::new(Or {
            parsers: parsers, //parser.iter().map(|p| p.clone()).collect(),
            tag: Rc::new(tag.to_string())
        }))
    }
}
impl<T> Parser<T> for Or<T> {        
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Result<Node<'a, T>, Error> {
        for m in self.parsers.iter() {
            match m.parse(source) {
                Ok(m) => return Ok(m), _ => ()
            };
        }
        return Err(Error {
            tag: self.parse_tag(),
            desc: "No matches in set".to_string(),
            offset: 0
        });
    }
}

#[test]
fn or() {
    let n = Match::new("name token", uv!("Name"));
    let a = Match::new("age token", uv!("Age"));
    let s: Vec<char> = "Age hehe".chars().collect();
    
    let p = Or::new("age or name", vec![n, a]);
    assert!(p.parse(&s).unwrap().slice() == ul!("Age"));
}




pub struct Perhaps<T> {
    parser: Rc<Box<Parser<T>>>,
    tag: Rc<String>
}
impl<T: 'static> Perhaps<T> {
    pub fn new(tag: &str, parser: Rc<Box<Parser<T>>>) -> Rc<Box<Parser<T>>> {
        Rc::new(Box::new(Perhaps {
            parser: parser,
            tag: Rc::new(tag.to_string())
        }))
    }
}
impl<T> Parser<T> for Perhaps<T> {    
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Result<Node<'a, T>, Error> {
        return match self.parser.parse(&source) {
            Ok(n) => Ok(n),
            Err(_) => Ok(Leaf::new(self.parse_tag(), &source[..0]))
        };
    }
}




pub struct And<T> {
    parsers: Vec<Rc<Box<Parser<T>>>>,
    tag: Rc<String>
}
impl<T: 'static> And<T> {
    pub fn new(tag: &str, parsers: Vec<Rc<Box<Parser<T>>>>) -> Rc<Box<Parser<T>>> {
        Rc::new(Box::new(And {
            parsers: parsers,
            tag: Rc::new(tag.to_string())
        }))
    }
}
impl<T> Parser<T> for And<T> {    
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Result<Node<'a, T>, Error> {
        let mut branch = Branch::new(self.parse_tag(), source);
        let mut cur = 0;
                
        for m in self.parsers.iter() {
            let n = match m.parse(&source[cur..]) {
                Ok(n) => n, 
                Err(e) => {
                    return Err(Error{
                        tag: e.tag,
                        desc: e.desc,
                        offset: e.offset + cur
                    })
                }
            };
            
            cur += n.len();
            branch.attach(n);
        }
        
        // This will likely cause problems, see if can remove 
        if branch.len() == 0 {
            return Err(Error{
                tag: self.parse_tag(),
                desc: "(This error might be a bug in the library) Zero length match".to_string(),
                offset: 0
            })
        }
        else {Ok(branch)}
    }
}






pub struct Repeat<T> {
    parser: Rc<Box<Parser<T>>>,
    tag: Rc<String>,
    minimum: usize,
    maximum: usize
}

impl<T: 'static> Repeat<T> {
    pub fn new(tag: &str, parser: Rc<Box<Parser<T>>>, min: usize, max: Option<usize>) -> Rc<Box<Parser<T>>> {
        use std::usize;
        Rc::new(Box::new(Repeat {
            parser: parser,
            tag: Rc::new(tag.to_string()),
            minimum: min,
            maximum: match max { Some(r) => r, None => usize::MAX}
        }))
    }
}
impl<T> Parser<T> for Repeat<T> {    
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Result<Node<'a, T>, Error> {
        
        let mut branch = Branch::new(self.parse_tag(), source);
        let mut last_error: Option<Error> = None;
        let mut cur = 0;
        let mut reps = 0;
        
        
        while reps < self.maximum && cur < source.len() {
            let n = match self.parser.parse(&source[cur..]) {
                Ok(n) => n, Err(e) => {
                    last_error = Some(e);
                    break;
                }
            };
            
            // Required to avoid infinant loops
            if n.len() == 0 {break};
            
            cur += n.len();
            branch.attach(n);
            reps += 1;
            
        }
        
        if reps < self.minimum {
            return Err(Error{
                tag: self.parse_tag(),
                desc: format!("Expected at least {} matches, found {}\n{}", self.minimum, reps, match last_error {
                    Some(e) => format!(" -> and the last error was {:?}", e),
                    None => String::new()
                }),
                offset: 0
            })
        }
        else if reps >= self.maximum {
            return Err(Error{
                tag: self.parse_tag(),
                desc: format!("Expected at most {} matches, found {}.", self.maximum-1, reps),
                offset: 0
            })
        }
        else {Ok(branch)}
    }
}





#[cfg(tessst)]
mod test {/*
Testing Analysis for this module
================================
    
Testing plan for this module. The term string in this document reffers to any
[T], and parser to any struct that implements the Parser trait.


Each parser affords the following relavant interactions:
    1. Initialization
    2. Parsing


Initialization inputs are:
    1. String
    2. Parser
    3. List of parsers

Initialization outputs are not relavant.


Parsing inputs are:
    1. String

Parsing outputs are:
    1. Node
    2. Error

    

   
Relavant Value Specifications
-----------------------------
A. Strings/Lists:
    1. Empty
    2. Non-empty


B. Node:
    1. String -> A

C. Error.



Testing Plans
=============
    
Match - Initialization
----------------------
Variables:
    initialization string `ii` = empty/non-empty   
    parsing input string `pi` = empty/non-empty 
    parsing output string `po` = Node Empty / Node Non-Empty / Error
    
Tests:
    ii      pi      po
1   empty   empty   Node-empty
2   non     empty   Node-empty
3   empty
    
Test cases:*/
#[test]
fn match_empty_string(){
    let m = Match::new("empty", vec![]);
    let m = Match::new("non-empty", vec![]);
    
}
/*
    





*/}
