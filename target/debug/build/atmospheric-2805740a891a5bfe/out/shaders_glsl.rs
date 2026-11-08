pub const INSTANCING_BLINN_PHONG_FRAGMENT : &'static str = "#version 330 core

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

struct Material {
    vec3 ambient_reflected_colour;
    vec3 diffuse_reflected_colour;
    vec3 specular_reflected_colour;
    float shininess; // specular power
};




uniform PointLight point_lights[NUM_POINT_LIGHTS];
uniform DirectionalLight directional_lights[NUM_DIRECTIONAL_LIGHTS];
uniform SpotLight spot_lights[NUM_SPOT_LIGHTS];






uniform sampler2D texture0;
uniform sampler2D texture1;


in vec4 point_colour;
in vec3 normal_vector;
in vec2 texturecoords;
in Material point_material;
in vec3 fragment_position;


uniform vec3 camera_viewpos;

out vec4 fragment_colour;


LightComponents calculate_directional_light(DirectionalLight light, vec3 norm, vec3 view_direction) {
    vec3 light_dir = normalize(-light.direction);
    float diffuse_magnitude = max(dot(norm, light_dir), 0);

    vec3 reflect_dir = reflect(-light_dir, norm);
    //float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), point_material.shininess);

    vec3 halfway_dir = normalize(light_dir + view_direction);
    float specular_magnitude = pow(max(dot(norm, halfway_dir), 0.0), point_material.shininess);


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
    //float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), point_material.shininess);

    vec3 halfway_dir = normalize(light_dir + view_direction);
    float specular_magnitude = pow(max(dot(norm, halfway_dir), 0.0), point_material.shininess);

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
    //float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), point_material.shininess);

    vec3 halfway_dir = normalize(light_dir + view_direction);
    float specular_magnitude = pow(max(dot(norm, halfway_dir), 0.0), point_material.shininess);

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

vec3 texture_material_map_light(LightComponents light, vec3 diffuse_map, vec3 specular_map) {
    return
        light.ambient_component  * diffuse_map  * point_material.ambient_reflected_colour +
        light.diffuse_component  * diffuse_map  * point_material.diffuse_reflected_colour +
        light.specular_component * specular_map * point_material.specular_reflected_colour
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


    LightComponents total = LightComponents(
        vec3(0., 0., 0), vec3(0., 0., 0), vec3(0., 0., 0)
    );


    for(int i = 0; i < NUM_DIRECTIONAL_LIGHTS; i++) {
        total = add2(total, calculate_directional_light(directional_lights[i], norm, view_direction));
    }
    for(int i = 0; i < NUM_POINT_LIGHTS; i++) {
        total = add2(total, calculate_point_light(point_lights[i], norm, view_direction, false));
    }
    for(int i = 0; i < NUM_SPOT_LIGHTS; i++) {
        total = add2(total, calculate_spot_light(spot_lights[i], norm, view_direction, false));
    }

    vec3 textured_colour = texture_material_map_light(total, diffuse_map, specular_map);

    //fragment_colour = vec4(textured_colour, 1.0);
    fragment_colour = point_colour * vec4(textured_colour, 1.0);
}";pub const SIMPLE_TRANSFORM_VERTEX : &'static str = "#version 330 core
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
}";pub const TWO_TEXTURE_VERTEX : &'static str = "#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texture_coords;

out vec2 texturecoords;

uniform mat4 orthographic_projection;
uniform mat4 camera_transformation;
uniform mat4 world_transform;

void main() {
    gl_Position = orthographic_projection * camera_transformation * world_transform * vec4(position, 1.0);
    texturecoords = texture_coords;
}";pub const PHONG_ORTHOGRAPHIC_FRAGMENT : &'static str = "#version 330 core

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

struct Material {
    vec3 ambient_reflected_colour;
    vec3 diffuse_reflected_colour;
    vec3 specular_reflected_colour;
    float shininess; // specular power
};

in vec3 point_colour;
in float point_opacity;
in vec3 normal_vector;
in vec3 fragment_position;

uniform vec3 camera_viewpos;

uniform Material object_material;

uniform PointLight point_lights[NUM_POINT_LIGHTS];
uniform DirectionalLight directional_lights[NUM_DIRECTIONAL_LIGHTS];
uniform SpotLight spot_lights[NUM_SPOT_LIGHTS];

out vec4 fragment_colour;


LightComponents calculate_directional_light(DirectionalLight light, vec3 norm, vec3 view_direction) {
    vec3 light_dir = normalize(-light.direction);
    float diffuse_magnitude = max(dot(norm, light_dir), 0);
    vec3 reflect_dir = reflect(-light_dir, norm);
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), object_material.shininess);

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
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), object_material.shininess);

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
    float specular_magnitude = pow(max(dot(view_direction, reflect_dir), 0.0), object_material.shininess);

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

LightComponents add2(LightComponents self, LightComponents other) {
    return LightComponents(
        self. ambient_component + other. ambient_component,
        self. diffuse_component + other. diffuse_component,
        self.specular_component + other.specular_component
    );
}

