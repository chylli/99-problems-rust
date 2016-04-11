//! Duplicate the elements of a list. (easy)
///# Example
///```
///use p99::p14::duplicate;
///assert_eq!(duplicate(&['a','b','c']),vec!['a','a','b','b','c','c'])
///```
///# Example
///```
///use p99::p14::duplicate2;
///assert_eq!(duplicate2(&['a','b','c']),vec!['a','a','b','b','c','c'])
///```
///# Example
///```
///use p99::p14::duplicate3;
///assert_eq!(duplicate3(&['a','b','c']),vec!['a','a','b','b','c','c'])
///```


pub fn duplicate<T: Copy>(list: &[T]) -> Vec<T> {
    let mut result: Vec<T> = vec![];
    for i in list {
        result.push(*i);
        result.push(*i);
    }
    result
}

pub fn duplicate2<T: Copy>(list: &[T]) -> Vec<T> {
    let result: Vec<T> = list.iter()
                             .map(|x| vec![*x, *x])
                             .fold(vec![], |sum: Vec<T>, x: Vec<T>| {
                                 let mut sum = sum.clone();
                                 let x = &mut x.clone();
                                 sum.append(x);
                                 sum
                             });
    result
}

pub fn duplicate3<T: Copy>(list: &[T]) -> Vec<T> {
    list.iter().flat_map(|x| vec![*x, *x]).collect::<Vec<T>>()
}
