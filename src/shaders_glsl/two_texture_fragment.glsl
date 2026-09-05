#version 330 core
out vec4 FragColor;
  
in vec2 texturecoords;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    vec4 color = mix(texture(texture1, texturecoords), texture(texture2, texturecoords), 0.8);
    FragColor = vec4(vec3(color), 1.0);
}