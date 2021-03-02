use oxygengine::prelude::*;

#[derive(Default)]
pub struct IntroState;

impl State for IntroState {
    fn on_enter(&mut self, world: &mut World) {
		// Instantiate from prefab.
        world
            .write_resource::<PrefabManager>()
            .instantiate_world("intro", world)
            .unwrap();
    }

    fn on_process(&mut self, _world: &mut World) -> StateChange {
        StateChange::None
    }
}