use oxygengine::prelude::*;

use crate::resources::camera::Camera;

const PIXL_BORDER: f32 = 20.;

pub struct CameraControlSystem;

impl<'s> System<'s> for CameraControlSystem {
    type SystemData = (
        ReadExpect<'s, WebCompositeRenderer>,
        Write<'s, Camera>,
		Read<'s, InputController>,
        ReadStorage<'s, CompositeCamera>,
        WriteStorage<'s, CompositeTransform>,
    );

    fn run(
		&mut self, 
		(renderer, mut camera_res, input, cameras, mut transforms)
		: Self::SystemData
	) {
        if camera_res.camera.is_none() {
            return;
        }

		let screen_size = renderer.view_size();
		
		let x = input.axis_or_default("mouse-x");
		let y = input.axis_or_default("mouse-y");
		
		//when mouse move
		if x != camera_res.prev_x || y != camera_res.prev_y {

			//when borders left or right
			let left = screen_size.x / PIXL_BORDER;
			let right = screen_size.x - left; 
			let mut x_inc = 0.;
			if x < left || x > right {
				x_inc = match x > right {
					true => 1.,
					false => -1.,
				};
			}
			
			//when borders top or bottom
			let top = screen_size.y / PIXL_BORDER;
			let bottom = screen_size.y - top; 
			let mut y_inc = 0.;
			if y < top || y > bottom {
				y_inc = match y > bottom {
					true => 1.,
					false => -1.,
				};
			}
			if x_inc != 0. || y_inc != 0. {
				self.apply_transform(&camera_res, cameras, screen_size, &mut transforms, x_inc, y_inc);
			}
		}
		camera_res.prev_x = x;
		camera_res.prev_y = y;
    }
}

impl<'s> CameraControlSystem {
	fn apply_transform(
		&self,
		camera_res: &Write<'s, Camera>,
		cameras: ReadStorage<'s, CompositeCamera>,
		screen_size: Vec2,
		transforms: &mut WriteStorage<'s, CompositeTransform>,
		x_inc: f32,
		y_inc: f32,
	) {
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
			view_box.x += x_inc;
			view_box.y += y_inc;
			transforms
				.get_mut(entity)
				.unwrap()
				.set_translation(view_box.center());
		}
	}
}