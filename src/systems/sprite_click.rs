use oxygengine::prelude::*;

use crate::resources::camera::Camera;
use crate::components::clickable::Clickable;

pub struct SpriteClickSystem;

impl<'s> System<'s> for SpriteClickSystem {
    type SystemData = (
        Read<'s, InputController>,
		ReadStorage<'s, CompositeSprite>,
		Read<'s, CompositeCameraCache>,
		Read<'s, Camera>,
		ReadStorage<'s, Clickable>,
    );

    fn run(
        &mut self, (input, sprites, camera_cache, camera_res, clickables) : Self::SystemData,
    ) {
        if input.trigger_or_default("mouse-left") == TriggerState::Pressed {
			let x = input.axis_or_default("mouse-x");
            let y = input.axis_or_default("mouse-y");
			let point = [x, y].into();
			
			for (sprite, clickable) in (&sprites, &clickables).join() {	
				
			}

			if let Some(camera_entity) = camera_res.camera {
				if let Some(pos) = camera_cache.screen_to_world_space(camera_entity, point) {

					debug!("clic");
				}
			}
        }
	}	
}