// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Synchronization primitives and concurrency utilties.

mod atomic;
pub mod channel;
mod semaphore;

pub use self::atomic::*;
pub use self::semaphore::Semaphore;
pub use event_listener::{Event, EventListener};
pub use futures_lite::pin;
pub use once_cell::sync::{Lazy, OnceCell};

/// Blocking concurrency primitives provided by the `parking_lot` crate.
#[doc(inline)]
pub use parking_lot as blocking;
