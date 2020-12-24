// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{encoding::base32, prelude::*};

use std::num::NonZeroU128;

/// A unique ID with `2^128 - 1` possible values.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UniqueId(NonZeroU128);

impl UniqueId {
  /// Creates a new, random IUID.
  pub fn new() -> Self {
    Self::random()
  }
}

// Implement `FromStr` to parse IUIDs from base32.

impl FromStr for UniqueId {
  type Err = fail::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s = s.trim();

    if s.len() != 26 {
      fail!("IUIDs must be exactly 26 characters.");
    }

    // Decode from base32.

    let mut bytes: ArrayVec<[u8; 16]> = default();

    base32::decode_simplified(s, &mut bytes)?;

    // Convert to a non-zero u128.

    let value = u128::from_be_bytes(bytes.into_inner().unwrap());

    NonZeroU128::new(value).map(UniqueId).ok_or_else(|| fail::err!("Zero is not valid IUID."))
  }
}

// Implement `Debug` and `Display` to encode IUIDs as base32.

impl Display for UniqueId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    base32::AsSimplifiedBase32(&self.0.get().to_be_bytes()[..]).fmt(f)
  }
}

impl Debug for UniqueId {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\"{}\"", self)
  }
}

// Implement `Random` for generating new IUIDs.

impl Random for UniqueId {
  fn random_with(rng: &mut Rng) -> Self {
    Self(rng.random())
  }
}

// Unit tests.

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

  /// Tests that `Iuid::new` generates unique values.
  #[test]
  pub fn test_uniqueness() {
    assert_eq!(1024, iter::repeat_with(UniqueId::new).take(1024).collect::<HashSet<_>>().len());
  }

  /// Tests that `Iuid::new` generates unique values.
  #[test]
  pub fn test_roundtrip() {
    let a = UniqueId::new();
    let b = UniqueId(NonZeroU128::new(1).unwrap());
    let c = UniqueId(NonZeroU128::new(u128::MAX).unwrap());

    assert_eq!(UniqueId::from_str(&a.to_string()).unwrap(), a);
    assert_eq!(UniqueId::from_str(&b.to_string()).unwrap(), b);
    assert_eq!(UniqueId::from_str(&c.to_string()).unwrap(), c);
  }

  /// Tests that zero is not a valid IUID.
  #[test]
  pub fn test_nonzero() {
    let zero = base32::encode_simplified(&0u128.to_be_bytes()[..]);

    assert!(UniqueId::from_str(&zero).is_err());
  }
}
