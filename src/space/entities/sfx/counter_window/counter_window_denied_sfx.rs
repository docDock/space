use bevy_transform::components::Transform;

use crate::space::core::{
    entity::components::{EntityData, EntityUpdates},
    sensable::components::Sensable,
    sfx::components::Sfx,
    static_body::components::StaticTransform,
};

pub struct CounterWindowDeniedSfxBundle;

pub const PLAY_BACK_DURATION: f32 = 1. + 1.;

impl CounterWindowDeniedSfxBundle {
    pub fn new(
        passed_transform: Transform,
    ) -> (StaticTransform, EntityData, Sensable, Sfx, EntityUpdates) {
        (
            StaticTransform {
                transform: passed_transform,
            },
            EntityData {
                entity_class: "SFX".to_string(),
                ..Default::default()
            },
            Sensable {
                is_audible: true,
                ..Default::default()
            },
            Sfx {
                unit_db: 20.,
                stream_id: "/content/audio/counterWindow/windowAccessDenied.sample".to_string(),
                play_back_duration: PLAY_BACK_DURATION,
                ..Default::default()
            },
            EntityUpdates::default(),
        )
    }
}
