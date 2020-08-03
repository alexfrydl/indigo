// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A IDN token.
#[derive(Clone, Debug, From)]
pub enum Token {
  Delimiter(Delimiter),
  Number(Number),
  StringLiteral(StringLiteral),
  Symbol(Symbol),
  Word(Word),
}

impl Token {
  /// Returns the span containing the token.
  pub fn span(&self) -> Span {
    match self {
      Self::Delimiter(d) => d.span(),
      Self::Number(n) => n.span(),
      Self::StringLiteral(s) => s.span(),
      Self::Symbol(d) => d.span(),
      Self::Word(w) => w.span(),
    }
  }
}

/// A wrapper that displays a description of a token.
pub struct DescribeToken<'a>(pub &'a Token);

// Implement `Display` to format tokens for use in human-readable messages.

impl<'a> Display for DescribeToken<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.0 {
      Token::Delimiter(d) => write!(f, "{}", fmt::AsDescription(d.as_char())),
      Token::Number(Number::Float(_)) => write!(f, "floating-point number"),
      Token::Number(Number::Integer(_)) => write!(f, "integer"),
      Token::StringLiteral(_) => write!(f, "string"),
      Token::Symbol(s) => write!(f, "{}", fmt::AsDescription(s.as_char())),
      Token::Word(w) => write!(f, "`{}`", w.as_str()),
    }
  }
}
