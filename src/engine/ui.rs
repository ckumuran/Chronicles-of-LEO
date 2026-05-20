use crate::engine::shader::Shader;

pub struct UI {

    crosshair_vao: u32,

    crosshair_vbo: u32,
}

impl UI {

    pub fn new() -> Self {

        let mut vao = 0;
        let mut vbo = 0;

        let vertices: [f32; 8] = [

            // vertical

             0.0, -0.02,
             0.0,  0.02,

            // horizontal

            -0.02, 0.0,
             0.02, 0.0,
        ];

        unsafe {

            gl::GenVertexArrays(
                1,
                &mut vao,
            );

            gl::GenBuffers(
                1,
                &mut vbo,
            );

            gl::BindVertexArray(
                vao
            );

            gl::BindBuffer(
                gl::ARRAY_BUFFER,
                vbo,
            );

            gl::BufferData(

                gl::ARRAY_BUFFER,

                (vertices.len() *
                 std::mem::size_of::<f32>())
                    as isize,

                vertices.as_ptr()
                    as *const _,

                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(

                0,

                2,

                gl::FLOAT,

                gl::FALSE,

                (2 *
                 std::mem::size_of::<f32>())
                    as i32,

                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(
                0
            );
        }

        Self {

            crosshair_vao: vao,

            crosshair_vbo: vbo,
        }
    }

    pub fn draw_crosshair(
        &self,

        shader: &Shader,
    ) {

        shader.use_program();

        unsafe {

            gl::Disable(
                gl::DEPTH_TEST
            );

            gl::BindVertexArray(
                self.crosshair_vao
            );

            gl::DrawArrays(
                gl::LINES,
                0,
                4,
            );

            gl::Enable(
                gl::DEPTH_TEST
            );
        }
    }

    pub fn draw_debug(
        &self,

        fps: u32,

        chunk_count: usize,
    ) {

        println!(
            "FPS: {} | Chunks: {}",
            fps,
            chunk_count,
        );
    }
}
