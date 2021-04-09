use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TextProps {
    #[serde(default)]
    pub label: String,
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
        let spl_props = props.read_cloned_or_default::<TextProps>();
        let label_props = TextPaperProps {
            text: spl_props.label.to_owned(),
           	use_main_color: true,
            alignment_override: Some(TextBoxAlignment::Center),
            ..Default::default()
        };
        widget! {
            (#{key} content_box: {props.clone()} [
                (#{"background"} image_box: {background_props})
                (#{"label"} text_paper: {label_props})
            ])
        }
    }
}

widget_component! {
    pub splash(key) {
        widget! {
            (#{key} splash_comp: {TextProps { label: "Splash".to_owned() }})
        }
    }
}