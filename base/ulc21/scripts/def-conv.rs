//usr/bin/env true; E=`mktemp`|| exit 1; rustc -o "$E" "$0" || exit 1; "$E" "$@"; R=$?; rm "$E"; exit $R;
/*
    
    The dumb C-defs to Rust constants Conversion Script
    Copyright (C) 2015  Urubi
    
    This program is free software: you can redistribute it and/or modify
    it under the terms of the [ GNU Lesser General Public License ] as 
    published by the Free Software Foundation, either version 3 of the 
    License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.

    ---------------------------------------------------------------------
    Note: Either compile is normally or chmod +x the file and run it as a 
    script. This should work with shells on real unix systems.
    ---------------------------------------------------------------------
    
    I don't have twitter so I'll vent here: I've wasted about 7 hours on 
    trying to make system call behave like it should. Turns out the 
    problem was with C's octal notation, which rust would happly accept 
    as valid (bug?). To whomerver wrote strace, I owe you a cup of tomato 
    soup, if you're willing to take up the offer.
*/

#![feature(str_words)]


const INFO: &'static str = "The dumb C-defs to Rust constants Conversion Script

Example:
    $echo '#define SOMECONST 12' | ./def-conv.rs --type=isize 
    # Result: pub const SOMECONST:isize = 12;

Options:
    --help                      This message.
    --type=TYPE                 What type to define the constants as. (Required)
    --qualifier=QUALIFIER(S)    What qualifiers to use for the constant. 
                                (Defaults to 'pub const')
Bugs:
    1. Dumb handling of macro names and values; will copy values as is without
       conversion. Be mindfull that C notation is not compatable with Rust. 
       Octal notation is especially problematic since it is valid syntax in 
       rust but reperesents a different value.
    
    Report to
    https://github.com/urubi

License: 
    LGPLv3 (see source.)
";

/* TODO Add conversion
fn segment_numbers_and_con(s: &str) -> String {
    let processing_number = false;
    let out: String = String.new();
    let left_cur:isize = 0;
    let right_cur:isize = 0;
    
    for right_cur in [0..s.len()].iter() {
        if s[right_cur].is_num()
    }
}
*/
fn main() {
    use std::io;
    use std::env;
    use std::io::BufRead;
    use std::io::Write;
    
    let mut def_type: Option<String>;       // def_type: "u8", "&str", ...
    let mut def_qualifier: Option<String>;  // def_qualifier: "pub const", ...
    
    let stdin = io::stdin();
    let mut stderr = io::stderr();
    
    def_type = None;
    def_qualifier = Some(format!("pub const"));
    
    
    
    // Parsing command line arguments
    for arg in env::args().skip(1) {
        let arg: &str = &arg;
        if &arg[..1] == "-" {
            let pair: Vec<&str> = arg.split('=').collect();
            if pair[0] == "--help" {
                println!("{}", INFO);
                return;
            }
            else if pair[0] == "--type" {
                if pair.len() != 2 {
                    stderr.write_all("usage: --type='TYPE'\n".as_bytes()).unwrap();
                    return;
                }
                def_type = Some(pair[1].to_string());
            }
            else if pair[0] == "--qualifier" {
                if pair.len() != 2 {
                    stderr.write_all("usage: --qualifier='TYPE'\n".as_bytes()).unwrap();
                    return;
                }
                def_qualifier = Some(pair[1].to_string());
            }
            else {
                stderr.write_all(format!("unknown option '{}'. type --help for more information.\n", pair[0]).as_bytes()).unwrap();
                return;
            }
        }
    }

    if def_type.is_none() {
        stderr.write_all("Const specification required. use --type\n".as_bytes()).unwrap();
        return;
    }
    
    
    let def_type = def_type.unwrap();
    let def_qualifier = def_qualifier.unwrap();
    let mut defs: Vec<(String, String)> = vec![];
    let mut longest: usize = 0;
    
    for line in stdin.lock().lines() {
        let mut line: String = line.unwrap();
        
        // Check if line is long enough and is a definition
        if line.len() <= 7 || &line[..7] != "#define" {continue;}

        // stripping comments
        line = line.split("/*").collect::<Vec<&str>>()[0].to_string();
        line = line.split("//").collect::<Vec<&str>>()[0].to_string();
        
        // segmenting line
        let words: Vec<&str> = line.words().collect();
        // make sure it's a definition with a value
        if words.len() < 3 {continue;} 
        // longest length for aesthetics
        if longest < words[1].len() {longest = words[1].len();}
        defs.push((words[1].to_string(), words[2..].connect(" ")));
    }
    
    // building rust equivalent
    for &(ref name, ref value) in defs.iter() {
        println!("{0:2$} = {1};", 
            format!("{} {}:{}", &def_qualifier, name, &def_type), 
            value,
            longest + def_qualifier.len() + def_type.len() + 5
        );    
    }
    
    
    
}

