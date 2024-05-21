use super::collision::CollisionLayer;
use bevy::prelude::*;
use bevy_xpbd_3d::{math::*, prelude::*};

pub struct PhysicsTypesPlugin;

impl Plugin for PhysicsTypesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LandingEvent>()
            .register_type::<MoveSpeed>()
            .register_type::<MoveDirection>()
            .register_type::<Character>()
            .register_type::<Grounded>()
            .register_type::<LateralDamping>()
            .register_type::<MeshColliderMarker>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MoveDirection {
    current_direction: Vec3,
    previous_direction: Vec3,
}

impl MoveDirection {
    pub fn get(&self) -> Vec3 {
        self.current_direction
    }

    pub fn set(&mut self, value: Vec3) {
        self.previous_direction = self.current_direction;
        self.current_direction = value;
    }

    pub fn is_any(&self) -> bool {
        self.current_direction != Vec3::ZERO
    }

    pub fn started_moving(&self) -> bool {
        self.current_direction != Vec3::ZERO && self.previous_direction == Vec3::ZERO
    }

    pub fn stopped_moving(&self) -> bool {
        self.current_direction == Vec3::ZERO && self.previous_direction != Vec3::ZERO
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Momentum(Vec3);

impl Momentum {
    pub fn get(&self) -> Vec3 {
        self.0
    }

    pub fn set(&mut self, value: Vec3) {
        self.0 = value;
    }

    pub fn is_any(&self) -> bool {
        self.0 != Vec3::ZERO
    }
}

#[derive(Default, Reflect)]
pub enum MoveSpeedState {
    #[default]
    Paused,
    Startup,
    Accelerating,
    Decelerating,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MoveSpeed {
    state: MoveSpeedState,
    base_speed: f32,
    acceleration: f32,
    current_speed: f32,
    max_speed: f32,
    accelerate_timer: Timer,
    decelerate_timer: Timer,
}

impl MoveSpeed {
    pub fn new(base_speed: f32) -> Self {
        MoveSpeed {
            state: MoveSpeedState::default(),
            base_speed,
            acceleration: 1.5,
            current_speed: base_speed,
            max_speed: base_speed * 2.0,
            accelerate_timer: Timer::from_seconds(0.3, TimerMode::Once),
            decelerate_timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }

    pub fn get(&self) -> f32 {
        self.current_speed
    }

    pub fn tick(&mut self, time: &Res<Time>) {
        match self.state {
            MoveSpeedState::Startup => {
                self.accelerate_timer.tick(time.delta());
                if self.accelerate_timer.finished() {
                    self.state = MoveSpeedState::Accelerating;
                }
            }
            MoveSpeedState::Decelerating => {
                self.decelerate_timer.tick(time.delta());
                if self.decelerate_timer.finished() {
                    self.decelarate(time.delta_seconds());
                }
            }
            MoveSpeedState::Accelerating => {
                self.accelarate(time.delta_seconds());
            }
            _ => (),
        }
    }

    pub fn start_moving(&mut self) {
        match self.state {
            MoveSpeedState::Decelerating => {
                self.state = MoveSpeedState::Accelerating;
                self.decelerate_timer.reset();
            }
            _ => {
                self.state = MoveSpeedState::Startup;
                self.accelerate_timer.reset();
                self.decelerate_timer.reset();
            }
        }
    }

    pub fn stop_moving(&mut self) {
        match self.state {
            MoveSpeedState::Accelerating => self.state = MoveSpeedState::Decelerating,
            _ => {
                self.state = MoveSpeedState::Paused;
                self.decelerate_timer.reset();
                self.accelerate_timer.reset();
            }
        }
    }

    fn accelarate(&mut self, delta_seconds: f32) {
        self.current_speed = self
            .current_speed
            .lerp(self.max_speed, delta_seconds * self.acceleration);
    }

    fn decelarate(&mut self, delta_seconds: f32) {
        self.current_speed = self
            .current_speed
            .lerp(self.base_speed, delta_seconds * self.acceleration);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MeshColliderMarker(f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Grounded;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Jumping;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Regrab;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LateralDamping(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CoyoteTime(Timer);

impl CoyoteTime {
    pub fn tick(&mut self, delta: std::time::Duration) {
        self.0.tick(delta);
    }

    pub fn finished(&self) -> bool {
        self.0.finished()
    }
}

impl Default for CoyoteTime {
    fn default() -> Self {
        CoyoteTime(Timer::from_seconds(0.33, TimerMode::Once))
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Character {
    pub ride_height: f32,
    pub spring_strength: f32,
    pub spring_damper: f32,
    pub jump_strength: f32,
    pub base_gravity_scale: f32,
    pub regrab_gravity_scale: f32,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            ride_height: 1.4,
            spring_strength: 23.0,
            spring_damper: 5.0,
            jump_strength: 17.5,
            base_gravity_scale: 2.0,
            regrab_gravity_scale: 1.5,
        }
    }
}

#[derive(Bundle)]
pub struct CharacterBundle {
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub external_force: ExternalForce,
    pub restitution: Restitution,
    pub character: Character,
    pub shape_caster: ShapeCaster,
    pub gravity_scale: GravityScale,
    pub lateral_damping: LateralDamping,
}

impl Default for CharacterBundle {
    fn default() -> Self {
        CharacterBundle {
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collider: Collider::capsule(1.0, 0.5),
            external_force: ExternalForce::new(Vec3::ZERO).with_persistence(false),
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombine::Min,
            },
            character: Character {
                ride_height: 1.4,
                spring_strength: 23.0,
                spring_damper: 5.0,
                jump_strength: 17.5,
                base_gravity_scale: 2.0,
                regrab_gravity_scale: 1.5,
            },
            shape_caster: ShapeCaster::new(
                Collider::capsule(1.0, 0.35),
                Vec3::NEG_Y * 0.05,
                Quaternion::default(),
                Direction3d::NEG_Y,
            )
            .with_max_time_of_impact(1.0)
            .with_max_hits(1)
            .with_ignore_self(true)
            .with_query_filter(SpatialQueryFilter::from_mask(
                CollisionLayer::standable_mask(),
            )),
            gravity_scale: GravityScale(2.0),
            lateral_damping: LateralDamping(5.0),
        }
    }
}

/// Marker component for pipeline convenience, will be removed on the first frame and will insert a
/// character bundle in it's place
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct CharacterPhysicsSettings {
    pub collider_height: f32,
    pub collider_radius: f32,
}

#[derive(Event)]
pub struct LandingEvent(pub Entity);
