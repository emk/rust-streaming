#![feature(macro_rules)]

#[cfg(test)] extern crate test;

// Want to share your experiments, hacks, etc.?  Just add a module.

pub mod iter;
pub mod buffers;

#[cfg(test)]
mod tests;
