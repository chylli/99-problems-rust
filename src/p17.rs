//! Split a list into two parts; the length of the first part is given. (easy)
//! If the length of the first part is longer than the entire list, then the
//! first part is the list and the second part is empty.
///# Exmaple
///```
///use p99::p17::split;
///assert_eq!(split(&['a','b','c','d','e','f','g','h'],3),
///           (vec!['a','b','c'],vec!['d','e','f','g','h']));
///let zero_list: Vec<char> = vec![];
///assert_eq!(split(&['a','b','c','d'],5),
///           (vec!['a','b','c','d'],zero_list));
///```

pub fn split<T: Copy>(list: &[T], i: usize) -> (Vec<T>, Vec<T>) {
    let mut vec1: Vec<T> = vec![];
    let mut vec2: Vec<T> = vec![];
    for j in 0..list.len() {
        if j <= i {
            vec1.push(list[j]);
        } else {
            vec2.push(list[j]);
        }
    }
    (vec1, vec2)
}
