// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::prelude::*;

/// A description of a descriptor bind operation.
#[derive(Clone, Copy)]
pub struct Bind<'a> {
  /// The index of the descriptor in the set.
  pub index: usize,
  /// The binding to assign.
  pub binding: Binding<'a>,
  /// The descriptor set containing the descriptor to bind.
  pub set: &'a Set,
}

/// A binding to assign to a descriptor.
#[derive(Clone, Copy)]
pub enum Binding<'a> {
  UniformBuffer(&'a backend::Buffer),
}

/// One of the possible kinds of descriptors.
#[derive(Clone, Copy)]
pub enum Kind {
  UniformBuffer,
}

/// A descriptor layout that describes the bindings in a descriptor set.
pub struct Layout {
  _bindings: Vec<hal::pso::DescriptorSetLayoutBinding>,
  counts: gfx_descriptor::DescriptorCounts,
  device: &'static Device,
  layout: ManuallyDrop<backend::DescriptorSetLayout>,
}

/// Allocates and frees descriptor sets of any layout.
pub struct Pool {
  alloc_buffer: Vec<gfx_descriptor::DescriptorSet<Backend>>,
  device: &'static Device,
  inner: gfx_descriptor::DescriptorAllocator<Backend>,
}

/// A descriptor set containing bindings to device resources.
pub struct Set {
  set: Option<gfx_descriptor::DescriptorSet<Backend>>,
}

impl Pool {
  /// Creates a new allocator.
  pub fn new() -> Result<Self> {
    Ok(Self {
      alloc_buffer: Vec::with_capacity(128),
      device: device()?,
      inner: unsafe { gfx_descriptor::DescriptorAllocator::new() },
    })
  }

  /// Allocates a number of descriptor sets with a given layout.
  pub fn alloc(&mut self, layout: &Layout, count: u32, output: &mut impl Extend<Set>) -> Result {
    self.inner.allocate(
      self.device,
      layout.raw(),
      &layout.counts,
      count,
      &mut self.alloc_buffer,
    )?;

    output.extend(self.alloc_buffer.drain(..).map(|set| Set { set: Some(set) }));

    Ok(())
  }

  /// Allocates one descriptor set in a given layout.
  pub fn alloc_one(&mut self, layout: &Layout) -> Result<Set> {
    let mut sets = ArrayVec::<[_; 1]>::new();

    self.alloc(layout, 1, &mut sets)?;

    Ok(sets.into_iter().next().unwrap())
  }

  /// Frees previously allocated descriptor sets.
  pub fn free(&mut self, sets: impl IntoIterator<Item = Set>) {
    unsafe { self.inner.free(sets.into_iter().map(|mut s| s.set.take().unwrap())) }
  }

  /// Frees a previously allocated descriptor set.
  pub fn free_one(&mut self, set: Set) {
    self.free(iter::once(set))
  }
}

impl Layout {
  /// Creates a new descriptor layout with the given bindings.
  pub fn new<I>(bindings: I) -> Result<Self>
  where
    I: IntoIterator,
    I::Item: Borrow<Kind>,
  {
    let device = device()?;

    let bindings = bindings
      .into_iter()
      .enumerate()
      .map(|(i, binding)| hal::pso::DescriptorSetLayoutBinding {
        binding: i as u32,
        count: 1,
        immutable_samplers: false,
        stage_flags: hal::pso::ShaderStageFlags::ALL,
        ty: match binding.borrow() {
          Kind::UniformBuffer => hal::pso::DescriptorType::Buffer {
            format: hal::pso::BufferDescriptorFormat::Structured { dynamic_offset: false },
            ty: hal::pso::BufferDescriptorType::Uniform,
          },
        },
      })
      .collect_vec();

    let layout = unsafe { device.create_descriptor_set_layout(&bindings, &[])? };
    let counts = bindings.iter().cloned().collect();

    Ok(Self { _bindings: bindings, counts, device, layout: ManuallyDrop::new(layout) })
  }

  /// Returns a reference to the underlying backend descriptor set layout.
  pub fn raw(&self) -> &backend::DescriptorSetLayout {
    &*self.layout
  }
}

impl Set {
  /// Returns a reference to the underlying backend descriptor set.
  pub fn raw(&self) -> &backend::DescriptorSet {
    self.set.as_ref().unwrap().raw()
  }
}

// Implement Drop to destroy resources.

impl Drop for Pool {
  fn drop(&mut self) {
    unsafe {
      self.inner.clear(self.device);
    }
  }
}

impl Drop for Layout {
  fn drop(&mut self) {
    unsafe {
      self.device.destroy_descriptor_set_layout(ManuallyDrop::take(&mut self.layout));
    }
  }
}

// Implement Drop for DescriptorSet to write a warning message that sets must be
// manually freed.

impl Drop for Set {
  fn drop(&mut self) {
    if self.set.is_some() {
      warn!(
        "An in-use descriptor set was dropped. All descriptor sets should be freed explicitly."
      );
    }
  }
}
