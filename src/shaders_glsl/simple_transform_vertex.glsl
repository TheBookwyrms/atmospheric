#version 330 core
layout (location = 0) in vec3 point_positions;
layout (location = 1) in vec4 point_colours;
// mat4 consumes locations 2, 3, 4, 5
layout (location = 2) in mat4 transformation_matrix;


out vec4 point_colour;

uniform mat4 orthographic_projection;
uniform mat4 camera_transformation;
uniform mat4 world_transform;

void main() {
    vec4 position = transformation_matrix * world_transform * vec4(point_positions, 1.0);
    gl_Position = orthographic_projection * camera_transformation * position;
    point_colour = point_colours;
}