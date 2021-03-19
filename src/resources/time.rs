use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

pub const TIME_STORAGE: &str = "time";

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Time {
    pub day: u32,
	pub hour: u32,
    pub phase: Scalar,
}