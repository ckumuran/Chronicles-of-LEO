#version 330 core

out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D atlasTexture;
uniform float waterTime;

void main()
{
    vec2 uv = TexCoord;

    uv.x += waterTime * 0.05;
    uv.y += waterTime * 0.03;

    vec4 color =
        texture(
            atlasTexture,
            uv
        );

    color.rgb *= vec3(
        0.7,
        0.85,
        1.0
    );

    color.a = 0.75;

    FragColor = color;
}
