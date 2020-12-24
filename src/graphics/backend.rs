// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_backend::Backend;

use super::prelude::*;
use crate::sync::Lazy;

#[cfg(target_os = "linux")]
const NAME: &str = "Vulkan";
#[cfg(target_os = "macos")]
const NAME: &str = "Metal";
#[cfg(target_os = "windows")]
const NAME: &str = "DirectX 12";

pub type Adapter = hal::adapter::Adapter<Backend>;
pub type Buffer = <Backend as hal::Backend>::Buffer;
pub type CommandBuffer = <Backend as hal::Backend>::CommandBuffer;
pub type CommandPool = <Backend as hal::Backend>::CommandPool;
pub type CommandQueue = <Backend as hal::Backend>::CommandQueue;
pub type DescriptorSet = <Backend as hal::Backend>::DescriptorSet;
pub type DescriptorSetLayout = <Backend as hal::Backend>::DescriptorSetLayout;
pub type Device = <Backend as hal::Backend>::Device;
pub type Framebuffer = <Backend as hal::Backend>::Framebuffer;
pub type GraphicsPipeline = <Backend as hal::Backend>::GraphicsPipeline;
pub type ImageView = <Backend as hal::Backend>::ImageView;
pub type Instance = <Backend as hal::Backend>::Instance;
pub type Memory = <Backend as hal::Backend>::Memory;
pub type QueueFamily = <Backend as hal::Backend>::QueueFamily;
pub type PipelineLayout = <Backend as hal::Backend>::PipelineLayout;
pub type RenderPass = <Backend as hal::Backend>::RenderPass;
pub type Semaphore = <Backend as hal::Backend>::Semaphore;
pub type ShaderModule = <Backend as hal::Backend>::ShaderModule;
pub type ShaderEntryPoint<'a> = hal::pso::EntryPoint<'a, Backend>;
pub type Surface = <Backend as hal::Backend>::Surface;
pub type SwapchainImage = <Surface as hal::window::PresentationSurface<Backend>>::SwapchainImage;

/// Returns a reference to a shared instance of the backend.
pub fn instance() -> Result<&'static Instance> {
  static INSTANCE: Lazy<Option<Instance>> = Lazy::new(|| {
    Instance::create("indigo", 1).ok().or_else(|| {
      error!("Failed to instantiate graphics backend. {} is not supported.", NAME);
      None
    })
  });

  INSTANCE.as_ref().ok_or_else(|| fail::err!("The graphics backend is not available."))
}
