//!
#![warn(missing_debug_implementations, missing_docs)]

pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl StrSplit<'_> {
    // split haystack by deimiter
    // By returning Self, we don't have to update all the methods in the future
    // if we decide to rename the type
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

// for desugars to `while let Some(e) = T.next()`
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            // TODO:: Bug here
            None
        } else {
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}
