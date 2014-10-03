//! Custom buffer support.
//!
//! WARNING: Don't believe the 'boundary' parameter.  It's a lie.

use std::cmp::min;
use std::iter::range;
use std::io::{Buffer,EndOfFile,IoError,IoResult};
use std::mem::transmute;
use std::rand::{Rng,task_rng};

#[cfg(test)] use std::io::{File,MemReader};
#[cfg(test)] use std::str::from_utf8;


/// An internal trait with some convenience functions.
pub trait SliceContains {
    /// Does `needle` appear in this buffer?
    fn contains_slice(&self, needle: &[u8]) -> bool;
    /// At what location does `needle` appear in this buffer?
    fn contains_slice_pos(&self, needle: &[u8]) -> Option<uint>;
}

impl<'a> SliceContains for &'a [u8] {
    #[inline(never)]
    fn contains_slice(&self, needle: &[u8]) -> bool {
        self.contains_slice_pos(needle).is_some()
    }

    // XXX - Ignores _needle for now, hardcoded for speed.
    #[inline(never)]
    fn contains_slice_pos(&self, _needle: &[u8]) -> Option<uint> {
        // This will burn 50% of our total program execution time if we let
        // it.
        //self.windows(needle.len()).position(|w| w == needle)
        //if self.len() < needle.len() { return None; }
        //'outer: for i in range(0, self.len()-(needle.len()+1)) {
        //    if self[i] == needle[0] {
        //        for j in range(0, needle.len()) {
        //            if self[i+j] != needle[j] { continue 'outer; }
        //        }
        //        return Some(i);
        //    }
        //}
        //return None;

        // XXX - Hardcoded for performance.
        if self.len() < 2 { return None; }
        for i in range(0, self.len()-1) {
            if self[i] == 10 && self[i+1] == 10 { return Some(i); }
        }
        None
    }
}

/// Used for testing other buffers.  Dribbles bytes through in small,
/// random increments.
pub struct DribbleBuffer<'a, T: Buffer+'a> {
    input: &'a mut T
}

impl<'a,T: Buffer+'a> DribbleBuffer<'a, T> {
    /// Create a new wrapper around `input`.
    pub fn new(input: &'a mut T) -> DribbleBuffer<'a, T> {
        DribbleBuffer{input: input}
    }
}

impl<'a,T: Buffer+'a> Reader for DribbleBuffer<'a,T> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        self.input.read(buf)
    }
}

impl<'a,T: Buffer+'a> Buffer for DribbleBuffer<'a,T> {
    fn fill_buf<'a>(&'a mut self) -> IoResult<&'a [u8]> {
        let original = try!(self.input.fill_buf());
        let limit = task_rng().gen::<uint>() % 6;
        Ok(original[..min(original.len(), limit)])
    }

    fn consume(&mut self, amt: uint) {
        self.input.consume(amt)
    }
}

#[cfg(test)]
fn test_data() -> Vec<u8> {
    let path = "test_data/fr/sample.conllx";
    File::open(&Path::new(path)).read_to_end().unwrap()
}

#[test]
fn dribble_buffer_read_to_string() {
    let data = test_data();
    let mut reader = MemReader::new(data.clone());
    let mut buffer = DribbleBuffer::new(&mut reader);
    let lines: Vec<String> = buffer.lines().map(|l| l.unwrap()).collect();
    let via_buffer = lines.concat();
    assert_eq!(from_utf8(data.as_slice()).unwrap(), via_buffer.as_slice());
}

/// A buffer which breaks chunks only after the specified boundary
/// sequence, or at the end of a file, but nowhere else.
pub struct ChunkBuffer<'a, T: Buffer+'a> {
    input:  &'a mut T,
    boundary: Vec<u8>,
    buffer: Vec<u8>
}

