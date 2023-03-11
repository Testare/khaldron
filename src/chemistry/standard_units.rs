use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, FromReflect, Serialize, Reflect)]
pub struct Temperature(u16);

impl Temperature {
    const ABSOLUTE_ZERO: Temperature = Temperature(0);
    const FREEZING: Temperature = Temperature(273);
    const ROOM_TEMPERATURE: Temperature = Temperature(293);
    const BOILING: Temperature = Temperature(373);
}

impl Default for Temperature {
    fn default() -> Self {
        Self::ROOM_TEMPERATURE
    }
}

pub type Volume = f32;
