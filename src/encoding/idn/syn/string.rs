// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A IDN string.
#[derive(Clone, Debug)]
pub struct StringLiteral {
  span: Span,
  value: Arc<str>,
}

impl StringLiteral {
  /// Constructs a new IDN string.
  pub fn new(span: impl Into<Span>, value: impl Into<Arc<str>>) -> Self {
    Self { span: span.into(), value: value.into() }
  }

  /// Returns the value of this string as a `&str`.
  pub fn as_str(&self) -> &str {
    self.value.as_ref()
  }

  /// Returns the span containing this string, including quotation marks.
  pub fn span(&self) -> Span {
    self.span
  }
}

// Implement conversion to regular strings.

impl From<StringLiteral> for Arc<str> {
  fn from(string: StringLiteral) -> Self {
    string.value.clone()
  }
}

impl From<&'_ StringLiteral> for Arc<str> {
  fn from(string: &'_ StringLiteral) -> Self {
    string.value.clone()
  }
}

impl From<StringLiteral> for String {
  fn from(string: StringLiteral) -> Self {
    string.as_str().into()
  }
}

impl From<&'_ StringLiteral> for String {
  fn from(string: &'_ StringLiteral) -> Self {
    string.as_str().into()
  }
}
