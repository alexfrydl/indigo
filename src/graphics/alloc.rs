// Copyright © 2020 Lexi Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::prelude::*;

/// Allocates a block of device memory that meets the given requirements.
pub fn alloc(requirements: hal::memory::Requirements) -> Result<backend::Memory> {
  let device = device()?;

  let memory_type_id =
    match device.memory_properties.memory_types.iter().enumerate().position(|(id, t)| {
      requirements.type_mask & (1 << id) != 0
        && t
          .properties
          .contains(hal::memory::Properties::CPU_VISIBLE | hal::memory::Properties::COHERENT)
    }) {
      Some(t) => t,
      None => fail!("No CPU-visible, coherent memory types available."),
    };

  let memory = unsafe { device.allocate_memory(memory_type_id.into(), requirements.size)? };

  Ok(memory)
}
