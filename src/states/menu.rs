use oxygengine::prelude::*;
use crate::{
    states::planet::PlanetState,
    ui::components::menu_btn::MenuBtnSignal,
	storage::sto_utils,
	resources::time::{Time, TIME_STORAGE},
};

#[derive(Default)]
pub struct MenuState;

impl State for MenuState {
    fn on_enter(&mut self, universe: &mut Universe) {
		// Instantiate from prefab.
        universe
            .expect_resource_mut::<PrefabManager>()
            .instantiate("menu", universe)
            .unwrap();
    }

    fn on_process(&mut self, universe: &mut Universe) -> StateChange {
	    
        let mut ui = universe.expect_resource_mut::<UserInterface>();
       	if let Some(data) = ui.get_mut("") {
			for (_caller, msg) in data.signals_received() {
				if let Some(msg) = msg.as_any().downcast_ref::<MenuBtnSignal>() {
					universe.expect_resource_mut::<Time>().launched = true;
					match &msg {
                        MenuBtnSignal::Continue => {
                            let sec_opt = sto_utils::get::<u32>(TIME_STORAGE);
							if let Some(sec) = sec_opt {
								universe.expect_resource_mut::<Time>().sec = sec;
							}
                        },
						_ => (),         
                    }
					return StateChange::Swap(Box::new(PlanetState));
                }
            }
        }
        
        StateChange::None
    }
}