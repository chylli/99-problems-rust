//! Find the last but one (last and penultimate) elements of a list.
///# example
///```
///use p99::p2::last_two;
///assert_eq!(last_two(&['a','b','c','d']),Some(['c','d']));
///assert_eq!(last_two(&['a']),None);
///let list:&[char] = &[];
///assert_eq!(last_two(list),None);
///```

pub fn last_two<T: Copy>(list: &[T]) -> Option<[T; 2]> {
    if list.len() > 1 {
        Some([list[list.len() - 2], list[list.len() - 1]])
    } else {
        None
    }
}
