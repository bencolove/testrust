/*
Smart pointers are structs with pointer referring to heap addresses 
with extra metadata like:
    1. capacity
    ...
with traits:
    1. Deref, making it like reference 
    2. Drop, called when out of scope
*/
mod test_box {
    /*
    Box<T> is like:
        struct<T> Box {
            pointer: &T,
        }
    While instances of `Box` is on the stack, data where pointer is refering is on the heap.
    */

    #[test]
    fn test_trivial_box() {
        // instance of Box itself (smart pointer) is stored on the stack, and 15 (of type i32) is on the heap
        let better_on_stack = Box::new(15);
        println!("value = {}", better_on_stack);
    }// when `Box` instances out of scope, themselves and data they referring to will be both deallocated 
}

mod test_deref {

    #[test]
    fn test_deref_operator() {
        let x = 5; // of type i32
        let y = &x; // ref to value of type i32
        assert_eq!(x, 5);
        assert!(*y == 5);
        // smart pointer acts like referrence coz of Deref trait
        let b = Box::new(x);
        assert_eq!(*b, 5);
    }

    // define as tuple like Box
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    
    fn test_custom_Box() {
        let x = 10;
        let b = MyBox::new(x);
        assert_eq!(x, *b);
    }
}