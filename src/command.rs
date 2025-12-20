use serde::Serialize;
use serde_repr::Serialize_repr;

#[allow(dead_code)]
#[derive(Serialize_repr)]
#[repr(u8)]
pub enum Timer {
    Off = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Six = 6,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub enum Color {
    #[serde(rename = "warm")]
    Warm,

    #[serde(rename = "cool")]
    Cool,

    #[serde(rename = "daylight")]
    Daylight,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub enum Command {
    #[serde(rename = "power")]
    Power(bool),

    #[serde(rename = "speed")]
    Speed(u8),

    #[serde(rename = "speedDelta")]
    SpeedDelta(i8),

    #[serde(rename = "sleep")]
    Sleep(bool),

    #[serde(rename = "timer")]
    Timer(Timer),

    #[serde(rename = "led")]
    Led(bool),

    #[serde(rename = "brightness")]
    Brightness(i8),

    #[serde(rename = "brightnessDelta")]
    BrightnessDelta(u8),

    #[serde(rename = "color")]
    Color(Color),
}

impl Command {
    // Return the JSON serialized string represention of this Command
    pub fn json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

