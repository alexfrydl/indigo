// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Encoding and decoding utilities.

pub mod base64;

/// JSON serialization provided by the `serde_json` crate.
///
#[doc(inline)]
pub use serde_json as json;
