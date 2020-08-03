// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::canvas;
use crate::graphics::{descriptor, prelude::*};

/// Cached rendering resources.
pub struct Cache {
  /// Canvas resources.
  pub canvas: canvas::Cache,
  /// A shared command pool.
  pub cmd_pool: backend::CommandPool,
  /// A shared descriptor pool.
  pub descriptor_pool: descriptor::Pool,
  /// The main render pass.
  pub render_pass: backend::RenderPass,
}

/// Creates the render pass.
fn create_render_pass(device: &Device) -> Result<backend::RenderPass, OutOfMemory> {
  unsafe {
    device.create_render_pass(
      // Attachments.
      &[
        // Swap chain image attachment.
        hal::pass::Attachment {
          format: Some(hal::format::Format::Bgra8Srgb),
          samples: 1,
          // Clear before rendering, store after.
          ops: hal::pass::AttachmentOps::new(
            hal::pass::AttachmentLoadOp::Clear,
            hal::pass::AttachmentStoreOp::Store,
          ),
          // Ignore stencil.
          stencil_ops: hal::pass::AttachmentOps::DONT_CARE,
          // Transition from any layout before rendering into the present layout
          // after rendering.
          layouts: hal::image::Layout::Undefined..hal::image::Layout::Present,
        },
      ],
      // Subpasses.
      &[hal::pass::SubpassDesc {
        colors: &[(0, hal::image::Layout::ColorAttachmentOptimal)],
        depth_stencil: None,
        inputs: &[],
        resolves: &[],
        preserves: &[],
      }],
      // Dependencies.
      &[],
    )
  }
}

impl Cache {
  /// Creates a new cache.
  pub fn new(device: &Device, queue: &device::Queue) -> Result<Self> {
    let cmd_pool = unsafe {
      device
        .create_command_pool(queue.family.id(), hal::pool::CommandPoolCreateFlags::RESET_INDIVIDUAL)
        .map_err(fail::with!("Failed to create command pool."))?
    };

    let mut descriptor_pool =
      descriptor::Pool::new().map_err(fail::with!("Failed to create descriptor pool."))?;

    let render_pass =
      create_render_pass(device).map_err(fail::with!("Failed to create render pass."))?;

    let canvas = canvas::Cache::new(device, &mut descriptor_pool, &render_pass)
      .map_err(fail::with!("Failed to create canvas cache."))?;

    Ok(Self { canvas, cmd_pool, descriptor_pool, render_pass })
  }

  /// Destroys all cached resources.
  pub(super) unsafe fn destroy(mut self, device: &Device) {
    self.canvas.destroy(device, &mut self.descriptor_pool);

    device.destroy_command_pool(self.cmd_pool);
    device.destroy_render_pass(self.render_pass);
  }
}
