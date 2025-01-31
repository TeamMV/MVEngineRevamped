#version 450

precision highp float;

layout (location = 0) in vec4 fColor;
layout (location = 1) in vec2 fUv;
layout (location = 2) in float fTex;
layout (location = 3) in vec2 fRes;
layout (location = 4) in vec3 fFragPos;

layout(location = 0) out vec4 outColor;

struct Light {
    vec2 pos;
    vec4 color;
    float intensity;
    float range;
    float falloff;
};

uniform sampler2D TEX_SAMPLER[16];
uniform Light LIGHTS[50];
uniform int NUM_LIGHTS;

void main() {
    vec4 baseColor;

    if (fTex > 0.0) {
        vec4 texColor = texture(TEX_SAMPLER[int(fTex) - 1], fUv);
        baseColor = mix(texColor, vec4(fColor.rgb, texColor.a), fColor.a);
    } else {
        baseColor = fColor;
    }

    vec4 lighting = vec4(0.0);

    for (int i = 0; i < NUM_LIGHTS; i++) {
        Light light = LIGHTS[i];

        float dist = length(fFragPos.xy - light.pos);
        float distanceAttenuation = 1.0 / (1.0 + light.falloff * dist * dist);

        float rangeAttenuation = smoothstep(light.range, 0.0, dist);

        float finalIntensity = light.intensity * distanceAttenuation * rangeAttenuation;

        vec4 lightEffect = light.color * finalIntensity;
        lighting += lightEffect;
    }

    outColor = baseColor * clamp(lighting + vec4(0.2, 0.2, 0.2, 1.0), 0.0, 1.0);
}
