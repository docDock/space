use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_core::FixedTimestep;
use bevy_ecs::{
    schedule::{ParallelSystemDescriptorCoercion, SystemSet},
    system::ResMut,
};
use bevy_log::info;
use bevy_math::Quat;

use bevy_transform::components::Transform;

use crate::space::{
    core::entity::resources::{EntityDataProperties, GridItemData},
    entities::{
        air_locks::spawn::AirlockBundle, computers::spawn::ComputerBundle,
        construction_tool_admin::spawn::ConstructionToolBundle,
        counter_windows::spawn::CounterWindowBundle, helmet_security::spawn::HelmetSecurityBundle,
        human_male_pawn::spawn::HumanMalePawnBundle,
        jumpsuit_security::spawn::JumpsuitSecurityBundle, pistol_l1::spawn::PistolL1Bundle,
    },
    PostUpdateLabels, StartupLabels,
};

use self::{
    events::{NetLoadEntity, NetSendEntityUpdates, NetShowcase, NetUnloadEntity},
    resources::EntityDataResource,
    systems::{
        broadcast_position_updates::{broadcast_position_updates, INTERPOLATION_LABEL1},
        send_entity_updates::send_entity_updates,
    },
};

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

pub fn startup_entities(mut entity_data: ResMut<EntityDataResource>) {
    let mut entities = vec![];

    entities.push(EntityDataProperties {
        name: "jumpsuitSecurity".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(JumpsuitSecurityBundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "helmetSecurity".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HelmetSecurityBundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "pistolL1".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(PistolL1Bundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "humanDummy".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HumanMalePawnBundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "humanMale".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(HumanMalePawnBundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "securityAirLock1".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(AirlockBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    });

    entities.push(EntityDataProperties {
        name: "vacuumAirLock".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(AirlockBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    });

    entities.push(EntityDataProperties {
        name: "governmentAirLock".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(AirlockBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    });

    entities.push(EntityDataProperties {
        name: "bridgeAirLock".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(AirlockBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: Transform::identity(),
            can_be_built_with_grid_item: vec![],
        }),
    });

    let mut transform = Transform::identity();
    transform.translation.y = 0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    entities.push(EntityDataProperties {
        name: "securityCounterWindow".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(CounterWindowBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: transform,
            can_be_built_with_grid_item: vec!["securityCounter1".to_string()],
        }),
    });

    let mut transform = Transform::identity();
    transform.translation.y = 0.86;
    transform.rotation = Quat::from_xyzw(0., 0.707, 0., 0.707);

    entities.push(EntityDataProperties {
        name: "bridgeCounterWindow".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(CounterWindowBundle::spawn),
        grid_item: Some(GridItemData {
            transform_offset: transform,
            can_be_built_with_grid_item: vec!["bridgeCounter".to_string()],
        }),
    });

    entities.push(EntityDataProperties {
        name: "constructionTool".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(ConstructionToolBundle::spawn),
        ..Default::default()
    });

    entities.push(EntityDataProperties {
        name: "bridgeComputer".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(ComputerBundle::spawn),
        ..Default::default()
    });

    info!("Loaded {} different entity types.", entities.len());

    for entity_properties in entities {
        entity_data
            .id_to_name
            .insert(entity_properties.id, entity_properties.name.clone());
        entity_data
            .name_to_id
            .insert(entity_properties.name.clone(), entity_properties.id);

        entity_data.data.push(entity_properties);
    }
}

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityDataResource>()
            .add_system_to_stage(
                PostUpdate,
                send_entity_updates
                    .after(PostUpdateLabels::EntityUpdate)
                    .label(PostUpdateLabels::SendEntityUpdates),
            )
            .add_event::<NetShowcase>()
            .add_event::<NetSendEntityUpdates>()
            .add_event::<NetUnloadEntity>()
            .add_event::<NetLoadEntity>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(broadcast_position_updates),
            )
            .add_startup_system(startup_entities.before(StartupLabels::BuildGridmap));
    }
}
