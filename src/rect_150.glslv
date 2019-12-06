#version 150 core

uniform vec4 rect;
uniform vec4 color;
out vec4 v_Color;
void main() {
    v_Color = color;

    float left = rect.x;
    float top = rect.y;
    float right = rect.z;
    float bottom = rect.w;

    switch (gl_VertexID) {
        case 0:
            gl_Position = vec4(left, top, 0.0, 1.0);
            break;
        case 1:
            gl_Position = vec4(right, top, 0.0, 1.0);
            break;
        case 2:
            gl_Position = vec4(left, bottom, 0.0, 1.0);
            break;
        case 3:
            gl_Position = vec4(right, bottom, 0.0, 1.0);
            break;
    }
}