use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, CHUNK_SIZE};
use crate::engine::face_culling::FaceCulling;

pub struct ChunkMesh {
    pub vertices: Vec<f32>,
}

impl ChunkMesh {

    pub fn build(chunk: &Chunk) -> Self {

        let mut vertices = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {

                    let block =
                        chunk.get_block(x, y, z);

                    if block == BlockType::Air {
                        continue;
                    }

                    let x = x as f32;
                    let y = y as f32;
                    let z = z as f32;

                    // FRONT
                    if FaceCulling::is_face_visible(
                        chunk,
                        x as i32,
                        y as i32,
                        z as i32 + 1,
                    ) {
                        Self::add_front_face(
                            &mut vertices,
                            x,y,z
                        );
                    }

                    // BACK
                    if FaceCulling::is_face_visible(
                        chunk,
                        x as i32,
                        y as i32,
                        z as i32 - 1,
                    ) {
                        Self::add_back_face(
                            &mut vertices,
                            x,y,z
                        );
                    }

                    // LEFT
                    if FaceCulling::is_face_visible(
                        chunk,
                        x as i32 - 1,
                        y as i32,
                        z as i32,
                    ) {
                        Self::add_left_face(
                            &mut vertices,
                            x,y,z
                        );
                    }

                    // RIGHT
                    if FaceCulling::is_face_visible(
                        chunk,
                        x as i32 + 1,
                        y as i32,
                        z as i32,
                    ) {
                        Self::add_right_face(
                            &mut vertices,
                            x,y,z
                        );
                    }

                    // TOP
                    if FaceCulling::is_face_visible(
                        chunk,
                        x as i32,
                        y as i32 + 1,
                        z as i32,
                    ) {
                        Self::add_top_face(
                            &mut vertices,
                            x,y,z
                        );
                    }

                    // BOTTOM
                    if FaceCulling::is_face_visible(
                        chunk,
                        x as i32,
                        y as i32 - 1,
                        z as i32,
                    ) {
                        Self::add_bottom_face(
                            &mut vertices,
                            x,y,z
                        );
                    }
                }
            }
        }

        Self { vertices }
    }

    fn add_front_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let face = [
            x,y,z+1.0,
            x+1.0,y,z+1.0,
            x+1.0,y+1.0,z+1.0,

            x+1.0,y+1.0,z+1.0,
            x,y+1.0,z+1.0,
            x,y,z+1.0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_back_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let face = [
            x,y,z,
            x,y+1.0,z,
            x+1.0,y+1.0,z,

            x+1.0,y+1.0,z,
            x+1.0,y,z,
            x,y,z,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_left_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let face = [
            x,y+1.0,z+1.0,
            x,y+1.0,z,
            x,y,z,

            x,y,z,
            x,y,z+1.0,
            x,y+1.0,z+1.0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_right_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let face = [
            x+1.0,y+1.0,z+1.0,
            x+1.0,y,z,
            x+1.0,y+1.0,z,

            x+1.0,y,z,
            x+1.0,y+1.0,z+1.0,
            x+1.0,y,z+1.0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_top_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let face = [
            x,y+1.0,z,
            x,y+1.0,z+1.0,
            x+1.0,y+1.0,z+1.0,

            x+1.0,y+1.0,z+1.0,
            x+1.0,y+1.0,z,
            x,y+1.0,z,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_bottom_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let face = [
            x,y,z,
            x+1.0,y,z,
            x+1.0,y,z+1.0,

            x+1.0,y,z+1.0,
            x,y,z+1.0,
            x,y,z,
        ];

        vertices.extend_from_slice(&face);
    }
}
