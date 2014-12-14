# EXPERIMENTAL: Zero-Allocation Streaming Parsers in Rust

Here there be hacks.  No APIs are stable.  Code may not do what the
comments claim.

Key goal:

* Build a `StreamingIterator` type that can return references to internal
  state, including as I/O buffers and the output buffers of libraries like
  [`flate2`](https://github.com/alexcrichton/flate2-rs).  This prevents
  implementing `collect`, but why can't we have `map`, `filter` and `fold`?

Target applications:

* [rust-csv](https://github.com/BurntSushi/rust-csv).
* Multicore map/reduce of
  [Snappy](https://code.google.com/p/snappy/)-compressed records.
* Anybody else who needs to iterate over a data stream without allocating.

Random useful things to read:

* [Higher-kinded types and why they're important](https://github.com/aturon/rfcs/blob/collections-conventions/text/0000-collection-conventions.md#lack-of-iterator-methods).
* [Iterating short-lived objects](http://discuss.rust-lang.org/t/iterating-short-lived-objects/274)
* [Borrow scopes should not always be lexical](https://github.com/rust-lang/rust/issues/6393) (aka, "why we have one line of `unsafe`")
* [Borrow checker gets confused by a conditionally returned borrows](https://github.com/rust-lang/rust/issues/12147) (same as above, but clearer)
* [Iterator returning items by reference, lifetime issue](http://stackoverflow.com/questions/24574741/iterator-returning-items-by-reference-lifetime-issue) (`Iterator` works the way it does for reasons explained here)

We beg for help::

* [Can I write zero-copy parsers with Iterator? It looks like it might get me a 47x speedup here.](http://www.reddit.com/r/rust/comments/2i6xry/can_i_write_zerocopy_parsers_with_iterator_it/)
* [Rust struct can borrow “&'a mut self” twice, so why can't a trait?](http://stackoverflow.com/questions/26192564/rust-struct-can-borrow-a-mut-self-twice-so-why-cant-a-trait)

