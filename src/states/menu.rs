use oxygengine::{
    prelude::*, 
    user_interface::raui::core::prelude::NavSignal, 
    widget::WidgetIdOrRef
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
            for (caller, msg) in app.consume_signals() {

                debug!("caller={0}", caller.as_ref());
                
                if let Some(msg) = msg.as_any().downcast_ref::<NavSignal>() {
                    debug!("NavSignal");
                    match msg {
                        NavSignal::Select(id_ref) => {
                            if let Some(id) = id_ref.read() {
                                debug!("id={0}", id.as_ref());
                            }
                            
                        },
                        _ => (),
                    }
                }
            }
        }
        
        StateChange::None
    }
}