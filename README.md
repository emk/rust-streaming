# EXPERIMENTAL: Zero-Allocation Streaming Parsers in Rust

Here there be hacks.  No APIs are stable.  Code may not do what the
comments claim.

Random useful things to read:

* [Iterating short-lived objects](http://discuss.rust-lang.org/t/iterating-short-lived-objects/274)
* [Borrow scopes should not always be lexical](https://github.com/rust-lang/rust/issues/6393) (aka, "why we have one line of `unsafe`")
* [Borrow checker gets confused by a conditionally returned borrows](https://github.com/rust-lang/rust/issues/12147) (same as above, but clearer)
* [Iterator returning items by reference, lifetime issue](http://stackoverflow.com/questions/24574741/iterator-returning-items-by-reference-lifetime-issue) (`Iterator` works the way it does for reasons explained here)
