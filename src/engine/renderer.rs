use glam::Mat4;

use crate::engine::camera::Camera;
use crate::engine::greedy_mesher::GreedyMesher;
use crate::engine::mesh::Mesh;
use crate::engine::shader::Shader;
use crate::engine::world::World;

pub struct Renderer {

    shader: Shader,

    world: World,
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
                    0.3,
                    0.8,
                    0.3,
                    1.0
                );
            }
        "#;

        Self {

            shader:
                Shader::new(
                    vertex_shader,
                    fragment_shader,
                ),

            world:
                World::new(),
        }
    }

    pub fn render(
        &mut self,

        camera: &Camera,
    ) {

        self.world.update(
            camera.position.x,
            camera.position.z,
        );

        self.shader.use_program();

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
            "view",
            &view,
        );

        self.shader.set_mat4(
            "projection",
            &projection,
        );

        let chunk_keys: Vec<_> =
            self.world
                .chunk_manager
                .chunks
                .keys()
                .cloned()
                .collect();

        for key in chunk_keys {

            let chunk =
                self.world
                    .chunk_manager
                    .chunks
                    .get_mut(&key)
                    .unwrap();

            let render_data =
                self.world
                    .chunk_manager
                    .render_data
                    .get_mut(&key)
                    .unwrap();

            // REMESH ONLY IF DIRTY

            if chunk.dirty {

                let vertices =
                    GreedyMesher::build(chunk);

                render_data.mesh =
                    Some(
                        Mesh::from_vertices(
                            &vertices
                        )
                    );

                chunk.dirty = false;
            }

            let model =
                Mat4::from_translation(
                    glam::Vec3::new(
                        key.0 as f32 * 16.0,
                        key.1 as f32 * 16.0,
                        key.2 as f32 * 16.0,
                    )
                );

            self.shader.set_mat4(
                "model",
                &model,
            );

            if let Some(mesh) =
                &render_data.mesh
            {
                mesh.draw();
            }
        }
    }
}
