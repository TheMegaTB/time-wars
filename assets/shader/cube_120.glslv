#version 120

attribute vec3 a_pos;
attribute vec4 a_color;
varying vec4 v_Color;

uniform mat4 u_model_view_proj;

void main() {
    v_Color = vec4(a_color);
    gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
}
