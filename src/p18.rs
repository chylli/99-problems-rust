//!Extract a slice from a list. (medium)
//!
//!Given two indices, i and k, the slice is the list containing the elements
//!    between the i'th and k'th element of the original list (both limits
//!                                                            included). Start
//!    counting the elements with 0 (this is the way the List module numbers
//!                                  elements).
///```
///use p99::p18::Slice;
///let a: Vec<usize> = (0..100).collect();
///assert_eq!(a.slice(10,20),(10..21).collect::<Vec<usize>>());
///```

pub trait Slice<T: Copy>{
    fn slice(&self, start: usize, end: usize) -> Vec<T>;
}

impl <T: Copy> Slice<T> for Vec<T>{
    fn slice(&self, start: usize, end: usize) -> Vec<T>{
        let mut result = Vec::with_capacity(end - start + 1);
        for i in &self[start..(end+1)] {
            result.push(*i);
        }
        result
    }
}
