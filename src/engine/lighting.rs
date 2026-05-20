use std::collections::VecDeque;

use crate::engine::block::BlockType;
use crate::engine::chunk::{
    Chunk,
    CHUNK_SIZE,
};

pub const MAX_LIGHT: u8 = 15;

pub struct Lighting;

impl Lighting {

    pub fn generate_sunlight(
        chunk: &Chunk,
    ) -> Vec<u8> {

        let mut light_map =
            vec![
                0;
                CHUNK_SIZE *
                CHUNK_SIZE *
                CHUNK_SIZE
            ];

        let mut queue =
            VecDeque::new();

        // SKY LIGHT PASS

        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {

                let mut blocked =
                    false;

                for y in
                    (0..CHUNK_SIZE).rev()
                {

                    let block =
                        chunk.get_block(
                            x,
                            y,
                            z,
                        );

                    let index =
                        Self::index(
                            x,
                            y,
                            z,
                        );

                    if !blocked &&
                       block == BlockType::Air
                    {

                        light_map[index] =
                            MAX_LIGHT;

                        queue.push_back(
                            (
                                x as i32,
                                y as i32,
                                z as i32,
                            )
                        );

                    } else {

                        blocked = true;
                    }
                }
            }
        }

        // FLOOD FILL

        while let Some(
            (x,y,z)
        ) = queue.pop_front()
        {

            let current =
                light_map[
                    Self::index(
                        x as usize,
                        y as usize,
                        z as usize,
                    )
                ];

            if current <= 1 {
                continue;
            }

            let neighbors = [

                (1,0,0),
                (-1,0,0),

                (0,1,0),
                (0,-1,0),

                (0,0,1),
                (0,0,-1),
            ];

            for (dx,dy,dz)
            in neighbors
            {

                let nx = x + dx;
                let ny = y + dy;
                let nz = z + dz;

                if nx < 0 ||
                   ny < 0 ||
                   nz < 0 ||
                   nx >= CHUNK_SIZE as i32 ||
                   ny >= CHUNK_SIZE as i32 ||
                   nz >= CHUNK_SIZE as i32
                {
                    continue;
                }

                let block =
                    chunk.get_block(
                        nx as usize,
                        ny as usize,
                        nz as usize,
                    );

                if block.is_solid() {
                    continue;
                }

                let next_index =
                    Self::index(
                        nx as usize,
                        ny as usize,
                        nz as usize,
                    );

                let next_light =
                    current - 1;

                if next_light >
                   light_map[next_index]
                {

                    light_map[next_index] =
                        next_light;

                    queue.push_back(
                        (nx,ny,nz)
                    );
                }
            }
        }

        light_map
    }

    fn index(
        x: usize,
        y: usize,
        z: usize,
    ) -> usize {

        x +
        y * CHUNK_SIZE +
        z * CHUNK_SIZE * CHUNK_SIZE
    }
}
