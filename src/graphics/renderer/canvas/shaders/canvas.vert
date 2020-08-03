#version 450

// Per-frame constants.
layout(set = 0, binding = 0) uniform FrameConstants {
  mat4 frame_projection;
};

// Per-object constants.
layout(push_constant) uniform PushConstants {
  mat4 push_transform;
  vec4 push_tint;
};

// Input.
layout(location = 0) in vec2 in_position;

// Output.
layout(location = 0) out vec4 out_tint;

void main() {
  // Multiply vertex position by frame projection and push constant transform.
  gl_Position = frame_projection * push_transform * vec4(in_position, 0.0, 1.0);

  // Use push constant tint.
  out_tint = push_tint;
}
