# EnumTools

EnumTools is a Rust procedural macro for making enums easier to use. After adding to `cargo.toml`, add 

```rust
#[macro_use(EnumTools)]
extern crate enum_tools;
```

and extra functionality can be added with 

```rust
#[derive(EnumTools)]
enum MyEnum { /* ... */ }
```

For each variant `A`, the following method will be derived:

```rust
fn is_A(&self) -> bool {/* ... */} // Returns true if self is A
```


In addition to the above, if the variant has data (either a tuple or struct) with types `T1, T2, ...`, the following methods will be derived:

```rust
fn unwrap_A(self) -> (T1, T2, ...) { /* ... */ }
fn unwrap_A_ref(&self) -> (&T1, &T2, ...) { /* ... */ }
```

If the variable is the wrong variant, a panic will be generated. 


