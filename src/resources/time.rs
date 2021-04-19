use serde::{Deserialize, Serialize};

pub const TIME_STORAGE: &str = "time";

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Time {
    pub phase: f32,
    pub timestamp: f32,
}