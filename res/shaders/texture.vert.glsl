#version 330 core

layout(location = 0) in vec4 position;
layout(location = 1) in vec2 texCoord;

out vec2 vTexCoord;

uniform mat4 uMVPMatrix;

void main()
{
    gl_Position = uMVPMatrix * position;
    vTexCoord = texCoord;
}