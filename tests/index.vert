#version 450

precision highp float;

layout (location = 0) in vec3 aVertPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aUv;
layout (location = 3) in float aTex;

uniform mat4 uProjection;
uniform mat4 uView;

uniform float uResX;
uniform float uResY;

out vec4 fColor;
out vec2 fUv;
out float fTex;
out vec2 fRes;

void main() {
    fColor = aColor;
    fUv = aUv;
    fTex = aTex;
    fRes = vec2(uResX, uResY);

    vec2 pos = aVertPos.xy;

    gl_Position = uProjection * uView * vec4(pos, aVertPos.z, 1.0);
}
