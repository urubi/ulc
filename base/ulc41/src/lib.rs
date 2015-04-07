#![feature(convert)]
#![feature(custom_derive)]

#[macro_use]
extern crate ulc22;

pub mod tree;
pub mod parser;

// for feature: convert
pub fn remove_this() {
    assert!(<Vec<char> as AsRef<[char]>>::as_ref(&"0".chars().collect::<Vec<char>>())[0] as u32 != 0);
}

