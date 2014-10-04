//! A replacement for Iterator

#![macro_escape]

/// Like `Iterator`, but it allows you to store temporary data in the
/// iterator itself, and return temporary references from `next`.
///
/// Massive thanks to Sharp for figuring out how to do this.
pub trait StreamingIterator<'a, T> {
    fn next(&'a mut self) -> Option<T>;
}

/// Similar to `for`, but doesn't enforce any trait restrictions on the
/// iterator.
#[macro_export]
macro_rules! streaming_for {
    ($var:pat in $expr:expr, $b:stmt) => {
        {
            // Only evaluate once!
            let ref mut iter = &mut $expr;
            loop {
                match iter.next() {
                    None => { break; }
                    Some($var) => { $b }
                }
            }
        }
    };
}
