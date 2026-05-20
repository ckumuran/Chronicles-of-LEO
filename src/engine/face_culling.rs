use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, CHUNK_SIZE};

pub struct FaceCulling;

impl FaceCulling {

    pub fn is_face_visible(
        chunk: &Chunk,

        x: i32,
        y: i32,
        z: i32,
    ) -> bool {

        // Outside chunk bounds
        // means visible for now

        if x < 0 ||
           y < 0 ||
           z < 0 ||
           x >= CHUNK_SIZE as i32 ||
           y >= CHUNK_SIZE as i32 ||
           z >= CHUNK_SIZE as i32
        {
            return true;
        }

        let neighbor =
            chunk.get_block(
                x as usize,
                y as usize,
                z as usize,
            );

        neighbor == BlockType::Air
    }
}
