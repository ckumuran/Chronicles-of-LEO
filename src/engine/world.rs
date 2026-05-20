use crate::engine::block::BlockType;
use crate::engine::chunk_manager::ChunkManager;

pub struct World {

    pub chunk_manager: ChunkManager,
}

impl World {

    pub fn new() -> Self {

        Self {

            chunk_manager:
                ChunkManager::new(),
        }
    }

    pub fn update(
        &mut self,

        player_x: f32,
        player_z: f32,
    ) {

        let player_chunk_x =
            (player_x / 16.0).floor() as i32;

        let player_chunk_z =
            (player_z / 16.0).floor() as i32;

        self.chunk_manager.update(
            player_chunk_x,
            player_chunk_z,
        );
    }

    pub fn set_block(
        &mut self,

        world_x: i32,
        world_y: i32,
        world_z: i32,

        block: BlockType,
    ) {

        let chunk_x =
            (world_x as f32 / 16.0).floor() as i32;

        let chunk_z =
            (world_z as f32 / 16.0).floor() as i32;

        let local_x =
            ((world_x % 16) + 16) % 16;

        let local_z =
            ((world_z % 16) + 16) % 16;

        let local_y =
            world_y;

        if let Some(chunk) =
            self.chunk_manager
                .chunks
                .get_mut(&(chunk_x, 0, chunk_z))
        {

            if local_y >= 0 &&
               local_y < 16
            {

                chunk.set_block(
                    local_x as usize,
                    local_y as usize,
                    local_z as usize,

                    block,
                );
            }
        }
    }
}
