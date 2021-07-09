use oxygengine::prelude::*;

use crate::{
	states::menu::MenuState,
	resources::{
		referential::Referential,
		stock::Stock,
	},
};

#[derive(Default)]
pub struct SplashState;

impl State for SplashState {
    fn on_enter(&mut self, universe: &mut Universe) {
		// Instantiate from prefab.
        universe
            .expect_resource_mut::<PrefabManager>()
            .instantiate("splash", universe)
            .unwrap();
    }

    fn on_process(&mut self, universe: &mut Universe) -> StateChange {

		let mut stock = universe.expect_resource_mut::<Stock>();
		if !stock.is_init {
            stock.init();
        }
        let mut refe = universe.expect_resource_mut::<Referential>();
        if !refe.is_init {
            refe.init(stock.refe.to_owned());
        }
        let input = &universe.expect_resource::<InputController>();
        if input.trigger_or_default("enter") == TriggerState::Pressed {
            return StateChange::Swap(Box::new(MenuState));
        }
        StateChange::None
    }
}