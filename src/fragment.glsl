#version 130

uniform mat4 mat_v;
uniform sampler2D tex;

out vec4 frag_color;
in vec3 pos_eye, norm_eye;
in vec2 frag_uv;

vec3 l1_pos = vec3(0.0, -3.0, 0.0);
vec3 l1_Ls = vec3(0.3, 0.3, 1.0);
vec3 l1_Ld = vec3(0.0, 0.0, 1.0);

vec3 l2_pos = vec3(0.0, 3.0, 0.0);
vec3 l2_Ls = vec3(0.3, 1.0, 0.3);
vec3 l2_Ld = vec3(0.0, 1.0, 0.0);

vec3 La = vec3(0.0, 0.0, 0.0);

vec3 light(vec3 pos, vec3 Ld, vec3 Ls, float exp) {
    vec3 light_eye = vec3(mat_v * vec4(pos, 1.0));
    vec3 dist = light_eye - pos_eye;
    vec3 dir = normalize(dist);
    float dd = max(dot(dir, norm_eye), 0.0);
    vec3 diffuse = Ld * dd;

    vec3 refl = reflect(-dir, norm_eye);
    vec3 surf_eye = normalize(-pos_eye);
    float ds = max(dot(refl, surf_eye), 0.0);
    vec3 specular = Ls * pow(ds, exp);

    return diffuse + specular;
}

void main() {
    vec3 color = vec3(texture(tex, frag_uv));

    color += light(l1_pos, l1_Ld, l1_Ls, 100);
    color += light(l2_pos, l2_Ld, l2_Ls, 100);

    frag_color = vec4(color, 1.0);
}
