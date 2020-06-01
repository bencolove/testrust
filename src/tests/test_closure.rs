#[cfg(test)]
mod closure {
    use std::thread;
    use std::time::Duration;

    struct Cache<T>
    where
        T: Fn(u32) -> u32,
        {
            calculation: T,
            value: Option<u32>,
        }

    impl<T> Cache<T>
    where
        T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cache<T> {
                Cache {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }

    #[test]
    fn test_closure_anonymous_function() {
        /*
        closures not access to environment variables
        */
        let mut cacher = Cache::new(|num| {
            println!("slow calcualtion ...");
            thread::sleep(Duration::from_secs(2));
            println!("done");
            num + 2
        });

        let first_value = cacher.value(0);
        assert_eq!(first_value, 2);

        let second_value = cacher.value(99);
        assert_eq!(second_value, 2);
    }

    #[test]
    fn test_closure_with_env_var() {
        /*
        Three ways for a closure to access its context:
            1. taking ownership (move), using FnOnce trait
            2. borrow mutable, mutable reference, using FnMut trait
            3. borrow immutable, immutable reference,  using Fn trait
        */
        let x = vec![1, 2, 3];
        let y = vec![1, 2, 3];

        /* let the compiler tell you what type it is by doing silly things:
            1. var.no_such_field;
            2. var.call_wrong_method();
            or:
            std::mem::size_of_val(&T)
        */

        /* env var x will be copied */
        let equal_to_x = |z| z == x;
        assert!(equal_to_x(y));
        println!("x is still owned here {:?}", x);
        
        /* x will be moved and ownership transitted */
        let equal_to_x_moved = move |z| z == x;

    
        let z = vec![1, 2, 3];
        assert!(equal_to_x_moved(z));
    }
}