use oxygengine::prelude::*;

use crate::{
	resources::{
		camera::Camera,
		ui_widget::UiWidget,
	}, 
	ui::components::{
		tab::TabSignal,
		side_panel::PanelSignal,
	},
};

#[derive(Default)]
pub struct PlanetState;

impl State for PlanetState {
    fn on_enter(&mut self, world: &mut World) {
        // instantiate world objects from scene prefab.
        world
			.write_resource::<PrefabManager>()
			.instantiate_world("planet", world)
			.unwrap()[0];

		// instantiate base from prefab.
        world
            .write_resource::<PrefabManager>()
            .instantiate_world("base", world)
            .unwrap()[0];

		// instantiate selector from prefab.
        world
            .write_resource::<PrefabManager>()
            .instantiate_world("selector", world)
            .unwrap()[0];

    }

    fn on_process(&mut self, world: &mut World) -> StateChange {
		// Set camera
		if world.read_resource::<Camera>().camera.is_none() {
			let camera = entity_find_world("camera", world);
			world.write_resource::<Camera>().camera = camera;
		}

		let mut ui = world.write_resource::<UserInterfaceRes>();
		if let Some(app) = ui.application_mut("") {
            for (caller, msg) in app.consume_signals() {

				// Register Top Bar
                /*if let Some(msg) = msg.as_any().downcast_ref::<TopBarSignal>() {

				}*/
				/*let mut refe = world.write_resource::<Referential>();
				if !refe.is_init {
					refe.init();
				}*/

				// Register Panel
                if let Some(msg) = msg.as_any().downcast_ref::<PanelSignal>() {
                    if msg == &PanelSignal::Register {
						if world.read_resource::<UiWidget>().side_panel.is_none() {
							world.write_resource::<UiWidget>().side_panel = Some(caller);
						}
					}
				}
				// Transfer Tab Signal
				if let Some(_msg) = msg.as_any().downcast_ref::<TabSignal>() {
                   	if let Some(side_panel) = &world.read_resource::<UiWidget>().side_panel {
						app.send_message(side_panel, PanelSignal::ActiveTab);
					}
				}
			}
		}
		StateChange::None
    }
}
