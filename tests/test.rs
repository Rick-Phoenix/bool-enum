use yes_or_no::boolean_enum;

boolean_enum!(NoVis);
boolean_enum!(pub(crate) PubCrate);
boolean_enum!(pub Public);

mod pub_super {
  use super::*;

  boolean_enum!(pub(super) PubSuper);
}
use pub_super::*;

macro_rules! test_bool_enum {
  ($($ident:ident),*) => {
    paste::paste! {
      $(
        #[test]
        fn [< $ident:snake >]() {
          let mut bool = $ident::No;

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
      )*
    }
  };
}

test_bool_enum!(NoVis, PubCrate, PubSuper, Public);
