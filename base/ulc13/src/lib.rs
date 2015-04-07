//! ACID Filesystem Operations
#![feature(io, convert)]
#![feature(std_misc, thread_sleep)]

extern crate ulc12;
#[macro_use]
extern crate ulc22;

// how is commit different from flush -> the two solve two very different problems
// flush is a low-level directive to write buffered data onto disk. It does not 
// take into account the logical state of thev buffered data (flushing a single set of 
// data in between two calls to write where the second fails is valid).
// furthermore, flush can fail durring disk write (power failure).
// commit insures logical consistacy by making the write to disk atomic. when
// commit is called, either all buffered data is written to disk, or none at all
// even in an event of sudden failure. (it does so by writting to a shadow file 
// then switching inodes)

pub mod file;
// dir?
