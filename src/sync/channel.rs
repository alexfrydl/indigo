// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A multi-producer, multi-consumer channel.

use crate::prelude::*;

use async_channel::{TryRecvError, TrySendError};

/// A cloneable receiver half of a `Channel`.
pub struct Receiver<T> {
  inner: async_channel::Receiver<T>,
}

/// A cloneable sender half of a `Channel`.
pub struct Sender<T> {
  inner: async_channel::Sender<T>,
}

/// An error indicating that the channel is closed.
#[derive(Clone, Copy, Debug, Default, Display, Error)]
#[display(fmt = "Channel is closed.")]
pub struct ClosedError;

/// One of the possible errors returned from `Receiver::recv_now`.
#[derive(Clone, Copy, Debug, Display, Error)]
pub enum RecvError {
  #[display(fmt = "Channel is closed.")]
  Closed,
  #[display(fmt = "Channel is empty.")]
  Empty,
}

/// One of the possible errors returned from `Sender::send_now`.
#[derive(Clone, Copy, Debug, Display, Error)]
pub enum SendError {
  #[display(fmt = "Channel is closed.")]
  Closed,
  #[display(fmt = "Channel is full.")]
  Full,
}

/// Returns a [`Sender`] and [`Receiver`] pair for an bounded channel with a
/// specified capacity.
///
/// Bounded channels can only buffer up to `capacity` unreceived messages. If
/// the channels is full, [`Sender:::send()`] will wait for space.
pub fn bounded<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
  let (tx, rx) = async_channel::bounded(capacity);

  (Sender { inner: tx }, Receiver { inner: rx })
}

/// Returns a [`Sender`] and [`Receiver`] pair for an unbounded channel.
///
/// Unbounded channels can buffer an unlimited number of unreceived messages,
/// and [`Sender:::send()`] will never wait.
pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
  let (tx, rx) = async_channel::unbounded();

  (Sender { inner: tx }, Receiver { inner: rx })
}

impl<T> Receiver<T> {
  /// Closes the channel.
  ///
  /// This function returns `true` if the channel was open and the function
  /// closed it, or `false` if the channel was already closed.
  pub fn close(&self) -> bool {
    self.inner.close()
  }

  /// Waits for an available message in the channel and then receive it.
  pub async fn recv(&self) -> Result<T, ClosedError> {
    self.inner.recv().await.map_err(|_| ClosedError)
  }

  /// Attempts to immediately receiveü an available message from the channel.
  pub fn try_recv(&self) -> Result<T, RecvError> {
    self.inner.try_recv().map_err(|err| match err {
      TryRecvError::Closed => RecvError::Closed,
      TryRecvError::Empty => RecvError::Empty,
    })
  }
}

impl<T> Sender<T> {
  /// Closes the channel.
  ///
  /// This function returns `true` if the channel was open and the function
  /// closed it, or `false` if the channel was already closed.
  pub fn close(&self) -> bool {
    self.inner.close()
  }

  /// Waits for available space in the channel and then sends a message to it.
  ///
  /// If the message was sent, this function returns `true`. If the channel is
  /// closed, it returns `false`.
  pub async fn send(&self, message: T) -> bool {
    self.inner.send(message).await.is_ok()
  }

  /// Attempts to immediately sends a message to the channel.
  pub fn try_send(&self, message: T) -> Result<(), SendError> {
    self.inner.try_send(message).map_err(|err| match err {
      TrySendError::Closed(_) => SendError::Closed,
      TrySendError::Full(_) => SendError::Full,
    })
  }
}

// Implement `Stream` for the receiver end.

impl<T> Stream for Receiver<T> {
  type Item = T;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<Option<T>> {
    Pin::new(&mut self.inner).poll_next(cx)
  }
}

// Manually implement `Clone` for all `T`.

impl<T> Clone for Receiver<T> {
  fn clone(&self) -> Self {
    Self { inner: self.inner.clone() }
  }
}

impl<T> Clone for Sender<T> {
  fn clone(&self) -> Self {
    Self { inner: self.inner.clone() }
  }
}
