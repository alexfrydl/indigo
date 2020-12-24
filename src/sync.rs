// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Synchronization primitives and concurrency utilties.

mod atomic;
pub mod blocking;
pub mod channel;
pub mod request;
mod semaphore;

#[doc(inline)]
pub use {
  self::atomic::*,
  self::request::{request, Request},
  self::semaphore::Semaphore,
  async_io::Timer,
  event_listener::{Event, EventListener},
  futures_lite::pin,
  once_cell::sync::{Lazy, OnceCell},
};

/// A concurrent hash map provided by the `dashmap` crate.
///
#[doc(inline)]
pub use dashmap::DashMap as ConcurrentHashMap;

/// A concurrent hash set provided by the `dashmap` crate.
///
#[doc(inline)]
pub use dashmap::DashSet as ConcurrentHashSet;
