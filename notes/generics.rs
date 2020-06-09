# Generics
syposis
```rust
// function
fn foo<T>(arg: T) {}
// explicit call a generic function
fn must_specify_param<T>() -> T {}
must_specify_param::<char>()

// struct
struct Gen<T>(T);
struct Gen2<T> { value: T}
// imple generic struct
impl<T> Gen2<T> {
    ...
}

```
---