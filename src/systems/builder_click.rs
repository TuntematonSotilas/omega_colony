use oxygengine::prelude::*;

pub struct BuilderClickSystem;

impl<'s> System<'s> for BuilderClickSystem {
    type SystemData = (
		Read<'s, InputController>,
		Read<'s, CompositeUiInteractibles>,
		ReadStorage<'s, CompositeUiElement>,
    );

    fn run(
        &mut self,
		(input, interactibles, ui_elements)
		: Self::SystemData,
    ) {
       if input.trigger_or_default("mouse-left") == TriggerState::Pressed {
			let x = input.axis_or_default("mouse-x");
            let y = input.axis_or_default("mouse-y");
			//let point = [x, y].into();
			
			for (ui_element) in (ui_elements).join() {	
				if let Some(btn_name) = &ui_element.interactive {
					debug!("{0}", btn_name);
				}
			}
        }
	}	
}

/*
impl<'s> ButtonActionSystem { 
	fn click_action(
		actions: &mut Write<'s, Actions>, 
		camera_entity: Entity,
		point: Vec2,
		camera_cache: &Read<'s, CompositeCameraCache>,
		button_action: &ButtonAction, 
		interactibles: &Read<CompositeUiInteractibles, DefaultProvider>,
		btn_name: &str)
	{
		if let Some(pos) = camera_cache.screen_to_world_space(camera_entity, point) {
			if interactibles.does_rect_contains_point(btn_name, pos) {
				actions.set_action(button_action.click_action.clone());
			}
		}
	}
}*/