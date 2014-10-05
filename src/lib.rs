//! Experimental Rust utilities for writing fast, streaming parsers without
//! allocating memory.

#![license = "Public domain (Unlicense)"]
#![experimental]
// Feel free to disable these if they become too annoying.
#![deny(missing_doc)]
#![deny(warnings)]

#![feature(macro_rules)]

#[cfg(test)] extern crate test;

// Want to share your experiments, hacks, etc.?  Just add a module.

pub mod csv;
pub mod iter;
pub mod buffers;
