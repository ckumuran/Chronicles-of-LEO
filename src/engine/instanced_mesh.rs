use std::mem;
use std::os::raw::c_void;

use glam::Mat4;

pub struct InstancedMesh {

    vao: u32,

    vbo: u32,

    instance_vbo: u32,

    vertex_count: i32,

    instance_count: i32,
}

impl InstancedMesh {

    pub fn new(
        vertices: &[f32],
        transforms: &[Mat4],
    ) -> Self {

        let mut vao = 0;
        let mut vbo = 0;
        let mut instance_vbo = 0;

        unsafe {

            gl::GenVertexArrays(
                1,
                &mut vao,
            );

            gl::GenBuffers(
                1,
                &mut vbo,
            );

            gl::GenBuffers(
                1,
                &mut instance_vbo,
            );

            gl::BindVertexArray(
                vao
            );

            // VERTEX BUFFER

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
                7 * mem::size_of::<f32>();

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

            // AO

            gl::VertexAttribPointer(
                2,
                1,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,

                (5 * mem::size_of::<f32>())
                    as *const c_void,
            );

            gl::EnableVertexAttribArray(2);

            // LIGHT

            gl::VertexAttribPointer(
                3,
                1,
                gl::FLOAT,
                gl::FALSE,
                stride as i32,

                (6 * mem::size_of::<f32>())
                    as *const c_void,
            );

            gl::EnableVertexAttribArray(3);

            // INSTANCE MATRICES

            gl::BindBuffer(
                gl::ARRAY_BUFFER,
                instance_vbo,
            );

            gl::BufferData(
                gl::ARRAY_BUFFER,

                (transforms.len() *
                 mem::size_of::<Mat4>())
                    as isize,

                transforms.as_ptr()
                    as *const c_void,

                gl::STATIC_DRAW,
            );

            let vec4_size =
                mem::size_of::<[f32;4]>();

            for i in 0..4 {

                gl::VertexAttribPointer(
                    4 + i,
                    4,
                    gl::FLOAT,
                    gl::FALSE,

                    mem::size_of::<Mat4>()
                        as i32,

                    (i * vec4_size)
                        as *const c_void,
                );

                gl::EnableVertexAttribArray(
                    4 + i
                );

                gl::VertexAttribDivisor(
                    4 + i,
                    1,
                );
            }
        }

        Self {

            vao,

            vbo,

            instance_vbo,

            vertex_count:
                (vertices.len() / 7)
                    as i32,

            instance_count:
                transforms.len()
                    as i32,
        }
    }

    pub fn draw(&self) {

        unsafe {

            gl::BindVertexArray(
                self.vao
            );

            gl::DrawArraysInstanced(
                gl::TRIANGLES,

                0,

                self.vertex_count,

                self.instance_count,
            );
        }
    }
}
