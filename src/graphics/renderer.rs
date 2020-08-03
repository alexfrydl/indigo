// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Rendering.

mod cache;
mod canvas;
mod render;

pub use self::{canvas::Canvas, render::Render};

use self::cache::Cache;
use super::{prelude::*, Image};

/// A renderer for rendering onto [`Image`]s.
pub struct Renderer {
  cache: ManuallyDrop<Cache>,
  device: &'static Device,
  queue: &'static device::Queue,
}

impl Renderer {
  /// Creates a new renderer.
  pub fn new() -> Result<Self> {
    let device = device()?;

    let queue = device
      .find_queue(|f| f.queue_type().supports_graphics())
      .ok_or_else(|| fail::err!("The device does not support graphics commands."))?;

    let cache = Cache::new(device, queue)?;

    Ok(Self { cache: ManuallyDrop::new(cache), device, queue })
  }

  /// Begins a render onto the given [`Image`].
  pub fn begin_render<'a>(&'a mut self, image: &'a mut Image) -> Result<Render<'a>> {
    Render::new(self, image)
  }
}

// Implement Drop to destroy renderer resources.

impl Drop for Renderer {
  fn drop(&mut self) {
    let Self { cache, device, .. } = self;

    unsafe {
      ManuallyDrop::take(cache).destroy(device);
    }
  }
}
