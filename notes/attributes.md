# Attributes
1. Compilation conditions
2.
3. disable lint(warns)
4.
5.
6.
7. preprocess

---
## General Usage
`#![crate_wise_attr]` and `#[non_crate_wise_attr]`.
Attributes take forms of:
1. `#[attr = "value"]`
2. `#[attr(key="value")]`
3. `#[attr(v1, v2)]`

---
## Compilation Conditions
1. `#[cfg(cond)]` in attribute declaretions
2. `cfg!(cond)` for boolean expression

```rust
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

fn main() {
    are_you_on_linux();
    
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}
```

Custom conditions may be set by passing `--cfg` for `rustc`:
```rust
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}
```
`$ rustc --cfg some_condition custom.rs && ./custom`

What about using cargo to pass the cfg value???

---
## Disable Lints(Warns)
`#[allow(dead_code)]`

## Preprocess
`#[derive(trait1, trait2)]`
Derivable traits:
1. Comparison ones:
    1. Eq
    2. PartialEq
    3. Ord
    4. PartialOrd
2. Clone (create `T` from `&T`)
3. Copy (statck-limited structs)
4. Hash (hash from `&T`)
5. Default (create an empty instance)
6. Debug (fmt::Debug for `"{:?}"` print marker)