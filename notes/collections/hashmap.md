# std::collections::HashMap

---
## KEY
A key can be any type that implements `Eq` and `Hash`:
1. `bool`
2. `int`, `uint` and their variations
3. `String` and `&str`
4. collections (container) with Keyable contained type
5. custom type with `#[derive(PartialEq, Eq, Hash)]`
6. `float` like `f32` and `f64` are **NOT**
```rust
// setup
let mut contacts = HashMap::new();

contacts.insert("Daniel", "798-1364");
contacts.insert("Ashley", "645-7689");
contacts.insert("Katie", "435-8291");
contacts.insert("Robert", "956-1745");

// `Denial` is of type &str, so `&"Denial""` is a reference to `&str`
// HashMap.get(&KEY) -> Option<&VALUE>
contacts.get(&"Denial")
match contacts.get(&"Daniel") {
    Some(&number) => println!("Calling Daniel: {}", call(number)),
    _ => println!("Don't have Daniel's number."),
}

// HashMap.insert(KEY, VALUE) -> Option<Value>
// if new key, Some(Value)
// if existed, None
contacts.insert("Daniel", "164-6743");

// HashMap.remove(&KEY)
contacts.remove(&"Daniel");

// loop
// HashMap.iter()/iter_mut() yields (&'a KEY, &'a VALUE) pairs
// HashMap.into_iter() yields (KEY, VALUE) prosumably
for (KEY, &VALUE) in contacts.iter() {

}
```

