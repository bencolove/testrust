# Set
std::collections::HashSet;
operations:
1. membership(test)
2. modification(add/remove)
3. union
4. difference
5. intersection
6. symmetric_difference

---
```rust
use std::collections::HashSet;

// setup
let mut sa: HashSet<i32> = vec![1,2,3].into_iter().collect();
let mut sb: HashSet<i32> = vec![3,4,5].into_iter().collect();

// HashSet::insert(VALUE) -> bool
// true indicating success if not existed before
// false means fail if present 
sa.insert(4);

// HashSet::contains(&ref) -> bool
sa.contains(&5);

// all members are refs
sa.union(&sb).collect::<Vec<&i32>>();

sa.intersection(&b).collect::<Vec<&i32>>();

sa.difference(&b).collect::<Vec<&i32>>();

sa.symmetric_difference(&b).collect::<Vec<&i32>>();
```
