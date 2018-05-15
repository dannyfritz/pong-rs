#version 140

in vec2 position;
out vec3 v_color;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

void main() {
  mat4 modelview = view * model;
  v_color = ((projection * modelview * vec4(position, 0.0, 1.0)).xyz + 1) / 2;
  gl_Position = projection * modelview * vec4(position, 0.0, 1.0);
}
