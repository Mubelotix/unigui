#version 450

layout(location=0) in vec2 a_position;
layout(location=1) in vec2 a_tex_coords;
layout(location=2) in vec4 text_color;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec4 text_color_out;

layout(set=0, binding=0)
uniform Uniforms {
    float screen_width;
    float screen_height;
};

void main() {
    v_tex_coords = a_tex_coords;
    text_color_out = text_color;
    vec2 p = ((2.0 / vec2(screen_width, screen_height)) * a_position - 1.0) * vec2(1.0, -1.0);
    gl_Position = vec4(p, 0.0, 1.0);
}
