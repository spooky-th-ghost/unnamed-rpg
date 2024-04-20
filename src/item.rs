use bevy::prelude::*;

pub struct Item {
    pub id: ItemId,
    pub value: f32,
    pub inspection_model: Handle<Scene>,
    pub inventory_icon: Handle<Image>,
}

impl Item {
    pub fn from_item_id(id: ItemId) -> Self {
        match id {
            ItemId::Milkshake => Item {
                id,
                value: 3.00,
                inspection_model: Handle::default(),
                inventory_icon: Handle::default(),
            },
        }
    }
}

pub enum ItemId {
    Milkshake,
}
