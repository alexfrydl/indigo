// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A base32 encoder and decoder.
//!
//! This module uses a non-standard simplified alphabet that omits similar
//! characters and requires lowercase letters. The characters are, in order:
//! `0123456789abcdefghjkmnpqrstvwxyz`.

use crate::prelude::*;

/// Simplified alphabet.
const SIMPLIFIED: &str = "0123456789abcdefghjkmnpqrstvwxyz";

/// A wrapper that displays a byte slice as a simplified base 32 string.
pub struct AsSimplifiedBase32<'a>(pub &'a [u8]);

/// Encode the given bytes as a simplified base 32 string.
pub fn encode_simplified(bytes: &[u8]) -> String {
  AsSimplifiedBase32(bytes).to_string()
}

/// Encode the given bytes as a simplified base 32 string.
pub fn decode_simplified<E: Extend<u8>>(encoded: &str, into: &mut E) -> Result {
  for chunk in &encoded.chars().chunks(8) {
    let mut count = 0;
    let mut value = 0u64;

    for c in chunk {
      let cval = decode_char(c)?;

      value |= (cval as u64) << (59 - (count * 5));
      count += 1;
    }

    into.extend(value.to_be_bytes().iter().copied().take(count * 5 / 8));
  }

  Ok(())
}

/// Decodes a single character.
fn decode_char(c: char) -> Result<u8> {
  Ok(match c {
    '0' | 'o' => 0,
    '1' | 'i' | 'l' => 1,
    '2' => 2,
    '3' => 3,
    '4' => 4,
    '5' => 5,
    '6' => 6,
    '7' => 7,
    '8' => 8,
    '9' => 9,
    'a' => 10,
    'b' => 11,
    'c' => 12,
    'd' => 13,
    'e' => 14,
    'f' => 15,
    'g' => 16,
    'h' => 17,
    'j' => 18,
    'k' => 19,
    'm' => 20,
    'n' => 21,
    'p' => 22,
    'q' => 23,
    'r' => 24,
    's' => 25,
    't' => 26,
    'v' => 27,
    'w' => 28,
    'x' => 29,
    'y' => 30,
    'z' => 31,
    _ => fail!("Unexpected {}.", fmt::AsDescription(c)),
  })
}

// Implement `Display` to encode bytes.

impl Display for AsSimplifiedBase32<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut buf: ArrayVec<[u8; 8]> = default();

    for chunk in self.0.chunks(5) {
      let mut bytes = [0u8; 8];
      let mut count = 0;

      for b in chunk.iter() {
        bytes[count] = *b;
        count += 1;
      }

      let mut value = u64::from_be_bytes(bytes);

      for _ in 0..count * 8 / 5 {
        let c = (value & !(u64::MAX >> 5)) >> 59;

        buf.push(SIMPLIFIED.as_bytes()[c as usize]);
        value <<= 5;
      }

      if count != 5 {
        let c = (value & !(u64::MAX >> 5)) >> 59;

        buf.push(SIMPLIFIED.as_bytes()[c as usize]);
      }

      f.write_str(unsafe { str::from_utf8_unchecked(&buf[..]) })?;

      buf.clear();
    }

    Ok(())
  }
}

// Unit tests.

#[cfg(test)]
mod tests {
  use super::*;

  /// Tests that values can be roundtripped.
  #[test]
  pub fn test_roundtrip() {
    let original = "round-trip test".as_bytes();

    for size in 0..original.len() {
      let encoded = encode_simplified(&original[..size]);
      let mut decoded = Vec::new();

      decode_simplified(&encoded, &mut decoded).expect("failed to decode");

      let mut expected_len = size * 8 / 5;

      if size % 5 != 0 {
        expected_len += 1;
      }

      assert_eq!(encoded.len(), expected_len, "invalid output length");
      assert_eq!(decoded, &original[..size], "failed to round-trip");
      assert_ne!(
        encoded,
        encode_simplified("control".as_bytes()),
        "not actually encoding anything?"
      );
    }
  }
}
