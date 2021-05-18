use oxygengine::prelude::*;

use crate::resources::referential::RefeCode;

#[derive(Default)]
pub struct Selected {
	pub pos: Vec2,
	pub code: RefeCode,
	pub visible: bool,
}