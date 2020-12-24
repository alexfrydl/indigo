// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::prelude::*;

use crate::math::Vector2;

/// An image on the graphics device.
pub struct Image {
  data: Data,
  size: Vector2<u16>,
}

/// One of the possible types on inner image data.
enum Data {
  SwapchainImage(backend::SwapchainImage),
}

impl Image {
  /// Wraps an image from a surface.
  pub(super) fn from_swapchain_image(image: backend::SwapchainImage, size: Vector2<u16>) -> Self {
    Self { data: Data::SwapchainImage(image), size }
  }

  /// Returns the size of this image in pixels.
  pub fn size(&self) -> Vector2<u16> {
    self.size
  }

  /// Unwraps an image from a surface.
  pub(super) fn into_swapchain_image(self) -> backend::SwapchainImage {
    match self.data {
      Data::SwapchainImage(image) => image,
    }
  }

  /// Returns a reference to the raw backend image view.
  pub(super) fn raw_view(&self) -> &backend::ImageView {
    match &self.data {
      Data::SwapchainImage(image) => image.borrow(),
    }
  }
}
