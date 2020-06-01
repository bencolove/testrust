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
        /* fail coz MyBox<{integer}> cannot be dereferenced
        assert_eq!(x, *b);
        */

    }

    struct MyDerefBox<T>(T);

    impl<T> MyDerefBox<T> {
        fn new(x: T) -> MyDerefBox<T> {
            MyDerefBox(x)
        }
    }
    
    use std::ops::Deref;

    impl<T> Deref for MyDerefBox<T> {
        // associated type
        type Target = T;

        // return a referenc to tuple's first component
        fn deref(&self) -> &T {
            &self.0
        }
    }

    #[test]
    fn test_myderefbox() {
        let x = 10;
        let b = MyDerefBox::new(x);
        // effectively call *(b.deref())
        assert_eq!(10, *b);
    }

    /*
    Deref coercion happens at compile time, no runtime overhead introduced.

    When types and trait implementations found in such three case:
        1. From `&T`     to `&U`     when T: Deref<Target=U>
        2. From `&mut T` to `&mut U` when T: DerefMut<Target=U>
        3. From `&mut T` to `&U`     when T: DerefMut<Target=U>
    */
    #[test]
    fn test_deref_coercion() {
        // function accepts &str type
        fn function_accept_str_ref (name: &str) -> String {
            // macro format! returns String instance, move ownership? (String.copy impl ?)
            format!("hello {}", name)
        }

        let m = MyDerefBox::new(String::from("Rust"));
        /*
            1. &m => m.deref() -> &String
            2. &String => String.deref() -> &str
            3. &str matches function arguments
        */
        let ret = function_accept_str_ref(&m);

        assert_eq!("hello Rust", ret);
    }

    // #[test]
    fn test_drop_trait() {
        // struct MyDrop {
        //     data: String,
        //     callback: dyn Fn(String) -> ()
        // }

        // impl Drop for MyDrop {
        //     // drop returns () (nothin, aka `()`)
        //     fn drop(&mut self) {
        //         let out = format!("release {}", self.data);
        //         println!("{}", out);
        //         (self.callback)(out);
        //     }
        // }
        // use std::vec::Vec;
        // let output: Vec<String> = Vec::new();
        // // let out_ref = &mut output;
        // let act_when_drop = |s| output.push(s);
        // {
        //     let c = MyDrop {
        //         data: String::from("c"),
        //         callback: act_when_drop,
        //     };
        //     let d = MyDrop {
        //         data: String::from("d"),
        //         callback: act_when_drop,
        //     };
        //     println!("about to be out of scope");
        // }
        // let should_eq = vec![String::from("release d"), String::from("release c")];
        // assert_eq!(should_eq, output);
    }

    #[test]
    fn test_drop_early() {
        struct CustomSmartPointer {
            data: String,
        }
        
        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Dropping CustomSmartPointer with data `{}`!", self.data);
            }
        }
        
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        use std::mem::drop;
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }
}