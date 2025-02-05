use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    prelude::{FromWorld, World},
};

pub struct HandleToEntity {
    pub map: HashMap<u32, Entity>,
    pub inv_map: HashMap<Entity, u32>,
}

impl FromWorld for HandleToEntity {
    fn from_world(_world: &mut World) -> Self {
        HandleToEntity {
            map: HashMap::new(),
            inv_map: HashMap::new(),
        }
    }
}
