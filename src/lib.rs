#![doc = include_str!("../README.md")]

use core::{
  fmt::{Debug, Display},
  ops::Deref,
};

pub trait BooleanEnum:
  Into<bool> + From<bool> + Default + Copy + Display + Debug + Eq + Ord + Deref<Target = bool>
{
  #[inline]
  fn as_bool(self) -> bool {
    *self
  }

  #[inline]
  fn invert(&mut self) {
    let b: bool = **self;
    *self = (!b).into()
  }

  #[inline]
  fn inverted(mut self) -> Self {
    self.invert();
    self
  }
}

#[doc(hidden)]
#[macro_export]
macro_rules! bool_enum {
  ($vis:vis $ident:ident) => {

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    $vis enum $ident {
      Yes,
      No
    }

    impl $crate::BooleanEnum for $ident {}

    impl Default for $ident {
      #[inline]
      fn default() -> Self {
        Self::No
      }
    }

    impl From<$ident> for bool {
      #[inline]
      fn from(value: $ident) -> Self {
        match value {
          $ident::Yes => true,
          $ident::No => false,
        }
      }
    }

    impl From<bool> for $ident {
      #[inline]
      fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
      }
    }

    impl core::ops::Deref for $ident {
      type Target = bool;

      #[inline]
      fn deref(&self) -> &Self::Target {
        match self {
          $ident::Yes => &true,
          $ident::No => &false,
        }
      }
    }

    impl core::fmt::Display for $ident {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", *self)
      }
    }
  };
}
