use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub struct Renderer {
    ctx: WebGlRenderingContext,
}

impl Renderer {
    pub fn new(ctx: WebGlRenderingContext) -> Self {
        Self {
            ctx
        }
    }
}