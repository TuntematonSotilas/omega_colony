use oxygengine::prelude::*;

use crate::resources::camera::Camera;

const PIXL_BORDER: f32 = 20.;
const SPEED: f32 = 2.;
const INIT_POS: f32 = 64.;

pub type CameraControlSystemResources<'a> = (
    WorldRef,
    &'a InputController,
    &'a UserInterface,
    &'a mut Camera,
    Comp<&'a CompositeCamera>,
    Comp<&'a WebCompositeRenderer>,
	Comp<&'a mut CompositeTransform>,
);

pub fn camera_control_system(universe: &mut Universe) {

	let (world, input, ui, mut camera, ..) =
        universe.query_resources::<CameraControlSystemResources>();
	
	if camera.camera.is_none() {
		return;
	}

	for (_, (composite_camera,  renderer, transform )) in world
	.query::<(
		&CompositeCamera,
		&WebCompositeRenderer,
		&mut CompositeTransform,
	)>()
	.iter()
	{
		let screen_size = renderer.view_size();

		debug!("iter");

		//Center camera
		if !camera.is_centered {
			
			//world.apply_transform(&camera, composite_camera, screen_size, &mut transforms, INIT_POS, INIT_POS);

			let tr = transform.get_translation();
			debug!("tr {:?}", tr);
		}
	}
}
			/*let entity = camera.camera.unwrap();
			let view_box = if let Some(transform) = transform.get(entity) {
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
			
			camera.is_centered = true;
			return;
		}*/
	

	/*

	
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
				true => SPEED,
				false => - SPEED,
			};
		}
		
		//when borders top or bottom
		let top = screen_size.y / PIXL_BORDER;
		let bottom = screen_size.y - top; 
		let mut y_inc = 0.;
		if y < top || y > bottom {
			y_inc = match y > bottom {
				true => SPEED,
				false => - SPEED,
			};
		}
		if x_inc != 0. || y_inc != 0. {
			self.apply_transform(&camera_res, cameras, screen_size, &mut transforms, x_inc, y_inc);
		}
	}
	camera_res.prev_x = x;
	camera_res.prev_y = y;*/

	/*fn apply_transform(
		camera: &mut Camera,
		cameras: &mut CompositeCamera,
		screen_size: Vec2,
		transforms: &mut CompositeTransform,
		x_inc: f32,
		y_inc: f32,
	) {
		let entity = camera.camera.unwrap();
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
	}*/
