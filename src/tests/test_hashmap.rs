#[cfg(test)]
mod hashmap {
    // `extern crare std;` by compiler at the beginning of crate root
    // when work with child mod, use `use` here instead of outer scope
    use std::collections::HashMap;
    use std::string::String;

    fn setup() -> HashMap<String, String> {
        let mut book_reviews = HashMap::new();

        // Review some books.
        book_reviews.insert(
            "Adventures of Huckleberry Finn".to_string(),
            "My favorite book.".to_string(),
        );
        book_reviews.insert(
            "Grimms' Fairy Tales".to_string(),
            "Masterpiece.".to_string(),
        );
        book_reviews.insert(
            "Pride and Prejudice".to_string(),
            "Very enjoyable.".to_string(),
        );
        book_reviews.insert(
            "The Adventures of Sherlock Holmes".to_string(),
            "Eye lyked it alot.".to_string(),
        );

        // transfer ownship, does copy happens here?
        return book_reviews;
    }

    #[test]
    fn test_access() {
        // Type inference lets us omit an explicit type signature (which
        // would be `HashMap<String, String>` in this example).
        let mut book_reviews = HashMap::new();

        // Review some books.
        book_reviews.insert(
            "Adventures of Huckleberry Finn".to_string(),
            "My favorite book.".to_string(),
        );
        book_reviews.insert(
            "Grimms' Fairy Tales".to_string(),
            "Masterpiece.".to_string(),
        );
        book_reviews.insert(
            "Pride and Prejudice".to_string(),
            "Very enjoyable.".to_string(),
        );
        book_reviews.insert(
            "The Adventures of Sherlock Holmes".to_string(),
            "Eye lyked it alot.".to_string(),
        );

        assert_eq!(book_reviews.contains_key("Les Misérables"), false);
        // panic otherwise
        assert_eq!(book_reviews["The Adventures of Sherlock Holmes"], "Eye lyked it alot.");
        // get(&T) -> options<&T>
        // string literals are &str and is different from String, same lieterals are refered to same address
        assert!(book_reviews.get("Pride and Prejudice") == Some(&"Very enjoyable.".to_string()));
        // auto deref
        assert!(book_reviews.get("Pride and Prejudice") == Some(&(*"Very enjoyable.").to_string()));
        // test not found
        assert!(book_reviews.get("No such book") == None); 
    }

    #[test]
    fn test_loop() {
        let book_reviews = setup();
        // loop with (key,value) from &ref
        for (book, review) in &book_reviews {
            println!("{}: {}", book, review);
        }
        // assert on options ?
        // assert_eq!(book_reviews.get(&"The Adventures of Sherlock Holmes"), Some(*)
    }
}