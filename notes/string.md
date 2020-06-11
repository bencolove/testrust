# String 
1. `String`
2. `&str`
3. `&String`, any boxed type

## `String`
`String` structs like:
```rust
struct String {
    pointer: &str,
    len: usize,
    capacity: usize
}
```
`String` is growable, heap-allocated data struct like `Vec` or `StringBuffer` from JAVA.
Used when need to consume/modify (own) it.

`&String` and its boxed variants are nothing but a pointer to it.
By passing `&mut String`, its contents are therefore modifiable.

## `str`
`str` is an **IMMUTABLE** sequence of UTF-8 bytes with dynamic length somewhere in memory.
It structs like any other view types `&[pointer;length]`.

`&str` on the other hand is a view onto it and it reads as a **slice**

A `&str` can reference to:
* static storage(ROM/binary) by
    string literals like `foo` which is `&'static str`
* heap allocated by `String`
    1. `&string[..]`
    2. `std::ops::Deref::deref(&string)` (deref a `&String` will be coerced into a `&str`)
* stack allocated by converting from `Vec<u8>` byte arrays
    ```rust
    use std::str;

    let byte_array: &[u8] = &[b'a', b'b', b'c'];
    let statck_str: &str = str::from_utf8().unwrap();
    ```