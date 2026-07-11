#version 330 core

struct Material {
    vec3 ambient_reflected_colour;
    vec3 diffuse_reflected_colour;
    vec3 specular_reflected_colour;
    float shininess; // specular power
};

struct Light {
    vec3 position;

    vec3 ambient_colour;
    vec3 diffuse_colour;
    vec3 specular_colour;
};

in vec3 point_colour;
in float point_opacity;
in vec3 normal_vector;
in vec3 fragment_position;


//uniform float ambient_strength;
//uniform vec3 ambient_colour;

//uniform float diffuse_strength;
//// uniform float diffuse_base;

//uniform vec3 light_source_pos;
//uniform vec3 light_source_colour;

//uniform float specular_strength;
uniform vec3 camera_viewpos;
//uniform float specular_power;

uniform Material object_material;
uniform Light point_light;


out vec4 fragment_colour;

void main() {

    // the ambient lighting component of a point's light
    // is the colour of the ambient light, multiplied by the strength of the ambient light
    //vec3 ambient_component = ambient_strength * ambient_colour;
    //vec3 ambient_component = ambient_strength * point_light.ambient_colour * object_material.ambient_reflected_colour;
    vec3 ambient_component = point_light.ambient_colour * object_material.ambient_reflected_colour;
    //vec3 ambient_component = ambient_strength * light_source_colour;

    // the normalised normal vector of the point
    vec3 norm = normalize(normal_vector);
    // the direction pointing from the light towards the current fragment's position
    vec3 light_dir = normalize(point_light.position - fragment_position);
    vec3 view_direction = normalize(camera_viewpos - fragment_position);
    vec3 halfway_vector = normalize(light_dir + view_direction);

    
    // the diffuse component of light is based off the angle between the normal vector and light vector
    // the closer they are together, the larger the dot product value is
    // then the max of (dot, 0) is taken
    // this is because a negative dot product value means the vectors are opposite
    // in this case, we are looking at the face opposite of light, which should get no diffuse light
    float diffuse_light = max(dot(normal_vector, light_dir), 0);
    //vec3 diffuse_component = diffuse_light * light_source_colour * object_material.diffuse_reflected_colour;
    //vec3 diffuse_component = diffuse_light * point_light.diffuse_colour;
    vec3 diffuse_component = diffuse_light * point_light.diffuse_colour * object_material.diffuse_reflected_colour;



    vec3 reflect_dir = reflect(-light_dir, norm);
    //float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), specular_power);
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), object_material.shininess);
    //vec3 specular_component = specular_strength * specular_magnitude * light_source_colour;
    //vec3 specular_component = specular_strength * specular_magnitude * point_light.specular_colour * object_material.specular_reflected_colour;
    vec3 specular_component = specular_magnitude * point_light.specular_colour * object_material.specular_reflected_colour;

    //float specular_magnitude = pow(max(dot(norm, halfway_vector), 0.0), specular_power);
    //vec3 specular_component = specular_strength * specular_magnitude * light_source_colour;
    //float specular_magnitude = pow(max(dot(norm, halfway_vector), 0.0), object_material.shininess);
    //vec3 specular_component = specular_magnitude * light_source_colour * object_material.specular_reflected_colour;


    vec3 result = point_colour * (ambient_component + diffuse_component + specular_component);

    fragment_colour = vec4(result, point_opacity);
    //fragment_colour = vec4(point_colour, point_opacity);
}