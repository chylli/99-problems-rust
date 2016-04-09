//! Reverse a list. (easy)
//! OCaml standard library has List.rev but we ask that you reimplement it.
///# example
///```
///use p99::p5::rev;
///assert_eq!(rev(&mut['a','b','c']),['c','b','a'])
///```

pub fn rev(list: &mut [char]) -> Vec<char> {
    let len = list.len();
    let index = len - 1;
    let mut v = vec!['d';len];
    for i in 0..len {
        println!("{}", i);
        v[index - i] = list[i];
    }
    v
}
