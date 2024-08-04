use pumpkin_protocol::VarInt;
use serde::{Deserialize, Serialize};

use super::{Entity, EntityId};

pub struct Player {
    pub entity: Entity,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,

    // Client side value, Should be not trusted
    pub on_ground: bool,

    // Current awaiting teleport id, None if did not teleport
    pub awaiting_teleport: Option<VarInt>,
}

impl Player {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
            on_ground: false,
            awaiting_teleport: None,
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity.entity_id
    }
}

pub enum Hand {
    Main,
    Off,
}

impl Hand {
    pub fn from_varint(varint: VarInt) -> Self {
        match varint {
            0 => Self::Off,
            1 => Self::Main,
            _ => {
                log::info!("Unexpected Hand {}", varint);
                Self::Main
            }
        }
    }
}

pub enum ChatMode {
    Enabled,
    CommandsOnly,
    Hidden,
}

impl ChatMode {
    pub fn from_varint(varint: VarInt) -> Self {
        match varint {
            0 => Self::Enabled,
            1 => Self::CommandsOnly,
            2 => Self::Hidden,
            _ => {
                log::info!("Unexpected ChatMode {}", varint);
                Self::Enabled
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameMode {
    Undefined,
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl GameMode {
    pub fn from_byte(byte: i8) -> Self {
        match byte {
            -1 => GameMode::Undefined,
            0 => GameMode::Survival,
            1 => GameMode::Creative,
            2 => GameMode::Adventure,
            3 => GameMode::Spectator,
            _ => {
                log::info!("Unexpected GameMode {}", byte);
                Self::Survival
            }
        }
    }

    pub fn to_byte(self) -> i8 {
        match self {
            Self::Undefined => -1,
            Self::Survival => 0,
            Self::Creative => 1,
            Self::Adventure => 2,
            Self::Spectator => 3,
        }
    }
}