vec3 object_map_light(LightComponents light, Material object_material) {
    return
        light.ambient_component  * object_material.ambient_reflected_colour +
        light.diffuse_component  * object_material.diffuse_reflected_colour +
        light.specular_component * object_material.specular_reflected_colour
    ;
}








void main() {
    vec3 norm = normalize(normal_vector);
    vec3 view_direction = normalize(camera_viewpos - fragment_position);

    LightComponents total = LightComponents(
        vec3(0., 0., 0), vec3(0., 0., 0), vec3(0., 0., 0)
    );

    for(int i = 0; i < NUM_DIRECTIONAL_LIGHTS; i++) {
        total = add2(total, calculate_directional_light(directional_lights[i], norm, view_direction));
    }
    for(int i = 0; i < NUM_POINT_LIGHTS; i++) {
        total = add2(total, calculate_point_light(point_lights[i], norm, view_direction, false));
    }
    for(int i = 0; i < NUM_SPOT_LIGHTS; i++) {
        total = add2(total, calculate_spot_light(spot_lights[i], norm, view_direction, false));
    }


    vec3 textured_colour = object_map_light(total, object_material);

    fragment_colour = vec4(textured_colour, point_opacity);
}";pub const PHONG_TEXTURE_FRAGMENT : &'static str = "#version 330 core

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
}";pub const INSTANCING_BLINN_PHONG_VERTEX : &'static str = "#version 330 core
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
}";pub const SIMPLE_ORTHOGRAPHIC_VERTEX : &'static str = "#version 330 core

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
}";pub const PHONG_ORTHOGRAPHIC_VERTEX : &'static str = "#version 330 core

layout (location = 0) in vec3 point_pos;
layout (location = 1) in vec3 point_col;
layout (location = 2) in float point_o;
layout (location = 3) in vec3 point_n;

out vec3 point_colour;
out float point_opacity;
out vec3 normal_vector;
out vec3 fragment_position;


uniform mat4 orthographic_projection;
uniform mat4 camera_transformation;
uniform mat4 world_transform;

void main() {
    gl_Position = orthographic_projection * camera_transformation * world_transform * vec4(point_pos, 1.0);
    fragment_position = vec3(world_transform * vec4(point_pos, 1.0));
    point_colour = point_col;
    point_opacity = point_o;
    //vec3 normal_vector = mat3(transpose(inverse(world_transform))) * point_n;
    //normal_vector = point_n;
    normal_vector = vec3(world_transform * vec4(point_n, 1.0));
}";pub const SIMPLE_TEXTURE_VERTEX : &'static str = "#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texture_coords;

out vec2 texturecoords;

uniform mat4 orthographic_projection;
uniform mat4 camera_transformation;
uniform mat4 world_transform;

void main() {
    gl_Position = orthographic_projection * camera_transformation * world_transform * vec4(position, 1.0);
    texturecoords = texture_coords;
}";pub const SIMPLE_TRANSFORM_FRAGMENT : &'static str = "#version 330 core

in vec4 point_colour;

out vec4 fragment_colour;

void main() {
    fragment_colour = point_colour;
}";pub const TWO_TEXTURE_FRAGMENT : &'static str = "#version 330 core
out vec4 FragColor;
  
in vec2 texturecoords;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    vec4 color = mix(texture(texture1, texturecoords), texture(texture2, texturecoords), 0.8);
    FragColor = vec4(vec3(color), 1.0);
}";pub const SIMPLE_TEXTURE_FRAGMENT : &'static str = "#version 330 core
out vec4 FragColor;
  
in vec2 texturecoords;

uniform sampler2D texture1;

void main() {
    FragColor = texture(texture1, texturecoords);
}";pub const PHONG_TEXTURE_VERTEX : &'static str = "#version 330 core
layout (location = 0) in vec3 point_pos;
layout (location = 1) in vec3 point_col;
layout (location = 2) in float point_o;
layout (location = 3) in vec3 point_n;
layout (location = 4) in vec2 texture_coords;

out vec2 texturecoords;
out vec3 point_colour;
out float point_opacity;
out vec3 normal_vector;
out vec3 fragment_position;

uniform mat4 orthographic_projection;
uniform mat4 camera_transformation;
uniform mat4 world_transform;

void main() {
    gl_Position = orthographic_projection * camera_transformation * world_transform * vec4(point_pos, 1.0);
    texturecoords = texture_coords;
    fragment_position = vec3(world_transform * vec4(point_pos, 1.0));
    point_colour = point_col;
    point_opacity = point_o;
    //vec3 normal_vector = mat3(transpose(inverse(world_transform))) * point_n;
    //normal_vector = point_n;
    normal_vector = vec3(world_transform * vec4(point_n, 1.0));
}";pub const SIMPLE_ORTHOGRAPHIC_FRAGMENT : &'static str = "#version 330 core

in vec3 point_colour;
in float point_opacity;

out vec4 fragment_colour;

void main() {
    fragment_colour = vec4(point_colour, point_opacity);
}";