// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Utilities for working with futures and async logic.

mod join;
mod race;

pub use self::{
  join::{join, Join},
  race::{race, Race},
};

#[doc(inline)]
pub use {
  blocking::unblock,
  futures_lite::future::{poll_fn, PollFn},
  std::{future::Future, task::Poll},
};

use crate::prelude::*;

/// A buffered future that separates polling from consuming the output value.
pub enum Buffered<F: Future> {
  Pending(F),
  Ready(F::Output),
  Taken,
}

/// A future that never returns.
struct Pending<Output> {
  phantom: PhantomData<Output>,
}

/// Returns a buffered future that separates polling from consuming the output
/// value.
pub fn buffered<F: Future>(future: F) -> Buffered<F> {
  Buffered::Pending(future)
}

/// Waits indefinitely.
pub async fn never() {
  pending().await
}

/// Waits indefinitely for a value of type `T`.
pub async fn pending<T>() -> T {
  Pending { phantom: PhantomData }.await
}

/// Waits for a given duration of time to elapse.
pub async fn delay(duration: Duration) {
  async_io::Timer::new(duration.to_std()).await;
}

impl<F: Future> Buffered<F> {
  /// Returns `true` if the output value is still pending.
  pub fn is_pending(&self) -> bool {
    match self {
      Self::Pending(_) => true,
      _ => false,
    }
  }

  /// Returns the output value of the future.
  ///
  /// If the output value is not available, this function will panic.
  pub fn into_output(self) -> F::Output {
    if let Self::Ready(output) = self {
      return output;
    }

    panic!("The output is not available.");
  }

  /// Removes and returns the output value of the future.
  ///
  /// If the output value is not available, this function will panic.
  pub fn take_output(&mut self) -> F::Output {
    let mut taken = Self::Taken;

    mem::swap(self, &mut taken);

    taken.into_output()
  }
}

// Implement polling for future types.

impl<F: Future> Future for Buffered<F> {
  type Output = ();

  fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<()> {
    unsafe {
      match Pin::as_mut(&mut self).get_unchecked_mut() {
        Self::Pending(f) => match Pin::new_unchecked(f).poll(cx) {
          Poll::Pending => Poll::Pending,

          Poll::Ready(value) => {
            self.set(Self::Ready(value));

            Poll::Ready(())
          }
        },

        _ => Poll::Ready(()),
      }
    }
  }
}

impl<Output> Future for Pending<Output> {
  type Output = Output;

  fn poll(self: Pin<&mut Self>, _: &mut task::Context) -> Poll<Output> {
    Poll::Pending
  }
}
