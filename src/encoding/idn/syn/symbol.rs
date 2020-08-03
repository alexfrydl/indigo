// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A IDN symbol character.
#[derive(Debug, Clone, Copy)]
pub struct Symbol {
  span: Span,
  value: char,
}

impl Symbol {
  /// Constructs a new IDN symbol.
  pub fn new(span: Span, value: char) -> Self {
    Self { span, value }
  }

  /// Returns the symbol character.
  pub fn as_char(&self) -> char {
    self.value
  }

  /// Returns the span containing the symbol.
  pub fn span(&self) -> Span {
    self.span
  }
}

impl Reader {
  /// Read one of a set of expected symbol characters.
  pub fn read_symbol(&mut self, expected: &str) -> Result<Symbol> {
    match self.try_read::<Element>() {
      Some(Element::Symbol(sym)) if expected.contains(sym.as_char()) => Ok(sym),

      Some(other) => abort!(
        other.span(),
        "Expected {}`{}`, found {}.",
        match expected.len() {
          1 => "",
          _ => "one of ",
        },
        expected.escape_debug(),
        DescribeElement(&other)
      ),

      None => abort!(
        self.span(),
        "Expected {}`{}`.",
        match expected.len() {
          1 => "",
          _ => "one of ",
        },
        expected.escape_debug(),
      ),
    }
  }

  /// Tries to read one of a set of symbol characters or returns `None`.
  pub fn try_read_symbol(&mut self, expected: &str) -> Option<Symbol> {
    let token = self.tokens().peek();

    if !matches!(token, Some(Token::Symbol(c)) if expected.contains(c.as_char())) {
      return None;
    }

    Some(self.read().expect("Unexpected read error"))
  }
}
