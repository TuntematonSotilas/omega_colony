#[macro_use]
extern crate oxygengine;

mod states;
mod components;
mod systems;
mod resources;

use crate::{
    states::loading::LoadingState,
	components::{
        flash::Flash,
		grow::Grow,
        text_ani::TextAni,
		interactive_sprite::InteractiveSprite,
		selector::Selector,
        panel::Panel,
	},
	systems::{
		flash::FlashSystem,
		grow::GrowSystem,
        text_ani::TextAniSystem,
		camera_control::CameraControlSystem,
		sprite_click::SpriteClickSystem,
		selector::SelectorSystem,
        day::DaySystem,
	},
	resources::{
		day::Day,
        camera::Camera,
		selector::SelectorPos,
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
            }),
        )
        // install core module prefabs management.
        .with_bundle(oxygengine::core::prefab::bundle_installer, |prefabs| {
            // install composite renderer prefabs.
            oxygengine::composite_renderer::prefabs_installer(prefabs);
            // install 2d physics prefabs.
            oxygengine::physics_2d::prefabs_installer(prefabs);
            // install prefabs for integration between 2D physics and composite rendering.
            oxygengine::integration_physics_2d_composite_renderer::prefabs_installer(prefabs);
            // register game prefabs component factories.
			prefabs.register_component_factory::<Panel>("Panel");
            prefabs.register_component_factory::<Flash>("Flash");
			prefabs.register_component_factory::<Grow>("Grow");
			prefabs.register_component_factory::<TextAni>("TextAni");
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
        })
        // install composite renderer.
        .with_bundle(
            oxygengine::composite_renderer::bundle_installer,
            WebCompositeRenderer::with_state(
                get_canvas_by_id("screen"), // canvas target.
                RenderState::new(Some(Color::rgb(5,5,5)))
				.image_smoothing(false)
				//.image_source_inner_margin(0.5) 
            ),
        )
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
        .with_resource(Day::default())
        .with_resource(Camera::default())
		.with_resource(Selector::default())
        .with_resource(SelectorPos::default())
		.with_system(FlashSystem, "flash", &[])
		.with_system(GrowSystem, "grow", &[])
        .with_system(TextAniSystem, "text_ani", &[])
		.with_system(CameraControlSystem, "camera_control", &[])
		.with_system(SpriteClickSystem, "sprite_clic", &[])
		.with_system(SelectorSystem, "selector", &[])
        .with_system(DaySystem, "day", &[])
        .build(LoadingState::default(), WebAppTimer::default());

    // Application run phase - spawn runner that ticks our app.
    AppRunner::new(app).run(WebAppRunner)?;

    Ok(())
}
