#[macro_use]
extern crate oxygengine;

mod states;
mod components;
mod systems;

use crate::{
    states::loading::LoadingState,
	components::{
		flash::Flash,
		grow::Grow,
        text_ani::TextAni,
	},
	systems::{
		flash::FlashSystem,
		grow::GrowSystem,
        text_ani::TextAniSystem,
	},
};
use oxygengine::prelude::*;
use wasm_bindgen::prelude::*;

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
                // register assets protocols from audio module.
                //oxygengine::audio::protocols_installer(assets);
            }),
        )
        // install core module prefabs management.
        .with_bundle(oxygengine::core::prefab::bundle_installer, |prefabs| {
            // install composite renderer prefabs.
            oxygengine::composite_renderer::prefabs_installer(prefabs);
            // install audio prefabs.
            //oxygengine::audio::prefabs_installer(prefabs);
            // install 2d physics prefabs.
            oxygengine::physics_2d::prefabs_installer(prefabs);
            // install prefabs for integration between 2D physics and composite rendering.
            oxygengine::integration_physics_2d_composite_renderer::prefabs_installer(prefabs);
            // register game prefabs component factories.
			prefabs.register_component_factory::<Flash>("Flash");
			prefabs.register_component_factory::<Grow>("Grow");
			prefabs.register_component_factory::<TextAni>("TextAni");
        })
        // install input managment.
        .with_bundle(oxygengine::input::bundle_installer, |input| {
            // register input devices.
            input.register(WebKeyboardInputDevice::new(get_event_target_document()));
            input.register(WebMouseInputDevice::new(get_event_target_by_id("screen")));
            // map input axes and triggers to devices.
            input.map_trigger("enter", "keyboard", "Enter");
        })
        // install composite renderer.
        .with_bundle(
            oxygengine::composite_renderer::bundle_installer,
            WebCompositeRenderer::with_state(
                get_canvas_by_id("screen"), // canvas target.
                RenderState::new(Some(Color::rgb(5,5,5))),
            ),
        )
        // install audio support.
        //.with_bundle(oxygengine::audio::bundle_installer, WebAudio::default())
        // install 2D physics with default gravity force vector.
        .with_bundle(
            oxygengine::physics_2d::bundle_installer,
            (
                Vector::y() * 9.81,
                Physics2dWorldSimulationMode::FixedTimestepMaxIterations(3),
            ),
        )
        // install integration between 2D physics and composite rendering.
        .with_bundle(
            oxygengine::integration_physics_2d_composite_renderer::bundle_installer,
            (),
        )
        // install web storage engine resource.
        .with_resource(WebStorageEngine)
		.with_system(FlashSystem, "flash", &[])
		.with_system(GrowSystem, "grow", &[])
        .with_system(TextAniSystem, "text_ani", &[])
        .build(LoadingState::default(), WebAppTimer::default());

    // Application run phase - spawn runner that ticks our app.
    AppRunner::new(app).run(WebAppRunner)?;

    Ok(())
}
