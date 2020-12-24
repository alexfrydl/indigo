// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[doc(inline)]
pub use indigo_macros::{idn_abort as abort, idn_err as err};

use super::*;

use crate::derive::Error;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// An error in a IDN document or source string.
#[derive(Clone, Error)]
pub struct Error {
  /// The error message.
  message: Arc<str>,
  /// The span of input the error refers to.
  span: Span,
}

/// A list of IDN errors.
#[derive(Clone, Debug, Default, Deref, Error, From)]
pub struct ErrorList {
  #[deref]
  errors: im::Vector<Error>,
}

impl Error {
  /// Constructs a new error.
  pub fn new(span: impl Into<Span>, message: impl Into<String>) -> Self {
    Self { span: span.into(), message: message.into().into() }
  }
}

impl ErrorList {
  /// Returns a reference to the error list of the given context.
  pub fn from_context(ctx: &mut Context) -> ctx::RefMut<Self> {
    ctx.get_mut("errors").unwrap()
  }

  /// Creates a new, empty error list.
  pub fn new() -> Self {
    default()
  }

  /// Adds an error to the list.
  pub fn add(&mut self, err: Error) {
    self.errors.push_back(err);
  }
}

// Implement `Debug` and `Display` to show the error with span info.

impl Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error({:?}, {:?})", self.span, self.message)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}: {}", self.span, self.message)
  }
}

impl Display for ErrorList {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.errors.len() == 1 {
      return Display::fmt(&self.errors[0], f);
    }

    write!(f, "{} errors:", self.errors.len())?;

    for error in &self.errors {
      write!(f, "\n{}", error)?;
    }

    Ok(())
  }
}
