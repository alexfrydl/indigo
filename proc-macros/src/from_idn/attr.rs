// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Parses the `idn` attribute arguments as a value of type `T`.
pub(super) fn parse<T: Default + Parse>(
  attrs: impl IntoIterator<Item = syn::Attribute>,
) -> parse::Result<T> {
  let ident = syn::Ident::new("idn", Span::call_site());
  let mut attrs = attrs.into_iter().filter(|a| a.path.is_ident(&ident));

  let result = match attrs.next() {
    Some(attr) => syn::parse2(attr.tokens),
    None => Ok(Default::default()),
  };

  for attr in attrs {
    emit_error!(attr.span(), "Duplicate `{}` attribute.", ident);
  }

  result
}

/// Parses each `idn` attribute argument with a function.
pub(super) fn parse_args(
  input: ParseStream,
  mut func: impl FnMut(syn::Ident, ParseStream) -> parse::Result<()>,
) -> parse::Result<()> {
  // If empty, exit immediately.

  if input.is_empty() {
    return Ok(());
  }

  // Parse the entire `(…)` group.

  let input = syn::group::parse_parens(input)?.content;

  // Run `func` once, then parse a `,` and optionally another `func` as long as
  // input remains.

  func(input.parse()?, &input)?;

  while !input.is_empty() {
    input.parse::<Token![,]>()?;

    if input.is_empty() {
      break;
    }

    func(input.parse()?, &input)?;
  }

  Ok(())
}
