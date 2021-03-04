use oxygengine::prelude::*;

#[derive(Default)]
pub struct Camera {
	pub camera: Option<Entity>,
	pub prev_x: f32,
}