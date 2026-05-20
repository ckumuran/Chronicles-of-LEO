#[derive(
    Copy,
    Clone,
    PartialEq
)]
pub enum ItemType {

    Grass,
    Dirt,
    Stone,
    Wood,
    Torch,
}

#[derive(
    Copy,
    Clone,
)]
pub struct ItemStack {

    pub item: ItemType,

    pub count: u32,
}

pub struct Inventory {

    pub slots:
        [Option<ItemStack>; 9],

    pub selected_slot: usize,
}

impl Inventory {

    pub fn new() -> Self {

        let mut slots:
            [Option<ItemStack>; 9]
            =
            [None; 9];

        slots[0] =
            Some(
                ItemStack {

                    item:
                        ItemType::Grass,

                    count: 64,
                }
            );

        slots[1] =
            Some(
                ItemStack {

                    item:
                        ItemType::Stone,

                    count: 64,
                }
            );

        slots[2] =
            Some(
                ItemStack {

                    item:
                        ItemType::Wood,

                    count: 64,
                }
            );

        slots[3] =
            Some(
                ItemStack {

                    item:
                        ItemType::Torch,

                    count: 64,
                }
            );

        Self {

            slots,

            selected_slot: 0,
        }
    }

    pub fn scroll_up(
        &mut self,
    ) {

        self.selected_slot =
            (self.selected_slot + 1)
            % 9;
    }

    pub fn scroll_down(
        &mut self,
    ) {

        if self.selected_slot == 0 {

            self.selected_slot = 8;

        } else {

            self.selected_slot -= 1;
        }
    }

    pub fn selected_item(
        &self,
    ) -> Option<ItemType> {

        self.slots[
            self.selected_slot
        ]
        .map(|s| s.item)
    }
}
