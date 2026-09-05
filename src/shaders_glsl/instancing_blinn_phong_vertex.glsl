#version 330 core
layout (location = 0) in vec3 point_positions;
layout (location = 1) in vec4 point_colours;
layout (location = 2) in vec3 point_norms;
layout (location = 3) in vec2 texture_coords;
layout (location = 4) in vec3 point_ambient_reflected_light;
layout (location = 5) in vec3 point_diffuse_reflected_light;
layout (location = 6) in vec3 point_specular_reflected_light;
layout (location = 7) in float point_shininess;
// mat4 consumes locations 8, 9, 10, 11
layout (location = 8) in mat4 transformation_matrix;




struct Material {
    vec3 ambient_reflected_colour;
    vec3 diffuse_reflected_colour;
    vec3 specular_reflected_colour;
    float shininess; // specular power
};



out vec4 point_colour;
out vec3 normal_vector;
out vec2 texturecoords;
out Material point_material;
out vec3 fragment_position;

uniform mat4 orthographic_projection;
uniform mat4 camera_transformation;
uniform mat4 world_transform;

void main() {
    vec4 position = transformation_matrix * world_transform * vec4(point_positions, 1.0);
    gl_Position = orthographic_projection * camera_transformation * position;
    //gl_Position = orthographic_projection * camera_transformation * world_transform * vec4(point_pos, 1.0);
    texturecoords = texture_coords;
    fragment_position = vec3(position);
    point_colour = point_colours;
    //vec3 normal_vector = mat3(transpose(inverse(world_transform))) * point_n;
    //normal_vector = point_n;
    normal_vector = vec3(world_transform * vec4(point_norms, 1.0));
    point_material = Material(
        point_ambient_reflected_light,
        point_diffuse_reflected_light,
        point_specular_reflected_light,
        point_shininess
    );
}