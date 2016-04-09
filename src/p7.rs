//! Flatten a nested list structure. (medium)
//! ```text
//! (* we need to define one
//!     first. A node of a nested list is either an element, or a list of
//!     nodes. *)
//!  type 'a node =
//!    | One of 'a
//!    | Many of 'a node list;;
//! type 'a node = One of 'a | Many of 'a node list
//! ```
///# example
///```
///use p99::p7::flatten;
///use p99::p7::Tree::*;
///assert_eq!(flatten(&Many(vec![One('a'), Many(vec![One('b'), Many(vec![One('c'), One('d')]),
///One('e')])])), ['a','b','c','d','e']);
///```

use self::Tree::*;
pub enum Tree {
    One(char),
    Many(Vec<Tree>),
}

pub fn flatten(tree: &Tree) -> Vec<char> {
    let mut result = vec![];
    match *tree {
        One(c) => result.push(c),
        Many(ref v) => {
            for i in v {
                let v2 = flatten(i);
                for j in &v2 {
                    result.push(*j);
                }
            }
        }
    }
    result
}
