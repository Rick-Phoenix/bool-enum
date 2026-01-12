Proc macro which generates an enum with Yes/No variants and a `Deref<bool>` impl (as well as From/Into `bool`, and the usual `Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display`), along with some utility methods and a trait that uses them, to enable usage with generics.

## Usage

Call the macro with visibility (optional) and ident.

```rust
use bool_enum::{bool_enum, BooleanEnum};

bool_enum!(NoVis);
bool_enum!(pub(crate) PubCrate);
bool_enum!(pub Public);

fn main() {
    let mut bool = Public::No;
    if *bool {
      panic!()
    }
    if !*(bool.inverted()) {
      panic!()
    }

    bool.invert();
    if !*bool {
      panic!()
    }
}
```
