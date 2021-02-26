use oxygengine::prelude::*;
use crate::components::grow::Grow;

pub struct GrowSystem;

impl<'s> System<'s> for GrowSystem {
    type SystemData = (
        ReadExpect<'s, AppLifeCycle>,
        WriteStorage<'s, Grow>,
		WriteStorage<'s, CompositeUiElement>,
    );

    fn run(
        &mut self,
        (lifecycle,  mut grows, mut ui_elements): Self::SystemData,
    ) {
        let delta_time = lifecycle.delta_time_seconds();
		for (mut grow, ui_element) in (&mut grows, &mut ui_elements).join() {
            grow.phase += delta_time;
            
		}
    }
}