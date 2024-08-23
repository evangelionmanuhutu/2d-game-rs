#version 450 core
layout (location = 0) out vec4 oColor;

layout (location = 0) in vec2 vTexCoord;

uniform sampler2D sampler0;

void main()
{
    vec3 tex = texture(sampler0, vTexCoord).rgb;
    oColor = vec4(tex, 1.0);
}