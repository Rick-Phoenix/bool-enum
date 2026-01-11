#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::*;
use quote::*;
use syn::{Token, Visibility, parse::ParseStream, parse_macro_input};

struct MacroArgs {
  vis: Visibility,
  ident: Ident,
}

fn parse_args(input: ParseStream) -> syn::Result<MacroArgs> {
  let vis: Visibility;
  let ident: Ident;

  if input.peek(Token![pub]) {
    vis = input.parse()?;

    ident = input.parse()?;
  } else {
    vis = Visibility::Inherited;
    ident = input.parse()?;
  }

  Ok(MacroArgs { vis, ident })
}

#[proc_macro]
pub fn boolean_enum(input: TokenStream) -> TokenStream {
  let args = parse_macro_input!(input with parse_args);

  let MacroArgs { vis, ident } = args;

  quote! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #vis enum #ident {
      Yes,
      No
    }

    impl #ident {
      #[inline]
      pub fn as_bool(&self) -> bool {
        **self
      }

      #[inline]
      pub fn invert(&mut self) {
        *self = match self {
          Self::Yes => Self::No,
          Self::No => Self::Yes,
        }
      }

      #[inline]
      pub fn inverted(mut self) -> Self {
        self.invert();
        self
      }
    }

    impl From<#ident> for bool {
      #[inline]
      fn from(value: #ident) -> Self {
        match value {
          #ident::Yes => true,
          #ident::No => false,
        }
      }
    }

    impl From<bool> for #ident {
      #[inline]
      fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
      }
    }

    impl core::ops::Deref for #ident {
      type Target = bool;

      #[inline]
      fn deref(&self) -> &Self::Target {
        match self {
          #ident::Yes => &true,
          #ident::No => &false,
        }
      }
    }

    impl core::fmt::Display for #ident {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_bool())
      }
    }
  }
  .into()
}
