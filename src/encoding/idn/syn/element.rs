// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A IDN syntax element.
#[derive(Debug, From)]
pub enum Element {
  Group(Group),
  Number(Number),
  Symbol(Symbol),
  StringLiteral(StringLiteral),
  Word(Word),
}

/// A wrapper type that displays a description of an element.
pub struct DescribeElement<'a>(pub &'a Element);

impl Element {
  /// Returns the span containing the element.
  pub fn span(&self) -> Span {
    match self {
      Self::Group(g) => g.span(),
      Self::Number(n) => n.span(),
      Self::Symbol(s) => s.span(),
      Self::StringLiteral(s) => s.span(),
      Self::Word(w) => w.span(),
    }
  }
}

// Implement `FromIdn` to parse elements.

impl FromIdn for Element {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    // First try to read a delimited group.

    if let Some(group) = Group::try_from_tokens(&mut reader.tokens)? {
      return Ok(group.into());
    }

    // If it's not a group, read one of the other types of elements.

    match reader.tokens.next() {
      None => abort!(reader.span(), "Expected element."),

      Some(token) => Ok(match token {
        Token::Delimiter(_) => unreachable!("Groups should already be read."),
        Token::Number(number) => Self::Number(number),
        Token::StringLiteral(string) => Element::StringLiteral(string),
        Token::Symbol(symbol) => Element::Symbol(symbol),
        Token::Word(word) => Element::Word(word),
      }),
    }
  }
}

impl TryFromIdn for Element {
  fn try_from_idn(reader: &mut Reader) -> Option<Self> {
    if reader.is_empty() {
      return None;
    }

    Some(reader.read().expect("Unexpected read error"))
  }
}

// Implement `Display` to show a description of an element.

impl Display for DescribeElement<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &self.0 {
      Element::Group(g) => write!(f, "`{}…{}`", g.open.as_char(), g.close.as_char()),
      Element::Number(Number::Float(_)) => write!(f, "floating-point number"),
      Element::Number(Number::Integer(_)) => write!(f, "integer"),
      Element::StringLiteral(_) => write!(f, "string"),
      Element::Symbol(s) => write!(f, "{}", fmt::AsDescription(s.as_char())),
      Element::Word(s) => write!(f, "`{}`", s.as_str()),
    }
  }
}
