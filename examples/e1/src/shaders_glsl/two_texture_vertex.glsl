#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texture_coords;

out vec2 texturecoords;

uniform mat4 orthographic_projection;
uniform mat4 camera_transformation;
uniform mat4 world_transform;

void main() {
    gl_Position = orthographic_projection * camera_transformation * world_transform * vec4(position, 1.0);
    texturecoords = texture_coords;
}