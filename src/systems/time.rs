use oxygengine::prelude::*;

use crate::{
	components::panel::Panel,
	resources::time::Time,
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
            time.phase = 0.;
        }
		
	}	
}