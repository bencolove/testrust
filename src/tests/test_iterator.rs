/*
Iterators are:
    1. lazy (no effect util one terminal operation tasks place)
    2. used with for-loop
    3. typed std::slice::Iter<'_, T>
    4. without `Copy` trait, can only be moved or borrowed
*/
#[test]
fn test_basics() {
    let v1 = vec![1, 2, 3];

    // typed std::slice::Iter<'_, T>
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
    /* a second round won't compile, as v1_iter has been moved away in first for-loop
    for val in v1_iter {
      
        println!("Got: {}", val);
    }
    */
    let v2 = vec![4, 5, 6];
    for &val in &v2 {
        println!("Got: {}", val);
    }
    for &val in &v2 {
        println!("Got: {}", val);
    }
}

#[test]
fn test_iter_function() {
    /* Iterator trait:
        Item: associated type
        &mut self: next() will change an Iterator's internal state, mutable borrow is needed

    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<self::Item>;
    }

    Three different ways how to deal with values ownership from collections:
        .iter()         immutable refs
        .iter_mut()     mutable refs
        .into_iter()    move values' ownership
    */
    let v1 = vec![1, 2, 3];
    // if re-use an object, borrow is needed
    let v1_ref = &v1;
    for value in v1_ref.iter() {
        println!("{}", value);
    }
    for value in v1_ref.iter() {
        println!("{}", value);
    }
    // Iterator.next() returns Option<&T> and must be mutable
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn test_iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    println!("{:?}", v1);
    // .sum() takes self onwership, making v1_iter invalid
    let total: i32 = v1_iter.sum();
    /* fails
    println!("{:?}", v1_iter);
    */
    assert_eq!(total, 6);
}

#[test]
fn test_iterator_adaptor() {
    let v1 = vec![1, 2, 3];
    // .iter() immutable borrow
    let plus_one: Vec<_> = v1.iter().map(|x| x+1).collect();
    assert_eq!(vec![2, 3, 4], plus_one);
}

mod test_iterator_adaptor {
    // use super::*;

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    // signature Vec<Shoe> means ownership move
    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // .into_iter() moves as well, so .filter()'s argument `s` moves as well 
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

mod test_custom_iterator {
    // use super::*;

    struct Counter {
        count: u32,
    }
    // implement constructor
    impl Counter {
        fn new() -> Counter {
            Counter {
                count: 0
            }
        }
    }
    // implement Iterator trait
    impl Iterator for Counter {
        type Item = u32;

        // Self s in capital
        fn next(&mut self) -> Option<u32> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    #[test]
    fn test_custom_iterator_counter() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
}

