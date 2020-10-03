use crate::shader; // Macro
use super::shader; // Module

pub struct ImguiRender {
    shader: shader::ShaderProgram
}

impl ImguiRender {
    pub fn new(ctx: &glow::Context) -> Self {
        Self {
            shader: shader!("imgui").build(ctx).expect("Imgui shader failed to build")
        }
    }
}

impl Drop for ImguiRender {
    fn drop(&mut self) {
        unimplemented!()
    }
}