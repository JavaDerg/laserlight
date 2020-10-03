#[macro_use]
use super::shader;

pub struct ImguiRender {
    shader: shader::ShaderProgram
}

impl ImguiRender {
    pub fn new() -> Self {
        Self {
            shader: shader!("imgui")
        }
    }
}

impl Drop for ImguiRender {
    fn drop(&mut self) {
        unimplemented!()
    }
}