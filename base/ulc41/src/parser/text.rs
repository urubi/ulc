use std::rc::Rc;
use ::tree;
use ::parser::Parser;


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

pub struct CharParser {
    tag: Rc<String>,
    spec: Vec<CharSpec>
}
impl CharParser {
    pub fn new(tag: &str, spec: &[CharSpec]) -> CharParser {
        CharParser {
            tag: Rc::new(tag.to_string()),
            spec:spec.to_vec()
        }
    }
}
impl Parser<char> for CharParser {
    fn parse<'t>(&self, source: &'t [char]) -> Option<tree::Node<'t, char>> {
        if source.len() == 0 {return None}
        
        for s in self.spec.iter() {
            if s.check(source[0]) {return Some(tree::Leaf::new(self.tag.clone(), &source[..1]))}
        }
        return None;
    }
    fn parse_tag(&self) -> Rc<String> {
        self.tag.clone()
    }
}
#[test]
fn test_char_parser() {
    use ::parser::Parser;
    let a = CharParser::new("RandomChars", &[
                                    CharSpec::Singleton('H'), 
                                    CharSpec::XRange('a' , 'f'), 
                                    CharSpec::Singleton('ز'), 
                                    CharSpec::XRange('ا', 'ج')]);
                                    
    assert!(a.parse(&[]) == None);
    assert!(a.parse(&['z']) == None);
    assert!(a.parse(&['H']) == Some(tree::Leaf::new(Rc::new("RandomChars".to_string()), &['H'])));
    assert!(a.parse(&['b']) == Some(tree::Leaf::new(Rc::new("RandomChars".to_string()), &['b'])));
    assert!(a.parse(&['و']) == None);
    assert!(a.parse(&['ز']) == Some(tree::Leaf::new(Rc::new("RandomChars".to_string()), &['ز'])));
    assert!(a.parse(&['ت']) == Some(tree::Leaf::new(Rc::new("RandomChars".to_string()), &['ت'])));
}



#[cfg(test)]
mod test {
    use tree::NodeExt;
    
    use parser::Parser;
    use super::CharSpec;
    use super::CharParser;
    
    #[test]
    fn test_complex() {
        use parser::Repeats;
        use parser::Perhaps;
        use parser::Or;
        use parser::And;
        use parser::Match;
        
        
        let alpha_spec = [CharSpec::IRange('a' , 'z'), CharSpec::IRange('A' , 'Z')];
        let alpha_atom = CharParser::new("alpha", &alpha_spec);
        let alpha_rep  = Repeats::new("alpha*", &alpha_atom, 0, None);

        let num_spec = [CharSpec::IRange('0' , '9')];
        let num_atom = CharParser::new("num", &num_spec);
        let num_rep  = Repeats::new("num*", &num_atom, 0, None);        
        
        let sym_spec = [CharSpec::Singleton('!')];
        let sym_atom = CharParser::new("sym", &sym_spec);
        let sym_rep  = Repeats::new("sym*", &sym_atom, 0, None);
        
        let alphanum_block = Or::new("alphanum", &[&alpha_rep, &num_rep]);
        let alphanum_block_rep = Repeats::new("alphanum*", &alphanum_block, 0, None);
        
        // unicode literal-ish: Will not live past the place it is used
        macro_rules! ul {
            ($e: expr) => (
                <Vec<char> as AsRef<[char]>>::as_ref(&$e.chars().collect::<Vec<char>>());
            )
        }
        let literal_match = Match::new("some match", ul!("Cheesee"));
        
        
        // --------------------------------------------------------------------
        
        
        let source: Vec<char> = "Cheesee1111sss!!!".chars().collect();

        assert!(literal_match.parse(&source).unwrap().slice() == ul!("Cheesee"));
        assert!(literal_match.parse(&source[2..]) == None);
        
        let m = alphanum_block_rep.parse(&source).unwrap();
        
        assert!(m.slice() == ul!("Cheesee1111sss"));
        assert!(m.get_node("alpha*").unwrap().slice() == ul!("Cheesee"));
        assert!(m.get_node("num*").unwrap().slice() == ul!("1111"));
        
        
        let seq = And::new("alpha* num* alpha* sym*", &[&alpha_rep, &num_rep, &alpha_rep, &sym_rep]);
        assert!(seq.parse(&source).unwrap().slice() == source);
        
        let seq = And::new("alpha* num*", &[&alpha_rep, &num_rep]);
        assert!(seq.parse(&source).unwrap().slice() == ul!("Cheesee1111"));
        
        let seq = And::new("alpha* sym*", &[&alpha_rep, &sym_rep]);
        assert!(seq.parse(&source) == None);
        
        let maybe_alpha = Perhaps::new("[alpha*]", &alpha_rep);
        let seq_imp = And::new("[alpha*] num* alpha*", &[&maybe_alpha, &num_rep, &alpha_rep]);
        assert!(seq_imp.parse(&source).unwrap().slice() == ul!("Cheesee1111sss"));
        assert!(seq_imp.parse(&source[7..]).unwrap().slice() == ul!("1111sss"));
        assert!(seq_imp.parse(&source[..10]) == None);
        
    }
    
}


#[cfg(test)]
mod simple_grammar {
    

}
