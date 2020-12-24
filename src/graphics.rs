// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The Indigo graphics engine.

pub mod mesh;

mod alloc;
mod backend;
mod buffer;
mod descriptor;
mod device;
mod image;
mod prelude;
mod renderer;

#[cfg(feature = "window")]
mod surface;

#[doc(inline)]
pub use self::{
  image::Image,
  mesh::{Mesh, Vertex},
  renderer::{Canvas, Render, Renderer},
};

#[cfg(feature = "window")]
#[doc(inline)]
pub use self::surface::Surface;

#[doc(inline)]
pub use gfx_hal::device::OutOfMemory;

use self::{
  alloc::alloc,
  backend::Backend,
  buffer::{Buffer, BufferKind},
  device::{device, Device},
};
