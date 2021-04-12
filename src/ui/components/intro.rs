use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct IntroTextProps {
    #[serde(default)]
    pub title: String,
}

implement_props_data!(IntroTextProps);

widget_component! {
    pub intro_comp(key, props, state) {
        let landscape = Props::new(ImageBoxProps {
            width: ImageBoxSizeValue::Fill,
            height: ImageBoxSizeValue::Fill,
            material: ImageBoxMaterial::Image(ImageBoxImage {
                id: "ui/landscape.png".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        });
        widget! {
            (#{key} content_box: {props.clone()} [
                (#{"landscape"} image_box: {landscape})
            ])
        }
	}
}

widget_component! {
    pub intro(key) {
        widget! {
            (#{key} intro_comp : { IntroTextProps { 
                title: "Intro".to_owned()
            }})
        }
    }
}