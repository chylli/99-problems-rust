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


pub fn duplicate(list: &[char]) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for i in list {
        result.push(*i);
        result.push(*i);
    }
    result
}

pub fn duplicate2(list: &[char]) -> Vec<char> {
    let result: Vec<char> = list.iter().map(|x| vec![*x,*x]).fold(vec![],|sum : Vec<char>, x: Vec<char>| {
        let mut sum = sum.clone();
        let x = &mut x.clone();
        sum.append(x);
        sum
    });
    result
}

pub fn duplicate3(list: &[char]) -> Vec<char> {
    list.iter().flat_map(|x| vec![*x,*x]).collect::<Vec<char>>()
}
