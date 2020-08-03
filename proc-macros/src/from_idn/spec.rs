// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Spec for generating struct-related code.
pub(super) struct Struct {
  pub desc: syn::LitStr,
  pub ident: syn::Ident,
  pub fields: Vec<Field>,
  pub style: Style,
}

/// Spec for generating field-related code.
pub(super) struct Field {
  pub default: Option<TokenStream>,
  pub desc: String,
  pub from: Option<syn::Type>,
  pub ident: Option<syn::Ident>,
  pub kind: FieldKind,
  pub name: String,
  pub variable: syn::Ident,
  pub ty: syn::Type,
}

/// The kind of field code to generate.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum FieldKind {
  Prefix,
  Property,
  Item,
  ItemList,
  ItemAll,
}

/// An overall “style” of struct parsing.
#[derive(Display, Eq, PartialEq)]
pub(super) enum Style {
  /// Parses fields from a list of _properties_, which are key-value pairs
  /// separated by `=`, and _items_, which are arbitrary elements (typically
  /// named blocks).
  #[display(fmt = "block")]
  Block,
  /// Parses fields as a simple sequence of consecutive values.
  #[display(fmt = "sequence")]
  Sequence,
  /// Parses fields from a tuple of values.
  #[display(fmt = "tuple")]
  Tuple,
}

impl FieldKind {
  /// Returns `true` if this is `Self::Prefix`.
  pub(super) fn is_prefix(&self) -> bool {
    *self == Self::Prefix
  }

  /// Returns `true` if this field may occur multiple times in IDN.
  ///
  /// Fields marked `items` or `items *` return `true`.
  pub(super) fn occurs_multiple_times(&self) -> bool {
    match self {
      Self::ItemList | Self::ItemAll => true,
      _ => false,
    }
  }
}

// Implement `FromStr` to parse style names.

impl FromStr for Style {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "block" => Ok(Self::Block),
      "sequence" => Ok(Self::Sequence),
      "tuple" => Ok(Self::Tuple),
      _ => Err(()),
    }
  }
}

// Implement `Default` to make `Property` the default kind of field.

impl Default for FieldKind {
  fn default() -> Self {
    Self::Property
  }
}
