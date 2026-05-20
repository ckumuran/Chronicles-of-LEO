use rand::Rng;

use crate::engine::block::BlockType;

use crate::engine::chunk::{
    Chunk,
    CHUNK_SIZE,
};

pub struct Structures;

impl Structures {

    pub fn generate_house(

        chunk: &mut Chunk,

        x: usize,
        y: usize,
        z: usize,
    ) {

        let width = 6;
        let height = 4;
        let depth = 6;

        // FLOOR

        for dx in 0..width {
            for dz in 0..depth {

                let px = x + dx;
                let pz = z + dz;

                if px >= CHUNK_SIZE ||
                   pz >= CHUNK_SIZE
                {
                    continue;
                }

                chunk.set_block(

                    px,
                    y,
                    pz,

                    BlockType::Wood,
                );
            }
        }

        // WALLS

        for dy in 1..height {

            for dx in 0..width {
                for dz in 0..depth {

                    let edge =
                        dx == 0 ||
                        dz == 0 ||
                        dx == width - 1 ||
                        dz == depth - 1;

                    if !edge {
                        continue;
                    }

                    let px = x + dx;
                    let py = y + dy;
                    let pz = z + dz;

                    if px >= CHUNK_SIZE ||
                       py >= CHUNK_SIZE ||
                       pz >= CHUNK_SIZE
                    {
                        continue;
                    }

                    chunk.set_block(

                        px,
                        py,
                        pz,

                        BlockType::Wood,
                    );
                }
            }
        }

        // ROOF

        for dx in 0..width {
            for dz in 0..depth {

                let px = x + dx;
                let py = y + height;
                let pz = z + dz;

                if px >= CHUNK_SIZE ||
                   py >= CHUNK_SIZE ||
                   pz >= CHUNK_SIZE
                {
                    continue;
                }

                chunk.set_block(

                    px,
                    py,
                    pz,

                    BlockType::Stone,
                );
            }
        }

        // DOOR

        chunk.set_block(
            x + 2,
            y + 1,
            z,

            BlockType::Air,
        );

        chunk.set_block(
            x + 2,
            y + 2,
            z,

            BlockType::Air,
        );
    }

    pub fn generate_tower(

        chunk: &mut Chunk,

        x: usize,
        y: usize,
        z: usize,
    ) {

        let height = 12;

        for dy in 0..height {

            for dx in 0..3 {
                for dz in 0..3 {

                    let edge =
                        dx == 0 ||
                        dz == 0 ||
                        dx == 2 ||
                        dz == 2;

                    if !edge {
                        continue;
                    }

                    let px = x + dx;
                    let py = y + dy;
                    let pz = z + dz;

                    if px >= CHUNK_SIZE ||
                       py >= CHUNK_SIZE ||
                       pz >= CHUNK_SIZE
                    {
                        continue;
                    }

                    chunk.set_block(

                        px,
                        py,
                        pz,

                        BlockType::Stone,
                    );
                }
            }
        }
    }

    pub fn try_generate(

        chunk: &mut Chunk,

        terrain_height: i32,

        x: usize,
        z: usize,
    ) {

        let mut rng =
            rand::thread_rng();

        let roll: f32 =
            rng.gen();

        // HOUSE

        if roll < 0.001 {

            Self::generate_house(

                chunk,

                x,
                terrain_height as usize + 1,
                z,
            );
        }

        // TOWER

        else if roll < 0.0015 {

            Self::generate_tower(

                chunk,

                x,
                terrain_height as usize + 1,
                z,
            );
        }
    }
}
