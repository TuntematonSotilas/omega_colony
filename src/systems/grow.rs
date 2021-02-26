use oxygengine::prelude::*;
use crate::components::grow::Grow;

pub struct GrowSystem;

impl<'s> System<'s> for GrowSystem {
    type SystemData = (
        ReadExpect<'s, AppLifeCycle>,
        WriteStorage<'s, Grow>,
		WriteStorage<'s, CompositeTransform>,
    );

    fn run(
        &mut self,
        (lifecycle,  mut grows, mut transforms): Self::SystemData,
    ) {
        let delta_time = lifecycle.delta_time_seconds();
		for (mut grow, transform) in (&mut grows, &mut transforms).join() {
            if grow.phase < grow.max {
				grow.phase += grow.speed * delta_time;
				let scale = grow.phase.sin();
				transform.set_scale(Vec2::new(scale, scale));
			}
		}
    }
}