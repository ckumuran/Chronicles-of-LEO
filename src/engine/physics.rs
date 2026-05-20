use glam::Vec3;

use crate::engine::block::BlockType;
use crate::engine::world::World;

pub struct PhysicsBody {

    pub velocity: Vec3,

    pub on_ground: bool,
}

impl PhysicsBody {

    pub fn new() -> Self {

        Self {

            velocity: Vec3::ZERO,

            on_ground: false,
        }
    }

    pub fn update(
        &mut self,

        world: &World,

        position: &mut Vec3,

        delta_time: f32,
    ) {

        // GRAVITY

        self.velocity.y -=
            25.0 * delta_time;

        // APPLY VELOCITY

        let mut next_position =
            *position +
            self.velocity * delta_time;

        // COLLISION

        if self.collides(
            world,
            next_position,
        ) {

            // Y COLLISION

            next_position.y =
                position.y;

            self.velocity.y = 0.0;

            self.on_ground = true;

        } else {

            self.on_ground = false;
        }

        *position = next_position;
    }

    fn collides(
        &self,

        world: &World,

        position: Vec3,
    ) -> bool {

        let player_width = 0.3;

        let player_height = 1.8;

        let min_x =
            (position.x - player_width)
            .floor() as i32;

        let max_x =
            (position.x + player_width)
            .floor() as i32;

        let min_y =
            position.y.floor() as i32;

        let max_y =
            (position.y + player_height)
            .floor() as i32;

        let min_z =
            (position.z - player_width)
            .floor() as i32;

        let max_z =
            (position.z + player_width)
            .floor() as i32;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {

                    if let Some(block) =
                        Self::get_block(
                            world,
                            x,
                            y,
                            z,
                        )
                    {

                        if block != BlockType::Air {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn get_block(
        world: &World,

        world_x: i32,
        world_y: i32,
        world_z: i32,
    ) -> Option<BlockType> {

        let chunk_x =
            (world_x as f32 / 16.0)
            .floor() as i32;

        let chunk_z =
            (world_z as f32 / 16.0)
            .floor() as i32;

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
           local_y >= 16
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
