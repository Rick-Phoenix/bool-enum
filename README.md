Proc macro which generates an enum with Yes/No variants and a `Deref<bool>` impl (as well as From/Into `bool`, and the usual `Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display`).

## Usage

Call the macro with visibility (optional) and ident.

```rust
use bool_enum::boolean_enum;

boolean_enum!(NoVis);
boolean_enum!(pub(crate) PubCrate);
boolean_enum!(pub Public);

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
