use crate::engine::chunk::Chunk;

use crate::engine::block::BlockType;

pub struct VegetationRenderer;

impl VegetationRenderer {

    pub fn generate_grass(

        vertices: &mut Vec<f32>,

        x: f32,
        y: f32,
        z: f32,
    ) {

        // CROSS QUAD 1

        Self::add_quad(

            vertices,

            x,
            y,
            z,

            x + 1.0,
            y + 1.0,
            z + 1.0,
        );

        // CROSS QUAD 2

        Self::add_quad(

            vertices,

            x + 1.0,
            y,
            z,

            x,
            y + 1.0,
            z + 1.0,
        );
    }

    fn add_quad(

        vertices: &mut Vec<f32>,

        x1: f32,
        y1: f32,
        z1: f32,

        x2: f32,
        y2: f32,
        z2: f32,
    ) {

        let quad = [

            // pos              uv     ao/light

            x1,y1,z1,  0.0,0.0, 1.0,1.0,
            x2,y1,z2,  1.0,0.0, 1.0,1.0,
            x2,y2,z2,  1.0,1.0, 1.0,1.0,

            x1,y1,z1,  0.0,0.0, 1.0,1.0,
            x2,y2,z2,  1.0,1.0, 1.0,1.0,
            x1,y2,z1,  0.0,1.0, 1.0,1.0,
        ];

        vertices.extend_from_slice(
            &quad
        );
    }

    pub fn generate_chunk_grass(

        chunk: &Chunk,

        vertices: &mut Vec<f32>,
    ) {

        for y in 0..16 {

            for z in 0..16 {

                for x in 0..16 {

                    let block =
                        chunk.get_block(
                            x,
                            y,
                            z,
                        );

                    if block ==
                       BlockType::GrassPlant
                    {

                        Self::generate_grass(

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
}
