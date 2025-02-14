// Copyright (c) 2017 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

// This module exposes what is needed in order to draw with a deferred rendering system.
//
// The main code is in the `system` module, while the other modules implement the different kinds
// of lighting sources.

use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

pub use self::system::{DrawPass, Frame, FrameSystem, LightingPass, Pass};

mod ambient_lighting_system;
mod directional_lighting_system;
mod point_lighting_system;
mod system;

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct LightingVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}
