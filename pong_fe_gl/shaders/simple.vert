#version 140

in vec2 position;
out vec2 v_color;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

void main() {
  mat4 modelview = view * model;
  v_color = position;
  gl_Position = projection * modelview * vec4(position, 0.0, 1.0);
}
