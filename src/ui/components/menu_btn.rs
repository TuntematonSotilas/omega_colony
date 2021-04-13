use oxygengine::user_interface::raui::core::{implement_props_data, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MenuBtnProps {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
}
implement_props_data!(MenuBtnProps);

widget_component! {
    pub menu_btn(id, key, props, state) [use_button_notified_state] {
        let btn_props = props.clone()
            .with(NavItemActive)
            .with(ButtonNotifyProps(id.to_owned().into()));
        let main_menu_button_props = props.read_cloned_or_default::<MenuBtnProps>();
        let ButtonProps { selected, ..} = state.read_cloned_or_default();

        let background_props = Props::new(ImageBoxProps {
            content_keep_aspect_ratio: Some(ImageBoxAspectRatio {
                horizontal_alignment: 0.5,
                vertical_alignment: 0.5,
            }),
            material: ImageBoxMaterial::Image(ImageBoxImage {
                id: if selected {
                    "ui/menu_btn_1.png".to_owned()
                } else {
                    "ui/menu_btn_0.png".to_owned()
                },
                ..Default::default()
            }),
            ..Default::default()
        });
        
        let text_props = Props::new(TextBoxProps {
            text: main_menu_button_props.label,
            height: TextBoxSizeValue::Exact(16.0),
            alignment: TextBoxAlignment::Center,
            font: TextBoxFont {
                name: "fonts/orbitron.json".to_owned(),
                size: 16.0,
            },
            /*color: if selected {
                Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
            } else {
                Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
            },*/
            ..Default::default()
        }).with(ContentBoxItemLayout {
            anchors: Rect {
                left: 0.0,
                right: 1.0,
                top: 1.0,
                bottom: 1.0,
            },
            /*margin: Rect {
                top: -16.0,
                ..Default::default()
            },*/
            ..Default::default()
        });
        
        widget! {
            (#{key} button: {btn_props} {
                content = (#{"content"} content_box [
                    (#{"background"} image_box: {background_props})
                    (#{"label"} text_box: {text_props})
                ])
            })
        }
    }
}
