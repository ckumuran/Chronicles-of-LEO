use glam::Mat4;

use crate::engine::camera::Camera;
use crate::engine::chunk_mesh::ChunkMesh;
use crate::engine::mesh::Mesh;
use crate::engine::shader::Shader;
use crate::engine::world::World;

pub struct Renderer {

    shader: Shader,

    meshes: Vec<Mesh>,
}

impl Renderer {

    pub fn new() -> Self {

        let vertex_shader = r#"
            #version 330 core

            layout (location = 0) in vec3 aPos;

            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 projection;

            void main()
            {
                gl_Position =
                    projection *
                    view *
                    model *
                    vec4(aPos, 1.0);
            }
        "#;

        let fragment_shader = r#"
            #version 330 core

            out vec4 FragColor;

            void main()
            {
                FragColor = vec4(
                    0.4,
                    0.8,
                    0.3,
                    1.0
                );
            }
        "#;

        let shader =
            Shader::new(
                vertex_shader,
                fragment_shader,
            );

        let world =
            World::new();

        let mut meshes = Vec::new();

        for (_, chunk) in &world.chunks {

            let chunk_mesh =
                ChunkMesh::build(chunk);

            let mesh =
                Mesh::from_vertices(
                    &chunk_mesh.vertices
                );

            meshes.push(mesh);
        }

        Self {
            shader,
            meshes,
        }
    }

    pub fn render(
        &self,
        camera: &Camera,
    ) {

        self.shader.use_program();

        let model =
            Mat4::IDENTITY;

        let view =
            camera.get_view_matrix();

        let projection =
            Mat4::perspective_rh_gl(
                70.0_f32.to_radians(),
                1280.0 / 720.0,
                0.1,
                1000.0,
            );

        self.shader.set_mat4(
            "model",
            &model,
        );

        self.shader.set_mat4(
            "view",
            &view,
        );

        self.shader.set_mat4(
            "projection",
            &projection,
        );

        for mesh in &self.meshes {
            mesh.draw();
        }
    }
}
