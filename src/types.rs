use bevy::{prelude::*, utils::HashMap};
use std::collections::HashSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EngineSystemSet {
    Input,
    CalculateMomentum,
    ApplyMomentum,
}
