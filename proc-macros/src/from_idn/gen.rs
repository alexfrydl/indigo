// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Generate an `impl FromIdn` for a normal struct.
pub(super) fn impl_for_normal_struct(spec: &spec::Struct) -> TokenStream {
  // Create a token stream containing a list of field assignments.

  let mut field_assignments = TokenStream::new();

  for spec::Field { ident, variable, .. } in &spec.fields {
    field_assignments.append_all(quote! { #ident: #variable, });
  }

  // Generate the impl.

  let spec::Struct { ident, .. } = &spec;
  let read_fields = read_fields(spec);

  quote! {
    impl FromIdn for #ident {
      fn from_idn(reader: &mut idn::Reader) -> idn::Result<Self> {
        #read_fields

        Ok(Self { #field_assignments })
      }
    }
  }
}

/// Generate an `impl FromIdn` for a tuple struct.
pub(super) fn impl_for_tuple_struct(spec: &spec::Struct) -> TokenStream {
  // Create a token stream containing a list of field variables.

  let mut field_variables = TokenStream::new();

  for spec::Field { variable, .. } in &spec.fields {
    field_variables.append_all(quote! { #variable, });
  }

  // Generate the impl.

  let spec::Struct { ident, .. } = &spec;
  let read_fields = read_fields(spec);

  quote! {
    impl FromIdn for #ident {
      fn from_idn(reader: &mut idn::Reader) -> idn::Result<Self> {
        #read_fields

        Ok(Self(#field_variables))
      }
    }
  }
}

/// Generate an `impl FromIdn` for a unit struct.
pub(super) fn impl_for_unit_struct(spec: &spec::Struct) -> TokenStream {
  let spec::Struct { ident, .. } = &spec;

  quote! {
    impl FromIdn for #ident {
      fn from_idn(reader: &mut idn::Reader) -> idn::Result<Self> {
        reader.finish();

        Ok(Self)
      }
    }
  }
}

/// Generate code to read the fields of a struct.
fn read_fields(spec: &spec::Struct) -> TokenStream {
  let mut output = TokenStream::new();

  // Output code to read prefix fields.

  let mut has_prefix = false;

  for spec::Field { kind, ty, variable, .. } in &spec.fields {
    if *kind != spec::FieldKind::Prefix {
      continue;
    }

    has_prefix = true;

    output.append_all(quote! {
      let #variable: #ty = reader.read()?;
    });
  }

  // Output `let` statements for the remaining feldss.

  for field in &spec.fields {
    let spec::Field { kind, variable, ty, .. } = field;

    output.append_all(match kind {
      spec::FieldKind::ItemList | spec::FieldKind::ItemAll => quote! {
        let mut #variable: #ty = Default::default();
      },

      spec::FieldKind::Prefix => {
        continue;
      }

      _ => quote! {
        let mut #variable: Option<#ty> = None;
      },
    });
  }

  // Output reader code for the specific parsing style.

  let read_fields = match spec.style {
    spec::Style::Block => read_fields_from_block(spec),
    spec::Style::Sequence => read_fields_from_sequence(spec),
    spec::Style::Tuple => read_fields_from_tuple(spec),
  };

  output.append_all(match has_prefix {
    false => read_fields,

    true => quote! {
      if !reader.is_empty() {
        #read_fields
      }
    },
  });

  // Output code to unwrap all field variables, using defaults where possible.

  for spec::Field { default, kind, variable, .. } in &spec.fields {
    if kind.is_prefix() || kind.occurs_multiple_times() {
      continue;
    }

    output.append_all(match default {
      Some(default) => quote! {
        let #variable = #variable.unwrap_or_else(|| #default);
      },

      None => quote! {
        let #variable = #variable.unwrap();
      },
    });
  }

  // Add an error for any excess elements.

  output.append_all(quote! {
    reader.finish();
  });

  // Return the output token stream.

  output
}

/// Generates code to read fields from a declaration block.
fn read_fields_from_block(spec: &spec::Struct) -> TokenStream {
  let mut output = TokenStream::new();
  let spec::Struct { desc: struct_desc, .. } = spec;

  // Create a token stream containing match arms for each field by name.

  let mut match_arms = TokenStream::new();

  for spec::Field { desc, from, kind, name, variable, .. } in &spec.fields {
    let read = match from {
      Some(from) => quote! { reader.read_to_end::<#from>().map(From::from) },
      None => quote! { reader.read_to_end() },
    };

    let tokens = match kind {
      spec::FieldKind::Item => quote! {
        Some(#name) => {
          reader.skip();

          match #variable.is_none() {
            true => #variable = Some(#read?),
            false => reader.add_error(idn::err!(key.span(), "Duplicate {} item in {}.", #desc, #struct_desc))
          }
        }
      },

      spec::FieldKind::ItemList => quote! {
        Some(#name) => {
          reader.skip();

          match #read {
            Ok(x) => #variable.push(x),
            Err(err) => reader.add_error(err),
          }
        }
      },

      spec::FieldKind::Property => quote! {
        Some(#name) => {
          reader.skip();
          reader.read_symbol("=")?;

          match #variable.is_none() {
            true => #variable = Some(#read?),
            false => reader.add_error(idn::err!(key.span(), "Duplicate {} property in {}.", #desc, #struct_desc))
          }
        }
      },

      _ => continue,
    };

    match_arms.append_all(tokens);
  }

  // Add a match arm for the default case.

  let mut item_all_fields = spec.fields.iter().filter(|f| f.kind == spec::FieldKind::ItemAll);

  if let Some(spec::Field { variable, .. }) = item_all_fields.next() {
    match_arms.append_all(quote! {
      _ => {
        #variable.push(reader.read_to_end()?);
      },
    });
  } else {
    match_arms.append_all(quote! {
      Some(other) => {
        reader.skip();

        let kind = match reader.try_read_symbol("=") {
          Some(_) => "property",
          None => "item",
        };

        reader.add_error(idn::err!(
          key.span(),
          "Unknown {} `{}` in {}.",
          kind,
          other.escape_debug(),
          #struct_desc,
        ));
      },

      _ => {
        reader.finish();
      }
    });
  }

  for spec::Field { ident, .. } in item_all_fields {
    emit_error!(ident.span(), "Only one field may be marked `item *`.")
  }

  // Output code to read all fields.

  output.append_all(quote! {
    let mut group = reader.read_group('{')?;
    let mut list = group.contents.read_list();

    while let Some(mut reader) = list.next() {
      let key = reader.peek_str();

      match key.as_str() {
        #match_arms
      }
    }
  });

  // Output code to check for missing fields.

  output.append_all(quote! {
    let mut missing = Vec::<&'static str>::new();
  });

  for spec::Field { default, desc, kind, variable, .. } in &spec.fields {
    if default.is_some() || kind.is_prefix() || kind.occurs_multiple_times() {
      continue;
    }

    output.append_all(quote! {
      if #variable.is_none() {
        missing.push(#desc);
      }
    });
  }

  output.append_all(quote! {
    match missing.len() {
      0 => {}
      1 => idn::abort!(group.close.span(), "Missing {} in {}.", missing[0], #struct_desc),
      _ => idn::abort!(group.close.span(), "Missing in {}: {}.", #struct_desc, missing.join(", ")),
    }
  });

  output
}

/// Generates code to read fields from a sequence of values.
fn read_fields_from_sequence(spec: &spec::Struct) -> TokenStream {
  let mut output = TokenStream::new();

  // Generate code to read each field consecutively.

  for spec::Field { default, kind, variable, .. } in &spec.fields {
    if kind.is_prefix() {
      continue;
    }

    output.append_all(match default {
      None => quote! { #variable = Some(reader.read()?); },

      Some(_) => quote! {
        if !reader.is_empty() {
          #variable = Some(reader.read()?);
        }
      },
    });
  }

  output
}

/// Generates code to read fields from a tuple of values.
fn read_fields_from_tuple(spec: &spec::Struct) -> TokenStream {
  let mut output = TokenStream::new();

  // Output code to read a group containing a list.

  output.append_all(quote! {
    let mut group = reader.read_group('(')?;
    let mut list = group.contents.read_list();
  });

  // Generate code to read each field consecutively.

  let required_count =
    spec.fields.iter().filter(|f| !f.kind.is_prefix() && f.default.is_none()).count();

  let expected_field = format!(
    "Expected {}{} element{} in {}.",
    match required_count < spec.fields.len() {
      true => "at least ",
      false => "",
    },
    required_count,
    match required_count {
      1 => "",
      _ => "s",
    },
    spec.desc.value(),
  );

  for spec::Field { default, variable, .. } in &spec.fields {
    output.append_all(match default {
      Some(default) => quote! {
        let #variable = match list.read_next()? {
          Some(x) => x,
          None => #default,
        };
      },

      None => quote! {
        let #variable = match list.read_next()? {
          Some(x) => x,
          None => idn::abort!(list.span(), #expected_field),
        };
      },
    });
  }

  // Generate a `finish` call to error on excess elements.

  output.append_all(quote! {
    list.finish();
  });

  output
}
