use oxygengine::prelude::*;

use crate::resources::camera::Camera;

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

        // instantiate panel from prefab.
        world
             .write_resource::<PrefabManager>()
             .instantiate_world("panel", world)
             .unwrap()[0];
		
    }

    fn on_process(&mut self, world: &mut World) -> StateChange {

		// Set cameras
		if world.read_resource::<Camera>().camera.is_none() {
			let camera = entity_find_world("camera", world);
			world.write_resource::<Camera>().camera = camera;
		}
		if world.read_resource::<Camera>().camera_panel.is_none() {
			let camera_panel = entity_find_world("camera_panel", world);
			world.write_resource::<Camera>().camera_panel = camera_panel;
		}

		StateChange::None
    }
}
