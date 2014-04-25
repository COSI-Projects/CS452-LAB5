#version 130

in vec3 pos;
in vec3 norm;
in vec2 uv;
out vec3 pos_eye, norm_eye;
out vec2 frag_uv;
uniform mat4 mat_m, mat_v, mat_p;

void main() {
    pos_eye = vec3(mat_v * mat_m * vec4(pos, 1.0));
    norm_eye = vec3(mat_v * mat_m * vec4(norm, 0.0));
	gl_Position = mat_p * vec4(pos_eye, 1.0);
    frag_uv = uv;
}
