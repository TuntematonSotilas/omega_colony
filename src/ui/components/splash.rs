use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

const FRAMES: Scalar = 50.;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SplashState {
	pub img_size: Scalar,
	pub text_size: Scalar,
	pub title_y: Scalar,
	pub press_y: Scalar,
	pub alpha: Scalar,
}
implement_props_data!(SplashState);

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SplashTextProps {
    #[serde(default)]
    pub press_label: String,
    #[serde(default)]
    pub title: String,
}

implement_props_data!(SplashTextProps);

widget_hook! {
    pub use_splash(life_cycle) {
		life_cycle.mount(|context| {
            drop(context.state.write(SplashState {
				img_size: 0.,
				title_y: 0.5,
				press_y: 0.5,
				text_size: 0.,
				alpha: 0.,
			}));
        });

		life_cycle.change(|context| {
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
}

widget_component! {
    pub splash_comp(key, props, state) [use_splash] {
		if let Ok(state) = state.read::<SplashState>() {
			let text_prop = props.read_cloned_or_default::<SplashTextProps>();
			let title = Props::new(TextBoxProps {
				height: TextBoxSizeValue::Exact(1.),
				text: text_prop.title,
				alignment: TextBoxAlignment::Center,
				font: TextBoxFont {
					name: "fonts/deadspace.json".to_owned(),
					size: 50.,
				},
				color: color_from_rgba(0, 153, 255, 1.),
				transform: Transform {
					pivot: Vec2 { x: 0.5, y: 0.5 },
					scale: Vec2 { x: state.text_size, y: 1. },
					..Default::default()
				},
				..Default::default()
			})
			.with(ContentBoxItemLayout {
                align: Vec2 { x: 0.5, y: state.title_y },
                ..Default::default()
            });
			let press_label = Props::new(TextBoxProps {
				height: TextBoxSizeValue::Exact(1.),
				text: text_prop.press_label,
				alignment: TextBoxAlignment::Center,
				font: TextBoxFont {
					name: "fonts/orbitron.json".to_owned(),
					size: 18.,
				},
				transform: Transform {
					pivot: Vec2 { x: 0.5, y: 0.5 },
					scale: Vec2 { x: state.text_size, y: 1.},
					..Default::default()
				},
				..Default::default()
			})
			.with(ContentBoxItemLayout {
                align: Vec2 { x: 0.5, y: state.press_y },
                ..Default::default()
            });
			let planet = Props::new(ImageBoxProps {
				content_keep_aspect_ratio: Some(ImageBoxAspectRatio {
					horizontal_alignment: 0.5,
					vertical_alignment: 0.5,
				}),
				material: ImageBoxMaterial::Image(ImageBoxImage {
					id: "ui/planet.png".to_owned(),
					..Default::default()
				}),
				transform: Transform {
					pivot: Vec2 { x: 0.5, y: 0.5},
					scale: Vec2 { x: state.img_size, y: state.img_size},
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
				(#{"paper"} paper: {bkg} [
					(#{"title"} text_box: {title} | {WidgetAlpha(state.alpha)})
					(#{"press_label"} text_box: {press_label} | {WidgetAlpha(state.alpha)})
					(#{"planet"} image_box: {planet})
				])
			}
		} else {
			widget!{()}
		}
	}
}

widget_component! {
    pub splash(key) {
        widget! {
            (#{key} splash_comp: { SplashTextProps { 
                title: "Omega Colony".to_owned(),
                press_label: "Press enter".to_owned() 
            }})
        }
    }
}