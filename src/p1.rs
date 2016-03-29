//!# Find the last box of a list.
//!# example
///```
///use p99::p1::my_last;
///assert_eq!(my_last(&['a','b','c','d']),['d'])
///```

pub fn my_last(list: &[char]) -> [char; 1] {
    [list[list.len()-1]]
}

