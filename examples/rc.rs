/*
Demonstration of 
    1. Rc<T>, share of *immutable* data among multiple parts of app
    2. Rc...

std::rc::Rc 
    1) like `Box<T>` is a smart pointer, acts like a reference with Deref in hand,
    2) with addintional meta data, the counter which will be returned by Rc::strong_count(&Rc),
    3) Rc::clone(&Rc) does not deep copy data on heap, but increase the internal counter and return itself
Rc may look like:
    struct <T> Rc {
        data: &T,
        ref_count: i32,
    }
    impl Deref for Rc {
        type Target = T,
        fn deref(&self) -> &T{
            ...
        }
    } 
    impl Drop for Rc {
        fn drop(&mut self) -> {
            self.ref_count -= 1;
            if self.ref_count == 0 {
                std::mem::drop(self);
            }
        }
    }
    fn <T> clone(target:&mut Rc<T>) -> &Rc<T> {
        target.ref_count += 1;
        return target;
    }
*/

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl Drop for List {
    fn drop(&mut self) {
        let ps = match self {
            Cons(v, l) => format!("dropping for {}, Rc::string_count()={}", v, 
                Rc::strong_count(l)),
            Nil => format!("dropping for {}", "Nil"),
        };
        println!("dropping from {}", ps);
    }
}

use self::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum RefList<'a> {
    Cons(i32, &'a RefList<'a>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let ref_base = RefList::Cons(2, &RefList::Cons(1, &RefList::Nil));
    let ref_a = RefList::Cons(3, &ref_base);
    let ref_b = RefList::Cons(4, &ref_base);

    println!("Ref version: {:?}", ref_a);
    println!("Ref version: {:?}", ref_b);
}
