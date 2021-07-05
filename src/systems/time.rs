/*use oxygengine::prelude::*;

use crate::{
    resources::time::{Time, TIME_STORAGE},
    storage::sto_utils,
};

pub struct TimeSystem;

impl<'s> System<'s> for TimeSystem {
    type SystemData = (
        ReadExpect<'s, AppLifeCycle>,
		Write<'s, Time>,
    );

    fn run(
        &mut self, 
		(lifecycle, mut time)
		: Self::SystemData,
    ) {
		if time.launched {
			let delta_time = lifecycle.delta_time_seconds();
			time.phase += delta_time;
			if time.phase > 10. {
				time.sec += 10; 
				sto_utils::save::<u32>(TIME_STORAGE, time.sec);
				time.phase = 0.;
			}
		}
	}	
}*/