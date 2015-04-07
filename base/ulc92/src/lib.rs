extern crate ulc41;


pub mod rfc_2822 {
    use ulc41::tree::Node;
    
    pub fn parse() -> Option<Node> {
        use ulc41::parser::text::CharSpec;
        use ulc41::parser::text::CharParser;
        use ulc41::parser::{And, Or, Repeats, Match};
        
        macro_rules! rule_match {
            ($l: ident, $e: expr) => (
                let $l = Match::new(stringify!($l), $e)
            );
        }
        
        
        let htab    = Match::new("htab", "\u9");
        let lf      = Match::new("lf", "\u10");
        let cr      = Match::new("cr", "\u13");
        
        let crlf    = Match::new("crlf", "\u13\u10");
        
        let sp      = Match::new("sp", "\u32");

        let wsp     = Or::new("wsp", &[&sp, &htab]);
        
        let alpha   = CharParser::new("alpha", &[
                                CharSpec::IRange('\u41', '\u5a'), 
                                CharSpec::IRange('\u61', '\u7a')]);
                                
        let digit   = CharParser::new("digit", &[
                                CharSpec::IRange('\u30', '\u39')]);
        
        
        //////// Primitive Tokens ////////
        
        let no_ws_ctl   = CharParser::new("no_ws_ctl", &[
                                CharSpec::IRange('\u1', '\u8'),
                                CharSpec::IRange('\u11', '\u12'),
                                CharSpec::IRange('\u14', '\u31'),
                                CharSpec::Singleton('\u127')]);
                                
        let text        = CharParser::new("text", &[
                                CharSpec::IRange('\u1', '\u9'),
                                CharSpec::IRange('\u11', '\u12'),
                                CharSpec::IRange('\u14', '\u127')]);
                                
        //////// Quoted Characters ////////
        
        let __backslash = Match::new("", "\\");
        let quoted_pair = And::new("quoted_pair", &[&__backslash, &text]);
        
        //////// Folding White Space and Comments ////////
        
        let fws__ = 
        let fws__at_least_1_wsp = Repeats::new("", &wsp, 1, None)
        let fws = And::new("fws", &[    &fws__optional_any_wsp_crlf,
                                        &fws__at_least_1_wsp]);
        
    }
    
    
    // folding white space, comments
    pub fn fws(s: &[u8]) -> usize {
        let f = |s: &[u8]| {
            let l = s.len();
            if l >= 3 && cr(s[0]) && lf(s[1]) && wsp(s[2]) {3}
            else if l > 0 && wsp(s[0]) {
                let mut c: usize = 0;
                for i in s {
                    if wsp(*i) {c += 1;}
                    else {break;}
                }
                c
            }
            else {0}
        };
        ::pattern::follow(s, f, 0, None)
    }
    #[test]
    pub fn test_fws() {
        assert!(fws(b" ") == 1);
        assert!(fws(b" \t ") == 3);
        assert!(fws(b" \n") == 1);
        assert!(fws(b" \r") == 1);
        assert!(fws(b" \r\n") == 1);
        assert!(fws(b" \r\n ") == 4);
        assert!(fws(b" \r\n  ") == 5);
        assert!(fws(b" \r \n  ") == 1);
        assert!(fws(b"\r\n  ") == 4);
    }
    
    pub fn ctext(c: u8) -> bool {
        no_ws_ctl(c) || IN_RANGES!(c, 33, 39, 42, 91, 93, 126)
    }
    
    // TODO change all function to accept strings
    // return type varies, use slices for input
    // avoids length checks
    
    pub fn ccontent(s: &[u8]) -> usize {
        if s.len() > 0 && ctext(s[0]) {1}
        else if quoted_pair(s) {2}
        else {comment(s)}
    }
    
    pub fn comment(s: &[u8]) -> usize {
        let mut acc = 0;
        
        if !::pattern::match_leads(s, b"(") {return 0;}
        acc += 1;
        
        acc += ::pattern::follow(&s[acc..], |s| {
            let mut acc = fws(s);
            acc += ccontent(&s[acc..]);
            acc
        }, 0, None);
        
        acc += fws(&s[acc..]);
        
        if !::pattern::match_leads(s, b")") {return 0;}
        acc += 1;
        
        acc
    }
    
    
    
    
    
    // atom
    pub fn atext(c: u8) -> bool {
        alpha(c) || alpha(c) || IN_CHARS!(c, 
            b"!"[0], b"#"[0], b"$"[0], b"%"[0], 
            b"&"[0], b"'"[0], b"*"[0], b"+"[0],
            b"-"[0], b"/"[0], b"="[0], b"?"[0],
            b"^"[0], b"_"[0], b"`"[0], b"{"[0],
            b"|"[0], b"}"[0], b"~"[0])
    }
    /*    
    pub fn atom(s: &[u8]) -> usize {
    }*/
    
    
    
    
    
    
}





/*


macro_rules! IN_RANGES {
    ($c: expr, $($S: expr, $E: expr),*) => ($($c >= $S && $c <= $E ||)* false)
}
macro_rules! IN_CHARS {
    ($c: expr, $($C: expr),*) => ($($C == $c ||)* false)
}

// No support for obsolete definitions
// blindly following the rfc as closely as posible

mod pattern {
    pub fn follow<T, U>(s: &[T], follower: U, min_reps: usize, max_reps: Option<usize>) -> usize 
        where U: Fn(&[T]) -> usize {
        use std::usize;
        
        let mut cur = 0;
        let mut reps = 0;
        let max_reps = match max_reps { Some(r) => r, None => usize::MAX};
        
        while reps < max_reps {
            let r = follower(&s[cur..]);
            
            if r == 0 {break;}
            else {cur += r;}
            
            if cur >= s.len() {break;}
            
            reps += 1;
        }
        
        if reps < min_reps {return 0;}
        else {return cur;}
    }
    #[test]
    fn test_follow() {
        assert!(follow(b"   b ", |s| if s.len() >= 1 && s[0] == b" "[0] {1} else {0}, 0, None)  == 3);
        assert!(follow(b"   b ", |s| if s.len() >= 1 && s[0] == b" "[0] {1} else {0}, 0, Some(2))  == 2);
        assert!(follow(b"   b ", |s| if s.len() >= 1 && s[0] == b" "[0] {1} else {0}, 1, Some(2))  == 2);
        assert!(follow(b"   b ", |s| if s.len() >= 1 && s[0] == b" "[0] {1} else {0}, 5, Some(2))  == 0);
        assert!(follow(b"   b ", |s| if s.len() >= 1 && s[0] == b" "[0] {1} else {0}, 5, Some(2))  == 0);
    }
    
    pub fn match_lead<T: Eq>(s: &[T], c: &T) -> bool {
        s.len() > 0 && s[0] == *c
    }
    pub fn match_leads<T: Eq>(s: &[T], c: &[T]) -> bool {
        if c.len() > 0 {match_lead(s, &c[0])} else {false}
    }

    
}

*/
