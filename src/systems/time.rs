use oxygengine::prelude::*;
use serde_json;
use web_sys::window;

use crate::{
	components::panel::Panel,
	resources::time::{Time, TIME_STORAGE},
};

pub struct TimeSystem;

impl<'s> System<'s> for TimeSystem {
    type SystemData = (
        ReadExpect<'s, AppLifeCycle>,
		WriteStorage<'s, CompositeUiElement>,
        ReadStorage<'s, Panel>,
		Write<'s, Time>,
    );

    fn run(
        &mut self, 
		(lifecycle, mut ui_elements, panels, mut time)
		: Self::SystemData,
    ) {
        let delta_time = lifecycle.delta_time_seconds();
        time.phase += delta_time;
        if time.phase > 10. {
            
			if time.hour == 24 {
				time.hour = 0;
				time.day += 1;
			}
			time.hour += 1;
			
            for (ui_element, _panel) in (&mut ui_elements, &panels).join() {	
                for child in &mut ui_element.children {
                   
                    if let Some(id) = &child.id {
                        if id == "time" {
                            if let UiElementType::Text(text) = &mut child.element_type {
                                let day_fmt = format!("{:04} H {:02}", time.day, time.hour);
                                text.text = day_fmt.clone().into();
                            }
                        }
                    }
                }
            }

			self.save(time.clone());

            time.phase = 0.;
        }
		
	}	
}

impl TimeSystem {
	fn save(&self, time: Time) {
		if let Some(storage) = self.get_storage() {
			if let Ok(json) = serde_json::to_string(&time) {
				let _res = storage.set_item(TIME_STORAGE, &json);
			}
		}
	}
	fn get_storage(&self) -> Option<web_sys::Storage> {
		if let Some(window) = window() {
			if let Ok(storage_opt) = window.local_storage() {
				return storage_opt;
			}
		}
		None
	}
}
