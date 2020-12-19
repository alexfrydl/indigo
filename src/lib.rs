// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod collections;
pub mod derive;
pub mod encoding;
pub mod env;
pub mod fail;
pub mod fmt;
pub mod fs;
pub mod future;
pub mod iter;
pub mod log;
pub mod math;
#[cfg(feature = "postgres")]
pub mod postgres;
pub mod prelude;
pub mod random;
#[cfg(feature = "runtime")]
pub mod runtime;
pub mod stream;
mod symbol;
pub mod sync;
pub mod task;
pub mod thread;
pub mod time;
mod unique_id;

#[doc(inline)]
pub use {
  self::{random::random, symbol::Symbol, unique_id::UniqueId},
  indigo_macros::{attempt, attempt_async},
  uuid::{self, Uuid},
};

#[cfg(feature = "runtime")]
#[doc(inline)]
pub use self::runtime::main;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "graphics")]
pub mod graphics;
