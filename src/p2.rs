//!Find the last but one (last and penultimate) elements of a list. 
///# example
///```
///use p99::p2::last_two;
///assert_eq!(last_two(&['a','b','c','d']),Some(['c','d']));
///assert_eq!(last_two(&['a']),None);
///assert_eq!(last_two(&[]),None);
///```

pub fn last_two(list: &[char]) -> Option<[char; 2]> {
    if list.len() > 1{
        Some([list[list.len()-2],list[list.len()-1]])
    }
    else{
        None
    }
}
