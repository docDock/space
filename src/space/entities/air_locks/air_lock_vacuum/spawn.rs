use std::collections::{BTreeMap, HashMap};

use bevy_ecs::{entity::Entity, system::Commands};
use bevy_math::Vec3;
use bevy_rapier3d::prelude::{
    ActiveEvents, ColliderBundle, ColliderFlags, ColliderShape, InteractionGroups, RigidBodyBundle,
    RigidBodyType,
};
use bevy_transform::components::Transform;

use crate::space::{
    core::{
        entity::{
            components::{
                DefaultMapEntity, EntityData, EntityGroup, EntityUpdates, Examinable, RichName,
                Sensable,
            },
            functions::transform_to_isometry::transform_to_isometry,
            resources::{SpawnHeldData, SpawnPawnData},
        },
        health::components::{Health, HealthFlag},
        pawn::{
            components::SpaceAccessEnum,
            functions::new_chat_message::{FURTHER_ITALIC_FONT, HEALTHY_COLOR},
        },
        physics::functions::{get_bit_masks, ColliderGroup},
        static_body::components::StaticTransform, networking::resources::{ConsoleCommandVariantValues},
    },
    entities::air_locks::components::AirLock,
};

pub struct VacuumAirlockBundle;

impl VacuumAirlockBundle {
    pub fn spawn(
        entity_transform: Transform,
        commands: &mut Commands,
        _correct_transform: bool,
        _pawn_data_option: Option<SpawnPawnData>,
        _held_data_option: Option<SpawnHeldData>,
        default_map_spawn: bool,
        _properties : HashMap<String,ConsoleCommandVariantValues>,
    ) -> Entity {
        let static_transform_component = StaticTransform {
            transform: entity_transform,
        };

        let rigid_body_component = RigidBodyBundle {
            body_type: RigidBodyType::Static.into(),
            position: transform_to_isometry(entity_transform).into(),
            ..Default::default()
        };

        let masks = get_bit_masks(ColliderGroup::Standard);

        let collider_component = ColliderBundle {
            shape: ColliderShape::cuboid(1., 1., 0.2).into(),
            position: Vec3::new(0., 1., 0.).into(),
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(masks.0, masks.1),
                active_events: (ActiveEvents::CONTACT_EVENTS),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        };

        let mut examine_map = BTreeMap::new();
        examine_map.insert(
            0,
            "An air lock with vacuum warning colors. Opening this door will expose you to space."
                .to_string(),
        );
        examine_map.insert(
            1,
            "[font=".to_string()
                + FURTHER_ITALIC_FONT
                + "][color="
                + HEALTHY_COLOR
                + "]It is fully operational.[/color][/font]",
        );

        let mut health_flags = HashMap::new();

        health_flags.insert(0, HealthFlag::ArmourPlated);

        let mut builder = commands.spawn_bundle(rigid_body_component);

        builder.insert_bundle(collider_component).insert_bundle((
            static_transform_component,
            Sensable::default(),
            AirLock {
                access_permissions: vec![SpaceAccessEnum::Security],
                ..Default::default()
            },
            EntityData {
                entity_class: "entity".to_string(),
                entity_name: "vacuumAirLock".to_string(),
                entity_group: EntityGroup::AirLock,
            },
            EntityUpdates::default(),
            Examinable {
                name: RichName {
                    name: "vacuum airlock".to_string(),
                    n: false,
                    ..Default::default()
                },
                assigned_texts: examine_map,
                ..Default::default()
            },
            Health {
                is_combat_obstacle: true,
                is_reach_obstacle: true,
                ..Default::default()
            },
        ));

        if default_map_spawn {
            builder.insert(DefaultMapEntity);
        }

        builder.id()
    }
}
