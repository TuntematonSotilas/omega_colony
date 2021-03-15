use oxygengine::prelude::*;

use crate::{
	components::panel::Panel,
	resources::day::Day,
};

pub struct DaySystem;

impl<'s> System<'s> for DaySystem {
    type SystemData = (
        ReadExpect<'s, AppLifeCycle>,
		WriteStorage<'s, CompositeUiElement>,
        ReadStorage<'s, Panel>,
		Write<'s, Day>,
    );

    fn run(
        &mut self, 
		(lifecycle, mut ui_elements, panels, mut day)
		: Self::SystemData,
    ) {
        let delta_time = lifecycle.delta_time_seconds();
        day.phase += delta_time;
        if day.phase > 60. {
            day.day += 1.;
            for (ui_element, _panel) in (&mut ui_elements, &panels).join() {	
                for child in &mut ui_element.children {
                   
                    if let Some(id) = &child.id {
                        if id == "day" {
                            if let UiElementType::Text(text) = &mut child.element_type {
                                let day_fmt = format!("{:04}", day.day);
                                text.text = day_fmt.clone().into();
                                debug!("{0}", day_fmt.clone());
                            }
                        }
                    }
                }
            }

            day.phase = 0.;
        }
		
	}	
}