//! Drop every N'th element from a list
///# Example
///```
///use p99::p16::drop;
///assert_eq!(drop(&['a','b','c','d','e','f','g','h','i','j'],3),
///vec!['a','b','d','e','g','h','j']);
///```

pub fn drop(list: &[char], i: usize) -> Vec<char> {
    let mut result: Vec<char> = list.iter().map(|x| *x).collect::<Vec<char>>();
    for j in 0..list.len() {
        let k = list.len() - j;
        if k % i == 2 {
            result.remove(k);
        }
    }
    result
}
