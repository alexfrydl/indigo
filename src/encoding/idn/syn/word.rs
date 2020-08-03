// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use unicode_xid::UnicodeXID;

/// A IDN word, which is similar to an identifier or keyword.
#[derive(Debug, Clone)]
pub struct Word {
  span: Span,
  value: Arc<str>,
}

impl Word {
  /// Constructs a new IDN word.
  pub fn new(span: Span, value: impl Into<Arc<str>>) -> Self {
    Self { span, value: value.into() }
  }

  /// Returns `true` if the given character is valid at the start of a word.
  pub fn is_start_char(c: char) -> bool {
    matches!(c, '$' | '@' | '#') || c.is_xid_start()
  }

  /// Returns `true` if the given character is valid in a word.
  pub fn is_continue_char(c: char) -> bool {
    matches!(c, '$' | '@' | '#') || c.is_xid_continue()
  }

  /// Returns the word as a string.
  pub fn as_str(&self) -> &str {
    self.value.as_ref()
  }

  /// Returns the span containing the word.
  pub fn span(&self) -> Span {
    self.span
  }
}

impl Reader {
  /// Reads a specified word.
  pub fn read_word(&mut self, expected: &str) -> Result<Word> {
    match self.try_read::<Element>() {
      None => abort!(self.span(), "Expected `{}`.", expected),
      Some(Element::Word(word)) if word.as_str() == expected => Ok(word),
      Some(other) => {
        abort!(other.span(), "Expected `{}`, found {}.", expected, DescribeElement(&other))
      }
    }
  }

  /// Tries to read a specified word or returns `None`.
  pub fn try_read_word(&mut self, expected: &str) -> Option<Word> {
    let token = self.tokens().peek();

    if !matches!(token, Some(Token::Word(w)) if w.as_str() == expected) {
      return None;
    }

    Some(self.read().expect("Unexpected read error"))
  }
}

// Implement conversion to regular strings.

impl From<Word> for Arc<str> {
  fn from(word: Word) -> Self {
    word.value.clone()
  }
}

impl From<&'_ Word> for Arc<str> {
  fn from(word: &'_ Word) -> Self {
    word.value.clone()
  }
}

impl From<Word> for String {
  fn from(word: Word) -> Self {
    word.as_str().into()
  }
}

impl From<&'_ Word> for String {
  fn from(word: &'_ Word) -> Self {
    word.as_str().into()
  }
}
