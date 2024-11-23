#version 140

in vec3 position;
uniform uvec2 screen_size;
// out vec4 gl_Position;
void main() {
    gl_Position = vec4(position.x/screen_size.x,  position.y/screen_size.y, position.z, 1.0);
}