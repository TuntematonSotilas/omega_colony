use oxygengine::prelude::*;

use crate::{
	resources::{
		camera::Camera,
		ui_widget::UiWidget,
	}, 
	ui::components::{
		side_panel::PanelSignal,
		top_bar::TopBarSignal,
		panel_item::PanelItemSignal,
	},
};

#[derive(Default)]
pub struct PlanetState;

impl State for PlanetState {
    fn on_enter(&mut self, universe: &mut Universe) {
        // instantiate world objects from scene prefab.
        universe
			.expect_resource_mut::<PrefabManager>()
			.instantiate("planet", universe)
			.unwrap()[0];

		// instantiate base from prefab.
        universe
            .expect_resource_mut::<PrefabManager>()
            .instantiate("base", universe)
            .unwrap()[0];

		// instantiate selector from prefab.
        universe
            .expect_resource_mut::<PrefabManager>()
            .instantiate("selector", universe)
            .unwrap()[0];

    }

    fn on_process(&mut self, universe: &mut Universe) -> StateChange {
		// Set camera
		if universe.expect_resource::<Camera>().camera.is_none() {
			let camera = entity_find_world("camera", world);
			world.write_resource::<Camera>().camera = camera;
		}

		let mut ui = world.expect_resource_mut::<UserInterfaceRes>();
		if let Some(app) = ui.application_mut("") {
            for (caller, msg) in app.consume_signals() {

				// Register Top Bar
                if let Some(msg) = msg.as_any().downcast_ref::<TopBarSignal>() {
					if msg == &TopBarSignal::Register {
						world.write_resource::<UiWidget>().top_bar = Some(caller.to_owned());
					}
				}
				// Register Panel
                else if let Some(msg) = msg.as_any().downcast_ref::<PanelSignal>() {
                    if msg == &PanelSignal::Register {
						world.write_resource::<UiWidget>().side_panel = Some(caller.to_owned());
					}
				}
				// Panel Item Signal
				else if let Some(_msg) = msg.as_any().downcast_ref::<PanelItemSignal>() {
                   	debug!("PanelItemSignal");
				}
			}
		}
		StateChange::None
    }
}
