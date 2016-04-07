//!Pack consecutive duplicates of list elements into sublists. (medium)
///```
///    use p99::p9::pack;
///assert_eq!(pack(&['a','a','a','a','b','c','c','a','a','d','d','e','e','e','e']),vec![vec!['a', 'a', 'a', 'a'], vec!['b'], vec!['c', 'c'], vec!['a', 'a'], vec!['d', 'd'], vec!['e', 'e', 'e', 'e']])
///```

pub fn pack(list: &[char]) ->Vec<Vec<char>> {
    let list = list.to_vec();
    let mut result  = vec![];
    let mut last = list[0];
    let mut sub_result = vec![last];
    for c in &list[1..] {
        if *c == last {
            sub_result.push(last);
        }
        else {
            last = *c;
            result.push(sub_result.clone());
            sub_result = vec![last];
        }
    }
    result.push(sub_result.clone());
    result
}
 
