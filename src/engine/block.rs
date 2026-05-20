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
}
