#version 150 core

uniform sampler2D t_texture;

in vec4 v_Color;
in vec2 v_Uv;

out vec4 target;

void main() {
    vec3 aw = texture(t_texture, v_Uv).rgb;

    if(aw == vec3(0.0, 0.0, 0.0)) {
        target = 0.20 * v_Color;
    } else {
        target = vec4(aw, 1.0);
    }
}