#version 330 core

#define NUM_POINT_LIGHTS       find_and_replace_with_max_number_of_point_lights
#define NUM_DIRECTIONAL_LIGHTS find_and_replace_with_max_number_of_directional_lights
#define NUM_SPOT_LIGHTS        find_and_replace_with_max_number_of_spot_lights


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

struct LightComponents {
    vec3 ambient_component;
    vec3 diffuse_component;
    vec3 specular_component;
};


uniform sampler2D texture0;
uniform sampler2D texture1;

in vec2 texturecoords;
in vec3 point_colour;
in float point_opacity;
in vec3 normal_vector;
in vec3 fragment_position;


uniform vec3 camera_viewpos;

uniform float object_shininess;

uniform PointLight point_lights[NUM_POINT_LIGHTS];
uniform DirectionalLight directional_lights[NUM_DIRECTIONAL_LIGHTS];
uniform SpotLight spot_lights[NUM_SPOT_LIGHTS];



out vec4 fragment_colour;



LightComponents calculate_directional_light(DirectionalLight light, vec3 norm, vec3 view_direction) {
    vec3 light_dir = normalize(-light.direction);
    float diffuse_magnitude = max(dot(norm, light_dir), 0);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), object_shininess);

    vec3 ambient_component  = light.ambient_colour;
    vec3 diffuse_component  = light.diffuse_colour  * diffuse_magnitude;
    vec3 specular_component = light.specular_colour * specular_magnitude;

    return LightComponents(ambient_component, diffuse_component, specular_component);
}

LightComponents calculate_point_light(PointLight light, vec3 norm, vec3 view_direction, bool attenuate_ambient) {
    vec3 light_dir = normalize(light.position - fragment_position);
    vec3 reflect_dir = reflect(-light_dir, norm);

    float lf_distance = length(light.position - fragment_position);
    float attenuation = light.attenuation_factor / (light.attenuation_factor + lf_distance*lf_distance);    

    float diffuse_magnitude = max(dot(norm, light_dir), 0); 
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), object_shininess);

    vec3 ambient_component  = light.ambient_colour;
    vec3 diffuse_component  = light.diffuse_colour  * diffuse_magnitude;
    vec3 specular_component = light.specular_colour * specular_magnitude;

    if(attenuate_ambient) {
        ambient_component *= attenuation;
    }
    diffuse_component  *= attenuation;
    specular_component *= attenuation;

    return LightComponents(ambient_component, diffuse_component, specular_component);    
}

LightComponents calculate_spot_light(SpotLight light, vec3 norm, vec3 view_direction, bool attenuate_ambient) {
    vec3 light_dir = normalize(light.position - fragment_position);
    vec3 reflect_dir = reflect(-light_dir, norm);

    float lf_distance = length(light.position - fragment_position);
    float attenuation = light.attenuation_factor / (light.attenuation_factor + lf_distance*lf_distance);    

    float diffuse_magnitude = max(dot(norm, light_dir), 0);
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), object_shininess);

    float fragment_theta = dot(light_dir, normalize(-light.direction));
    float epsilon = light.inner_cutoff_angle - light.outer_cutoff_angle;
    float cone_intensity = clamp((fragment_theta - light.outer_cutoff_angle) / epsilon, 0.0, 1.0); 

    vec3 ambient_component  = light.ambient_colour;
    vec3 diffuse_component  = light.diffuse_colour  * diffuse_magnitude;
    vec3 specular_component = light.specular_colour * specular_magnitude;

    if(attenuate_ambient) {
        ambient_component *= attenuation;
    }
    ambient_component  *= cone_intensity;
    diffuse_component  *= cone_intensity * attenuation;
    specular_component *= cone_intensity * attenuation;
   
    return LightComponents(ambient_component, diffuse_component, specular_component);
}

vec3 texture_map_light(LightComponents light, vec3 diffuse_map, vec3 specular_map) {
    return
        light.ambient_component  * diffuse_map +
        light.diffuse_component  * diffuse_map +
        light.specular_component * specular_map
    ;
}

LightComponents add2(LightComponents self, LightComponents other) {
    return LightComponents(
        self. ambient_component + other. ambient_component,
        self. diffuse_component + other. diffuse_component,
        self.specular_component + other.specular_component
    );
}



void main() {
    vec3 diffuse_map  = vec3(texture(texture0, texturecoords));
    vec3 specular_map = vec3(texture(texture1, texturecoords));

    vec3 norm = normalize(normal_vector);
    vec3 view_direction = normalize(camera_viewpos - fragment_position);

    //LightComponents d_light = calculate_directional_light(directional_light, norm, view_direction);
    //LightComponents p_light = calculate_point_light(point_light, norm, view_direction, false);
    //LightComponents s_light = calculate_spot_light(spot_light, norm, view_direction, false);
//
    //vec3 dl2 = texture_map_light(d_light, diffuse_map, specular_map);
    //vec3 pl2 = texture_map_light(p_light, diffuse_map, specular_map);
    //vec3 sl2 = texture_map_light(s_light, diffuse_map, specular_map);

    LightComponents total = LightComponents(
        vec3(0., 0., 0), vec3(0., 0., 0), vec3(0., 0., 0)
    );

    //LightComponents total = calculate_directional_light(directional_lights[0], norm, view_direction);


    for(int i = 0; i < NUM_DIRECTIONAL_LIGHTS; i++) {
        total = add2(total, calculate_directional_light(directional_lights[i], norm, view_direction));
    }
    for(int i = 0; i < NUM_POINT_LIGHTS; i++) {
        total = add2(total, calculate_point_light(point_lights[i], norm, view_direction, false));
    }
    for(int i = 0; i < NUM_SPOT_LIGHTS; i++) {
        total = add2(total, calculate_spot_light(spot_lights[i], norm, view_direction, false));
    }

    vec3 textured_colour = texture_map_light(total, diffuse_map, specular_map);


    //vec3 result = dl2+pl2+sl2;


    //fragment_colour = vec4(d_light, point_opacity);
    //fragment_colour = vec4(dl2, point_opacity);
    //fragment_colour = vec4(pl2, point_opacity);
    //fragment_colour = vec4(sl2, point_opacity);
    fragment_colour = vec4(textured_colour, point_opacity);

    //fragment_colour = vec4(result, point_opacity);
    //fragment_colour = vec4(point_colour, point_opacity);
}