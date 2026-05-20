use crate::engine::chunk::{
    Chunk,
    CHUNK_SIZE,
};

use crate::engine::block::BlockType;

pub struct AmbientOcclusion;

impl AmbientOcclusion {

    pub fn sample_light(

        chunk: &Chunk,

        x: i32,
        y: i32,
        z: i32,
    ) -> f32 {

        // OUT OF BOUNDS

        if x < 0 ||
           y < 0 ||
           z < 0 ||
           x >= CHUNK_SIZE as i32 ||
           y >= CHUNK_SIZE as i32 ||
           z >= CHUNK_SIZE as i32
        {
            return 1.0;
        }

        let block =
            chunk.get_block(
                x as usize,
                y as usize,
                z as usize,
            );

        match block {

            BlockType::Air => 1.0,

            BlockType::Water => 0.85,

            _ => 0.55,
        }
    }

    pub fn vertex_light(

        chunk: &Chunk,

        x: i32,
        y: i32,
        z: i32,
    ) -> f32 {

        let mut total = 0.0;

        let mut samples = 0.0;

        for ox in -1..=1 {

            for oy in -1..=1 {

                for oz in -1..=1 {

                    total +=
                        Self::sample_light(

                            chunk,

                            x + ox,
                            y + oy,
                            z + oz,
                        );

                    samples += 1.0;
                }
            }
        }

        total / samples
    }
}
