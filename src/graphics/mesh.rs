// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{prelude::*, Buffer, BufferKind};

use crate::math::Vector2;

/// A mesh containing an index buffer and a vertex buffer.
pub struct Mesh {
  /// The index buffer of the mesh.
  pub index_buffer: Buffer<u16>,
  /// The vertex buffer of the mesh.
  pub vertex_buffer: Buffer<Vertex>,
}

/// A single vertex in a mesh.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
  pub position: Vector2<f32>,
}

impl Mesh {
  /// Cretaes a new mesh from vertices and indices.
  pub fn new(vertices: &[Vertex], indices: &[u16]) -> Result<Self> {
    let mut vertex_buffer = Buffer::new(BufferKind::Vertex, vertices.len())
      .map_err(fail::with!("Failed to create vertex buffer."))?;

    let mut index_buffer = Buffer::new(BufferKind::Index, indices.len())
      .map_err(fail::with!("Failed to create index buffer."))?;

    vertex_buffer.copy_from_slice(vertices);
    index_buffer.copy_from_slice(indices);

    Ok(Self { index_buffer, vertex_buffer })
  }
}
