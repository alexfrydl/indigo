// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod pipeline;
mod shaders;

use self::{pipeline::Pipeline, shaders::Shaders};
use super::Render;

use crate::{
  graphics::{descriptor, prelude::*, Buffer, BufferKind, Mesh, Vertex},
  math::{Matrix4, Vector2},
};

/// A rendering helper for drawing images, meshes, and text.
pub struct Canvas<'a, 'b> {
  push_constants: bool,
  render: &'a mut Render<'b>,
  tint: [f32; 4],
  transform: Matrix4<f32>,
}

/// Cached canvas resources.
pub struct Cache {
  frame_constants: Buffer<FrameConstants>,
  frame_constants_descriptors: descriptor::Set,
  pipeline: Pipeline,
  quad_mesh: Mesh,
  transform_stack: Vec<Matrix4<f32>>,
}

/// Type describing the contents of the frame constants uniform buffer.
#[repr(C)]
#[derive(Default)]
struct FrameConstants {
  projection: Matrix4<f32>,
}

impl<'a, 'b> Canvas<'a, 'b> {
  /// Creates a new canvas.
  pub fn new(render: &'a mut Render<'b>) -> Result<Self> {
    let cache = &mut render.renderer.cache.canvas;

    cache.frame_constants[0] = FrameConstants {
      projection: Matrix4::orthographic_projection(render.size.into()),
      ..default()
    };

    cache.transform_stack.clear();

    unsafe {
      render.cmd.bind_graphics_pipeline(&cache.pipeline);

      render.cmd.bind_graphics_descriptor_sets(
        cache.pipeline.raw_layout(),
        0,
        iter::once(cache.frame_constants_descriptors.raw()),
        &[],
      );
    }

    Ok(Self { push_constants: true, render, tint: [1.0, 1.0, 1.0, 1.0], transform: default() })
  }

  /// Draws a quad with the given tint.
  pub fn draw_quad(&mut self) {
    let cache = &mut self.render.renderer.cache.canvas;
    let cmd = &mut *self.render.cmd;

    unsafe {
      if mem::replace(&mut self.push_constants, false) {
        cmd.push_graphics_constants(
          cache.pipeline.raw_layout(),
          hal::pso::ShaderStageFlags::ALL,
          0,
          pipeline::PushConstants { tint: self.tint, transform: self.transform }.as_u32_slice(),
        );
      }

      let Mesh { vertex_buffer, index_buffer } = &cache.quad_mesh;

      cmd.bind_vertex_buffers(0, iter::once((vertex_buffer.raw(), default())));

      cmd.bind_index_buffer(hal::buffer::IndexBufferView {
        buffer: index_buffer.raw(),
        index_type: hal::IndexType::U16,
        range: default(),
      });

      cmd.draw_indexed(0..index_buffer.len() as u32, 0, 0..1);
    }
  }

  /// Sets the tint, which multiplies all colors drawn.
  pub fn set_tint(&mut self, tint: [f32; 4]) {
    self.push_constants = self.push_constants || self.tint != tint;
    self.tint = tint;
  }

  /// Applies a transform, pushing it onto the stack.
  ///
  /// This transform will be used for all future operations until a matching
  /// [`pop_transform()`] is called.
  pub fn push_transform(&mut self, transform: Matrix4<f32>) {
    let new = self.transform * transform;

    self.push_constants = self.push_constants || self.transform != new;
    self.render.renderer.cache.canvas.transform_stack.push(self.transform);
    self.transform = new;
  }

  /// Removes the most recently pushed transform.
  pub fn pop_transform(&mut self) {
    let cache = &mut self.render.renderer.cache.canvas;

    if let Some(transform) = cache.transform_stack.pop() {
      self.push_constants = self.push_constants || self.transform != transform;
      self.transform = transform;
    }
  }

  /// Finishes drawing on the canvas.
  pub fn finish(self) {}
}

impl Cache {
  /// Creates a new canvas resource cache.
  pub fn new(
    device: &Device,
    descriptor_pool: &mut descriptor::Pool,
    render_pass: &backend::RenderPass,
  ) -> Result<Self> {
    // Create a uniform buffer to store frame constants.

    let mut frame_constants = Buffer::new(BufferKind::Uniform, 1)
      .map_err(fail::with!("Failed to create frame constants buffer."))?;

    frame_constants[0] = default();

    // Create a descriptor layout for the frame constants.

    let frame_constants_layout = descriptor::Layout::new(&[descriptor::Kind::UniformBuffer])
      .map_err(fail::with!("Failed to create frame constants descriptor layout."))?;

    // Create and bind a descriptor set for the frame constants.

    let frame_constants_descriptors = descriptor_pool
      .alloc_one(&frame_constants_layout)
      .map_err(fail::with!("Failed to allocate frame constants descriptor set."))?;

    device.bind_descriptors(iter::once(descriptor::Bind {
      binding: descriptor::Binding::UniformBuffer(frame_constants.raw()),
      index: 0,
      set: &frame_constants_descriptors,
    }));

    // Create a pipeline.

    let pipeline = Pipeline::new(device, render_pass, &[&frame_constants_layout])?;

    // Create the quad mesh.

    let quad_mesh = Mesh::new(
      &[
        Vertex { position: Vector2::new(-0.5, -0.5) },
        Vertex { position: Vector2::new(0.5, -0.5) },
        Vertex { position: Vector2::new(0.5, 0.5) },
        Vertex { position: Vector2::new(-0.5, 0.5) },
      ],
      &[0, 1, 2, 2, 3, 0],
    )
    .map_err(fail::with!("Failed to create quad mesh."))?;

    // Return the final cache.

    Ok(Self {
      frame_constants,
      frame_constants_descriptors,
      pipeline,
      quad_mesh,
      transform_stack: vec![default()],
    })
  }

  /// Destroys the canvas resources.
  pub unsafe fn destroy(self, device: &Device, descriptor_pool: &mut descriptor::Pool) {
    self.pipeline.destroy(device);

    descriptor_pool.free_one(self.frame_constants_descriptors);
  }
}
