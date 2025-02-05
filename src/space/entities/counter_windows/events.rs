use bevy_ecs::entity::Entity;

use crate::space::core::entity::components::EntityGroup;

pub struct CounterWindowSensorCollision {
    pub collider1_entity: Entity,
    pub collider2_entity: Entity,

    pub collider1_group: EntityGroup,
    pub collider2_group: EntityGroup,

    pub started: bool,
}

pub struct InputCounterWindowToggleOpen {
    pub opener: Entity,
    pub opened: u64,
}
pub struct CounterWindowLockOpen {
    pub locked: Entity,
    pub locker: Entity,
}

pub struct CounterWindowLockClosed {
    pub locked: Entity,
    pub locker: Entity,
}
