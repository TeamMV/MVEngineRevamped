#version 450

in vec2 fUv;

layout(location = 0) out vec4 outColor;

uniform sampler2D COLOR;
//uniform sampler2D DEPTH;

void main() {
    vec4 color = texture(COLOR, fUv);

    outColor = vec4(1.0 - color.r, 1.0 - color.g, 1.0 - color.b, 1.0);
}