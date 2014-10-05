#![allow(missing_doc)]
#![allow(dead_code)]
#![allow(unused_variable)]

use std::path::BytesContainer;

trait StreamIterator<Sized? A> {
    fn next_item<'a>(&'a mut self) -> Option<&'a A>;
}

struct CsvRdr;
struct CsvWtr;

impl CsvRdr {
    /// Returns `true` when the underlying data stream has been exhausted.
    fn done(&self) -> bool { false }
}

/// An iterator over fields in the current record.
///
/// When the end of the record is reached, the iterator yields `None`.
/// Subsequent invocations of the iterator yield fields from the next
/// record. If the underlying data stream has been exhausted (or if there
/// was an error parsing the data), `None` is returned indefinitely.
impl StreamIterator<[u8]> for CsvRdr {
    fn next_item<'a>(&'a mut self) -> Option<&'a [u8]> {
        // In real usage, this would return a slice of bytes from the CSV's
        // underlying data stream.
        // The slow version is allocating a new `Vec<u8>` and yielding that
        // instead.
        // The advantage of this approach is that it does not require an
        // allocation.
        None
    }
}

impl CsvWtr {
    /// Writes a single record to the CSV data.
    ///
    /// The input is an iterator of things that can produce a `&[u8]`.
    fn write_record<A: BytesContainer, I: StreamIterator<A>>
                   (&mut self, it: I) -> Result<(), String> {
        Ok(())
    }

    // A dummy impl to make the code below compile.
    fn write_record_regular_iter<A: BytesContainer, I: Iterator<A>>
                                (&mut self, it: I) -> Result<(), String> {
        Ok(())
    }
}

// A dummy impl to make the code below compile.
impl<'a> Iterator<&'a [u8]> for CsvRdr {
    fn next(&mut self) -> Option<&'a [u8]> { None }
}

/// The payoff.
///
/// Crucially, a "streaming iterator" puts the choice of allocation in the
/// hands of the caller. This is important because it lets the caller do
/// transformations either without allocating or without allocating space for
/// an entire record.
///
/// For example, consider the task of reading CSV data with 100 columns and
/// transforming it to CSV data with only 2 columns. A forced allocation here
/// can be quite costly. But if the caller is left to choose, then they can
/// "select" their two fields to write to new CSV data.
fn main() {
    let rdr = CsvRdr;
    let mut wtr = CsvWtr;

    while !rdr.done() {
        // This should be `wtr.write_record`.
        wtr.write_record_regular_iter(
            // None of these methods work on `StreamIterator`, but AFAIK,
            // there is no *fundamental* reason why they can't. It just may
            // not be expressible in Rust.
            rdr.enumerate()
               .filter(|&(i, _)| i == 4 || i == 58)
               .map(|(_, field)| field)).unwrap();
    }
}
