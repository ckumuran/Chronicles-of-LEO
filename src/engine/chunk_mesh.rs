use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, CHUNK_SIZE};

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

                    Self::add_cube(
                        &mut vertices,
                        x as f32,
                        y as f32,
                        z as f32,
                    );
                }
            }
        }

        Self { vertices }
    }

    fn add_cube(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {
        let cube = [
            // FRONT
            x,y,z+1.0,
            x+1.0,y,z+1.0,
            x+1.0,y+1.0,z+1.0,

            x+1.0,y+1.0,z+1.0,
            x,y+1.0,z+1.0,
            x,y,z+1.0,

            // BACK
            x,y,z,
            x,y+1.0,z,
            x+1.0,y+1.0,z,

            x+1.0,y+1.0,z,
            x+1.0,y,z,
            x,y,z,

            // LEFT
            x,y+1.0,z+1.0,
            x,y+1.0,z,
            x,y,z,

            x,y,z,
            x,y,z+1.0,
            x,y+1.0,z+1.0,

            // RIGHT
            x+1.0,y+1.0,z+1.0,
            x+1.0,y,z,
            x+1.0,y+1.0,z,

            x+1.0,y,z,
            x+1.0,y+1.0,z+1.0,
            x+1.0,y,z+1.0,

            // TOP
            x,y+1.0,z,
            x,y+1.0,z+1.0,
            x+1.0,y+1.0,z+1.0,

            x+1.0,y+1.0,z+1.0,
            x+1.0,y+1.0,z,
            x,y+1.0,z,

            // BOTTOM
            x,y,z,
            x+1.0,y,z,
            x+1.0,y,z+1.0,

            x+1.0,y,z+1.0,
            x,y,z+1.0,
            x,y,z,
        ];

        vertices.extend_from_slice(&cube);
    }
}
