///* Find the last box of a list.
///** example
///``` * (my-last '(a b c d))
///(D)```

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new()->List {
        List::Nil
    }

    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn my_last(&self) -> List{
        match *self {
            List::Cons(head, ref tail) => {
                let tail_last = tail.my_last();
                match tail_last {
                    List::Nil => {
                        List::Cons(head, Box::new(List::Nil))
                    }
                    _ => {tail_last}
                }
            }
            List::Nil => {
                List::Nil
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
        List::Nil => {
            assert!(false);
        }
        List::Cons(head, tail) => {
            assert!(head == 1);
            match *tail {
                List::Nil => {
                    assert!(true);
                }
                _ => {
                    assert!(false);
                }
            }
        }
    }
}
