// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Asynchronous tasks.

pub use std::task::Context;

use crate::prelude::*;

/// A handle to a task running a future on the Indigo runtime.
///
/// If this handle is dropped, the task is canceled. Use [`detach()`] to
/// prevent this.
#[must_use = "Tasks get canceled when dropped. Use `.detach()` to run them in the background."]
pub struct Task<T> {
  detached: bool,
  inner: Option<async_executor::Task<T>>,
}

impl<T: Send + 'static> Task<T> {
  /// Spawns a new task onto the Indigo runtime.
  #[cfg(feature = "runtime")]
  pub fn spawn(future: impl Future<Output = T> + Send + 'static) -> Self {
    Self { detached: false, inner: Some(runtime::executor().spawn(future)) }
  }
}

impl<T> Task<T> {
  /// Cancels the task and waits for it to stop.
  ///
  /// If the task has already completed, this function returns the output of the
  /// task.
  pub async fn cancel(mut self) -> Option<T> {
    self.inner.take().unwrap().cancel().await
  }

  /// Cancels the task and does not wait for it to complete.
  pub fn cancel_now(mut self) {
    self.detached = false;
  }

  /// Detaches this handle so that the task will continue running when it is
  /// dropped.
  pub fn detach(&mut self) {
    self.detached = true;
  }
}

// Implement `Future` to poll the inner task.

impl<T> Future for Task<T> {
  type Output = T;

  fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context) -> future::Poll<Self::Output> {
    Pin::new(self.inner.as_mut().unwrap()).poll(cx)
  }
}

// Implement `Drop` to detach the task if `detach()` was called.

impl<T> Drop for Task<T> {
  fn drop(&mut self) {
    if let Some(handle) = self.inner.take() {
      if self.detached {
        handle.detach();
      }
    }
  }
}
