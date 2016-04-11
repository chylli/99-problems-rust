//! Decode a run-length encoded list. (medium)
//!
//! Given a run-length code list generated as specified in the previous problem,
//! construct its uncompressed version.
/// # example
///```
///use p99::p12::decode;
///use p99::p11::Node::*;
///assert_eq!(decode(&vec![Many(3,'a'),One('b'),Many(2,'c')]),vec!['a','a','a','b','c','c']);
///```

use super::p11::Node;
use super::p11::Node::*;

pub fn decode<T: PartialEq + Copy>(list: &Vec<Node<T>>) -> Vec<T> {
    let mut result: Vec<T> = vec![];
    for node in list {
        match *node {
            One(c) => result.push(c),
            Many(count, c) => {
                for _ in 0..count {
                    result.push(c)
                }
            }
        }
    }
    result
}
