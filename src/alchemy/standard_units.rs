use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, FromReflect, Serialize, Reflect)]
pub struct Temperature(u16);

pub type Volume = f32;

impl Default for Temperature {
    fn default() -> Self {
        Self::ROOM_TEMPERATURE
    }
}

impl Temperature {
    pub const ABSOLUTE_ZERO: Temperature = Temperature(0);
    pub const FREEZING: Temperature = Temperature(273);
    pub const ROOM_TEMPERATURE: Temperature = Temperature(293);
    pub const BOILING: Temperature = Temperature(373);
    const KELVIN_TO_CELSIUS: i16 = 273;

    fn from_kelvin(kelvin: u16) -> Self {
        Temperature(kelvin)
    }

    fn from_celsius(celsius: i16) -> Self {
        Temperature((celsius + Self::FREEZING.0 as i16) as u16)
    }

    fn kelvin(&self) -> u16 {
        self.0
    }

    fn celsius(&self) -> i16 {
        self.0 as i16 - Self::FREEZING.0 as i16
    }

    fn aproximate_fahrenheit(&self) -> i16 {
        ((self.0 as i16 - Self::KELVIN_TO_CELSIUS) * 9) / 5 + 32
    }
}
