// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A reader for parsing values from tokens.

use super::*;

/// Reads IDN elements from a sequence of tokens.
pub struct Reader {
  ctx: Context,
  pub(super) tokens: Tokens,
}

impl Reader {
  /// Constructs a new reader from a sequence of tokens.
  pub fn new(tokens: Tokens) -> Self {
    Self { ctx: default(), tokens }
  }

  /// Constructs a new reader with an existing context from a sequence of
  /// tokens.
  pub fn with_context(ctx: impl Into<Context>, tokens: Tokens) -> Self {
    Self { ctx: ctx.into(), tokens }
  }

  /// Adds an error to the error list.
  pub fn add_error(&mut self, err: Error) {
    ErrorList::from_context(&mut self.ctx).add(err);
  }

  /// Returns a reference to the current context.
  pub fn ctx(&mut self) -> &mut Context {
    &mut self.ctx
  }

  /// Returns the list of errors encountered so far.
  pub fn errors(&self) -> ErrorList {
    self.ctx.get::<ErrorList>("errors").unwrap().clone()
  }

  /// Finishes reading and adds a non-fatal error if any tokens remain.
  pub fn finish(&mut self) {
    if let Some(el) = self.try_read::<syn::Element>() {
      self.add_error(err!(el.span(), "Unexpected {}.", syn::DescribeElement(&el)));
    }

    while self.tokens.next().is_some() {}
  }

  /// Returns `true` if no input remains.
  pub fn is_empty(&self) -> bool {
    self.tokens.is_empty()
  }

  /// Peeks ahead to the next element and returns its value if it is a string
  /// literal or a word; otherwise, returns `None`.
  pub fn peek_str(&self) -> Spanned<Option<Arc<str>>> {
    match self.tokens().peek() {
      Some(Token::StringLiteral(s)) => s.span().with_value(Some(s.into())),
      Some(Token::Word(w)) => w.span().with_value(Some(w.into())),
      _ => Spanned::new(self.span().start(), None),
    }
  }

  /// Reads a value of type `T`.
  pub fn read<T: FromIdn>(&mut self) -> Result<T> {
    T::from_idn(self)
  }

  /// Reads a value of type `T` from the remaining tokens.
  ///
  /// This is equivalent to invoking `read()` and then `finish()`.
  pub fn read_to_end<T: FromIdn>(&mut self) -> Result<T> {
    let error_count = self.errors().len();
    let res = T::from_idn(self);

    if self.errors().len() == error_count {
      self.finish();
    }

    res
  }

  /// Returns a reader for the next line of elements or `None` of no input
  /// remains.
  pub fn next_line(&mut self) -> Option<Reader> {
    if self.is_empty() {
      return None;
    }

    let mut tokens = self.tokens.clone();
    let mut el: syn::Element = self.read().expect("unexpected read error");

    while !self.is_empty() {
      let token = self.tokens.peek();

      if matches!(token, Some(t) if t.span().start().line() > el.span().last_line()) {
        break;
      }

      el = self.read().expect("unexpected read error");
    }

    tokens.truncate_before(&self.tokens);

    Some(Reader::with_context(&self.ctx, tokens))
  }

  /// Skips the next element in the input and returns `false` when the input is
  /// empty.
  pub fn skip(&mut self) -> bool {
    self.try_read::<syn::Element>().is_some()
  }

  /// Returns the remaining span of the reader.
  pub fn span(&self) -> Span {
    self.tokens.span()
  }

  /// Returns a reference to the remaining tokens.
  pub fn tokens(&self) -> &Tokens {
    &self.tokens
  }

  /// Tries to read a value of type `T` or returns `None` without consuming
  /// input.
  pub fn try_read<T: TryFromIdn>(&mut self) -> Option<T> {
    T::try_from_idn(self)
  }
}

impl FromIdn for Reader {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let tokens = reader.read_to_end()?;

    Ok(Reader::with_context(&reader.ctx, tokens))
  }
}

impl Debug for Reader {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "idn::Reader")
  }
}
