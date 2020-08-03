// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Shaders;

use crate::{
  graphics::{descriptor, prelude::*},
  math::{Matrix4, Vector2},
};

/// The canvas pipeline.
#[derive(Deref)]
pub struct Pipeline {
  layout: backend::PipelineLayout,
  #[deref]
  pipeline: backend::GraphicsPipeline,
  shaders: Shaders,
}

/// Push constants for the pipeline.
#[repr(C)]
pub struct PushConstants {
  pub transform: Matrix4<f32>,
  pub tint: [f32; 4],
}

impl Pipeline {
  /// Creates the graphics pipeine.
  pub fn new(
    device: &Device,
    pass: &backend::RenderPass,
    descriptor_layouts: &[&descriptor::Layout],
  ) -> Result<Self> {
    let shaders = Shaders::new(device)?;

    // Create the pipeline layout.

    let layout = unsafe {
      let result = device.create_pipeline_layout(
        descriptor_layouts.iter().map(|layout| layout.raw()),
        &[(hal::pso::ShaderStageFlags::ALL, 0..mem::size_of::<PushConstants>() as u32)],
      );

      match result {
        Ok(layout) => layout,

        Err(err) => {
          shaders.destroy(device);

          return Err(fail::err!("Failed to create pipeline layout. {}", err));
        }
      }
    };

    // Create the graphics pipeline.

    let mut desc = hal::pso::GraphicsPipelineDesc::new(
      hal::pso::PrimitiveAssemblerDesc::Vertex {
        attributes: &[hal::pso::AttributeDesc {
          binding: 0,
          location: 0,
          element: hal::pso::Element { format: hal::format::Format::Rg32Sfloat, offset: 0 },
        }],
        buffers: &[hal::pso::VertexBufferDesc {
          binding: 0,
          stride: mem::size_of::<Vector2<f32>>() as u32,
          rate: hal::pso::VertexInputRate::Vertex,
        }],
        input_assembler: hal::pso::InputAssemblerDesc::new(hal::pso::Primitive::TriangleList),
        geometry: None,
        tessellation: None,
        vertex: shaders.vertex_entry_point(),
      },
      hal::pso::Rasterizer::FILL,
      Some(shaders.fragment_entry_point()),
      &layout,
      hal::pass::Subpass { index: 0, main_pass: pass },
    );

    desc.blender.targets.push(hal::pso::ColorBlendDesc {
      mask: hal::pso::ColorMask::ALL,
      blend: Some(hal::pso::BlendState::ALPHA),
    });

    let pipeline = unsafe {
      match device.create_graphics_pipeline(&desc, None) {
        Ok(pipeline) => pipeline,

        Err(err) => {
          shaders.destroy(device);
          device.destroy_pipeline_layout(layout);

          return Err(fail::err!("Failed to create graphics pipeline. {}", err));
        }
      }
    };

    Ok(Self { layout, pipeline, shaders })
  }

  /// Destroys the pipeline.
  pub unsafe fn destroy(self, device: &Device) {
    self.shaders.destroy(device);

    device.destroy_graphics_pipeline(self.pipeline);
    device.destroy_pipeline_layout(self.layout);
  }

  /// Returns a reference to the underlying backend pipeline layout.
  pub fn raw_layout(&self) -> &backend::PipelineLayout {
    &self.layout
  }
}

impl PushConstants {
  /// Returns the push constants as a slice of `u32`.
  pub fn as_u32_slice(&self) -> &[u32] {
    unsafe { slice::from_raw_parts(self as *const Self as *const u32, mem::size_of::<Self>() / 4) }
  }
}
