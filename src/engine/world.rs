use std::collections::HashMap;

use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, CHUNK_SIZE};

pub struct World {
    pub chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            chunks: HashMap::new(),
        };

        world.generate_test_world();

        world
    }

    fn generate_test_world(&mut self) {
        let mut chunk = Chunk::new(0, 0, 0);

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                chunk.set_block(
                    x,
                    0,
                    z,
                    BlockType::Grass,
                );

                chunk.set_block(
                    x,
                    1,
                    z,
                    BlockType::Dirt,
                );

                chunk.set_block(
                    x,
                    2,
                    z,
                    BlockType::Stone,
                );
            }
        }

        self.chunks.insert((0, 0, 0), chunk);
    }

    pub fn get_chunk(
        &self,
        x: i32,
        y: i32,
        z: i32,
    ) -> Option<&Chunk> {
        self.chunks.get(&(x, y, z))
    }
}
