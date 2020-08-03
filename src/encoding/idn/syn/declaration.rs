// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A IDN declaration, which is either a _property_, a key/value pair separated
/// by an `=` symbol, or an _item_, a set of elements without an `=` symbol.
#[derive(Debug, From)]
pub enum Declaration {
  Property(Property),
  Item(Reader),
}

/// A IDN property declaration, which is a key/value pair separated by an `=`
/// symbol.
#[derive(Debug)]
pub struct Property {
  pub key: Reader,
  pub eq: Symbol,
  pub value: Reader,
}

// Implement `FromIdn` to parse declarations.

impl FromIdn for Declaration {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    if reader.is_empty() {
      abort!(reader.span(), "Expected declaration.");
    }

    // Look ahead in the token list to disambiguate properties from items.

    let mut lookahead = reader.tokens().clone();

    let is_property = match lookahead.next().unwrap() {
      Token::Number(Number::Integer(_)) | Token::StringLiteral(_) | Token::Word(_) => {
        match lookahead.next() {
          Some(Token::Symbol(s)) if s.as_char() == '=' => true,

          _ => false,
        }
      }

      _ => false,
    };

    // Read the final output.

    match is_property {
      true => reader.read_to_end().map(Self::Property),
      false => reader.read_to_end().map(Self::Item),
    }
  }
}

impl FromIdn for Property {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let mut key_tokens = reader.tokens().clone();

    if reader.try_read::<Element>().is_none() {
      abort!(reader.span(), "Expected property.");
    }

    key_tokens.truncate_before(reader.tokens());

    Ok(Property {
      key: Reader::with_context(reader.ctx(), key_tokens),
      eq: reader.read_symbol("=")?,
      value: reader.read_to_end()?,
    })
  }
}
