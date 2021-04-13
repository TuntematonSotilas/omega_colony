use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MenuTextProps {
    #[serde(default)]
    pub title: String,
}

implement_props_data!(MenuTextProps);

widget_component! {
    pub menu_comp(key, props, state) {
        let text_prop = props.read_cloned_or_default::<MenuTextProps>();
        let title = TextPaperProps {
            text: text_prop.title,
            variant: String::new(),
            use_main_color: true,
            ..Default::default()
        };
        widget! {
            (#{key} content_box: {props.clone()} [
                (#{"title"} text_paper: {title})
            ])
        }
	}
}

widget_component! {
    pub menu(key) {
        widget! {
            (#{key} menu_comp : { MenuTextProps { 
                title: "Menu".to_owned()
            }})
        }
    }
}