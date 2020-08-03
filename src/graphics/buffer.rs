// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::prelude::*;

/// A buffer of device memory.
pub struct Buffer<T: 'static> {
  buffer: ManuallyDrop<backend::Buffer>,
  device: &'static Device,
  mapped: &'static mut [T],
  memory: ManuallyDrop<backend::Memory>,
}

/// One of the possible kinds of `Buffer`.
pub enum BufferKind {
  Vertex,
  Index,
  Uniform,
}

impl<T> Buffer<T> {
  /// Creates a new buffer.
  pub fn new(kind: BufferKind, len: usize) -> Result<Self> {
    let device = device()?;

    // Convert the given `kind` to a HAL buffer usage.

    let usage = match kind {
      BufferKind::Vertex => hal::buffer::Usage::VERTEX,
      BufferKind::Index => hal::buffer::Usage::INDEX,
      BufferKind::Uniform => hal::buffer::Usage::UNIFORM,
    };

    // Compute the byte size (length * size of element).

    let byte_size = match len.checked_mul(mem::size_of::<T>()) {
      Some(s) => s,
      None => fail!("Buffer would require more than `usize` bytes."),
    };

    // Create the buffer.

    let mut buffer = unsafe { device.create_buffer(byte_size as u64, usage)? };

    // Allocate the buffer memory.

    let requirements = unsafe { device.get_buffer_requirements(&buffer) };

    let memory = match alloc(requirements) {
      Ok(m) => m,
      Err(err) => {
        unsafe { device.destroy_buffer(buffer) };
        fail!("Failed to allocate buffer memory. {}", err);
      }
    };

    // Bind the memory to the buffer.

    if let Err(err) = unsafe { device.bind_buffer_memory(&memory, 0, &mut buffer) } {
      unsafe {
        device.destroy_buffer(buffer);
        device.free_memory(memory);
      }

      fail!("Failed to bind buffer memory. {}", err);
    }

    // Map the memory so it can be modified by the CPU.

    let mapped = match unsafe { device.map_memory(&memory, hal::memory::Segment::ALL) } {
      Ok(m) => unsafe { slice::from_raw_parts_mut(m as *mut T, len) },
      Err(err) => {
        unsafe {
          device.destroy_buffer(buffer);
          device.free_memory(memory);
        }

        fail!("Failed to map buffer memory. {}", err);
      }
    };

    // Return a new `Buffer`.

    Ok(Self {
      buffer: ManuallyDrop::new(buffer),
      device,
      mapped,
      memory: ManuallyDrop::new(memory),
    })
  }

  /// Returns a reference to the underlying backend buffer.
  pub(super) fn raw(&self) -> &backend::Buffer {
    &self.buffer
  }
}

// Implement Deref and DerefMut to access the mapped memory.

impl<T> Deref for Buffer<T> {
  type Target = [T];

  fn deref(&self) -> &Self::Target {
    self.mapped
  }
}

impl<T> DerefMut for Buffer<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.mapped
  }
}

// Implement `Drop` to manually drop device resources.

impl<T> Drop for Buffer<T> {
  fn drop(&mut self) {
    unsafe {
      self.device.destroy_buffer(ManuallyDrop::take(&mut self.buffer));
      self.device.free_memory(ManuallyDrop::take(&mut self.memory))
    }
  }
}
