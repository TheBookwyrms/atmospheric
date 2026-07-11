#version 330 core

struct Material {
    sampler2D diffuse_map;
    sampler2D specular_map;
    float shininess; // specular power
};

struct PointLight {
    vec3 position;

    vec3 ambient_colour;
    vec3 diffuse_colour;
    vec3 specular_colour;

    float attenuation_factor;
};

struct DirectionalLight {
    vec3 direction;

    vec3 ambient_colour;
    vec3 diffuse_colour;
    vec3 specular_colour;
};

struct SpotLight {
    vec3 position;
    vec3 direction;

    vec3 ambient_colour;
    vec3 diffuse_colour;
    vec3 specular_colour;

    float inner_cutoff_angle;
    float outer_cutoff_angle;

    float attenuation_factor;
};


uniform sampler2D texture0;
uniform sampler2D texture1;

in vec2 texturecoords;
in vec3 point_colour;
in float point_opacity;
in vec3 normal_vector;
in vec3 fragment_position;


uniform vec3 camera_viewpos;

uniform Material object_material;

uniform PointLight point_light;
uniform DirectionalLight directional_light;
uniform SpotLight spot_light;


out vec4 fragment_colour;


vec3 calculate_directional_light(DirectionalLight light, vec3 norm, vec3 view_direction, float shininess, vec3 diffuse_map, vec3 specular_map) {
    vec3 light_dir = normalize(-light.direction);
    float diffuse_magnitude = max(dot(norm, light_dir), 0);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), shininess);

    vec3 ambient_component  = light.ambient_colour  * diffuse_map;
    vec3 diffuse_component  = light.diffuse_colour  * diffuse_map  * diffuse_magnitude;
    vec3 specular_component = light.specular_colour * specular_map * specular_magnitude;

    return ambient_component + diffuse_component + specular_component;
}

vec3 calculate_point_light(PointLight light, vec3 norm, vec3 view_direction, vec3 fragment_position, float shininess, bool attenuate_ambient, vec3 diffuse_map, vec3 specular_map) {
    vec3 light_dir = normalize(light.position - fragment_position);
    vec3 reflect_dir = reflect(-light_dir, norm);

    float lf_distance = length(light.position - fragment_position);
    float attenuation = light.attenuation_factor / (light.attenuation_factor + lf_distance*lf_distance);    

    float diffuse_magnitude = max(dot(norm, light_dir), 0); 
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), shininess);

    vec3 ambient_component  = light.ambient_colour  * diffuse_map;
    vec3 diffuse_component  = light.diffuse_colour  * diffuse_map  * diffuse_magnitude;
    vec3 specular_component = light.specular_colour * specular_map * specular_magnitude;

    if(attenuate_ambient) {
        ambient_component *= attenuation;
    }
    diffuse_component  *= attenuation;
    specular_component *= attenuation;

    return ambient_component + diffuse_component + specular_component;    
}

vec3 calculate_spot_light(SpotLight light, vec3 norm, vec3 view_direction, vec3 fragment_position, float shininess, bool attenuate_ambient, vec3 diffuse_map, vec3 specular_map) {
    vec3 light_dir = normalize(light.position - fragment_position);
    vec3 reflect_dir = reflect(-light_dir, norm);

    float lf_distance = length(light.position - fragment_position);
    float attenuation = light.attenuation_factor / (light.attenuation_factor + lf_distance*lf_distance);    

    float diffuse_magnitude = max(dot(norm, light_dir), 0);
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), shininess);

    float fragment_theta = dot(light_dir, normalize(-light.direction));
    float epsilon = light.inner_cutoff_angle - light.outer_cutoff_angle;
    float cone_intensity = clamp((fragment_theta - light.outer_cutoff_angle) / epsilon, 0.0, 1.0); 

    vec3 ambient_component  = light.ambient_colour  * diffuse_map;
    vec3 diffuse_component  = light.diffuse_colour  * diffuse_map  * diffuse_magnitude;
    vec3 specular_component = light.specular_colour * specular_map * specular_magnitude;

    if(attenuate_ambient) {
        ambient_component *= attenuation;
    }
    ambient_component  *= cone_intensity;
    diffuse_component  *= cone_intensity * attenuation;
    specular_component *= cone_intensity * attenuation;
   
    return ambient_component + diffuse_component + specular_component;
}



