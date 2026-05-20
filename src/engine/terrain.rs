use noise::{NoiseFn, Perlin};

use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, CHUNK_SIZE};

pub struct TerrainGenerator {
    noise: Perlin,
}

impl TerrainGenerator {

    pub fn new() -> Self {
        Self {
            noise: Perlin::new(1337),
        }
    }

    pub fn generate_chunk(
        &self,
        chunk_x: i32,
        chunk_y: i32,
        chunk_z: i32,
    ) -> Chunk {

        let mut chunk =
            Chunk::new(chunk_x, chunk_y, chunk_z);

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {

                let world_x =
                    chunk_x * CHUNK_SIZE as i32 + x as i32;

                let world_z =
                    chunk_z * CHUNK_SIZE as i32 + z as i32;

                let terrain_height =
                    self.get_height(
                        world_x,
                        world_z,
                    );

                for y in 0..CHUNK_SIZE {

                    let world_y =
                        chunk_y * CHUNK_SIZE as i32 + y as i32;

                    let block = if world_y > terrain_height {

                        BlockType::Air

                    } else if world_y == terrain_height {

                        BlockType::Grass

                    } else if world_y > terrain_height - 3 {

                        BlockType::Dirt

                    } else {

                        BlockType::Stone
                    };

                    chunk.set_block(
                        x,
                        y,
                        z,
                        block,
                    );
                }
            }
        }

        chunk
    }

    fn get_height(
        &self,
        x: i32,
        z: i32,
    ) -> i32 {

        let scale = 0.03;

        let height =
            self.noise.get([
                x as f64 * scale,
                z as f64 * scale,
            ]);

        let base_height = 20.0;

        let amplitude = 12.0;

        (height * amplitude + base_height) as i32
    }
}
