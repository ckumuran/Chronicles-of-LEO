use std::collections::HashMap;

use crate::engine::chunk::Chunk;
use crate::engine::chunk_render_data::ChunkRenderData;
use crate::engine::terrain::TerrainGenerator;

pub const RENDER_DISTANCE: i32 = 4;

pub struct ChunkManager {

    pub chunks:
        HashMap<
            (i32, i32, i32),
            Chunk
        >,

    pub render_data:
        HashMap<
            (i32, i32, i32),
            ChunkRenderData
        >,

    terrain: TerrainGenerator,
}

impl ChunkManager {

    pub fn new() -> Self {

        Self {

            chunks: HashMap::new(),

            render_data: HashMap::new(),

            terrain:
                TerrainGenerator::new(),
        }
    }

    pub fn update(
        &mut self,

        player_chunk_x: i32,
        player_chunk_z: i32,
    ) {

        self.load_chunks(
            player_chunk_x,
            player_chunk_z,
        );

        self.unload_chunks(
            player_chunk_x,
            player_chunk_z,
        );
    }

    fn load_chunks(
        &mut self,

        player_chunk_x: i32,
        player_chunk_z: i32,
    ) {

        for x in
            player_chunk_x - RENDER_DISTANCE
            ..
            =
            player_chunk_x + RENDER_DISTANCE
        {

            for z in
                player_chunk_z - RENDER_DISTANCE
                ..
                =
                player_chunk_z + RENDER_DISTANCE
            {

                let key = (x, 0, z);

                if self.chunks.contains_key(&key) {
                    continue;
                }

                let chunk =
                    self.terrain.generate_chunk(
                        x,
                        0,
                        z,
                    );

                self.chunks.insert(
                    key,
                    chunk,
                );

                self.render_data.insert(
                    key,

                    ChunkRenderData {
                        mesh: None,
                    }
                );
            }
        }
    }

    fn unload_chunks(
        &mut self,

        player_chunk_x: i32,
        player_chunk_z: i32,
    ) {

        self.chunks.retain(
            |&(x, _, z), _| {

                let dx =
                    (x - player_chunk_x).abs();

                let dz =
                    (z - player_chunk_z).abs();

                dx <= RENDER_DISTANCE &&
                dz <= RENDER_DISTANCE
            }
        );

        self.render_data.retain(
            |&(x, _, z), _| {

                let dx =
                    (x - player_chunk_x).abs();

                let dz =
                    (z - player_chunk_z).abs();

                dx <= RENDER_DISTANCE &&
                dz <= RENDER_DISTANCE
            }
        );
    }
}