void main() {

    vec3 diffuse_map  = vec3(texture(texture0, texturecoords));
    vec3 specular_map = vec3(texture(texture1, texturecoords));

    // the ambient lighting component of a point's light
    // is the colour of the ambient light, multiplied by the strength of the ambient light
    //vec3 ambient_component = point_light.ambient_colour * object_material.ambient_reflected_colour;
    //vec3 ambient_component = point_light.ambient_colour;
    vec3 ambient_component = point_light.ambient_colour * diffuse_map;


    // the normalised normal vector of the point
    vec3 norm = normalize(normal_vector);
    // the direction pointing from the light towards the current fragment's position
    vec3 point_light_dir = normalize(point_light.position - fragment_position);
    vec3 directional_light_dir = normalize(-directional_light.direction);

    //point_light_dir = directional_light_dir;

    vec3 view_direction = normalize(camera_viewpos - fragment_position);
    vec3 halfway_vector = normalize(point_light_dir + view_direction);


    float light_fragment_distance    = length(point_light.position - fragment_position);
    float attenuation = point_light.attenuation_factor / (point_light.attenuation_factor + light_fragment_distance*light_fragment_distance);    

    
    // the diffuse component of light is based off the angle between the normal vector and light vector
    // the closer they are together, the larger the dot product value is
    // then the max of (dot, 0) is taken
    // this is because a negative dot product value means the vectors are opposite
    // in this case, we are looking at the face opposite of light, which should get no diffuse light
    float diffuse_light = max(dot(norm, point_light_dir), 0);
    //vec3 diffuse_component = diffuse_light * point_light.diffuse_colour;
    vec3 diffuse_component = diffuse_light * point_light.diffuse_colour * diffuse_map;  

    vec3 reflect_dir = reflect(-point_light_dir, norm);
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), object_material.shininess);
    //float specular_magnitude = pow(max(dot(norm, halfway_vector), 0.0), object_material.shininess);
    //vec3 specular_component = specular_magnitude * point_light.specular_colour;
    vec3 specular_component = specular_magnitude * point_light.specular_colour * specular_map;



    //vec3 result = point_colour * (ambient_component + diffuse_component + specular_component);
    //vec3 result = (ambient_component + diffuse_component + specular_component);
    
    //vec3 result = point_colour * (attenuation * ambient_component + attenuation * diffuse_component + attenuation * specular_component);
    
    //ambient_component  *= 1;//attenuation;
    diffuse_component  *= attenuation;
    specular_component *= attenuation;

    float fragment_theta = dot(point_light_dir, normalize(-spot_light.direction));
    float epsilon = spot_light.inner_cutoff_angle - spot_light.outer_cutoff_angle;
    float intensity = clamp((fragment_theta - spot_light.outer_cutoff_angle) / epsilon, 0.0, 1.0);    
    diffuse_component  *= intensity;
    specular_component *= intensity;
    vec3 result = (ambient_component + diffuse_component + specular_component);

    //vec3 result;
    //if(fragment_theta > spot_light.inner_cutoff_angle) {
    //    result = (ambient_component + diffuse_component + specular_component);
    //} else {
    //    result = spot_light.ambient_colour * diffuse_map;
    //}

    vec3 diffuse_map2  = vec3(texture(texture0, texturecoords));
    vec3 specular_map2 = vec3(texture(texture1, texturecoords));
    vec3 norm2 = normalize(normal_vector);
    vec3 view_direction2 = normalize(camera_viewpos - fragment_position);
    vec3 d_light = calculate_directional_light(directional_light, norm2, view_direction2, object_material.shininess, diffuse_map2, specular_map2);
    vec3 p_light = calculate_point_light(point_light, norm2, view_direction2, fragment_position, object_material.shininess, false, diffuse_map2, specular_map2);
    vec3 s_light = calculate_spot_light(spot_light, norm2, view_direction2, fragment_position, object_material.shininess, false, diffuse_map2, specular_map2);


    //fragment_colour = vec4(d_light, point_opacity);
    //fragment_colour = vec4(p_light, point_opacity);
    fragment_colour = vec4(s_light, point_opacity);

    //fragment_colour = vec4(result, point_opacity);
    //fragment_colour = vec4(point_colour, point_opacity);
}