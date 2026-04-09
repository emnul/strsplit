# StrSplit: Re-implementing `str::split`

A minimal, zero-allocation reimplementation of Rust’s `str::split`, built to understand **lifetimes, iterators, and trait-based design**.

Source Code attributed to Jon Gjengset's:
[Crust of Rust: Lifetime Annotations](https://www.youtube.com/watch?v=rAl-9HwD858&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3)
---

## Highlights

* **Zero allocations**: returns `&str` slices into the original input
* **Generic delimiter** via trait (`&str` and `char` supported)
* **Iterator-based** API (`impl Iterator`)

---

## Core API

```rust
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}
```

```rust
impl<'h, D: Delimiter> Iterator for StrSplit<'h, D> {
    type Item = &'h str;
    fn next(&mut self) -> Option<Self::Item> { /* ... */ }
}
```

---

## Key Learnings

* **Lifetimes**: Multiple lifetimes are often unnecessary, but can be useful in certain situations. Defining good Traits is often a more flexible solution.
* **No heap fallback**: using `String` for the delimiter “fixes” lifetimes but breaks `no_std` and adds allocations - suboptimal solution.
* **Slicing**: `&str` slicing is zero-cost but requires correct byte indices.

---

## Examples

```rust
let it = StrSplit::new("a b c", " ");
assert!(it.eq(["a", "b", "c"].into_iter()));
```

```rust
let v: Vec<_> = StrSplit::new("a b c ", " ").collect();
assert_eq!(v, vec!["a", "b", "c", ""]);
```

```rust
pub fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c).next().unwrap()
}
```

---

## Edge Cases

* Trailing delimiter → yields `""`
* No delimiter → yields the entire string once

---

## Why This Matters

This small project forces you to reason about:

* borrowing vs ownership
* lifetime boundaries
* iterator state machines
* zero-cost abstractions

---

## Next

* `split_once`
* predicate-based splitting (`Fn(char) -> bool`)
* benchmarks vs `std`
* `no_std` support

