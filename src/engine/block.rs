#[derive(Copy, Clone, PartialEq)]
pub enum BlockType {

    Air,

    Grass,
    Dirt,
    Stone,
}

impl BlockType {

    pub fn is_solid(&self) -> bool {

        match self {

            BlockType::Air => false,

            _ => true,
        }
    }

    pub fn texture_index_top(
        &self
    ) -> (u32, u32) {

        match self {

            BlockType::Grass => (0, 0),

            BlockType::Dirt => (2, 0),

            BlockType::Stone => (3, 0),

            _ => (0, 0),
        }
    }

    pub fn texture_index_side(
        &self
    ) -> (u32, u32) {

        match self {

            BlockType::Grass => (1, 0),

            BlockType::Dirt => (2, 0),

            BlockType::Stone => (3, 0),

            _ => (0, 0),
        }
    }

    pub fn texture_index_bottom(
        &self
    ) -> (u32, u32) {

        match self {

            BlockType::Grass => (2, 0),

            BlockType::Dirt => (2, 0),

            BlockType::Stone => (3, 0),

            _ => (0, 0),
        }
    }
}
