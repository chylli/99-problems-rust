//! Replicate the elements of a list a given number of times. (medium)
///# Example
///```
///use p99::p15::repli;
///assert_eq!(repli(&['a','b','c'],3),vec!['a','a','a','b','b','b','c','c','c']);
///```

pub fn repli<T: Copy>(list: &[T], time: i32) -> Vec<T> {
    list.iter()
        .flat_map(|x| {
            let mut a = vec![];
            for _ in 0..time {
                a.push(*x)
            }
            a
        })
        .collect::<Vec<T>>()
}
