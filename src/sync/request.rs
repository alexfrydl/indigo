// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Asynchronous request handling (also known as a “oneshot” channel).

#[doc(inline)]
pub use super::channel::ClosedError;
#[doc(inline)]
pub use indigo_macros::async_request as request;

use super::channel as mpmc;
use crate::prelude::*;

/// An error indicating that the [`Resolver`] was dropped.
#[derive(Clone, Copy, Debug, Default, Display, Error)]
#[display(fmt = "Request dropped.")]
pub struct DroppedError;

/// An asynchronous request for a value of type `T`.
pub struct Request<T> {
  sender: mpmc::Sender<T>,
}

/// A receiver for awaiting the result of a [`Request`].
pub struct Receiver<T> {
  receiver: mpmc::Receiver<T>,
}

impl<T> Request<T> {
  /// Creates a new request and returns it with its associated [`Receiver`].
  pub fn new() -> (Self, Receiver<T>) {
    let (sender, receiver) = mpmc::bounded(1);

    (Self { sender }, Receiver { receiver })
  }

  /// Resolves the request with the given value.
  pub fn resolve(self, value: T) {
    let _ = self.try_resolve(value);
  }

  /// Attempts to resolve the request with the given value.
  pub fn try_resolve(self, value: T) -> Result<(), ClosedError> {
    self.sender.try_send(value).map_err(|_| ClosedError)
  }
}

impl<T> Receiver<T> {
  /// Waits for and receives a value from the resolved [`Request`].
  pub async fn recv(self) -> Result<T, DroppedError> {
    self.receiver.recv().await.map_err(|_| DroppedError)
  }
}
