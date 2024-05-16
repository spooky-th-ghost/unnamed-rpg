use bevy::prelude::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Openable>()
            .register_type::<Open>()
            .register_type::<Closed>()
            .register_type::<Door>()
            .register_type::<Dumpster>()
            .register_type::<Chest>()
            .register_type::<ChestContents>();
    }
}

#[derive(Reflect, Default)]
pub enum ChestContents {
    Money(f32),
    #[default]
    Nothing,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Chest {
    pub contents: ChestContents,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Openable;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Open;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Closed;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Dumpster;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Door;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Transition {
    pub destination: TransitionDestination,
}

#[derive(Reflect)]
pub enum TransitionDestination {
    Location(Vec3),
    Scene,
}

impl Default for TransitionDestination {
    fn default() -> Self {
        Self::Location(Vec3::ZERO)
    }
}

#[derive(Event)]
pub struct OpenEvent(pub Entity);

pub fn read_open_events(
    mut commands: Commands,
    mut events: EventReader<OpenEvent>,
    query: Query<Entity, (With<Openable>, With<Closed>)>,
) {
    for event in events.read() {
        if let Ok(entity) = query.get(event.0) {
            commands.entity(entity).insert(Open).remove::<Closed>();
        }
    }
}

pub fn open_object(query: Query<(Has<Door>, Option<&Chest>), Added<Open>>) {
    for (is_door, has_chest) in &query {
        //Open up bitch
    }
}
