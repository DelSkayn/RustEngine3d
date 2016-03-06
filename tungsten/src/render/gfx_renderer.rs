pub use super::super::gfx::{
    Device,
    Resources,
    Factory,
};

pub use super::super::gfx::format::{
    Depth,
    I8Scaled,
    R8_G8_B8,
    Srgb8,
};

use super::Renderer;

use std::marker::PhantomData;

gfx_constant_struct!(LightInfo{
    pos: [f32; 4],
});

gfx_vertex_struct!( BlitVertex{
    pos: [I8Scaled; 3] = "a_pos",
    tex_coord: [I8Scaled; 2] = "a_TexCoord",
});

gfx_pipeline!(blit{
    vbuf: gfx::VertexBuffer<BlitVertex> = (),
    tex: gfx::TextureSampler<[f32; 4]> = "u_Tex",
    out: gfx::RenderTarget<Srgb8> = "o_Color",
    
});

static BLIT_VERTEX_SRC: &'static [u8] = b"
#version 150 core

in vec3 a_Pos;
in vec2 a_TexCoord;
out vec2 v_TexCoord;

void main(){
    v_TexCoord = a_TexCoord;
    gl_Poition = vec4(a_Pos,1.0);
}
";

static BLIT_FRAGMANT_SRC: &'static [u8] = b"
#version 150 core
uniform sampler2D u_Tex;
in vec2 v_TexCoord;
out vec4 o_Color;

void main(){
    vec4 tex = texture(u_Tex, v_TexCoord);
    o_Color = tex;
}
";

gfx_vertex_struct!(CubeVertex{
    pos: [I8Scaled; 3] = "a_Pos",
});

gfx_pipeline!(light {
    vbuf: gfx::VertexBuffer<CubeVertex> = (),
    transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
    light_pos_buf: gfx::ConstantBuffer<LightInfo> = "u_LightPosBlock",
    radius: gfx::Global<f32> = "u_Radius",
    cam_pos: gfx::Global<[f32; 3]> = "u_FrameRes",
    tex_pos: gfx::TextureSampler<[f32; 4]> = "u_TexPos",
    tex_normal: gfx::TextureSampler<[f32; 4]> = "u_TexNormal",
    tex_diffuse: gfx::TextureSampler<[f32; 4]> = "u_TexDiffuse",
    out_color: gfx::BlendTarget<[f32; 4]> = 
        ("o_Color", gfx::state::MASK_ALL, gfx::preset::blend::ADD),
    out_depth: gfx::DepthTarget<Depth> = 
        gfx::preset::depth::LESS_EQUAL_TEST,
});

pub static LIGHT_VERTEX_SRC: &'static [u8] = b"
    #version 150 core
    uniform mat4 u_Transform;
    uniform float u_Radius;

    in vec3 a_Pos;
    out vec3 v_LightPos;

    const int NUM_LIGHTS = 250;

    layout(std140)
    uniform u_LightPosBlock {
        vec4 offs[NUM_LIGHTS];
    };
    void main() {
        v_LightPos = offs[gl_InstanceID].xyz;
        gl_Position = u_Transform * vec4(u_Radius * a_Pos + offs[gl_InstanceID].xyz, 1.0);
    }
";

pub static LIGHT_FRAGMENT_SRC: &'static [u8] = b"
    #version 150 core
    uniform float u_Radius;
    uniform vec3 u_CameraPos;
    uniform vec2 u_FrameRes;
    uniform sampler2D u_TexPos;
    uniform sampler2D u_TexNormal;
    uniform sampler2D u_TexDiffuse;

    in vec3 v_LightPos;
    out vec4 o_Color;

    void main() {
        vec2 texCoord = gl_FragCoord.xy / u_FrameRes;
        vec3 pos     = texture(u_TexPos,     texCoord).xyz;
        vec3 normal  = texture(u_TexNormal,  texCoord).xyz;
        vec3 diffuse = texture(u_TexDiffuse, texCoord).xyz;
        vec3 light    = v_LightPos;
        vec3 to_light = normalize(light - pos);
        vec3 to_cam   = normalize(u_CameraPos - pos);
        vec3 n = normalize(normal);
        float s = pow(max(0.0, dot(to_cam, reflect(-to_light, n))), 20.0);
        float d = max(0.0, dot(n, to_light));
        float dist_sq = dot(light - pos, light - pos);
        float scale = max(0.0, 1.0-dist_sq/(u_Radius*u_Radius));
        vec3 res_color = d*vec3(diffuse) + vec3(s);
        o_Color = vec4(scale*res_color, 1.0);
    }
";

pub struct GfxRenderer<D: Device,R: Resources,F: Factory<R>>{
    _p: PhantomData<R>,
    device: D,
    factory: F,
}

impl<D: Device,R: Resources,F: Factory<R>> GfxRenderer<D,R,F>{

    pub fn new(device: D,factory: F) -> Self{
        GfxRenderer{
            device: device,
            factory: factory,
            _p: PhantomData,
        }
    }
}
impl<D: Device,R: Resources,F: Factory<R>> Renderer for GfxRenderer<D,R,F>{
    fn render(&mut self){}
}

