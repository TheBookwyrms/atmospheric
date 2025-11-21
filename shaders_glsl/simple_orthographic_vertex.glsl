#version 330 core

layout (location = 0) in vec3 point_pos;
layout (location = 1) in vec3 point_col;
layout (location = 2) in float point_o;

out vec3 point_colour;
out float point_opacity;

uniform mat4 orthographic_projection;
uniform mat4 camera_transformation;
uniform mat4 world_transform;

void main() {
    gl_Position = orthographic_projection * camera_transformation * world_transform * vec4(point_pos, 1.0);
    point_colour = point_col;
    point_opacity = point_o;
}