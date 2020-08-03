// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Renderer;

use crate::{
  graphics::{prelude::*, Image},
  math::{Matrix4, Vector2},
  sync::blocking::unblock,
};

/// An in-progress render to an [`Image`].
pub struct Render<'a> {
  pub(super) cmd: ManuallyDrop<backend::CommandBuffer>,
  framebuffer: ManuallyDrop<backend::Framebuffer>,
  pub(super) renderer: &'a mut Renderer,
  pub(super) size: Vector2<u16>,
}

/// Type describing the contents of the frame constants uniform buffer.
#[repr(C)]
#[derive(Default)]
struct FrameConstants {
  projection: Matrix4<f32>,
}

impl<'a> Render<'a> {
  /// Begins a render onto the given [`Image`].
  pub fn new(renderer: &'a mut Renderer, image: &'a mut Image) -> Result<Self> {
    let Renderer { cache, device, .. } = renderer;
    let size = image.size();

    let viewport = hal::pso::Viewport {
      rect: hal::pso::Rect { x: 0, y: 0, w: size.x as i16, h: size.y as i16 },
      depth: 0.0..1.0,
    };

    unsafe {
      let mut cmd = cache.cmd_pool.allocate_one(hal::command::Level::Primary);

      let framebuffer = device
        .create_framebuffer(
          &cache.render_pass,
          iter::once(image.raw_view()),
          hal::image::Extent { width: size.x as u32, height: size.y as u32, depth: 1 },
        )
        .map_err(fail::with!("Failed to create framebuffer."))?;

      cmd.begin(hal::command::CommandBufferFlags::ONE_TIME_SUBMIT, default());

      cmd.begin_render_pass(
        &cache.render_pass,
        &framebuffer,
        viewport.rect,
        &[hal::command::ClearValue {
          color: hal::command::ClearColor { float32: [0.007985829, 0.007985829, 0.012334518, 1.0] },
        }],
        hal::command::SubpassContents::Inline,
      );

      cmd.set_viewports(0, &[viewport.clone()]);
      cmd.set_scissors(0, &[viewport.rect]);

      Ok(Render {
        cmd: ManuallyDrop::new(cmd),
        framebuffer: ManuallyDrop::new(framebuffer),
        renderer,
        size,
      })
    }
  }

  /// Returns the size of the render area.
  pub fn size(&self) -> Vector2<u16> {
    self.size
  }

  /// Finishes rendering, waits for the render to complete, and returns the
  /// image with the final result.
  pub async fn finish(mut self) -> Result {
    let cmd = &mut *self.cmd;
    let device = self.renderer.device;
    let Renderer { queue, .. } = &mut self.renderer;

    unsafe {
      cmd.end_render_pass();
      cmd.finish();

      let fence =
        device.create_fence(false).map_err(fail::with!("Failed to create frame fence."))?;

      queue.lock().submit(
        hal::queue::Submission {
          command_buffers: iter::once(&*cmd),
          signal_semaphores: iter::empty::<&backend::Semaphore>(),
          wait_semaphores: iter::empty::<_>(),
        },
        Some(&fence),
      );

      unblock! {
        let result = device.wait_for_fence(&fence, !0);

        device.destroy_fence(fence);

        result.map(|_| ()).map_err(fail::with!("Failed to wait for frame fence."))
      }
    }
  }
}

// Implement Drop to free device resources.

impl<'a> Drop for Render<'a> {
  fn drop(&mut self) {
    let Self { cmd, framebuffer, renderer, .. } = self;
    let Renderer { cache, device, .. } = renderer;

    unsafe {
      cache.cmd_pool.free(iter::once(ManuallyDrop::take(cmd)));

      device.destroy_framebuffer(ManuallyDrop::take(framebuffer));
    }
  }
}
