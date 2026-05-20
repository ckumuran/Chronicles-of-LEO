use crate::engine::block::BlockType;

pub const CHUNK_SIZE: usize = 16;

pub struct Chunk {
    pub blocks: Vec<BlockType>,

    pub position: (i32, i32, i32),

    pub dirty: bool,
}

impl Chunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            blocks: vec![
                BlockType::Air;
                CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE
            ],

            position: (x, y, z),

            dirty: true,
        }
    }

    fn index(x: usize, y: usize, z: usize) -> usize {
        x +
        y * CHUNK_SIZE +
        z * CHUNK_SIZE * CHUNK_SIZE
    }

    pub fn get_block(
        &self,
        x: usize,
        y: usize,
        z: usize,
    ) -> BlockType {
        self.blocks[
            Self::index(x, y, z)
        ]
    }

    pub fn set_block(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        block: BlockType,
    ) {
        let index = Self::index(x, y, z);

        self.blocks[index] = block;

        self.dirty = true;
    }
}
