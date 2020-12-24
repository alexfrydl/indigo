// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// An unsigned integer or floating-point number.
#[derive(Debug, Clone, Copy, From)]
pub enum Number {
  Float(Float),
  Integer(Integer),
}

/// A floating-point number.
#[derive(Debug, Clone, Copy)]
pub struct Float {
  span: Span,
  value: f64,
}

/// An unsigned integer.
#[derive(Debug, Clone, Copy)]
pub struct Integer {
  span: Span,
  value: u64,
}

impl Number {
  /// Returns the span containing the number.
  pub fn span(&self) -> Span {
    match self {
      Number::Float(float) => float.span,
      Number::Integer(int) => int.span,
    }
  }
}

impl Float {
  /// Constructs a new IDN floating-point number.
  pub fn new(span: impl Into<Span>, value: f64) -> Self {
    Self { span: span.into(), value }
  }

  /// Returns the value of this floating-point number as an `f64`.
  pub fn as_f64(&self) -> f64 {
    self.value
  }

  /// Returns the span containing this floating-point number.
  pub fn span(&self) -> Span {
    self.span
  }
}

impl Integer {
  /// Constructs a new IDN integer.
  pub fn new(span: impl Into<Span>, value: u64) -> Self {
    Self { span: span.into(), value }
  }

  /// Returns the value of this integer as an `f64`.
  pub fn as_u64(&self) -> u64 {
    self.value
  }

  /// Returns the span containing this integer.
  pub fn span(&self) -> Span {
    self.span
  }
}

// Implement conversions between the types of numbers.

impl From<Number> for Float {
  fn from(number: Number) -> Self {
    match number {
      Number::Float(float) => float,
      Number::Integer(int) => Float { span: int.span, value: int.value as f64 },
    }
  }
}

impl TryFrom<Number> for Integer {
  type Error = Error;

  fn try_from(number: Number) -> Result<Self> {
    match number {
      Number::Float(float) => abort!(float.span, "Expected integer, found floating-point number."),
      Number::Integer(int) => Ok(int),
    }
  }
}

// Implement `FromIdn` to parse specific number types.

impl FromIdn for Integer {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let number: Number = reader.read()?;

    number.try_into()
  }
}

impl FromIdn for Float {
  fn from_idn(reader: &mut Reader) -> Result<Self> {
    let number: Number = reader.read()?;

    Ok(number.into())
  }
}
