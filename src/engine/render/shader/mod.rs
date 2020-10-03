use crate::glc;
use glow::HasContext;

pub struct ShaderBuilder {
    vert_src: String,
    frag_src: String,
}

pub struct ShaderProgram {
    vert: glow::WebShaderKey,
    frag: glow::WebShaderKey,
    program: glow::WebProgramKey,
}

#[macro_export]
macro_rules! shader {
    ($shader:literal) => {
        shader!($shader, $shader)
    };
    ($vert:literal, $frag:literal) => {
        $crate::engine::render::shader::ShaderBuilder::new(
            include_str!(concat!("shader/", $vert, ".vert")),
            include_str!(concat!("shader/", $frag, ".frag")),
        )
    };
}

impl ShaderBuilder {
    pub fn new<S1, S2>(vert_src: S1, frag_src: S2) -> Self
    where
        S1: ToString,
        S2: ToString,
    {
        Self {
            vert_src: vert_src.to_string(),
            frag_src: frag_src.to_string(),
        }
    }

    pub fn build(&self, ctx: &glow::Context) -> Result<ShaderProgram, String> {
        let vert = sub_compile_shader(ctx, glow::VERTEX_SHADER, self.vert_src.as_str())?;
        let frag = sub_compile_shader(ctx, glow::FRAGMENT_SHADER, self.frag_src.as_str())?;
        let program = glc!(ctx, ctx.create_program()).expect("Unable to create Program");
        glc!(ctx, ctx.attach_shader(program, vert));
        glc!(ctx, ctx.attach_shader(program, frag));
        glc!(ctx, ctx.link_program(program));
        let log = glc!(ctx, ctx.get_program_info_log(program));
        if !glc!(ctx, ctx.get_program_link_status(program)) {
            log::error!("Shader failed to compile:\n{}", log);
            return Err(log);
        } else if !log.is_empty() {
            log::info!("Program linked successfully and returned log:\n{}", log);
        }
        Ok(ShaderProgram {
            vert,
            frag,
            program,
        })
    }
}

fn sub_compile_shader(ctx: &glow::Context, shader_type: u32, src: &str) -> Result<glow::WebShaderKey, String> {
    let shader = glc!(ctx, ctx.create_shader(shader_type))?;
    glc!(ctx, ctx.shader_source(shader, src));
    glc!(ctx, ctx.compile_shader(shader));
    let log = glc!(ctx, ctx.get_shader_info_log(shader));
    if !glc!(ctx, ctx.get_shader_compile_status(shader)) {
        log::error!("Shader failed to compile:\n{}", src);
        return Err(log);
    } else if !log.is_empty() {
        log::info!("Shader compiled successfully and returned log:\n{}", log);
    }
    Ok(shader)
}

