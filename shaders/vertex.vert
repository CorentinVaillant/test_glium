#version 140

in vec4 position;
uniform uvec2 screen_size;

void main() {
    gl_Position = vec4(position.x/screen_size.x,  position.y/screen_size.y, position.z, 1.0);
}