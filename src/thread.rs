// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Thread utilities.

use crate::prelude::*;
use std::thread::*;

/// A handle to a spawned thread.
///
/// When this handle is dropped, the thread is joined.  Use [`detach()`] to
/// prevent this.
#[must_use = "Threads get joined when dropped. Use `.detach()` to run them in the background."]
pub struct Thread<T> {
  detached: bool,
  handle: Option<JoinHandle<T>>,
}

/// Sleeps the current thread for a given duration.
pub fn sleep(dur: Duration) {
  std::thread::sleep(dur.into());
}

impl<T: Send + 'static> Thread<T> {
  /// Spawns a new thread.
  pub fn spawn(name: impl Into<String>, func: impl FnOnce() -> T + Send + 'static) -> Self {
    let name = name.into();

    Self {
      detached: false,
      handle: Builder::new().name(name).spawn(func).expect("Failed to spawn thread").into(),
    }
  }
}

impl<T> Thread<T> {
  /// Blocks the current thread until this thread completes and returns its
  /// output.
  pub fn join(mut self) -> T {
    self.join_mut().unwrap()
  }

  /// Detaches this handle so that the thread will continue running when it is
  /// dropped.
  pub fn detach(&mut self) {
    self.detached = true;
  }

  /// Internal `join` implementation that makes it possible to join in `drop`.
  fn join_mut(&mut self) -> Option<T> {
    self.handle.take()?.join().expect("The thread panicked.").into()
  }
}

// Implement `Drop` to join threads that are not detached.

impl<T> Drop for Thread<T> {
  fn drop(&mut self) {
    if !self.detached {
      self.join_mut();
    }
  }
}
