#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform float waterTime;

void main()
{
    vec3 position = aPos;

    position.y +=
        sin(
            position.x * 0.12 +
            waterTime * 1.5
        ) * 0.04;

    position.y +=
        cos(
            position.z * 0.08 +
            waterTime
        ) * 0.03;

    TexCoord = aTexCoord;

    gl_Position =
        projection *
        view *
        model *
        vec4(position, 1.0);
}
