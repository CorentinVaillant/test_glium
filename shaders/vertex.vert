#version 140

in vec4 position;
uniform uvec2 screen_size;
uniform mat4 projection_matrix;

void main() {
    gl_Position = projection_matrix * vec4(position.x/screen_size.x,  position.y/screen_size.y, position.z, 1.0);
}