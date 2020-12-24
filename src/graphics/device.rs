// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{backend, descriptor, prelude::*};

use crate::sync::{blocking::Mutex, Lazy};

/// A graphics device.
#[derive(Deref)]
pub struct Device {
  /// The adapter from which the graphics device was opened.
  pub adapter: backend::Adapter,
  /// The properties of the available device memory.
  pub memory_properties: hal::adapter::MemoryProperties,
  #[deref]
  device: backend::Device,
  /// The available command queues of the device.
  pub queues: Vec<Queue>,
}

/// A graphics device command queue.
#[derive(Deref)]
pub struct Queue {
  #[deref]
  queue: Mutex<backend::CommandQueue>,
  /// The family of the queue.
  pub family: backend::QueueFamily,
}

/// Returns a reference to the graphics device.
pub fn device() -> Result<&'static Device> {
  static DEVICE: Lazy<Option<Device>> = Lazy::new(|| {
    Device::new().map_err(|err| error!("Failed to open graphics device. {}", err)).ok()
  });

  DEVICE.as_ref().ok_or_else(|| fail::err!("The graphics device is not available."))
}

impl Device {
  /// Opens a graphics device from the best available adapter.
  fn new() -> Result<Self> {
    // First, select the best available graphics adapter.

    let adapters = backend::instance()?.enumerate_adapters();

    if adapters.is_empty() {
      fail!("No graphics adapter.");
    }

    debug!("Initializing…\n{}", fmt::AsDescription(&adapters));

    let adapter_index = adapters
      .iter()
      .position(|a| a.info.device_type == hal::adapter::DeviceType::DiscreteGpu)
      .or_else(|| {
        adapters.iter().position(|a| a.info.device_type == hal::adapter::DeviceType::IntegratedGpu)
      })
      .or_else(|| {
        adapters.iter().position(|a| a.info.device_type == hal::adapter::DeviceType::VirtualGpu)
      })
      .or_else(|| adapters.first().map(|_| 0))
      .ok_or_else(|| fail::err!("No suitable adapter is available."))?;

    let adapter = adapters.into_iter().nth(adapter_index).unwrap();
    let memory_properties = adapter.physical_device.memory_properties();

    // Then, open the device, requesting one queue from every queue family.

    let queue_requests: Vec<_> =
      adapter.queue_families.iter().map(|f| (f, &[1.0f32][..])).collect();

    let hal::adapter::Gpu { device, queue_groups } =
      unsafe { adapter.physical_device.open(&queue_requests, hal::Features::empty()) }?;

    // Extract the returned qeueues.

    let mut queues = Vec::new();

    if queue_groups.is_empty() {
      fail!("Device returned no queue groups.");
    }

    for group in queue_groups {
      queues.push(Queue {
        family: adapter
          .queue_families
          .iter()
          .find(|f| f.id() == group.family)
          .ok_or_else(|| fail::err!("Device returned a queue in an unknown family."))?
          .clone(),
        queue: group
          .queues
          .into_iter()
          .next()
          .ok_or_else(|| fail::err!("Device returned an empty queue group."))?
          .into(),
      });
    }

    // Log info about the device and return it.

    debug!(
      "Initialized with adapter {} {:?}.\n{}{}",
      adapter_index,
      fmt::style(&adapter.info.name).cyan().bright(),
      fmt::AsDescription(&memory_properties),
      fmt::AsDescription(&queues),
    );

    Ok(Self { adapter, device, memory_properties, queues })
  }

  /// Finds a queue that matches the given predicate.
  pub fn find_queue(
    &'static self,
    mut predicate: impl FnMut(&backend::QueueFamily) -> bool,
  ) -> Option<&'static Queue> {
    self.queues.iter().find(|q| predicate(&q.family))
  }

  /// Binds descriptors in descriptor sets.
  pub fn bind_descriptors<'a>(&self, writes: impl IntoIterator<Item = descriptor::Bind<'a>>) {
    unsafe {
      self.device.write_descriptor_sets(writes.into_iter().map(|w| hal::pso::DescriptorSetWrite {
        set: w.set.raw(),
        binding: w.index as u32,
        array_offset: 0,
        descriptors: iter::once(match w.binding {
          descriptor::Binding::UniformBuffer(buffer) => {
            hal::pso::Descriptor::Buffer(buffer, default())
          }
        }),
      }));
    }
  }
}

// Implement Describe for better logging of various types.

impl Describe for Vec<backend::Adapter> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for (i, adapter) in self.iter().enumerate() {
      write!(
        f,
        "{}  adapter {} {{ type = {:?}, name = {:?}, vendor_id = {:04x}, device_id = {:04x} }}",
        if i == 0 { "" } else { "\n" },
        i,
        fmt::style(&adapter.info.device_type).cyan().bright(),
        fmt::style(&adapter.info.name).cyan().bright(),
        fmt::style(adapter.info.vendor).cyan().bright(),
        fmt::style(adapter.info.device).cyan().bright(),
      )?;
    }

    Ok(())
  }
}

impl Describe for Vec<Queue> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for (i, queue) in self.iter().enumerate() {
      write!(
        f,
        "{}  queue {} {{ family = {}, type = {:?} }}",
        if i == 0 { "" } else { "\n" },
        i,
        fmt::style(queue.family.id().0).cyan().bright(),
        fmt::style(queue.family.queue_type()).cyan().bright(),
      )?;
    }

    Ok(())
  }
}

impl Describe for hal::adapter::MemoryProperties {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for (i, heap) in self.memory_heaps.iter().enumerate() {
      let mut props = hal::memory::Properties::empty();

      for types in self.memory_types.iter().filter(|t| t.heap_index == i) {
        props |= types.properties;
      }

      writeln!(
        f,
        "  heap {} {{ size = {} {}, properties = {:?} }}",
        i,
        fmt::style(heap / 1024 / 1024).cyan().bright(),
        fmt::style("mb").cyan(),
        fmt::style(props).cyan().bright(),
      )?;
    }

    Ok(())
  }
}
