// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Collection types.

/// Fixed-capacity vectors and strings provided by the `arrayvec` crate.
///
#[doc(inline)]
pub use arrayvec;

#[doc(inline)]
pub use arrayvec::{Array, ArrayString, ArrayVec};

/// Immutable collection types provided by the `im` crate.
///
#[doc(inline)]
pub use im as immutable;

pub use std::collections::*;
