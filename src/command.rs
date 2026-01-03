use serde::Serialize;
use serde_repr::Serialize_repr;

#[allow(dead_code)]
#[derive(Serialize_repr, Debug, Clone, clap::ValueEnum)]
#[repr(u8)]
pub enum Timer {
    /// Turn off timer
    Off = 0,

    /// Set timer for one hour
    One = 1,

    /// Set timer for two hours
    Two = 2,

    /// Set timer for three hours
    Three = 3,

    /// Set timer for six hours
    Six = 6,
}

#[allow(dead_code)]
#[derive(Serialize, Debug, Clone, clap::ValueEnum)]
pub enum Color {
    #[serde(rename = "warm")]
    Warm,

    #[serde(rename = "cool")]
    Cool,

    #[serde(rename = "daylight")]
    Daylight,
}

#[allow(dead_code)]
#[derive(Serialize, Debug)]
pub enum Command {
    /// Turn the fan ON or OFF
    #[serde(rename = "power")]
    Power(bool),

    /// Set the speed of the fan to an absolute value
    #[serde(rename = "speed")]
    Speed(u8),

    /// Increase/decrease speed of the fan
    #[serde(rename = "speedDelta")]
    SpeedDelta(i8),

    ///  Enable or disable sleep mode
    #[serde(rename = "sleep")]
    Sleep(bool),

    /// Set timer
    #[serde(rename = "timer")]
    Timer(Timer),

    ///  Turn the light ON or OFF. For Aris Starlight, it will set the fan light at the last known
    ///  color or brightness values.
    #[serde(rename = "led")]
    Led(bool),

    ///  Set the brightness of the fan to an absolute value
    #[serde(rename = "brightness")]
    Brightness(i8),

    /// Increase/decrease brightness of the fan
    #[serde(rename = "brightnessDelta")]
    BrightnessDelta(u8),

    /// Change the color of the light
    #[serde(rename = "color")]
    Color(Color),
}

impl Command {
    // Return the JSON serialized string represention of this Command
    pub fn json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
