use glam::Vec3;

use crate::engine::block::BlockType;
use crate::engine::chunk::CHUNK_SIZE;
use crate::engine::world::World;

pub struct RaycastHit {

    pub position: (i32, i32, i32),

    pub previous: (i32, i32, i32),
}

pub struct Raycaster;

impl Raycaster {

    pub fn cast(
        world: &World,

        origin: Vec3,
        direction: Vec3,

        max_distance: f32,
    ) -> Option<RaycastHit> {

        let step = 0.1;

        let mut distance = 0.0;

        let mut previous_block =
            (
                origin.x.floor() as i32,
                origin.y.floor() as i32,
                origin.z.floor() as i32,
            );

        while distance < max_distance {

            let point =
                origin + direction * distance;

            let block_x =
                point.x.floor() as i32;

            let block_y =
                point.y.floor() as i32;

            let block_z =
                point.z.floor() as i32;

            if let Some(block) =
                Self::get_block(
                    world,

                    block_x,
                    block_y,
                    block_z,
                )
            {

                if block != BlockType::Air {

                    return Some(
                        RaycastHit {

                            position: (
                                block_x,
                                block_y,
                                block_z,
                            ),

                            previous:
                                previous_block,
                        }
                    );
                }
            }

            previous_block =
                (
                    block_x,
                    block_y,
                    block_z,
                );

            distance += step;
        }

        None
    }

    fn get_block(
        world: &World,

        world_x: i32,
        world_y: i32,
        world_z: i32,
    ) -> Option<BlockType> {

        let chunk_x =
            (world_x as f32 / 16.0).floor() as i32;

        let chunk_z =
            (world_z as f32 / 16.0).floor() as i32;

        let local_x =
            ((world_x % 16) + 16) % 16;

        let local_y =
            world_y;

        let local_z =
            ((world_z % 16) + 16) % 16;

        let chunk =
            world
                .chunk_manager
                .chunks
                .get(&(chunk_x, 0, chunk_z))?;

        if local_y < 0 ||
           local_y >= CHUNK_SIZE as i32
        {
            return None;
        }

        Some(
            chunk.get_block(
                local_x as usize,
                local_y as usize,
                local_z as usize,
            )
        )
    }
}
