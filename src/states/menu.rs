use oxygengine::prelude::*;
use crate::{
    states::planet::PlanetState,
    ui::components::menu_btn::MenuBtnSignal
};

#[derive(Default)]
pub struct MenuState;

impl State for MenuState {
    fn on_enter(&mut self, world: &mut World) {
		// Instantiate from prefab.
        world
            .write_resource::<PrefabManager>()
            .instantiate_world("menu", world)
            .unwrap();
    }

    fn on_process(&mut self, world: &mut World) -> StateChange {
	    
        let mut ui = world.write_resource::<UserInterfaceRes>();
        if let Some(app) = ui.application_mut("") {
            for (_caller, msg) in app.consume_signals() {
                if let Some(msg) = msg.as_any().downcast_ref::<MenuBtnSignal>() {
                    if msg == &MenuBtnSignal::NewGame {
                        return StateChange::Swap(Box::new(PlanetState))
                    }
                }
            }
        }
        
        StateChange::None
    }
}