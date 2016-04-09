//! Find out whether a list is a palindrome. (easy)
//!
//! HINT: a palindrome is its own reverse.
///# example
///```
/// use p99::p6::is_palindrome;
///assert!(is_palindrome(&['a','b','a']));
///assert!(!is_palindrome(&['a','b','c']));
///```

pub fn is_palindrome(list: &[char]) -> bool {
    let mut new_list1 = list.to_vec();
    let new_list2 = list.to_vec();
    new_list1.reverse();
    new_list1 == new_list2
}
