// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

#[derive(Default)]
struct StructOptions {
  desc: Option<syn::LitStr>,
  style: Option<spec::Style>,
}

#[derive(Default)]
struct FieldOptions {
  default: Option<TokenStream>,
  from: Option<syn::Type>,
  kind: spec::FieldKind,
  rename: Option<String>,
}

// Implement `TryFrom` to convert struct items to struct specs.

impl TryFrom<syn::ItemStruct> for spec::Struct {
  type Error = parse::Error;

  fn try_from(item: syn::ItemStruct) -> parse::Result<Self> {
    let options: StructOptions = attr::parse(item.attrs)?;

    let ident = item.ident;

    let desc = match options.desc {
      Some(d) => d,
      None => syn::LitStr::new(
        &format!("{}", ident.to_string().to_sentence_case().to_lowercase()),
        ident.span(),
      ),
    };

    let style = options.style.unwrap_or(match &item.fields {
      syn::Fields::Named(_) => spec::Style::Block,
      syn::Fields::Unnamed(f) if f.unnamed.len() != 1 => spec::Style::Tuple,
      _ => spec::Style::Sequence,
    });

    let mut fields = Vec::new();

    for field in item.fields.into_iter().enumerate() {
      match spec::Field::try_from(field) {
        Ok(field) => fields.push(field),
        Err(err) => emit_error!(err.span(), err),
      }
    }

    Ok(Self { desc, ident, fields, style })
  }
}

// Implement `TryFrom` to convert fields into field specs.

impl TryFrom<(usize, syn::Field)> for spec::Field {
  type Error = parse::Error;

  fn try_from((index, field): (usize, syn::Field)) -> parse::Result<Self> {
    let span = field.ident.as_ref().map(|i| i.span()).unwrap_or_else(|| field.span());
    let syn::Field { attrs, ident, ty, .. } = field;
    let FieldOptions { default, from, kind, rename } = attr::parse(attrs)?;

    let name = match rename {
      Some(name) => name,
      None => {
        let mut name = ident.as_ref().map(ToString::to_string).unwrap_or_else(|| index.to_string());

        if kind.occurs_multiple_times() {
          name = name.to_singular();
        }

        name
      }
    };

    let desc = format!("`{}`", name);

    let variable = match &ident {
      Some(ident) => syn::Ident::new(&format!("_{}", ident), ident.span()),
      None => syn::Ident::new(&format!("_{}", index), Span::call_site()),
    };

    if kind == spec::FieldKind::Prefix && default.is_some() {
      emit_error!(span, "Prefix fields cannot have default values.");
    }

    Ok(Self { default, desc, from, ident, kind, name, ty, variable })
  }
}

// Implement `Parse` to parse struct options from attribute arguments.

impl Parse for StructOptions {
  fn parse(input: ParseStream) -> parse::Result<Self> {
    let mut options = Self::default();

    attr::parse_args(input, |ident, input| {
      let name = ident.to_string();

      match name.as_str() {
        "desc" => {
          input.parse::<Token![=]>()?;

          options.desc = Some(input.parse()?);
        }

        "block" | "sequence" | "tuple" => {
          let style = name.parse().unwrap();

          match &options.style {
            None => options.style = Some(style),

            Some(s) if *s == style => emit_warning!(ident.span(), "Duplicate `{}` option.", style),

            Some(other) => emit_error!(
              ident.span(),
              "The `{}` option is exclusive with the `{}` option.",
              name,
              other
            ),
          }
        }

        _ => emit_error!(ident.span(), "Unknown option `{}`.", ident),
      }

      Ok(())
    })?;

    Ok(options)
  }
}

// Implement `Parse` to parse field options from attribute arguments.

impl Parse for FieldOptions {
  fn parse(input: ParseStream) -> parse::Result<Self> {
    let mut options = Self::default();

    attr::parse_args(input, |ident, input| {
      let name = ident.to_string();

      match name.as_str() {
        "default" => {
          options.default = Some(match input.is_empty() {
            true => quote! { Default::default() },

            false => {
              input.parse::<Token![=]>()?;
              input.parse::<syn::Expr>()?.to_token_stream()
            }
          });
        }

        "from" => {
          input.parse::<Token![=]>()?;

          options.from = Some(input.parse()?);
        }

        "item" => options.kind = spec::FieldKind::Item,

        "items" => {
          options.kind = match input.peek(Token![*]) {
            false => spec::FieldKind::ItemList,

            true => {
              input.parse::<Token![*]>()?;

              spec::FieldKind::ItemAll
            }
          };
        }

        "name" => {
          input.parse::<Token![=]>()?;

          options.rename = Some(match input.peek(syn::LitStr) {
            true => input.parse::<syn::LitStr>()?.value(),
            false => input.parse::<syn::Ident>()?.to_string(),
          });
        }

        "prefix" => options.kind = spec::FieldKind::Prefix,

        _ => emit_error!(ident.span(), "Unknown option `{}`.", ident),
      }

      Ok(())
    })?;

    Ok(options)
  }
}
