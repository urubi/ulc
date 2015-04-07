use std::rc::Rc;
use ::tree;

pub mod text;

pub trait Parser<T> { // replace parse with Matcher
    fn parse<'a>(&self, source: &'a[T]) -> Option<tree::Node<'a, T>>;
    fn parse_tag(&self) -> Rc<String>;
}



pub struct Match<T> {
    slice: Vec<T>,
    tag: Rc<String>
}
impl<T: Clone> Match<T> {
    pub fn new(tag: &str, slice: &[T]) -> Match<T> {
        Match {
            slice: slice.to_vec(),
            tag: Rc::new(tag.to_string())
        }
    }
}
impl<T: PartialEq> Parser<T> for Match<T> {    
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Option<tree::Node<'a, T>> {
        let mlen = self.slice.len();
        if mlen == 0 || source.len() < mlen || self.slice != &source[..mlen] {
            None
        }
        else {
            Some(tree::Leaf::new(self.parse_tag(), &source[..mlen]))
        }
    }
}




pub struct Or<'b, T> {
    parsers: Vec<&'b Parser<T>>,
    tag: Rc<String>
}
impl<'b, T> Or<'b, T> {
    pub fn new(tag: &str, parser: &[&'b Parser<T>]) -> Or<'b, T> {
        Or {
            parsers: parser.to_vec(),
            tag: Rc::new(tag.to_string())
        }
    }
}
impl<'b, T> Parser<T> for Or<'b, T> {    
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Option<tree::Node<'a, T>> {
        for m in self.parsers.iter() {
            match m.parse(source) {
                Some(m) => return Some(m), _ => ()
            };
        }
        return None;
    }
}






pub struct Perhaps<'b, T> {
    parser: &'b Parser<T>,
    tag: Rc<String>
}
impl<'b, T> Perhaps<'b, T> {
    pub fn new(tag: &str, parser: &'b Parser<T>) -> Perhaps<'b, T> {
        Perhaps {
            parser: parser,
            tag: Rc::new(tag.to_string())
        }
    }
}
impl<'b, T> Parser<T> for Perhaps<'b, T> {    
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Option<tree::Node<'a, T>> {
        return match self.parser.parse(&source) {
            Some(n) => Some(n),
            None => Some(tree::Leaf::new(self.parse_tag(), &source[..0]))
        };
    }
}




pub struct And<'b, T> {
    parsers: Vec<&'b Parser<T>>,
    tag: Rc<String>
}
impl<'b, T> And<'b, T> {
    pub fn new(tag: &str, parser: &[&'b Parser<T>]) -> And<'b, T> {
        And {
            parsers: parser.to_vec(),
            tag: Rc::new(tag.to_string())
        }
    }
}
impl<'b, T> Parser<T> for And<'b, T> {    
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Option<tree::Node<'a, T>> {
        use tree::NodeExt;    
        let mut branch = tree::Branch::new(self.parse_tag(), source);
        let mut cur = 0;
                
        for m in self.parsers.iter() {
            let n = match m.parse(&source[cur..]) {
                Some(n) => n, None => return None
            };
            
            cur += n.len();
            branch.attach(n);
        }
        if branch.len() == 0 {return None}
        else {Some(branch)}
    }
}






pub struct Repeats<'b, T> {
    parser: &'b Parser<T>,
    tag: Rc<String>,
    minimum: usize,
    maximum: usize
}

impl<'b, T> Repeats<'b, T> {
    pub fn new(tag: &str, parser: &'b Parser<T>, min: usize, max: Option<usize>) -> Repeats<'b, T> {
        use std::usize;
        Repeats {
            parser: parser,
            tag: Rc::new(tag.to_string()),
            minimum: min,
            maximum: match max { Some(r) => r, None => usize::MAX}
        }
    }
}
impl<'b, T> Parser<T> for Repeats<'b, T> {    
    fn parse_tag(&self) -> Rc<String> {self.tag.clone()}
    fn parse<'a>(&self, source: &'a[T]) -> Option<tree::Node<'a, T>> {
        use tree::NodeExt;    
        
        let mut branch = tree::Branch::new(self.parse_tag(), source);
        
        let mut cur = 0;
        let mut reps = 0;
        
        
        while reps < self.maximum && cur < source.len() {
            let n = match self.parser.parse(&source[cur..]) {
                Some(n) => n, None => break
            };
            
            if n.len() == 0 {break};
            
            cur += n.len();
            branch.attach(n);
            reps += 1;
            
        }
        
        if reps < self.minimum || branch.len() == 0 {return None}
        else {Some(branch)}
    }
}


