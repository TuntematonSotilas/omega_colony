use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(PropsData, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SplashState {
	pub img_size: Scalar,
	pub text_size: Scalar,
	pub title_y: Scalar,
	pub press_y: Scalar,
	pub alpha: Scalar,
}

const FRAMES: Scalar = 50.;

fn use_splash(context: &mut WidgetContext) {
	context.life_cycle.mount(|context| {
		drop(context.state.write(SplashState {
			img_size: 0.,
			title_y: 0.5,
			press_y: 0.5,
			text_size: 0.,
			alpha: 0.,
		}));
	});

	context.life_cycle.change(|context| {
		let mut state = context.state.read_cloned_or_default::<SplashState>();
		if state.img_size < 0.4 {
			state.img_size += 0.4 / FRAMES;
		}
		if state.title_y > 0.1 {
			state.title_y -= 0.5 / FRAMES;
		}
		if state.press_y < 0.8 {
			state.press_y += 0.5 / FRAMES;
		}
		if state.text_size < 1. {
			state.text_size += 1. / FRAMES;
		}
		if state.alpha < 1. {
			state.alpha += 1. / FRAMES;
		}
		drop(context.state.write(state));
	});
}

#[pre_hooks(use_splash)]
pub fn splash(mut context: WidgetContext) -> WidgetNode {
	let WidgetContext {
		key,
        state,
        ..
    } = context;

	let splash_state = state.read_cloned_or_default::<SplashState>();
	
	let title = Props::new(TextBoxProps {
		height: TextBoxSizeValue::Exact(1.),
		text: "Omega Colony".to_owned(),
		horizontal_align: TextBoxHorizontalAlign::Center,
		font: TextBoxFont {
			name: "fonts/deadspace.json".to_owned(),
			size: 50.,
		},
		color: color_from_rgba(0, 153, 255, 1.),
		transform: Transform {
			pivot: Vec2 { x: 0.5, y: 0.5 },
			scale: Vec2 { x: splash_state.text_size, y: 1. },
			..Default::default()
		},
		..Default::default()
	})
	.with(ContentBoxItemLayout {
		align: Vec2 { x: 0.5, y: splash_state.title_y },
		..Default::default()
	});
	let press_label = Props::new(TextBoxProps {
		height: TextBoxSizeValue::Exact(1.),
		text: "Press enter".to_owned(),
		horizontal_align: TextBoxHorizontalAlign::Center,
		font: TextBoxFont {
			name: "fonts/orbitron.json".to_owned(),
			size: 18.,
		},
		transform: Transform {
			pivot: Vec2 { x: 0.5, y: 0.5 },
			scale: Vec2 { x: splash_state.text_size, y: 1.},
			..Default::default()
		},
		..Default::default()
	})
	.with(ContentBoxItemLayout {
		align: Vec2 { x: 0.5, y: splash_state.press_y },
		..Default::default()
	});
	let planet = Props::new(ImageBoxProps {
		content_keep_aspect_ratio: Some(ImageBoxAspectRatio {
			horizontal_alignment: 0.5,
			vertical_alignment: 0.5,
			outside: false,
		}),
		material: ImageBoxMaterial::Image(ImageBoxImage {
			id: "ui/planet.png".to_owned(),
			..Default::default()
		}),
		transform: Transform {
			pivot: Vec2 { x: 0.5, y: 0.5},
			scale: Vec2 { x: splash_state.img_size, y: splash_state.img_size},
			..Default::default()
		},
		..Default::default()
	})
	.with(ContentBoxItemLayout {
		align: Vec2 { x: 0.5, y: 0.5 },
		..Default::default()
	});
	let bkg = PaperProps { 
		frame: None, 
		..Default::default() 
	};

	widget! {
		(#{key} paper: {bkg} | {WidgetAlpha(splash_state.alpha)} [
			(#{"title"} text_box: {title})
			(#{"press_label"} text_box: {press_label})
			(#{"planet"} image_box: {planet})
		])
	}
}