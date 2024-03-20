use bevy::prelude::*;

#[derive(Reflect, Default, PartialEq)]
pub enum ChestStatus {
    Open,
    #[default]
    Closed,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Dumpster {
    pub status: ChestStatus,
}
