use oxygengine::prelude::*;

#[derive(Default)]
pub struct Time {
    pub day: u32,
	pub hour: u32,
    pub phase: Scalar,
}