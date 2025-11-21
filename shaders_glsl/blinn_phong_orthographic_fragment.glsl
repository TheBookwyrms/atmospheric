#version 330 core

in vec3 point_colour;
in float point_opacity;
//in vec3 norm;
//in vec3 fragment_position;


//uniform mat4 world_transform;


uniform float ambient_strength;
uniform vec3 ambient_colour;

//uniform float diffuse_strength;
//// uniform float diffuse_base;
//
//uniform vec3 light_source_pos;
//uniform vec3 light_source_colour;
//
//uniform float specular_strength;
//uniform vec3 camera_viewpos;
//uniform float specular_power;
//
//uniform mat4 light_y_transform;


out vec4 fragment_colour;

void main() {

    //vec3 a = gl_FragCoord.xyz;

    //vec3 ambient_light = ambient_strength * ambient_colour;


    //vec3 light_dir = normalize(light_source_pos - fragment_position);
    //
    //float diffuse_component = max(dot(norm, light_dir), 0);
    //vec3 diffuse_light = (diffuse_component * light_source_colour * diffuse_strength);
//
//
    //vec3 view_direction = normalize(fragment_position - camera_viewpos);
    ////vec3 view_direction = normalize(camera_viewpos - fragment_position);
//
    //vec3 light_reflect_dir = normalize(
    //        vec3(
    //            light_y_transform *vec4(
    //                normalize(reflect(-light_dir, norm)), 1
    //            ))
    //    );
    ////vec3 light_reflect_dir = normalize(reflect(-light_dir, norm));
//
    //// specular nonsense that doesn't work
    //float specular_magnitude = pow(max(dot(view_direction, light_reflect_dir), 0.0), specular_power);
    //vec3 specular_light = specular_strength * specular_magnitude * light_source_colour;

    //vec3 result = point_colour * (ambient_light + diffuse_light + specular_light);
    
    vec3 ambient_light = ambient_strength * ambient_colour;
    vec3 result = point_colour * (ambient_light); // doesn't work
    //vec3 result = point_colour * ambient_strength; // doesn't work
    //vec3 result = vec3(ambient_strength, ambient_strength, ambient_strength);
    //vec3 result = point_colour; // works
    //vec3 result = point_colour * diffuse_light; // doesn't work
    //vec3 result = point_colour * specular_light; // doesn't work
    

    fragment_colour = vec4(result, point_opacity);
    //fragment_colour = vec4(ambient_colour, point_opacity);
    //fragment_colour = vec4(point_colour, point_opacity);
}