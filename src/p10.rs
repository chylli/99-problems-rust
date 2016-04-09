//!Run-length encoding of a list. (easy)
//!
//!If you need so, refresh your memory about run-length encoding.
///```
///use p99::p10::encode;
///assert_eq!(encode(&['a','a','a','b','c','c']),vec![(3,'a'),(1,'b'),(2,'c')]);
///```

pub fn encode(list: &[char]) -> Vec<(i32,char)>{
    let mut result : Vec<(i32, char)> = vec![];
    if list.len() == 0 {
        return result;
    }
    let mut last = list[0];
    let mut count = 1;
    for i in 1..list.len(){
        if  list[i] == last {
            count = count + 1;
        }
        else{
            result.push((count,last));
            last = list[i];
            count = 1;
        }
    }
    result.push((count,last));
    result
}
