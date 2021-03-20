use oxygengine::prelude::*;

use crate::resources::camera::Camera;

pub struct PanelClickSystem;

impl<'s> System<'s> for PanelClickSystem {
    type SystemData = (
		Read<'s, InputController>,
		ReadStorage<'s, CompositeUiElement>,
		Read<'s, CompositeCameraCache>,
		Read<'s, Camera>,
    );

    fn run(
        &mut self,
		(input, ui_elements, camera_cache, camera_res)
		: Self::SystemData,
    ) {
       if input.trigger_or_default("mouse-left") == TriggerState::Pressed {
			let x = input.axis_or_default("mouse-x");
            let y = input.axis_or_default("mouse-y");
			//let point = [x, y].into();
			
			if let Some(camera_entity) = camera_res.camera {
				for ui_element in (ui_elements).join() {	
					for child in &ui_element.children {  
						if let Some(id) = &child.id {
							if id == "item" {
								if let Some(w) = &child.fixed_width {
									//debug!("{0}", w);
								}
								
								
							}
						}
					}
				}
			}
        }
	}	
}