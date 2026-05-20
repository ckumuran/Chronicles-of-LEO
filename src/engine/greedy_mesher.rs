use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, CHUNK_SIZE};

pub struct GreedyMesher;

const TILE_SIZE: f32 = 1.0 / 16.0;

impl GreedyMesher {

    pub fn build(
        chunk: &Chunk
    ) -> Vec<f32> {

        let mut vertices =
            Vec::new();

        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {

                    let block =
                        chunk.get_block(x, y, z);

                    if block == BlockType::Air {
                        continue;
                    }

                    let x = x as f32;
                    let y = y as f32;
                    let z = z as f32;

                    // TOP

                    let visible_top =
                        if y + 1 >= CHUNK_SIZE {

                            true

                        } else {

                            chunk.get_block(
                                x as usize,
                                y as usize + 1,
                                z as usize,
                            ) == BlockType::Air
                        };

                    if visible_top {

                        let uv =
                            block.texture_index_top();

                        Self::add_top_face(
                            &mut vertices,

                            x,y,z,

                            uv,
                        );
                    }

                    // BOTTOM

                    let visible_bottom =
                        if y <= 0 {

                            true

                        } else {

                            chunk.get_block(
                                x as usize,
                                y as usize - 1,
                                z as usize,
                            ) == BlockType::Air
                        };

                    if visible_bottom {

                        let uv =
                            block.texture_index_bottom();

                        Self::add_bottom_face(
                            &mut vertices,

                            x,y,z,

                            uv,
                        );
                    }

                    // FRONT

                    let visible_front =
                        if z as usize + 1 >= CHUNK_SIZE {

                            true

                        } else {

                            chunk.get_block(
                                x as usize,
                                y as usize,
                                z as usize + 1,
                            ) == BlockType::Air
                        };

                    if visible_front {

                        let uv =
                            block.texture_index_side();

                        Self::add_front_face(
                            &mut vertices,

                            x,y,z,

                            uv,
                        );
                    }

                    // BACK

                    let visible_back =
                        if z <= 0.0 {

                            true

                        } else {

                            chunk.get_block(
                                x as usize,
                                y as usize,
                                z as usize - 1,
                            ) == BlockType::Air
                        };

                    if visible_back {

                        let uv =
                            block.texture_index_side();

                        Self::add_back_face(
                            &mut vertices,

                            x,y,z,

                            uv,
                        );
                    }

                    // LEFT

                    let visible_left =
                        if x <= 0.0 {

                            true

                        } else {

                            chunk.get_block(
                                x as usize - 1,
                                y as usize,
                                z as usize,
                            ) == BlockType::Air
                        };

                    if visible_left {

                        let uv =
                            block.texture_index_side();

                        Self::add_left_face(
                            &mut vertices,

                            x,y,z,

                            uv,
                        );
                    }

                    // RIGHT

                    let visible_right =
                        if x as usize + 1 >= CHUNK_SIZE {

                            true

                        } else {

                            chunk.get_block(
                                x as usize + 1,
                                y as usize,
                                z as usize,
                            ) == BlockType::Air
                        };

                    if visible_right {

                        let uv =
                            block.texture_index_side();

                        Self::add_right_face(
                            &mut vertices,

                            x,y,z,

                            uv,
                        );
                    }
                }
            }
        }

        vertices
    }

    fn uv_coords(
        tile_x: u32,
        tile_y: u32,
    ) -> (f32, f32, f32, f32) {

        let u0 =
            tile_x as f32 * TILE_SIZE;

        let v0 =
            tile_y as f32 * TILE_SIZE;

        let u1 =
            u0 + TILE_SIZE;

        let v1 =
            v0 + TILE_SIZE;

        (u0, v0, u1, v1)
    }

    fn add_top_face(
        vertices: &mut Vec<f32>,

        x: f32,
        y: f32,
        z: f32,

        uv_tile: (u32, u32),
    ) {

        let (u0,v0,u1,v1) =
            Self::uv_coords(
                uv_tile.0,
                uv_tile.1,
            );

        let face = [

            x,y+1.0,z,      u0,v0,
            x,y+1.0,z+1.0,  u0,v1,
            x+1.0,y+1.0,z+1.0,u1,v1,

            x+1.0,y+1.0,z+1.0,u1,v1,
            x+1.0,y+1.0,z,u1,v0,
            x,y+1.0,z,u0,v0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_bottom_face(
        vertices: &mut Vec<f32>,

        x: f32,
        y: f32,
        z: f32,

        uv_tile: (u32, u32),
    ) {

        let (u0,v0,u1,v1) =
            Self::uv_coords(
                uv_tile.0,
                uv_tile.1,
            );

        let face = [

            x,y,z,u0,v0,
            x+1.0,y,z,u1,v0,
            x+1.0,y,z+1.0,u1,v1,

            x+1.0,y,z+1.0,u1,v1,
            x,y,z+1.0,u0,v1,
            x,y,z,u0,v0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_front_face(
        vertices: &mut Vec<f32>,

        x: f32,
        y: f32,
        z: f32,

        uv_tile: (u32, u32),
    ) {

        let (u0,v0,u1,v1) =
            Self::uv_coords(
                uv_tile.0,
                uv_tile.1,
            );

        let face = [

            x,y,z+1.0,u0,v0,
            x+1.0,y,z+1.0,u1,v0,
            x+1.0,y+1.0,z+1.0,u1,v1,

            x+1.0,y+1.0,z+1.0,u1,v1,
            x,y+1.0,z+1.0,u0,v1,
            x,y,z+1.0,u0,v0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_back_face(
        vertices: &mut Vec<f32>,

        x: f32,
        y: f32,
        z: f32,

        uv_tile: (u32, u32),
    ) {

        let (u0,v0,u1,v1) =
            Self::uv_coords(
                uv_tile.0,
                uv_tile.1,
            );

        let face = [

            x,y,z,u0,v0,
            x,y+1.0,z,u0,v1,
            x+1.0,y+1.0,z,u1,v1,

            x+1.0,y+1.0,z,u1,v1,
            x+1.0,y,z,u1,v0,
            x,y,z,u0,v0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_left_face(
        vertices: &mut Vec<f32>,

        x: f32,
        y: f32,
        z: f32,

        uv_tile: (u32, u32),
    ) {

        let (u0,v0,u1,v1) =
            Self::uv_coords(
                uv_tile.0,
                uv_tile.1,
            );

        let face = [

            x,y+1.0,z+1.0,u1,v1,
            x,y+1.0,z,u0,v1,
            x,y,z,u0,v0,

            x,y,z,u0,v0,
            x,y,z+1.0,u1,v0,
            x,y+1.0,z+1.0,u1,v1,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_right_face(
        vertices: &mut Vec<f32>,

        x: f32,
        y: f32,
        z: f32,

        uv_tile: (u32, u32),
    ) {

        let (u0,v0,u1,v1) =
            Self::uv_coords(
                uv_tile.0,
                uv_tile.1,
            );

        let face = [

            x+1.0,y+1.0,z+1.0,u1,v1,
            x+1.0,y,z,u0,v0,
            x+1.0,y+1.0,z,u0,v1,

            x+1.0,y,z,u0,v0,
            x+1.0,y+1.0,z+1.0,u1,v1,
            x+1.0,y,z+1.0,u1,v0,
        ];

        vertices.extend_from_slice(&face);
    }
}
