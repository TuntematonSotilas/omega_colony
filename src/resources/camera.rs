use oxygengine::prelude::*;

#[derive(Default)]
pub struct Camera {
	pub camera: Option<Entity>,
	pub prev_x: f32,
	pub prev_y: f32,
}