// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! IDN, a general purpose data serialization format.

pub mod ctx;
mod error;
mod from_idn;
pub mod lex;
pub mod reader;
mod span;
pub mod syn;

#[doc(inline)]
pub use self::ctx::Context;
pub use self::error::{abort, err, Error, ErrorList, Result};
pub use self::from_idn::{FromIdn, TryFromIdn};
#[doc(inline)]
pub use self::lex::lex;
#[doc(inline)]
pub use self::reader::Reader;
pub use self::span::{Pos, Span, Spanned};
pub use self::syn::{Token, Tokens};

use super::idn;
use crate::prelude::*;

/// Parses a value of type `T` from a IDN string.
pub fn parse<T: FromIdn>(input: impl AsRef<str>) -> Result<T, ErrorList> {
  let mut reader = Reader::new(input.as_ref().parse()?);
  let result: Result<T> = reader.read_to_end();
  let mut errors = reader.errors().clone();

  match result {
    Ok(_) if errors.len() > 0 => Err(errors),

    Ok(value) => Ok(value),

    Err(err) => {
      errors.add(err);

      Err(errors)
    }
  }
}

// Integration tests.

#[cfg(test)]
mod tests {
  use super::*;

  /// Parse a `Vec<String>`.
  #[test]
  fn test_parse_vec_string() -> Result<(), ErrorList> {
    let result: Vec<String> = parse(
      r#"
        ["hello", "world",
        "multi",
          'line'
        "test",
        ]
      "#,
    )?;

    for (i, expected) in ["hello", "world", "multi", "line", "test"].iter().enumerate() {
      assert_eq!(result[i], *expected);
    }

    Ok(())
  }

  /// Parse a tuple.
  #[test]
  fn test_parse_tuple() -> Result<(), ErrorList> {
    let result: (bool, Option<f32>, ()) = parse(
      r#"
        (true
        37.5, ())
      "#,
    )?;

    assert_eq!(result, (true, Some(37.5), ()));

    Ok(())
  }

  /// Parse a map.
  #[test]
  fn test_parse_map() -> Result<(), ErrorList> {
    let result: HashMap<String, i64> = parse(
      r#"
        {
          a = 10
          "quoted" = - 15
          last_with_comma = +2,
        }
      "#,
    )?;

    let mut expected = HashMap::new();

    expected.insert("a".to_owned(), 10);
    expected.insert("quoted".to_owned(), -15);
    expected.insert("last_with_comma".to_owned(), 2);

    assert_eq!(result, expected);

    Ok(())
  }
}
