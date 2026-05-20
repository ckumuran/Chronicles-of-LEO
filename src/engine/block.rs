#[derive(Copy, Clone, PartialEq)]
pub enum BlockType {

    Air,

    Grass,
    Dirt,
    Stone,

    Water,

    Torch,
}

impl BlockType {

    pub fn is_solid(&self) -> bool {

        match self {

            BlockType::Air => false,

            BlockType::Water => false,

            BlockType::Torch => false,

            _ => true,
        }
    }

    pub fn is_transparent(
        &self
    ) -> bool {

        match self {

            BlockType::Water => true,

            BlockType::Torch => true,

            _ => false,
        }
    }

    pub fn emits_light(
        &self
    ) -> u8 {

        match self {

            BlockType::Torch => 14,

            _ => 0,
        }
    }

    pub fn texture_index_top(
        &self
    ) -> (u32, u32) {

        match self {

            BlockType::Grass => (0, 0),

            BlockType::Grass => (0,0),

            BlockType::Dirt => (2, 0),

            BlockType::Stone => (3, 0),

            BlockType::Water => (4, 0),

            BlockType::Torch => (5, 0),

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

            BlockType::Water => (4, 0),

            BlockType::Torch => (5, 0),

            _ => (0, 0),
        }
    }

    pub fn texture_index_bottom(
        &self
    ) -> (u32, u32) {

        self.texture_index_side()
    }
}
