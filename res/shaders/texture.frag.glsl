#version 330 core

layout(location = 0) out vec4 color;

uniform vec4 uColor;
uniform sampler2D uTextureSlot;

in vec2 vTexCoord;

void main()
{
    vec4 texColor = texture(uTextureSlot, vTexCoord);
    color = texColor;
}