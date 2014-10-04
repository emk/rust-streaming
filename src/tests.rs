use iter::StreamingIterator;

struct BufferIter {
    bytes: Vec<u8>,
    cur: uint,
}

impl<'a> StreamingIterator<'a, &'a [u8]> for BufferIter {
    fn next(&mut self) -> Option<&[u8]> {
        if self.cur >= self.bytes.len() {
            None
        } else {
            self.cur += 1;
            Some(self.bytes.slice(self.cur - 1, self.cur))
        }
    }
}

// #[test] 
// fn generic_over_stream() { 
    // fn count<'a, I: 'a + StreamingIterator<'a, &'a [u8]>>(iter: I) -> uint { 
        // let mut count = 0u; 
        // streaming_for!(val in iter, { 
            // count += 1; 
        // }); 
        // count 
    // } 
//  
    // let buf = BufferIter { bytes: vec![0, 1, 2], cur: 0 }; 
    // assert_eq!(count(buf), 3); 
// } 

#[test]
fn not_generic_over_stream() {
    fn count(mut iter: BufferIter) -> uint {
        let mut count = 0u;
        streaming_for!(val in iter, {
            count += 1;
        });
        count
    }

    let buf = BufferIter { bytes: vec![0, 1, 2], cur: 0 };
    assert_eq!(count(buf), 3);
}
