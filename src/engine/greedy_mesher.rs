use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, CHUNK_SIZE};

pub struct GreedyMesher;

impl GreedyMesher {

    pub fn build(chunk: &Chunk) -> Vec<f32> {

        let mut vertices = Vec::new();

        let mut visited =
            vec![false; CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE];

        for y in 0..CHUNK_SIZE {

            for z in 0..CHUNK_SIZE {

                for x in 0..CHUNK_SIZE {

                    let index =
                        x +
                        y * CHUNK_SIZE +
                        z * CHUNK_SIZE * CHUNK_SIZE;

                    if visited[index] {
                        continue;
                    }

                    let block =
                        chunk.get_block(x, y, z);

                    if block == BlockType::Air {
                        continue;
                    }

                    // Only top faces for now

                    let above_air =
                        if y + 1 >= CHUNK_SIZE {

                            true

                        } else {

                            chunk.get_block(
                                x,
                                y + 1,
                                z,
                            ) == BlockType::Air
                        };

                    if !above_air {
                        continue;
                    }

                    // GREEDY WIDTH

                    let mut width = 1;

                    while x + width < CHUNK_SIZE {

                        let next =
                            chunk.get_block(
                                x + width,
                                y,
                                z,
                            );

                        if next != block {
                            break;
                        }

                        let next_above_air =
                            if y + 1 >= CHUNK_SIZE {

                                true

                            } else {

                                chunk.get_block(
                                    x + width,
                                    y + 1,
                                    z,
                                ) == BlockType::Air
                            };

                        if !next_above_air {
                            break;
                        }

                        let next_index =
                            (x + width) +
                            y * CHUNK_SIZE +
                            z * CHUNK_SIZE * CHUNK_SIZE;

                        if visited[next_index] {
                            break;
                        }

                        width += 1;
                    }

                    // GREEDY HEIGHT

                    let mut height = 1;

                    'height_loop:
                    while z + height < CHUNK_SIZE {

                        for dx in 0..width {

                            let next =
                                chunk.get_block(
                                    x + dx,
                                    y,
                                    z + height,
                                );

                            if next != block {
                                break 'height_loop;
                            }

                            let next_above_air =
                                if y + 1 >= CHUNK_SIZE {

                                    true

                                } else {

                                    chunk.get_block(
                                        x + dx,
                                        y + 1,
                                        z + height,
                                    ) == BlockType::Air
                                };

                            if !next_above_air {
                                break 'height_loop;
                            }

                            let next_index =
                                (x + dx) +
                                y * CHUNK_SIZE +
                                (z + height) *
                                CHUNK_SIZE *
                                CHUNK_SIZE;

                            if visited[next_index] {
                                break 'height_loop;
                            }
                        }

                        height += 1;
                    }

                    // MARK VISITED

                    for dz in 0..height {
                        for dx in 0..width {

                            let visited_index =
                                (x + dx) +
                                y * CHUNK_SIZE +
                                (z + dz) *
                                CHUNK_SIZE *
                                CHUNK_SIZE;

                            visited[visited_index] = true;
                        }
                    }

                    Self::add_top_quad(
                        &mut vertices,

                        x as f32,
                        y as f32,
                        z as f32,

                        width as f32,
                        height as f32,
                    );
                }
            }
        }

        vertices
    }

    fn add_top_quad(
        vertices: &mut Vec<f32>,

        x: f32,
        y: f32,
        z: f32,

        width: f32,
        height: f32,
    ) {

        let quad = [

            x, y + 1.0, z,

            x, y + 1.0, z + height,

            x + width,
            y + 1.0,
            z + height,

            x + width,
            y + 1.0,
            z + height,

            x + width,
            y + 1.0,
            z,

            x,
            y + 1.0,
            z,
        ];

        vertices.extend_from_slice(&quad);
    }
}
