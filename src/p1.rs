///* Find the last box of a list.
///** example
///``` * (my-last '(a b c d))
///(D)```
use self::List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new()->List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn my_last(&self) -> List{
        match *self {
            // what's the meaning of ref here?
            Cons(head, ref tail) => {
                let tail_last = tail.my_last();
                match tail_last {
                    Nil => {
                        Cons(head, Box::new(Nil))
                    }
                    _ => {tail_last}
                }
            }
            Nil => {
                Nil
            }
        }
    }
}

#[test]
fn my_last_test(){
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    let last = list.my_last();
    match last {
        Nil => {
            assert!(false);
        }
        // why cannot use ref here?
        Cons(head, tail) => {
            assert!(head == 1);
            match *tail {
                Nil => {
                    assert!(true);
                }
                _ => {
                    assert!(false);
                }
            }
        }
    }
}
