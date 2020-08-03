// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Blocking utilities and concurrency primitives.
//!
//! Operations in this module may block the current thread and should not be
//! used in an `async` context.

#[doc(inline)]
pub use {::blocking::unblock, parking_lot::*};

use crate::prelude::*;

/// Blocks the current thread until the given future completes.
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
  async_executor::LocalExecutor::new().run(future)
}
