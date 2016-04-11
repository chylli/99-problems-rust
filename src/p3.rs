//! Find the k'th element of a list. (easy)
///# example
///```
///use p99::p3::at;
///assert_eq!(at(3, &['a','b','c','d']), Some('c'));
///assert_eq!(at(3, &['a','b',]), None)
///```

pub fn at<T: PartialEq + Copy>(k: usize, list: &[T]) -> Option<T> {
    if list.len() >= k {
        Some(list[k - 1])
    } else {
        None
    }
}
