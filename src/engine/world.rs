use std::collections::HashMap;

use crate::engine::chunk::Chunk;
use crate::engine::terrain::TerrainGenerator;

pub struct World {
    pub chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl World {

    pub fn new() -> Self {

        let mut world = Self {
            chunks: HashMap::new(),
        };

        world.generate();

        world
    }

    fn generate(&mut self) {

        let terrain =
            TerrainGenerator::new();

        for chunk_x in -2..=2 {
            for chunk_z in -2..=2 {

                let chunk =
                    terrain.generate_chunk(
                        chunk_x,
                        0,
                        chunk_z,
                    );

                self.chunks.insert(
                    (chunk_x, 0, chunk_z),
                    chunk,
                );
            }
        }
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
