// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// Runs the `runtime::main` attribute macro.
pub fn main(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  // Extract function item information.

  let syn::ItemFn { attrs, vis, sig, block } = syn::parse_macro_input!(item as syn::ItemFn);
  let name = &sig.ident;
  let output = &sig.output;

  // Require an async function.

  if sig.asyncness.is_none() {
    return syn::Error::new_spanned(sig.fn_token, "An indigo::main function must be async.")
      .to_compile_error()
      .into();
  }

  // Create a token stream for cli argument parsing.

  let (cli_args, cli_parse) = match sig.inputs.len() {
    0 => (quote! {}, quote! {}),
    1 => (quote! { cli_args }, quote! { let cli_args = indigo::cli::StructOpt::from_args(); }),

    _ => {
      return syn::Error::new_spanned(
        sig.inputs,
        "An indigo::main function cannot have more than one parameter.",
      )
      .to_compile_error()
      .into()
    }
  };

  // Generate code to print errors.

  let wrap_result = match output {
    syn::ReturnType::Default => quote! { Ok(result) },
    _ => quote! { Ok(result?) },
  };

  // Generate the output.

  let mut init = TokenStream::new();

  #[cfg(feature = "dotenv")]
  init.extend(quote! {
    indigo::env::load_dotenv();
  });

  #[cfg(feature = "logger")]
  init.extend(quote! {
    indigo::log::init!();
  });

  let result = quote! {
    #vis fn main() {
      #(#attrs)*
      #sig #block

      #init

      #cli_parse

      indigo::runtime::run(async {
        let result = #name(#cli_args).await;

        #wrap_result
      })
    }
  };

  result.into()
}
