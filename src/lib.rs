#[cfg(test)]
mod tests {

    mod test_hashmap;

    mod test_closure;

    mod test_iterator;

    mod test_smart_pointer;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    /*
    ownership movement, copy and borrow:
        1. when passed-by-value,
            if it `impl`s `Copy` trait, the target will be copied and passed ownership
            otherwise, the target's ownership will be moved
        2. when passed-by-ref(borrow), ownership remains, and the only-one-mutable-borrowing rule applies
            can be mutable borrowed or
            immutable borrowed 
    */
    #[test]
    fn test_ownership() {
        // on statck or in heap?
        let v = vec![1, 2, 3];
        // move ownership since no Copy trait impl for std:vec::Vec<i32>
        let v1 = v;
        /*
        println!("{:?}", v);
        so it fails, println! marco borrows (pass-by-ref) arguments
        */
        println!("{:?}", v1);
        // immutable borrowed
        for value in &v1 {
            println!("{}", value);
        }
        // move happens
        for value in v1 {
            println!("{}", value);
        }
        /*
        it fails because v1 is moved already
        for value in v1 {
        */
    }
}
