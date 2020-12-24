// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Asynchronous tasks.

use super::executor;
use crate::prelude::*;

/// A handle to a task running a future on the Indigo runtime.
///
/// If this handle is dropped, the task is canceled. Use [`detach()`] to
/// prevent this.
#[must_use = "Tasks get canceled when dropped. Use `.detach()` to run them in the background."]
pub struct Task<T> {
  inner: async_executor::Task<T>,
}

/// Starts a new asynchronous task.
#[cfg(feature = "runtime")]
pub fn start<F>(future: F) -> Task<F::Output>
where
  F: Future + Send + 'static,
  F::Output: Send + 'static,
{
  Task { inner: executor().spawn(future) }
}

/// Starts a new asynchronous task that runs to completion in thebackground.
///
/// Equivalent to `start(…).detach()`.
#[cfg(feature = "runtime")]
pub fn start_detached<F>(future: F)
where
  F: Future + Send + 'static,
{
  start(async move {
    future.await;
  })
  .detach()
}

impl<T> Task<T> {
  /// Stops the task, dropping the original future.
  ///
  /// If the task has already completed, its output is returned.
  pub async fn stop(self) -> Option<T> {
    self.inner.cancel().await
  }

  /// Detaches this task so it runs to completion in the background.
  pub fn detach(self) {
    self.inner.detach()
  }
}

// Implement `Future` to poll the inner task.

impl<T> Future for Task<T> {
  type Output = T;

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<Self::Output> {
    let inner = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.inner) };

    inner.poll(cx)
  }
}
