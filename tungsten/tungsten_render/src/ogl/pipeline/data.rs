pub static FRAGMENT_SHADER: &'static str = r#"
#version 330
#define M_PI 3.1415926535897932384626433832795

uniform float reflectance = 0.2;
uniform float roughness = 0.80;
uniform float metalness = 1.0;
uniform vec3 spec_color = vec3(1.0,0.86,0.57);

in VS_OUT{
	vec3 N;
	vec3 L;
	vec3 V;
	vec3 color;
} fs_in;

out vec4 color;

void main(){
	vec3 N = normalize(fs_in.N);
	vec3 V = normalize(fs_in.V);
	vec3 L = normalize(fs_in.L);
    vec3 H = normalize(L + V);

    //D term
    float alpha = roughness * roughness;
    float alpha2 = alpha * alpha;
    float n_dot_h = clamp(dot(N,H),0.0,1.0);
    float term = n_dot_h * n_dot_h * (alpha2 -1) +1;
    vec3 D = vec3(alpha2 / (M_PI * (term * term)));

    //F term
    vec3 f0 = spec_color;
    vec3 F = f0 + (1 - f0) * pow(1-dot(N,V),5);

    //temp G term
    float k = alpha /2;
    float n_dot_l = clamp(dot(N,L),0.0,1.0);
    float n_dot_v = clamp(dot(N,V),0.0,1.0);
    float G1 = n_dot_v / (n_dot_v * (1.0 -k) + k);
    float G2 = n_dot_l / (n_dot_l * (1.0 -k) + k);
    vec3 G = vec3(clamp(G1*G2,0.0,1.0));
    
    vec3 specular = (D * F* G) / (4 * dot(N,L)*dot(N,V));
    vec3 diffuse = vec3(reflectance/M_PI);

    vec3 lightColor = n_dot_l * (specular + mix(diffuse,vec3(0,0,0),metalness));
    color = vec4(lightColor*spec_color,0);
}
"#;

pub static VERTEX_SHADER: &'static str = r#"
#version 330

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normals;

uniform mat4 MVPMat;
uniform mat4 MVMat;
uniform mat4 PMat;

uniform vec3 Ligpos = vec3(10,10.0,10.0);

out VS_OUT{
    vec3 N;
    vec3 L;
    vec3 V;
    vec3 color;
} vs_out;

void main()
{
    vec4 P = MVMat * vec4(position,1.0);
    vs_out.N = mat3(MVMat) * normals;
    vs_out.L = (MVMat * vec4(Ligpos,1.0) - P).xyz;
    vs_out.V = P.xyz * -1;
    vs_out.color = normalize(position) /9 ;

    gl_Position = MVPMat * vec4(position,1.0);
}
"#;

pub static SIMPLE_FRAGMENT_SHADER: &'static str = r#"
#version 330


in VS_OUT{
	vec3 color;
} fs_in;

out vec4 color;

void main(){
    color = vec4(fs_in.color,1.0);
}
"#;

pub static SIMPLE_VERTEX_SHADER: &'static str = r#"
#version 330

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normals;

uniform mat4 MVPMat;
uniform mat4 MVMat;
uniform mat4 PMat;

out VS_OUT{
    vec3 color;
} vs_out;

void main()
{
    vs_out.color = position;

    gl_Position =  MVPMat * vec4(position,1.0);
}
"#;
