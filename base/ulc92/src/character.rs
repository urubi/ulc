use std::rc::Rc;
use ulc41::tree;
use ulc41::tree::NodeExt;
use ulc41::parser::{Parser, And, Or, Perhaps, Repeat, Match};
use ulc41::parser::Error;

#[macro_use]
use ulc22;

pub enum CharSpec {
    Singleton(char),
    IRange(char, char),
    XRange(char, char)
}
impl CharSpec {
    pub fn check(&self, c: char) -> bool {
        match self {
            &CharSpec::Singleton(a) => c == a,
            &CharSpec::IRange(start, end) => c >= start && c <= end,
            &CharSpec::XRange(start, end) => c >= start && c < end
        }
    }
}
// Clone derivation doesn;t want to work with CharSpec, derive as normal when fixed
impl Clone for CharSpec {
    fn clone(&self) -> CharSpec {
        match self {
            &CharSpec::Singleton(c) => CharSpec::Singleton(c),
            &CharSpec::XRange(a,b) => CharSpec::XRange(a,b),
            &CharSpec::IRange(a,b) => CharSpec::IRange(a,b)
        }
    }
}

pub struct Character {
    tag: Rc<String>,
    spec: Vec<CharSpec>
}
impl Character {
    pub fn new(tag: &str, spec: Vec<CharSpec>) -> Rc<Box<Parser<char>>> {
        Rc::new(Box::new(Character {
            tag: Rc::new(tag.to_string()),
            spec:spec
        }))
    }
}
impl Parser<char> for Character {
    fn parse<'t>(&self, source: &'t [char]) -> Result<tree::Node<'t, char>, Error> {
        if source.len() == 0 {
            return Err(Error{
                tag: self.parse_tag(),
                desc: "Empty string".to_string(),
                offset: 0
            })
        }
        
        for s in self.spec.iter() {
            if s.check(source[0]) {return Ok(tree::Leaf::new(self.tag.clone(), &source[..1]))}
        }
        return Err(Error{
            tag: self.parse_tag(),
            desc: format!("'{}' does not match character specification", source[0]),
            offset: 0
        })
    }
    fn parse_tag(&self) -> Rc<String> {
        self.tag.clone()
    }
}
#[test]
fn test_char_parser() {
    let a = Character::new("RandomChars", vec![CharSpec::Singleton('H'), 
                                                CharSpec::XRange('a' , 'f'), 
                                                CharSpec::Singleton('ز'), 
                                                CharSpec::XRange('ا', 'ج')]);
                                    
    assert!(a.parse(&[]).is_err());
    assert!(a.parse(&['z']).is_err());
    assert!(a.parse(&['H']) == Ok(tree::Leaf::new(Rc::new("RandomChars".to_string()), &['H'])));
    assert!(a.parse(&['b']) == Ok(tree::Leaf::new(Rc::new("RandomChars".to_string()), &['b'])));
    assert!(a.parse(&['و']).is_err());
    assert!(a.parse(&['ز']) == Ok(tree::Leaf::new(Rc::new("RandomChars".to_string()), &['ز'])));
    assert!(a.parse(&['ت']) == Ok(tree::Leaf::new(Rc::new("RandomChars".to_string()), &['ت'])));
}



#[cfg(test)]
mod test {
    use std::rc::Rc;
    use ulc41::tree;
    use ulc41::tree::NodeExt;
    use ulc41::parser::{Parser, And, Or, Perhaps, Repeat, Match};
    use ulc41::parser::Error;
    use super::CharSpec;
    use super::Character;
    
    #[test]
    fn test_complex() {
        
        
        let alpha_spec = vec![CharSpec::IRange('a' , 'z'), CharSpec::IRange('A' , 'Z')];
        let alpha_atom = Character::new("alpha", alpha_spec);
        let alpha_rep  = Repeat::new("alpha*", alpha_atom.clone(), 0, None);
        let alpha_plus  = Repeat::new("alpha+", alpha_atom.clone(), 1, None);

        let num_spec = vec![CharSpec::IRange('0' , '9')];
        let num_atom = Character::new("num", num_spec);
        let num_rep  = Repeat::new("num*", num_atom.clone(), 0, None);        
        let num_plus  = Repeat::new("num+", num_atom.clone(), 1, None);
        
        let sym_spec = vec![CharSpec::Singleton('!')];
        let sym_atom = Character::new("sym", sym_spec);
        let sym_rep  = Repeat::new("sym*", sym_atom.clone(), 0, None);
        let sym_plus  = Repeat::new("sym+", sym_atom.clone(), 1, None);
        
        //let alphanum_atom = Or::new("alphanum", &[&alpha_atom, &num_atom]);
        //let alphanum_rep = Repeat::new("alphanum*", &alphanum_atom, 0, None);
        //let alphanum_plus = Repeat::new("alphanum+", &alphanum_atom, 1, None);
        
        // --------------------------------------------------------------------
        
        
        let source: Vec<char> = "Cheesee1111sss!!!".chars().collect();

        {
            let matcher = Match::new("some match", uv!("Cheesee"));
            
            assert!(matcher.parse(&source).unwrap().slice() == ul!("Cheesee"));
            assert!(matcher.parse(&source[2..]).is_err());
        }
        
        {
            let alphanumblock = Or::new("alphanumblock", vec![alpha_plus.clone(), num_plus.clone()]);
            let matcher = Repeat::new("alphanumblock*", alphanumblock, 0, None);
            let m = matcher.parse(&source).unwrap();
            assert!(m.slice() == ul!("Cheesee1111sss"));
            assert!(m.get_node("alpha+").unwrap().slice() == ul!("Cheesee"));
            assert!(m.get_node("num+").unwrap().slice() == ul!("1111"));
        }
        
        
        let seq = And::new("alpha* num* alpha* sym*", vec![alpha_rep.clone(), num_rep.clone(), alpha_rep.clone(), sym_rep.clone()]);
        assert!(seq.parse(&source).unwrap().slice() == source);
        
        let seq = And::new("alpha* num*", vec![alpha_rep.clone(), num_rep.clone()]);
        assert!(seq.parse(&source).unwrap().slice() == ul!("Cheesee1111"));
        
        // This is not a string search library, it starts matching a patern at
        // the bignening of the string, and not search through it
        let seq = And::new("alpha* sym+", vec![alpha_rep.clone(), sym_plus.clone()]);
        match seq.parse(&source) {
            Err(x) => assert!(x.offset == 7), 
            _ => panic!("should not succeed")
        };
        
        
        let maybe_alpha = Perhaps::new("[alpha*]", alpha_rep.clone());
        let seq_imp = And::new("[alpha*] num* alpha*", vec![maybe_alpha.clone(), num_rep.clone(), alpha_rep.clone()]);
        assert!(seq_imp.parse(&source).unwrap().slice() == ul!("Cheesee1111sss"));
        assert!(seq_imp.parse(&source[7..]).unwrap().slice() == ul!("1111sss"));
        assert!(seq_imp.parse(&source[..10]).unwrap().slice() == ul!("Cheesee111"));
        
    }
    
}
