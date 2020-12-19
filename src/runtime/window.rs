// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Create and manage platform windows.

use crate::env;
use crate::math::Vector2;
use crate::prelude::*;
use crate::runtime::event_loop;
use crate::sync::request;
use winit::window::Window as WinitWindow;

/// A handle for an open window.
///
/// When this handle is dropped, the window is closed.
pub struct Window {
  window: WinitWindow,
}

/// Options for creating a window.
#[derive(Debug)]
pub struct Options {
  /// If `true`, the user can resize the window.
  pub is_resizable: bool,
  /// The size of the window in pixels.
  pub size: Vector2<u16>,
  /// The title of the window.
  pub title: String,
}

impl Window {
  /// Creates a new window with the given options.
  pub async fn new(options: Options) -> Result<Arc<Self>> {
    let req = request!(|req| event_loop::send(event_loop::Command::CreateWindow(options, req)));

    Ok(Arc::new(Self { window: req.await?? }))
  }

  /// Returns the size of the window's inner contents in pixels.
  pub fn size(&self) -> Vector2<u16> {
    let (x, y) = self.window.inner_size().into();

    Vector2::new(x, y)
  }

  /// Returns a reference to the inner [`winit::window::Window`].
  #[cfg(feature = "graphics")]
  pub(crate) fn as_winit(&self) -> &WinitWindow {
    &self.window
  }
}

// Implement `Default` to set default window options.

impl Default for Options {
  fn default() -> Self {
    Self { is_resizable: true, size: Vector2::new(1280, 720), title: env::exe_name().into() }
  }
}
