use oxygengine::prelude::*;
use serde_json;
use web_sys::window;

use crate::resources::time::{Time, TIME_STORAGE};

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
        let delta_time = lifecycle.delta_time_seconds();
        time.phase += delta_time;
		time.timestamp += delta_time;
        if time.phase > 10. {
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