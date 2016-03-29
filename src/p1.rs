//!# Find the last box of a list.
//!# example
///```
///use p99::p1::my_last;
///assert_eq!(my_last(&['a','b','c','d']),Some(['d']));
///assert_eq!(my_last(&['d']),Some(['d']));
///assert_eq!(my_last(&[]),None);
///```

pub fn my_last(list: &[char]) -> Option<[char; 1]> {
    if list.len() > 0{
        Some([list[list.len()-1]])
    }
    else{
        None
    }
}

