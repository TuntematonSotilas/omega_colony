use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SplashState {
	pub title_size: Scalar,
	pub title_y: Scalar,
	pub img_size: Scalar,
	pub press_y: Scalar,
	pub press_size: Scalar,
}
implement_props_data!(SplashState);

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TextProps {
    #[serde(default)]
    pub press_label: String,
    #[serde(default)]
    pub title: String,
}

implement_props_data!(TextProps);


widget_hook! {
    pub use_splash(life_cycle) {
		life_cycle.mount(|context| {
            drop(context.state.write(SplashState {
				img_size: 0.,
				title_y: 0.4,
				title_size: 0.,
				press_y: 0.5,
				press_size: 0.,
			}));
        });

		life_cycle.change(|context| {
			let mut state = context.state.read_cloned_or_default::<SplashState>();
			if state.title_size < 50. {
				state.title_size += 1.;
			}
			if state.img_size < 300. {
				state.img_size += 6.;
			}
			if state.title_y > 0.1 {
				state.title_y -= 0.01;
			}
			if state.press_y < 0.8 {
				state.press_y += 0.01;
			}
			if state.press_size < 18. {
				state.press_size += 1.;
			}
			drop(context.state.write(state));
		});
	}
}

widget_component! {
    pub splash_comp(key, props, state) [use_splash] {
		if let Ok(state) = state.read::<SplashState>() {
			let stars = Props::new(ImageBoxProps {
				content_keep_aspect_ratio: Some(ImageBoxAspectRatio {
					horizontal_alignment: 0.5,
					vertical_alignment: 0.5,
				}),
				material: ImageBoxMaterial::Image(ImageBoxImage {
					id: "ui/stars.png".to_owned(),
					..Default::default()
				}),
				..Default::default()
			});
			let text_prop = props.read_cloned_or_default::<TextProps>();
			let title = Props::new(TextBoxProps {
				height: TextBoxSizeValue::Exact(1.),
				text: text_prop.title,
				alignment: TextBoxAlignment::Center,
				font: TextBoxFont {
					name: "fonts/deadspace.json".to_owned(),
					size: state.title_size,
				},
				color: color_from_rgba(0, 153, 255, 1.),
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
					size: state.press_size,
				},
				..Default::default()
			})
			.with(ContentBoxItemLayout {
                align: Vec2 { x: 0.5, y: state.press_y },
                ..Default::default()
            });
			let planet = Props::new(ImageBoxProps {
				width: ImageBoxSizeValue::Exact(state.img_size),
				height: ImageBoxSizeValue::Exact(state.img_size),
				material: ImageBoxMaterial::Image(ImageBoxImage {
					id: "ui/planet.png".to_owned(),
					..Default::default()
				}),
				..Default::default()
			})
			.with(ContentBoxItemLayout {
                align: Vec2 { x: 0.5, y: 0.5 },
                ..Default::default()
            });
			widget! {
				(#{key} content_box: {props.clone()} [
					(#{"stars"} image_box: {stars})
					(#{key} content_box: {props.clone()} [
						(#{"title"} text_box: {title})
						(#{"press_label"} text_box: {press_label})
						(#{"planet"} image_box: {planet})
					])
				])
			}
		} else {
			widget!{()}
		}
	}
}

widget_component! {
    pub splash(key) [use_splash]{
        widget! {
            (#{key} splash_comp: { TextProps { 
                title: "Omega Colony".to_owned(),
                press_label: "Press enter".to_owned() 
            }})
        }
    }
}