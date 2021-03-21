use oxygengine::prelude::*;

use crate::resources::camera::Camera;

pub struct PanelClickSystem;

impl<'s> System<'s> for PanelClickSystem {
    type SystemData = (
		Read<'s, InputController>,
		Read<'s, CompositeUiInteractibles>,
		ReadStorage<'s, CompositeUiElement>,
		Read<'s, CompositeCameraCache>,
		Read<'s, Camera>,
    );

    fn run(
        &mut self,
		(input, interactibles, ui_elements, camera_cache, camera_res)
		: Self::SystemData,
    ) {
       if input.trigger_or_default("mouse-left") == TriggerState::Pressed {
			let x = input.axis_or_default("mouse-x");
            let y = input.axis_or_default("mouse-y");
			let point = [x, y].into();

			if let Some(camera_entity) = camera_res.camera_panel {
				for ui_element in (ui_elements).join() {	
					if let Some(inter_name) = &ui_element.interactive {

						if let Some(pos) = camera_cache.screen_to_world_space(camera_entity, point) {
							if interactibles.does_rect_contains_point(inter_name, pos) {

								debug!("{0}", inter_name);


							}
						}
					}
				}
			}
        }
	}	
}