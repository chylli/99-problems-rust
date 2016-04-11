//! # Find the last box of a list.
//! # example
///```
///use p99::p1::my_last;
///assert_eq!(my_last(&['a','b','c','d']),Some(['d']));
///assert_eq!(my_last(&['d']),Some(['d']));
///let list: &[char] = &[];
///assert_eq!(my_last(list),None);
///```
// TODO should rewrite about line to assert_eq!(&[]:&[char])
// after issue 23416 implemented

pub fn my_last<T>(list: &[T]) -> Option<[T; 1]>
    where T: Copy
{
    if list.len() > 0 {
        Some([list[list.len() - 1]])
    } else {
        None
    }
}
