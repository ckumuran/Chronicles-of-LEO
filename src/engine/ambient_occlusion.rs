use crate::engine::block::BlockType;
use crate::engine::chunk::Chunk;

pub struct AmbientOcclusion;

impl AmbientOcclusion {

    pub fn vertex_ao(
        side1: bool,
        side2: bool,
        corner: bool,
    ) -> f32 {

        if side1 && side2 {
            return 0.2;
        }

        let mut occlusion = 0;

        if side1 {
            occlusion += 1;
        }

        if side2 {
            occlusion += 1;
        }

        if corner {
            occlusion += 1;
        }

        match occlusion {

            0 => 1.0,

            1 => 0.8,

            2 => 0.6,

            _ => 0.4,
        }
    }

    pub fn is_solid(
        chunk: &Chunk,

        x: i32,
        y: i32,
        z: i32,
    ) -> bool {

        if x < 0 ||
           y < 0 ||
           z < 0 ||
           x >= 16 ||
           y >= 16 ||
           z >= 16
        {
            return false;
        }

        let block =
            chunk.get_block(
                x as usize,
                y as usize,
                z as usize,
            );

        block != BlockType::Air
    }
}
