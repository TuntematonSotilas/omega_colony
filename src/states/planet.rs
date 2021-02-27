use oxygengine::prelude::*;

#[derive(Default)]
pub struct PlanetState;

impl State for PlanetState {
    fn on_enter(&mut self, world: &mut World) {
        // instantiate world objects from scene prefab.
        world
			.write_resource::<PrefabManager>()
			.instantiate_world("planet", world)
			.unwrap()[0];
    }

    fn on_process(&mut self, _world: &mut World) -> StateChange {
		StateChange::None
    }
}
