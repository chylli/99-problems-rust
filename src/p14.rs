//! Duplicate the elements of a list. (easy)
///# Example
///```
///use p99::p14::duplicate;
///assert_eq!(duplicate(&['a','b','c']),vec!['a','a','b','b','c','c'])
///```

pub fn duplicate(list: &[char]) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for i in list {
        result.push(*i);
        result.push(*i);
    }
    result
}
