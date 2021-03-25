use oxygengine::prelude::*;

use crate::{
	states::intro::IntroState,
	resources::referential::{Referential, ItemType},
};

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

		/*let items: HashMap::new()
		let referential = Referential {
			itype: 
		};

		world.write_resource::<Referential>();*/

        let input = &world.read_resource::<InputController>();
        if input.trigger_or_default("enter") == TriggerState::Pressed {
            return StateChange::Swap(Box::new(IntroState));
        }
        StateChange::None
    }
}