#[macro_use]
extern crate oxygengine;

use oxygengine::prelude::*;
use wasm_bindgen::prelude::*;
use std::marker::PhantomData;

use crate::{
    states::loading::LoadingState,
	components::{
		interactive_sprite::InteractiveSprite,
		selector::Selector,
	},
	systems::{
		camera_control::CameraControlSystem,
		sprite_click::SpriteClickSystem,
		selector::SelectorSystem,
        time::TimeSystem
	},
	resources::{
		time::Time,
        camera::Camera,
		selected::Selected,
        referential::Referential,
	},
};

mod states;
mod components;
mod systems;
mod resources;
mod ui;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // initialize logger to see logs in web browser (debug only).
    #[cfg(debug_assertions)]
    logger_setup(WebLogger);

    // Application build phase - install all systems and resources and setup them.
    let app = App::build()
        // install core module assets managment.
        .with_bundle(
            oxygengine::core::assets::bundle_installer,
            (WebFetchEngine::default(), |assets| {
                // register assets loading error reporter that shows errors in console.
                #[cfg(debug_assertions)]
                assets.register_error_reporter(LoggerAssetsDatabaseErrorReporter);
                // register assets protocols from composite renderer module.
                oxygengine::composite_renderer::protocols_installer(assets);
            }),
        )
        // install core module prefabs management.
        .with_bundle(oxygengine::core::prefab::bundle_installer, |prefabs| {
            // install composite renderer prefabs.
            oxygengine::composite_renderer::prefabs_installer(prefabs);
            // install UI prefabs.
            oxygengine::user_interface::prefabs_installer(prefabs);
            // install prefabs for integration between UI and composite rendering.
            oxygengine::integration_user_interface_composite_renderer::prefabs_installer(prefabs);
            // register game prefabs component factories.
			prefabs.register_component_factory::<InteractiveSprite>("InteractiveSprite");
			prefabs.register_component_factory::<Selector>("Selector");
            prefabs.register_component_factory::<CompositeVisibility>("CompositeVisibility");
        })
        // install input managment.
        .with_bundle(oxygengine::input::bundle_installer, |input| {
            // register input devices.
            input.register(WebKeyboardInputDevice::new(get_event_target_document()));
            input.register(WebMouseInputDevice::new(get_event_target_by_id("screen")));
            // map input axes and triggers to devices.
            input.map_trigger("enter", "keyboard", "Enter");
			input.map_axis("mouse-x", "mouse", "x");
            input.map_axis("mouse-y", "mouse", "y");
			input.map_trigger("mouse-left", "mouse", "left");
            input.map_trigger("mouse-right", "mouse", "right");
        })
        // install composite renderer.
        .with_bundle(
            oxygengine::composite_renderer::bundle_installer,
            WebCompositeRenderer::with_state(
                get_canvas_by_id("screen"), // canvas target.
                RenderState::new(Some(Color::rgb(39,32,49)))
				.image_smoothing(false)
				//.image_source_inner_margin(0.5) 
            ),
        )
        // install UI support.
        .with_bundle(
            oxygengine::user_interface::bundle_installer,
            UserInterfaceRes::new(ui::setup)
                .with_pointer_axis("mouse-x", "mouse-y")
                .with_pointer_trigger("mouse-left", "mouse-right"),
        )
        // install integration between UI and composite rendering.
        .with_bundle(
            oxygengine::integration_user_interface_composite_renderer::bundle_installer,
            PhantomData::<WebCompositeRenderer>::default(),
        )
        .with_resource(Time::default())
        .with_resource(Camera::default())
		.with_resource(Selected::default())
        .with_resource(Referential::default())
		.with_system(TimeSystem, "time", &[])
        .with_system(CameraControlSystem, "camera_control", &[])
		.with_system(SpriteClickSystem, "sprite_click", &[])
		.with_system(SelectorSystem, "selector", &[])
        .build(LoadingState::default(), WebAppTimer::default());

    // Application run phase - spawn runner that ticks our app.
    AppRunner::new(app).run(WebAppRunner)?;

    Ok(())
}
