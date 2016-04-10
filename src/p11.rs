//! Modify the result of the previous problem in such a way that if an element
//! has no duplicates it is simply copied into the result list. Only elements
//! with duplicates are transferred as (N E) lists.
/// # example
///```
///use p99::p11::encode;
///use p99::p11::Node::*;
///assert_eq!(encode(&['a','a','a','b','c','c']),vec![Many(3,'a'),One('b'),Many(2,'c')]);
///```

use self::Node::*;
#[derive(PartialEq,Debug)]
pub enum Node {
    One(char),
    Many(i32, char),
}

pub fn encode(list: &[char]) -> Vec<Node> {
    let mut result: Vec<Node> = vec![];
    if list.len() == 0 {
        return result;
    }
    let mut last = list[0];
    let mut count = 1;
    for i in 1..list.len() {
        if list[i] == last {
            count = count + 1;
        } else {
            result.push(create_node(count, last));
            last = list[i];
            count = 1;
        }
    }
    result.push(create_node(count, last));
    result
}

fn create_node(count: i32, last: char) -> Node {
    if count == 1 {
        One(last)
    } else {
        Many(count, last)
    }
}
