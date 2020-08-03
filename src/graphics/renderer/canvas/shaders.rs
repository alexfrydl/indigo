// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::graphics::prelude::*;

/// The canvas shader set.
pub struct Shaders {
  fragment: backend::ShaderModule,
  vertex: backend::ShaderModule,
}

/// Creates a shader module from SPIR-V bytes.
fn create_shader_module(device: &Device, bytes: &[u8]) -> Result<backend::ShaderModule> {
  if bytes.len() % 4 != 0 {
    fail!("Invalid SPIR-V code.");
  }

  let spirv =
    unsafe { slice::from_raw_parts(&bytes[0] as *const u8 as *const u32, bytes.len() / 4) };

  let shader = unsafe { device.create_shader_module(&spirv)? };

  Ok(shader)
}

impl Shaders {
  /// Creates the shader set.
  pub fn new(device: &Device) -> Result<Self> {
    let fragment = create_shader_module(device, include_bytes!("shaders/canvas.frag.spv"))
      .map_err(fail::with!("Failed to create fragment shader."))?;

    let vertex = match create_shader_module(device, include_bytes!("shaders/canvas.vert.spv")) {
      Ok(shader) => shader,

      Err(err) => {
        unsafe {
          device.destroy_shader_module(fragment);
        }

        fail!("Failed to create vertex shader. {}", err);
      }
    };

    Ok(Self { fragment, vertex })
  }

  /// Returns an entry point for the fragment shader.
  pub fn fragment_entry_point(&self) -> backend::ShaderEntryPoint {
    backend::ShaderEntryPoint { entry: "main", module: &self.fragment, specialization: default() }
  }

  /// Returns an entry point for the vertex shader.
  pub fn vertex_entry_point(&self) -> backend::ShaderEntryPoint {
    backend::ShaderEntryPoint { entry: "main", module: &self.vertex, specialization: default() }
  }

  /// Destroys the shader modules.
  pub unsafe fn destroy(self, device: &Device) {
    device.destroy_shader_module(self.fragment);
    device.destroy_shader_module(self.vertex);
  }
}
