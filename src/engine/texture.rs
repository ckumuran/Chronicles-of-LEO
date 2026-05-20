use image::GenericImageView;

use std::ffi::c_void;

pub struct Texture {

    pub id: u32,
}

impl Texture {

    pub fn from_file(path: &str) -> Self {

        let image =
            image::open(path)
                .expect("Failed to load texture");

        let image =
            image.flipv();

        let data =
            image.to_rgba8();

        let (width, height) =
            image.dimensions();

        let mut texture = 0;

        unsafe {

            gl::GenTextures(
                1,
                &mut texture,
            );

            gl::BindTexture(
                gl::TEXTURE_2D,
                texture,
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as i32,
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as i32,
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT as i32,
            );

            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT as i32,
            );

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );

            gl::GenerateMipmap(
                gl::TEXTURE_2D
            );
        }

        Self {
            id: texture,
        }
    }

    pub fn bind(&self) {

        unsafe {

            gl::BindTexture(
                gl::TEXTURE_2D,
                self.id,
            );
        }
    }
}
