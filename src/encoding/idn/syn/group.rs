// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A IDN group element, which is a sequence of tokens surrounded by matching
/// delimiters.
#[derive(Clone, Debug)]
pub struct Group {
  /// The opening delimiter.
  pub open: Delimiter,
  /// The tokens between the delimiters.
  pub contents: Tokens,
  /// The closing delimiter.
  pub close: Delimiter,
}

/// A delimiter for a IDN group element.
#[derive(Clone, Debug)]
pub struct Delimiter {
  span: Span,
  value: char,
}

/// A helper for reading the contents of a group surrounded by matching
/// delimiters.
pub struct GroupReader {
  /// The opening delimiter.
  pub open: Delimiter,
  /// A reader for reading the contents of the group.
  pub contents: Reader,
  /// The closing delimiter.
  pub close: Delimiter,
}

impl Group {
  /// Returns the span containing the entire group, including its delimiters.
  pub fn span(&self) -> Span {
    self.open.span() + self.close.span()
  }

  /// Parses a group from a set of tokens.
  pub(crate) fn try_from_tokens(tokens: &mut Tokens) -> Result<Option<Self>> {
    let open = match tokens.peek() {
      Some(Token::Delimiter(d)) => d.clone(),
      _ => return Ok(None),
    };

    tokens.next();

    // Loop until the group is closed by its matching end delimiter.
    //
    // The lexer only outputs matched delimiters, so this function only counts
    // open groups and ignores the exact delimiter characters.

    let mut opened = 1;
    let mut contents = tokens.clone();

    loop {
      let token = tokens.next().unwrap();

      match token {
        Token::Delimiter(d) if d.is_open() => {
          opened += 1;
        }

        Token::Delimiter(close) if opened == 1 => {
          contents.truncate_before(&tokens);
          contents.pop();

          return Ok(Some(Self { open, contents, close }));
        }

        Token::Delimiter(_) => {
          opened -= 1;
        }

        _ => {}
      }
    }
  }
}

impl Delimiter {
  /// Constructs a delimiter.
  pub fn new(span: Span, value: char) -> Self {
    Self { span, value }
  }

  /// Reverses a delimiter character.
  pub fn rev_char(c: char) -> char {
    match c {
      '(' => ')',
      '[' => ']',
      '{' => '}',

      ')' => '(',
      ']' => '[',
      '}' => '{',

      other => other,
    }
  }

  /// Returns the delimiter character.
  pub fn as_char(&self) -> char {
    self.value
  }

  /// Returns the reverse of the delimiter character.
  pub fn as_rev_char(&self) -> char {
    Self::rev_char(self.value)
  }

  /// Returns true if this delimiter is one of `(`, `[`, or `{`.
  pub fn is_open(&self) -> bool {
    matches!(self.value, '(' | '[' | '{')
  }

  /// Returns the span containing the delimiter.
  pub fn span(&self) -> Span {
    self.span
  }
}

impl GroupReader {
  /// Returns the span containing the group, including delimiters
  pub fn span(&self) -> Span {
    self.open.span + self.close.span
  }
}

impl Reader {
  /// Reads a group with the specified open delimiter.
  pub fn read_group(&mut self, open: char) -> Result<GroupReader, Error> {
    let el: Option<Element> = self.try_read();

    match el {
      Some(Element::Group(group)) if group.open.as_char() == open => Ok(GroupReader {
        open: group.open,
        contents: Reader::with_context(self.ctx(), group.contents),
        close: group.close,
      }),

      Some(el) => abort!(
        el.span(),
        "Expected `{}…{}`, found {}.",
        open,
        Delimiter::rev_char(open),
        DescribeElement(&el)
      ),

      None => abort!(self.span(), "Expected `{}…{}`.", open, Delimiter::rev_char(open)),
    }
  }

  /// Tries to read a group with the specified delimiter or returns `None`.
  pub fn try_read_group(&mut self, start_delimiter: char) -> Option<GroupReader> {
    let token = self.tokens().peek();

    if !matches!(token, Some(Token::Delimiter(d)) if d.as_char() == start_delimiter) {
      return None;
    }

    let group: Group = self.read().expect("Unexpected read error");

    Some(GroupReader {
      open: group.open,
      contents: Reader::with_context(self.ctx(), group.contents),
      close: group.close,
    })
  }
}
