use oxygengine::user_interface::raui::{
    core::{implement_props_data, prelude::*},
    material::prelude::*,
};
use serde::{Deserialize, Serialize};
use crate::ui::components::stars;
use crate::ui::components::menu_btn;

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
        let cont_box = Props::new(ContentBoxItemLayout {
            margin: Rect {
                left: 0.,
                right: 0.,
                top: 50.0,
                bottom: 50.0,
            },
            ..Default::default()
        });
        widget! {
            (#{key} content_box: {props.clone()} [
                (#{"stars"} stars::stars)
                (#{"content"} content_box: {cont_box} [
                   (#{"v-box"} vertical_box [
                        (#{"title"} text_paper: {title})
                        (#{"new_btn"} menu_btn::menu_btn: { menu_btn::MenuBtnProps {
                            id: "new_game".to_string(),
                            label: "New Game".to_string(),
                        }})
                    ])
                ])
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