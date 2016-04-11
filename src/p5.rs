//! Reverse a list. (easy)
//! OCaml standard library has List.rev but we ask that you reimplement it.
///# example
///```
///use p99::p5::rev;
///assert_eq!(rev(&mut['a','b','c']),['c','b','a'])
///```

pub fn rev<T: Copy>(list: &mut [T]) -> Vec<T> {
    let len = list.len();
    let index = len - 1;
    let mut v: Vec<T> = Vec::with_capacity(len);
    for i in 0..len {
        v.push(list[index - i]);
    }
    v
}
