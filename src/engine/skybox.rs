use glam::{
    Mat4,
    Vec3,
};

use crate::engine::shader::Shader;

pub struct Skybox {

    vao: u32,

    vbo: u32,

    pub time_of_day: f32,
}

impl Skybox {

    pub fn new() -> Self {

        let vertices: [f32; 108] = [

            -1.0,  1.0, -1.0,
            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0, -1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0,  1.0,

             1.0, -1.0, -1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0, -1.0,
             1.0, -1.0, -1.0,

            -1.0, -1.0,  1.0,
            -1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
             1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0,

            -1.0,  1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0,  1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0,  1.0, -1.0,

            -1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0,
        ];

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
                 std::mem::size_of::<f32>())
                    as isize,

                vertices.as_ptr()
                    as *const _,

                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(

                0,

                3,

                gl::FLOAT,

                gl::FALSE,

                (3 *
                 std::mem::size_of::<f32>())
                    as i32,

                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(
                0
            );
        }

        Self {

            vao,

            vbo,

            time_of_day: 0.0,
        }
    }

    pub fn update(
        &mut self,

        delta_time: f32,
    ) {

        self.time_of_day +=
            delta_time * 0.01;

        if self.time_of_day > 1.0 {

            self.time_of_day = 0.0;
        }
    }

    pub fn draw(
        &self,

        shader: &Shader,

        view: &Mat4,

        projection: &Mat4,
    ) {

        unsafe {

            gl::DepthFunc(
                gl::LEQUAL
            );
        }

        shader.use_program();

        let sky_view =
            Mat4::from_mat3(
                glam::Mat3::from_mat4(
                    *view
                )
            );

        shader.set_mat4(
            "view",
            &sky_view,
        );

        shader.set_mat4(
            "projection",
            projection,
        );

        unsafe {

            gl::BindVertexArray(
                self.vao
            );

            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                36,
            );

            gl::DepthFunc(
                gl::LESS
            );
        }
    }

    pub fn sky_color(
        &self,
    ) -> Vec3 {

        let t =
            self.time_of_day;

        // NIGHT

        if t < 0.25 {

            return Vec3::new(
                0.02,
                0.02,
                0.08,
            );
        }

        // SUNRISE

        if t < 0.35 {

            return Vec3::new(
                0.8,
                0.4,
                0.2,
            );
        }

        // DAY

        if t < 0.75 {

            return Vec3::new(
                0.55,
                0.75,
                0.95,
            );
        }

        // SUNSET

        Vec3::new(
            0.9,
            0.35,
            0.25,
        )
    }
}
