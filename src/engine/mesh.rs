use std::mem;
use std::os::raw::c_void;

pub struct Mesh {

    vao: u32,
    vbo: u32,

    vertex_count: i32,
}

impl Mesh {

    pub fn from_vertices(
        vertices: &[f32]
    ) -> Self {

        let mut vao = 0;
        let mut vbo = 0;

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
                 mem::size_of::<f32>())
                    as isize,

                vertices.as_ptr()
                    as *const c_void,

                gl::STATIC_DRAW,
            );

            let stride =
                5 * mem::size_of::<f32>();

            // POSITION

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(0);

            // UV

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,

                (3 * mem::size_of::<f32>())
                    as *const c_void,
            );

            gl::EnableVertexAttribArray(1);
        }

        Self {

            vao,
            vbo,

            vertex_count:
                (vertices.len() / 5) as i32,
        }
    }

    pub fn draw(&self) {

        unsafe {

            gl::BindVertexArray(
                self.vao
            );

            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                self.vertex_count,
            );
        }
    }
}
