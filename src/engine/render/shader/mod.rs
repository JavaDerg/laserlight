pub struct ShaderBuilder {
    vert_src: String,
    frag_src: String,
}

pub struct ShaderProgram {
    vert: usize,
    frag: usize,
    program: usize,
}

#[macro_export]
macro_rules! shader {
    ($vert:literal, $frag:literal) => {
        $self::ShaderBuilder::new(
            include_str!(concat!($vert, ".vert")),
            include_str!(concat!($frag, ".vert")),
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

    pub fn compile(&self, ctx: glow::Context) -> ShaderProgram {
        todo!()
    }
}
