#version 330 core

// input vertex data, different for all executions of this shaders
layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;

// output data; will be interpolated for each fragment
out vec4 frag_color;

uniform mat4 model_mat;
uniform mat4 view_mat;
uniform mat4 proj_mat;

void main() {
  frag_color = color;
  gl_Position = proj_mat * view_mat * model_mat * vec4(position, 1);
}
