use crate::engine::block::BlockType;
use crate::engine::chunk::{Chunk, CHUNK_SIZE};

pub struct GreedyMesher;

impl GreedyMesher {

    pub fn build(chunk: &Chunk) -> Vec<f32> {

        let mut vertices = Vec::new();

        Self::mesh_top(chunk, &mut vertices);
        Self::mesh_bottom(chunk, &mut vertices);

        Self::mesh_front(chunk, &mut vertices);
        Self::mesh_back(chunk, &mut vertices);

        Self::mesh_left(chunk, &mut vertices);
        Self::mesh_right(chunk, &mut vertices);

        vertices
    }

    fn mesh_top(
        chunk: &Chunk,
        vertices: &mut Vec<f32>,
    ) {

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

                    let visible =
                        if y + 1 >= CHUNK_SIZE {

                            true

                        } else {

                            chunk.get_block(
                                x,
                                y + 1,
                                z,
                            ) == BlockType::Air
                        };

                    if !visible {
                        continue;
                    }

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

                        let visible =
                            if y + 1 >= CHUNK_SIZE {

                                true

                            } else {

                                chunk.get_block(
                                    x + width,
                                    y + 1,
                                    z,
                                ) == BlockType::Air
                            };

                        if !visible {
                            break;
                        }

                        width += 1;
                    }

                    let mut height = 1;

                    'height:
                    while z + height < CHUNK_SIZE {

                        for dx in 0..width {

                            let next =
                                chunk.get_block(
                                    x + dx,
                                    y,
                                    z + height,
                                );

                            if next != block {
                                break 'height;
                            }

                            let visible =
                                if y + 1 >= CHUNK_SIZE {

                                    true

                                } else {

                                    chunk.get_block(
                                        x + dx,
                                        y + 1,
                                        z + height,
                                    ) == BlockType::Air
                                };

                            if !visible {
                                break 'height;
                            }
                        }

                        height += 1;
                    }

                    for dz in 0..height {
                        for dx in 0..width {

                            let i =
                                (x + dx) +
                                y * CHUNK_SIZE +
                                (z + dz) *
                                CHUNK_SIZE *
                                CHUNK_SIZE;

                            visited[i] = true;
                        }
                    }

                    Self::add_top_quad(
                        vertices,

                        x as f32,
                        y as f32,
                        z as f32,

                        width as f32,
                        height as f32,
                    );
                }
            }
        }
    }

    fn mesh_bottom(
        chunk: &Chunk,
        vertices: &mut Vec<f32>,
    ) {

        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {

                    let block =
                        chunk.get_block(x, y, z);

                    if block == BlockType::Air {
                        continue;
                    }

                    let visible =
                        if y == 0 {

                            true

                        } else {

                            chunk.get_block(
                                x,
                                y - 1,
                                z,
                            ) == BlockType::Air
                        };

                    if visible {

                        Self::add_bottom_face(
                            vertices,

                            x as f32,
                            y as f32,
                            z as f32,
                        );
                    }
                }
            }
        }
    }

    fn mesh_front(
        chunk: &Chunk,
        vertices: &mut Vec<f32>,
    ) {

        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {

                    let block =
                        chunk.get_block(x, y, z);

                    if block == BlockType::Air {
                        continue;
                    }

                    let visible =
                        if z + 1 >= CHUNK_SIZE {

                            true

                        } else {

                            chunk.get_block(
                                x,
                                y,
                                z + 1,
                            ) == BlockType::Air
                        };

                    if visible {

                        Self::add_front_face(
                            vertices,

                            x as f32,
                            y as f32,
                            z as f32,
                        );
                    }
                }
            }
        }
    }

    fn mesh_back(
        chunk: &Chunk,
        vertices: &mut Vec<f32>,
    ) {

        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {

                    let block =
                        chunk.get_block(x, y, z);

                    if block == BlockType::Air {
                        continue;
                    }

                    let visible =
                        if z == 0 {

                            true

                        } else {

                            chunk.get_block(
                                x,
                                y,
                                z - 1,
                            ) == BlockType::Air
                        };

                    if visible {

                        Self::add_back_face(
                            vertices,

                            x as f32,
                            y as f32,
                            z as f32,
                        );
                    }
                }
            }
        }
    }

    fn mesh_left(
        chunk: &Chunk,
        vertices: &mut Vec<f32>,
    ) {

        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {

                    let block =
                        chunk.get_block(x, y, z);

                    if block == BlockType::Air {
                        continue;
                    }

                    let visible =
                        if x == 0 {

                            true

                        } else {

                            chunk.get_block(
                                x - 1,
                                y,
                                z,
                            ) == BlockType::Air
                        };

                    if visible {

                        Self::add_left_face(
                            vertices,

                            x as f32,
                            y as f32,
                            z as f32,
                        );
                    }
                }
            }
        }
    }

    fn mesh_right(
        chunk: &Chunk,
        vertices: &mut Vec<f32>,
    ) {

        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {

                    let block =
                        chunk.get_block(x, y, z);

                    if block == BlockType::Air {
                        continue;
                    }

                    let visible =
                        if x + 1 >= CHUNK_SIZE {

                            true

                        } else {

                            chunk.get_block(
                                x + 1,
                                y,
                                z,
                            ) == BlockType::Air
                        };

                    if visible {

                        Self::add_right_face(
                            vertices,

                            x as f32,
                            y as f32,
                            z as f32,
                        );
                    }
                }
            }
        }
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

    fn add_front_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {

        let face = [

            x,y,z+1.0,
            x+1.0,y,z+1.0,
            x+1.0,y+1.0,z+1.0,

            x+1.0,y+1.0,z+1.0,
            x,y+1.0,z+1.0,
            x,y,z+1.0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_back_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {

        let face = [

            x,y,z,
            x,y+1.0,z,
            x+1.0,y+1.0,z,

            x+1.0,y+1.0,z,
            x+1.0,y,z,
            x,y,z,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_left_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {

        let face = [

            x,y+1.0,z+1.0,
            x,y+1.0,z,
            x,y,z,

            x,y,z,
            x,y,z+1.0,
            x,y+1.0,z+1.0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_right_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {

        let face = [

            x+1.0,y+1.0,z+1.0,
            x+1.0,y,z,
            x+1.0,y+1.0,z,

            x+1.0,y,z,
            x+1.0,y+1.0,z+1.0,
            x+1.0,y,z+1.0,
        ];

        vertices.extend_from_slice(&face);
    }

    fn add_bottom_face(
        vertices: &mut Vec<f32>,
        x: f32,
        y: f32,
        z: f32,
    ) {

        let face = [

            x,y,z,
            x+1.0,y,z,
            x+1.0,y,z+1.0,

            x+1.0,y,z+1.0,
            x,y,z+1.0,
            x,y,z,
        ];

        vertices.extend_from_slice(&face);
    }
}
