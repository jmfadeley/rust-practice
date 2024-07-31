From https://doc.rust-lang.org/rust-by-example/attribute.html

An attribute is metadta applied to some module, crate or item. Used for:
- Conditional compilation of code
- Set crate name, version and type (binary or library)
- Disable lint
- Enable compiler features (macros, glob imports)
- Link to a foreign library
- Mark functions as unit tests
- Mark functions that will be part of a benchmark
- Attributes like macros

Attributes look like #[outer_attribute] or #![inner_attribute], with the difference between them being where they apply.

#[outer_attribute] applies to the item immediately following it like a function, module declaration, constant, structure,
or enum. IE:
```
  #[derive(Debug)]
  struct Rectangle {
    width: u32,
    height: u32,
  }
```

#![inner_attribute] applies to the enclosing item (usually a module or a crate). In other words, it's applying to the whole
scope. Like `#![allow(unused_variables)]` would apply to the whole create if used over `main.rs`:
```
#![allow(unused_variables)]

fn main() {
  let x = 3; // This would normally warn.
}
```

Attributes can take arguments in different syntaxes:
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]

They can also hae multiple values like so:
#[attribute(value, value2)]
#[attribute(value, value2, value3, ..., value5)]
