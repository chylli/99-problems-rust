//! Eliminate consecutive duplicates of list elements. (medium)
//! # example
//! ```
//! use p99::p8::compress;
//! assert_eq!(compress(&['a','a','a','a','b','c','c','a','a','d','e','e','e','e']),
//! ['a','b','c','a','d','e']);
//!
//! ```

pub fn compress<T: PartialEq + Copy>(list: &[T]) -> Vec<T> {
    let mut result = vec![];
    result.push(list[0]);
    let mut last = list[0];
    for c in list {
        if *c != last {
            last = *c;
            result.push(last);
        }
    }
    result
}
