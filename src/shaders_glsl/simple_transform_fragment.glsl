#version 330 core

in vec4 point_colour;

out vec4 fragment_colour;

void main() {
    fragment_colour = point_colour;
}