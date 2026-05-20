#version 330 core

out vec4 FragColor;

in vec2 TexCoord;
in float AO;
in float Light;

uniform sampler2D atlasTexture;

void main()
{
    vec4 color =
        texture(
            atlasTexture,
            TexCoord
        );

    if(color.a < 0.1)
        discard;

    color.rgb *= AO;
    color.rgb *= Light;

    FragColor = color;
}