impl<'a, T: Buffer+'a> ChunkBuffer<'a,T> {
    /// Create a new `ChunkBuffer` wrapping `input` and breaking at
    /// `boundary`.
    pub fn new(input: &'a mut T, boundary: &[u8]) -> ChunkBuffer<'a,T> {
        ChunkBuffer{input: input, boundary: boundary.to_vec(),
                    buffer: vec![]}
    }

    // Called internally to make `buffer` valid.  This is where all our
    // evil magic lives.
    fn top_up<'b>(&'b mut self) -> IoResult<&'b [u8]> {
        assert!(!self.buffer.as_slice()
                .contains_slice(self.boundary.as_slice()));
        loop {
            let (consumed, done) = {
                let read_or_err = self.input.fill_buf();
                match read_or_err {
                    Err(IoError{kind: EndOfFile, ..}) => {
                        // Exit 1: We're at the end of the file, so use
                        // whatever we've got.
                        return Ok(self.buffer.as_slice())
                    },
                    Err(err) => {
                        // Exit 2: We've got a hard error.
                        return Err(err)
                    },
                    Ok(read) => {
                        // Try to grab enough so that we know we have a
                        // chunk.
                        match read.contains_slice_pos(self.boundary.as_slice()) {
                            Some(pos) => {
                                let bytes = pos + self.boundary.len();
                                self.buffer.push_all(read[..bytes]);
                                (bytes, true)
                            }
                            None => {
                                let buf_len = self.buffer.len();
                                let bound_len = self.boundary.len();
                                // We'll look here for a split boundary token.
                                let scan_start =
                                    buf_len - min(buf_len, bound_len-1);
                                let scan_end = min(buf_len + (bound_len-1),
                                                   buf_len + read.len());
                                self.buffer.push_all(read);
                                let check =
                                    self.buffer.slice(scan_start, scan_end);
                                (read.len(), 
                                 check.contains_slice(self.boundary.as_slice()))
                            }
                        }
                    }
                }
            };
            self.input.consume(consumed);
            if done {
                // Exit 3: We've got at least one boundary in our buffer.
                assert!(self.buffer.as_slice()
                        .contains_slice(self.boundary.as_slice()));
                return Ok(self.buffer.as_slice())
            }
        }
    }    

}

impl<'a,T: Buffer+'a> Reader for ChunkBuffer<'a,T> {
    fn read(&mut self, _buf: &mut [u8]) -> IoResult<uint> {
        // We need to drain our internal buffer first, then our external
        // buffer.
        fail!("Not yet implemented");
    }
}

impl<'a,T: Buffer+'a> Buffer for ChunkBuffer<'a,T> {
    fn fill_buf<'a>(&'a mut self) -> IoResult<&'a [u8]> {
        if self.buffer.as_slice().contains_slice(self.boundary.as_slice()) {
            // Exit 1: Valid data in our local buffer.
            Ok(self.buffer.as_slice())
        } else if self.buffer.len() > 0 {
            // Exit 2: Add some more data to our local buffer so that it's
            // valid (see invariants for top_up).
            self.top_up()
        } else {
            {
                let read_or_err = self.input.fill_buf();
                // Exit 3: Error when reading underlying buffer.
                match read_or_err {
                    Err(err) => { return Err(err); }
                    Ok(read) => {
                        if read.contains_slice(self.boundary.as_slice()) {
                            // Exit 4: We can return this directly, but see
                            // https://github.com/rust-lang/rust/issues/6393
                            // https://github.com/rust-lang/rust/issues/12147
                            // for a discussion of why we need unsafe here.
                            // Basically, we need to break the lifetime
                            // propagation between `read` and our return
                            // value, so `read` can be allowed to lapse
                            // when we leave this lexical scope.
                            return Ok(unsafe { transmute(read) });
                        }
                    }
                }
            }

            // Exit 5: Accumulate sufficient data in our local buffer (see
            // invariants for top_up).
            self.top_up()
        }
    }

    fn consume(&mut self, amt: uint) {
        if self.buffer.len() > 0 {
            assert!(amt <= self.buffer.len());
            let keeping = self.buffer.len() - amt;
            for i in range(0, keeping) {
                self.buffer.swap_remove(keeping-(i+1));
            }
            self.buffer.truncate(keeping);
        } else {
            self.input.consume(amt);
        }
    }
}

#[cfg(test)]
fn read_chunks<T: Buffer>(chunked: &mut T, boundary: &[u8]) -> Vec<u8> {
    let boundary_len = boundary.len();
    let mut read = vec![];
    loop {
        let consumed = { 
            match chunked.fill_buf() {
                Ok(ref data) => {
                    read.push_all(data.as_slice());
                    let data_len = data.len();
                    assert!(data_len >= boundary_len);
                    assert!(data.contains_slice(boundary));
                    data_len
                }
                Err(IoError{kind: EndOfFile, ..}) => { break; }
                Err(err) => { fail!("{}", err); }
            }
        };
        chunked.consume(consumed);
    }
    read
}

#[test]
fn reading_chunks() {
    let data = test_data();
    let mut reader = MemReader::new(data.clone());
    let mut chunked = ChunkBuffer::new(&mut reader, &[10, 10]);
    let read = read_chunks(&mut chunked, &[10, 10]);
    assert_eq!(data, read);
}

#[test]
fn reading_chunks_via_dribble() {
    let data = test_data();
    let mut reader = MemReader::new(data.clone());
    let mut dribble = DribbleBuffer::new(&mut reader);
    let mut chunked = ChunkBuffer::new(&mut dribble, &[10, 10]);
    let read = read_chunks(&mut chunked, &[10, 10]);
    assert_eq!(data, read);
}
