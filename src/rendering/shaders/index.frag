#version 450

precision highp float;

layout (location = 0) in vec4 fColor;
layout (location = 1) in vec2 fUv;
layout (location = 2) in float fTex;
layout (location = 3) in vec2 fRes;
layout (location = 4) in vec3 fFragPos;

out vec4 outColor;

uniform sampler2D TEX_SAMPLER[16];

void main() {
    if (fTex > 0) {
        vec4 c = texture(TEX_SAMPLER[int(fTex) - 1], fUv);

        if (fColor.w > 0.0) {
            outColor = vec4(fColor.x, fColor.y, fColor.z, c.w);
        } else {
            outColor = c;
        }
    }
    else {
        outColor = fColor;
    }
}