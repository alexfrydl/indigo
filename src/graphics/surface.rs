// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Rendering surface created from an [`indigo::Window`](crate::Window).

use super::{prelude::*, Image};
use crate::{math::Vector2, runtime::Window};

/// A rendering surface created from a [`Window`].
#[allow(dead_code)]
pub struct Surface {
  device: &'static Device,
  queue: &'static device::Queue,
  size: Option<Vector2<u16>>,
  surface: backend::Surface,
  window: ArcWeak<Window>,
}

impl Surface {
  /// Creates a new surface from the given window.
  pub fn new(window: &Arc<Window>) -> Result<Self> {
    let backend = backend::instance()?;
    let device = device()?;

    let surface = unsafe { backend.create_surface(window.as_winit()) }?;

    let queue = device
      .find_queue(|f| surface.supports_queue_family(f))
      .ok_or_else(|| fail::err!("No device queues support presentation to the window surface."))?;

    Ok(Self { device, queue, size: None, surface, window: Arc::downgrade(window) })
  }

  /// Acquires a backbuffer from the surface.
  pub async fn acquire(&mut self) -> Result<Image> {
    const ATTEMPTS: usize = 5;

    for _ in 0..ATTEMPTS {
      self.configure()?;

      match unsafe { self.surface.acquire_image(!0) } {
        Ok((image, None)) => {
          return Ok(Image::from_swapchain_image(image, self.size.unwrap()));
        }

        Ok((_, Some(hal::window::Suboptimal))) | Err(hal::window::AcquireError::OutOfDate) => {
          self.unconfigure();
        }

        Err(err) => fail!(err),
      };
    }

    fail!("The image acquired from each of {} attempts was out of date or suboptimal.", ATTEMPTS);
  }

  /// Presents a previously acquired backbuffer to the surface.
  pub fn present(&mut self, backbuffer: Image) -> Result {
    let image = backbuffer.into_swapchain_image();
    let result = unsafe { self.queue.lock().present(&mut self.surface, image, None) };

    match result {
      Ok(Some(hal::window::Suboptimal)) | Err(hal::window::PresentError::OutOfDate) => {
        self.unconfigure();
      }

      Err(err) => {
        fail!("Failed to present swapchain image. {}", err);
      }

      _ => {}
    }

    Ok(())
  }

  /// Ensures that the swapchain of the surface is configured.
  fn configure(&mut self) -> Result {
    // Do nothing if the window has not been resized and the swapchain is
    // already configured.

    let window_size = self.window.upgrade().ok_or_else(|| fail::err!("Window closed."))?.size();

    if matches!(self.size, Some(s) if s == window_size) {
      return Ok(());
    }

    // Configure the swapchain based on surface capabilities.

    let caps = self.surface.capabilities(&self.device.adapter.physical_device);

    let config = hal::window::SwapchainConfig::from_caps(
      &caps,
      hal::format::Format::Bgra8Srgb,
      hal::window::Extent2D { width: window_size.x as u32, height: window_size.y as u32 },
    );

    let extent = config.extent;

    unsafe {
      self.surface.configure_swapchain(self.device, config)?;
    }

    // If the surface was created successfully, store its extent.

    self.size = Some(Vector2::new(extent.width as u16, extent.height as u16));

    Ok(())
  }

  /// Unconfigure the swapchain and free its resources.
  fn unconfigure(&mut self) {
    unsafe {
      self.surface.unconfigure_swapchain(self.device);
    }

    self.size = None;
  }
}
