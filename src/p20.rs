//!Remove the K'th element from a list. (easy)
//!
//!The first element of the list is numbered 0, the second 1,...
///```
/// use p99::p20::Remove;
///let mut a = vec![0,1,2,3,4,5,6];
///a.remove_at(2);
///assert_eq!(a,vec![0,1,3,4,5,6]);
///```

pub trait Remove<T: Copy>{
    fn remove_at(&mut self, index: usize) -> T;
}

impl <T: Copy> Remove<T> for Vec<T> {
    fn remove_at(&mut self, index: usize) -> T {
        self.remove(index)
    }
}
