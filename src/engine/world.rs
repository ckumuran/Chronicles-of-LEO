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
}
