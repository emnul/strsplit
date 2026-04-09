//!
// #![warn(missing_debug_implementations, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    // split haystack by deimiter
    // By returning Self, we don't have to update all the methods in the future
    // if we decide to rename the type
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// for desugars to `while let Some(e) = T.next()`
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have remainder
        // Without ref mut we would move value in remainder out
        // ref mut allows us to get a mutable ref to value in remainder instead of moving value
        // if it is Some.
        // "Give me a reference to the matched value"
        // if let Some(&mut remainder) does not work because the compiler would try to match
        // an Option<&mut T> instead of an Option<&T>
        //
        // Every let statement is a pattern match
        // ? is the try operator
        // Pattern match on what's inside the Some() in remainder
        let ref mut remainder = self.remainder?;
        // Find next delimiter
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            // set new remainder as everything after delim
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            // return everything until next delim
            Some(until_delimiter)
        // If there is no delimiter in remainder, take everything from
        // remainder and replace it with a None
        } else {
            self.remainder.take()
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));

    // Another way of testing haystack
}

// Need to handle the case where a delimiter tails the string
// Last element should be the empty string in this case
#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
