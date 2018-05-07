#version 140

in vec2 v_color;
out vec4 color;

void main() {
    color = vec4(v_color, 0.0, 1.0);
}
