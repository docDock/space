use crate::space_core::ecs::networking::resources::ReliableServerMessage;


pub struct NetLoadEntity {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetShowcase {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetUnloadEntity {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetSendEntityUpdates {
    pub handle : u32,
    pub message : ReliableServerMessage
}
