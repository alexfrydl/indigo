// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// A reference to a span of text in a IDN file or source string.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
  start: Pos,
  end: Pos,
}

/// A value with an associated `Span`.
#[derive(Clone, Copy, Deref, DerefMut)]
pub struct Spanned<T> {
  span: Span,
  #[deref]
  #[deref_mut]
  value: T,
}

/// A position in a IDN file or source string.
#[derive(Clone, Copy, Eq, Ord)]
pub struct Pos {
  /// The byte offset.
  offset: usize,
  /// The line number, starting from 1.
  line: usize,
  /// The column number, starting from 1.
  column: usize,
}

impl Span {
  /// Returns the length of the span in bytes.
  pub fn byte_len(&self) -> usize {
    self.byte_range().len()
  }

  /// Returns the byte range of the span.
  pub fn byte_range(&self) -> Range<usize> {
    self.start.offset..self.end.offset
  }

  /// Returns `true` if the span is empty.
  pub fn is_empty(&self) -> bool {
    self.byte_len() == 0
  }

  /// Returns the last line in the span.
  pub fn last_line(&self) -> usize {
    match self.end.column {
      1 => cmp::max(self.end.line - 1, self.start.line),
      _ => self.end.line,
    }
  }

  /// Returns the start position of the span.
  pub fn start(&self) -> Pos {
    self.start
  }

  /// Create a new `Spanned` with the specified value.
  pub fn with_value<T>(&self, value: T) -> Spanned<T> {
    Spanned::new(*self, value)
  }
}

impl<T> Spanned<T> {
  /// Create a new spanned value.
  pub fn new(span: impl Into<Span>, value: T) -> Self {
    Self { span: span.into(), value }
  }

  /// Returns the span containing the value.
  pub fn span(&self) -> Span {
    self.span
  }
}

impl Spanned<Option<Arc<str>>> {
  /// Returns a reference to the inner string slice.
  pub fn as_str(&self) -> Option<&str> {
    self.value.as_ref().map(AsRef::as_ref)
  }
}

impl Pos {
  /// Returns the byte offset of this position.
  pub fn byte(&self) -> usize {
    self.offset
  }

  /// Returns the line number of this position, starting from 1.
  pub fn line(&self) -> usize {
    self.line
  }

  /// Returns the column number of this position, starting from 1.
  pub fn column(&self) -> usize {
    self.column
  }

  /// Advances the position by one character.
  pub(crate) fn advance(&mut self, c: char) {
    self.offset += c.len_utf8();

    if c == '\n' {
      self.column = 1;
      self.line += 1;
    } else {
      self.column += 1;
    }
  }

  /// Advances the position by a string of characters.
  pub(crate) fn advance_str(&mut self, string: &str) {
    for c in string.chars() {
      self.advance(c);
    }
  }
}

// Implement conversion to `Span` for convenience.

impl From<Pos> for Span {
  fn from(pos: Pos) -> Self {
    Self { start: pos, end: pos }
  }
}

impl From<Range<Pos>> for Span {
  fn from(range: Range<Pos>) -> Self {
    Self { start: range.start, end: range.end }
  }
}

// Implement `Debug` and `Display` for more useful output.

impl Debug for Span {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}..{:?}", self.start, self.end)
  }
}

impl<T: Debug> Debug for Spanned<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_tuple("Spanned").field(&self.span).field(&self.value).finish()
  }
}

impl Debug for Pos {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.line, self.column)
  }
}

impl Display for Span {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.start)
  }
}

impl Display for Pos {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{},{}", self.line, self.column)
  }
}

// Implement `Default` for `Pos` to start at line 1 and column 1.

impl Default for Pos {
  fn default() -> Self {
    Self { offset: 0, line: 1, column: 1 }
  }
}

// Implement `PartialOrd` and `PartialEq` for `Pos` to compare the only
// meaningful value.

impl PartialOrd for Pos {
  fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
    self.offset.partial_cmp(&rhs.offset)
  }
}

impl PartialEq for Pos {
  fn eq(&self, rhs: &Self) -> bool {
    self.offset == rhs.offset
  }
}

// Implement `Add` for `Span` to return a span that includes both operands.

impl Add<Self> for Span {
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self {
    self += rhs;
    self
  }
}

impl AddAssign<Self> for Span {
  fn add_assign(&mut self, rhs: Self) {
    self.start = cmp::min(self.start, rhs.start);
    self.end = cmp::max(self.end, rhs.end);
  }
}
