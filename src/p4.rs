//! Find the number of elements of a list. (easy)
//! Rust standard library has List.length but we ask
//! that you reimplement it. Bonus for a tail recursive solution.
///#example
///```
///use p99::p4::length;
///assert_eq!(length(&['a','b']),2);
///```
// TODO: it should be written with the feature #![feature(slice_patterns)],
// but stable version doesn't support this feature.
pub fn length(list: &[char]) -> usize {
    list.len()
}
