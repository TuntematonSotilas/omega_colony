use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SplashState(pub Scalar);
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
            drop(context.state.write(SplashState::default()));
        });

		life_cycle.change(|context| {
			let mut state = context.state.read_cloned_or_default::<SplashState>();
			// debug!("state {0}", state.0);
			if state.0 < 50. {
				state.0 += 2.;
				drop(context.state.write(state));
			}
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
				text: text_prop.title,
				alignment: TextBoxAlignment::Center,
				font: TextBoxFont {
					name: "fonts/deadspace.json".to_owned(),
					size: state.0,
				},
				color: color_from_rgba(0, 153, 255, 1.),
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
				..Default::default()
			});
			let press_label = TextPaperProps {
				text: text_prop.press_label.to_owned(),
				use_main_color: true,
				alignment_override: Some(TextBoxAlignment::Center),
				..Default::default()
			};
			widget! {
				(#{key} content_box: {props.clone()} [
					(#{"stars"} image_box: {stars})
					(#{key} vertical_box: {props.clone()} [
						(#{"title"} text_box: {title})
						(#{"planet"} image_box: {planet})
						(#{"press_label"} text_paper: {press_label})
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