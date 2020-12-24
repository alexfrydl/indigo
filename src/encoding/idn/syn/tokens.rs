// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

use std::ptr;

/// A sequence of IDN tokens.
#[derive(Clone)]
pub struct Tokens {
  span: Span,
  list: Arc<[Token]>,
  range: Range<usize>,
}

impl Tokens {
  /// Constructs a new IDN token sequence.
  pub(crate) fn new(span: impl Into<Span>, list: impl Into<Arc<[Token]>>) -> Self {
    let list = list.into();
    let mut tokens = Self { span: span.into(), range: 0..list.len(), list };

    tokens.update_span();
    tokens
  }

  /// Returns `true` if the token sequence is empty.
  pub fn is_empty(&self) -> bool {
    self.list().is_empty()
  }

  /// Returns a reference to the tokens in the sequence.
  pub fn list(&self) -> &[Token] {
    &self.list[self.range.clone()]
  }

  /// Removes and returns the next spanned token or `None` if no tokens remain.
  pub fn next(&mut self) -> Option<Token> {
    let token = self.peek()?.clone();

    self.range.start += 1;
    self.update_span();

    Some(token.clone())
  }

  /// Returns the next token in the sequence.
  pub fn peek(&self) -> Option<&Token> {
    self.list().first()
  }

  /// Returns the span of the tokens in the sequence.
  pub fn span(&self) -> Span {
    self.span
  }

  /// Removes the last token of the sequence.
  pub(crate) fn pop(&mut self) {
    self.range.end =
      cmp::min(self.range.end, cmp::max(self.range.start, self.range.end.saturating_sub(1)));
    self.update_span()
  }

  /// Truncates this sequence so that it ends before the given sequence.
  pub(crate) fn truncate_before(&mut self, seq: &Self) {
    if !ptr::eq(&*self.list, &*seq.list) {
      return;
    }

    self.range.end = cmp::min(self.range.end, cmp::max(self.range.start, seq.range.start));
    self.update_span();
  }

  /// Updates the span based on the remaining tokens.
  fn update_span(&mut self) {
    self.span = match self.list() {
      [] => self.span.start().into(),
      [token] => token.span(),
      [start, .., end] => start.span() + end.span(),
    }
  }
}

// Implement `FromStr` to run the lexer.

impl FromStr for Tokens {
  type Err = ErrorList;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lex(s)
  }
}

// Implement `FromIdn` to take the remaining tokens from a reader.

impl FromIdn for Tokens {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let tokens = reader.tokens().clone();

    while reader.skip() {}

    Ok(tokens)
  }
}

// Format sequences nicely.

impl Debug for Tokens {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Tokens({:?}, ", self.span)?;

    let mut list = f.debug_list();

    for token in self.list() {
      list.entry(token);
    }

    list.finish()?;

    write!(f, ")")
  }
}
