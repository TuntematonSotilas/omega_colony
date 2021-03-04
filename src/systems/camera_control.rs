use oxygengine::prelude::*;

use crate::resources::camera::Camera;

const PIXL_BORDER: f32 = 20.;

pub struct CameraControlSystem;

impl<'s> System<'s> for CameraControlSystem {
    type SystemData = (
        ReadExpect<'s, WebCompositeRenderer>,
        Read<'s, Camera>,
		Read<'s, InputController>,
        ReadStorage<'s, CompositeCamera>,
        WriteStorage<'s, CompositeTransform>,
    );

    fn run(
		&mut self, 
		(renderer, camera_res, input, cameras, mut transforms)
		: Self::SystemData
	) {
        if camera_res.camera.is_none() {
            return;
        }

		let screen_size = renderer.view_size();
		
		let x = input.axis_or_default("mouse-x");
		let y = input.axis_or_default("mouse-y");
		
		let x_min = 0.;
		let x_max = screen_size.x;
		
		let x_left = screen_size.x / PIXL_BORDER;
		let x_right = screen_size.x - x_left; 

		//when border
		if x > x_min && x < x_max && (x < x_left || x > x_right) {
			debug!("------");
			let entity = camera_res.camera.unwrap();
			let view_box = if let Some(transform) = transforms.get(entity) {
				if let Some(camera) = cameras.get(entity) {
					camera.view_box(transform, screen_size)
				} else {
					None
				}
			} else {
				None
			};
			if let Some(mut view_box) = view_box {
				let x_inc = match x > x_right {
					true => 1.,
					false => -1.,
				};
				view_box.x += x_inc;
				transforms
					.get_mut(entity)
					.unwrap()
					.set_translation(view_box.center());
			}
		}
	
    }
}