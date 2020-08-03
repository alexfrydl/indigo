// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! File system utilities.
//!
//! This module does not support non-Unicode paths. Paths with non-Unicode
//! characters are either ignored or cause an error, depending on the context.

pub mod path;
mod utils;

#[cfg(feature = "fs-watch")]
mod watch;

#[doc(inline)]
pub use self::{path::PathLike, utils::*};

#[cfg(feature = "fs-watch")]
pub use self::watch::Watcher;
