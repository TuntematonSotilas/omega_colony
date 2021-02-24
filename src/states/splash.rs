use oxygengine::prelude::*;

#[derive(Default)]
pub struct SplashState;

impl State for SplashState {
    fn on_enter(&mut self, world: &mut World) {
		// Instantiate from prefab.
        world
            .write_resource::<PrefabManager>()
            .instantiate_world("splash", world)
            .unwrap();
    }

    fn on_process(&mut self, world: &mut World) -> StateChange {
        let input = &world.read_resource::<InputController>();
        if input.trigger_or_default("enter") == TriggerState::Pressed {
            //return StateChange::Swap(Box::new(IntroState));
        }
        StateChange::None
    }
}