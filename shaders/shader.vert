#version 450 core
layout (location = 0) in vec3 aPosition;

uniform mat4 viewProjection;
uniform mat4 model;

void main()
{
    gl_Position = viewProjection * model * vec4(aPosition, 1.0);
}