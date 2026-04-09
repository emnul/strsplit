//! This is a re-implementation of the split method on str in the Rust std library

// until_char will not work when we tell Rust to expect that the remainder and delimiter
// lifetimes will be the same. The lifetime is shortened to the smallest of the two,
// which is the scope of the until_char fn in our case
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    // We want remainder to be an Option because it can possibly be empty
    remainder: Option<&'haystack str>,
    // One solution to the lifetime issue above is to make delimiter a String, but this is
    // suboptimal because we now need to do an allocation everytime we create a StrSplit
    // Another downside is that this choice would make the library no longer compatible with all
    // devices that do not have an allocator like embedded systems
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    // split haystack by deimiter
    // By returning Self, we don't have to update all the methods in the future
    // if we decide to rename the type
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

// for desugars to `while let Some(e) = T.next()`
impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

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
        // Normally the pattern below would cause a move but &str is Copy
        // as_mut required to get a mutable reference to value inside Option
        let remainder = self.remainder.as_mut()?;
        // Find next delimiter
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            // set new remainder as everything after delim
            *remainder = &remainder[delim_end..];
            // return everything until next delim
            Some(until_delimiter)
        // If there is no delimiter in remainder, take everything from
        // remainder and replace it with a None
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

// This trait lets us avoid an allocation via format! in until_char
// We can pass the char directly to StrSplit because we have a Delimiter impl for char
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}
pub fn until_char(s: &str, c: char) -> &str {
    // Here the compiler tries to tie the lifetime of the returned string to
    // the temporary string we create with format!()
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world", 'o'), "hell");
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
