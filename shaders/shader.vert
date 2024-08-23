#version 450 core
layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 viewProjection;
uniform mat4 model;

layout (location = 0) out vec2 vTexCoord;

void main()
{
    gl_Position = viewProjection * model * vec4(aPosition, 1.0);
    vTexCoord = aTexCoord;
}