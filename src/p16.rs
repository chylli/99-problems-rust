//! Drop every N'th element from a list
///# Example
///```
///use p99::p16::drop;
///assert_eq!(drop(&['a','b','c','d','e','f','g','h','i','j'],3),
///vec!['a','b','d','e','g','h','j']);
///```

pub fn drop<T: Copy>(list: &[T], i: usize) -> Vec<T> {
    let mut result: Vec<T> = list.iter().map(|x| *x).collect::<Vec<T>>();
    for j in 0..list.len() {
        let k = list.len() - j;
        if k % i == 2 {
            result.remove(k);
        }
    }
    result
}
