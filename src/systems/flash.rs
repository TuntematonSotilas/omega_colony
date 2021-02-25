use oxygengine::prelude::*;
use crate::components::flash::Flash;

pub struct FlashSystem;

impl<'s> System<'s> for FlashSystem {
    type SystemData = (
        ReadExpect<'s, AppLifeCycle>,
        WriteStorage<'s, Flash>,
		WriteStorage<'s, CompositeUiElement>,
    );

    fn run(
        &mut self,
        (lifecycle,  mut flashes, mut ui_elements): Self::SystemData,
    ) {
        let delta_time = lifecycle.delta_time_seconds();
		for (mut flash, ui_element) in (&mut flashes, &mut ui_elements).join() {
            flash.phase += delta_time;
			if flash.phase > flash.show_time {
                ui_element.hidden = !ui_element.hidden;
                flash.phase = 0.;
            }
		}
    }
}