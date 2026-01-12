use bool_enum::{BooleanEnum, bool_enum};

bool_enum!(NoVis);
bool_enum!(pub(crate) PubCrate);
bool_enum!(pub Public);

mod pub_super {
  use super::*;

  bool_enum!(pub(super) PubSuper);
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
            panic!("standard deref failed")
          }

          if !*(bool.inverted()) {
            panic!("inverted call failed")
          }

          bool.invert();

          if !*bool {
            panic!("invert call failed")
          }
        }
      )*
    }
  };
}

test_bool_enum!(NoVis, PubCrate, PubSuper, Public);
