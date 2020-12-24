// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod attr;
mod gen;
mod input;
mod spec;

use crate::prelude::*;

/// Generates an `impl FromIdn` for an item.
pub fn impl_for_item(item: syn::Item) -> proc_macro::TokenStream {
  match item {
    syn::Item::Struct(item) => {
      let gen = match item.fields {
        syn::Fields::Named(_) => gen::impl_for_normal_struct,
        syn::Fields::Unit => gen::impl_for_unit_struct,
        syn::Fields::Unnamed(_) => gen::impl_for_tuple_struct,
      };

      let spec: spec::Struct = match item.try_into() {
        Ok(s) => s,
        Err(err) => abort!(err.span(), err),
      };

      gen(&spec).into()
    }

    syn::Item::Enum(_) => unimplemented!(),

    _ => abort!(Span::call_site(), "FromIdn can only be derived on structs."),
  }
}
