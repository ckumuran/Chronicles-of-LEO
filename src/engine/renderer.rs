use crate::engine::mesh::Mesh;
use crate::engine::shader::Shader;

pub struct Renderer {
    shader: Shader,
    cube: Mesh,
}

impl Renderer {
    pub fn new() -> Self {
        let vertex_shader = r#"
            #version 330 core

            layout (location = 0) in vec3 aPos;

            void main()
            {
                gl_Position = vec4(aPos, 1.0);
            }
        "#;

        let fragment_shader = r#"
            #version 330 core

            out vec4 FragColor;

            void main()
            {
                FragColor = vec4(0.8, 0.2, 0.2, 1.0);
            }
        "#;

        Self {
            shader: Shader::new(vertex_shader, fragment_shader),

            cube: Mesh::cube(),
        }
    }

    pub fn render(&self) {
        self.shader.use_program();

        self.cube.draw();
    }
}
