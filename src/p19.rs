//!Rotate a list N places to the left. (medium)
///```
///use p99::p19::Rotate;
///let mut a: Vec<i32> = vec![0,1,2,3,4,5];
///a.rotate(2);
///assert_eq!(a, vec![2,3,4,5,0,1]);
///let mut a: Vec<i32> = vec![0,1,2,3,4,5];
///a.rotate(-2);
///assert_eq!(a, vec![4,5,0,1,2,3]);
///```

pub trait Rotate<T: Copy>{
    fn rotate(&mut self, mut num: isize);
}

impl <T: Copy> Rotate<T> for Vec<T>{
    fn rotate(&mut self, mut num: isize){
        let len = self.len();
        if num >= 0 {
            for _ in 0..num {
                let first = self[0];
                for j in 1..len {
                    self[j-1] = self[j];
                }
                self[len - 1] = first;
            }
        }
        else {
            num = -num;
            for _ in 0..num{
                let last = self[len - 1];
                for  j in 0..(len - 1) {
                    let k = len - 1 - j;
                    self[k] = self[k - 1];
                }
                self[0] = last;
            }
        }
    }
}
