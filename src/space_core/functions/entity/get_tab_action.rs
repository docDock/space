use std::sync::Arc;

use crate::space_core::{components::{pawn::TabAction, inventory::Inventory}, resources::{network_messages::GridMapType}};

use super::can_reach_entity::REACH_DISTANCE;

pub fn get_tab_action(
    id : &str,
) -> Option<TabAction> {

    let result;

    if id == "examine" {

        result = Some(TabAction {
            id: id.to_string(),
            text: "Examine".to_string(),
            tab_list_priority: u8::MAX,
            prerequisite_check: Arc::new(examine_tab_prerequisite_check),
        });

    } else if id == "pickup" {

        result = Some(TabAction {
            id: id.to_string(),
            text: "Pickup".to_string(),
            tab_list_priority: u8::MAX-1,
            prerequisite_check: Arc::new(pickup_tab_prerequisite_check),
        });

    } else {
        result = None;
    }

    result

}

pub fn examine_tab_prerequisite_check(
    entity_id_bits_option : Option<u64>,
    cell_id_option : Option<(GridMapType, i16,i16,i16)>,
    _distance : f32,
    _inventory_component : &Inventory,
) -> bool {
    cell_id_option.is_some() || entity_id_bits_option.is_some()
}

pub fn pickup_tab_prerequisite_check(
    entity_id_bits_option : Option<u64>,
    _cell_id_option : Option<(GridMapType, i16,i16,i16)>,
    distance : f32,
    inventory_component : &Inventory,
) -> bool {
    distance < REACH_DISTANCE && entity_id_bits_option.is_some() && inventory_component.get_active_slot_entity().is_none()
}
