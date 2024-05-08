use crate::player::Player;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<OverworldItem>()
            .register_type::<Inventory>()
            .register_type::<Item>()
            .register_type::<ItemId>()
            .insert_resource(Inventory::default())
            .add_systems(Update, pickup_items);
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn add_to_inventory(&mut self, item_id: ItemId) {
        self.items.push(Item::from_item_id(item_id));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct OverworldItem {
    pub id: ItemId,
}

#[derive(Reflect)]
pub struct Item {
    /// the type of item it is
    pub id: ItemId,
    /// used to determine what the item sells/can be bought for
    pub value: f32,
    /// 3d model to be displayed when inspecting the item in your inventory
    pub inspection_model: Handle<Scene>,
    /// Icon to display on the inventory page
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

#[derive(PartialEq, Reflect)]
pub enum ItemId {
    Milkshake,
}

fn pickup_items(
    // Commands is a bevy system param you need whenever you are adding/removing components,
    // entites and resources to and from the world
    mut commands: Commands,
    // A bevy_xpbd resource that lists all collisions,
    collisions: Res<Collisions>,
    // a resource that holds the players inventory
    inventory: Res<Inventory>,
    // A query that finds the player entity
    player_query: Query<Entity, With<Player>>,
    // A query that finds all entities with an OverWorldItem Component
    item_query: Query<(Entity, &OverworldItem)>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        for collision in collisions.collisions_with_entity(player_entity) {
            // in order to pickup an item you will need to do the following
            // 1. Use collisions to find all entities colliding with the player entity
            // 2. Check those collision pairs to see if one of the entites is the player and the other is
            //    the item
            // 3. Add the item to the players inventory
            // 4. Despawn the item
        }
    }
}
