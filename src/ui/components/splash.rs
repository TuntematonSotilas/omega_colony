use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TextProps {
    #[serde(default)]
    pub press_label: String,
    #[serde(default)]
    pub title: String,
}
implement_props_data!(TextProps);


widget_component! {
    pub splash_comp(key, props) {
        let background_props = Props::new(ImageBoxProps {
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
        let title = TextPaperProps {
            text: text_prop.title.to_owned(),
           	use_main_color: true,
            alignment_override: Some(TextBoxAlignment::Center),
            ..Default::default()
        };
        let press_label = TextPaperProps {
            text: text_prop.press_label.to_owned(),
           	use_main_color: true,
            alignment_override: Some(TextBoxAlignment::Center),
            ..Default::default()
        };
        widget! {
            (#{key} content_box: {props.clone()} [
                (#{"background"} image_box: {background_props})
                (#{key} vertical_box: {props.clone()} [
                    (#{"title"} text_paper: {title})
                    (#{"press_label"} text_paper: {press_label})
                ])
            ])
        }
    }
}

widget_component! {
    pub splash(key) {
        widget! {
            (#{key} splash_comp: { TextProps { 
                title: "Omega Colony".to_owned(),
                press_label: "Press enter".to_owned() 
            }})
        }
    }
}