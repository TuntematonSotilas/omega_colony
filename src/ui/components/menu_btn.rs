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
        let menu_btn_props = props.read_cloned_or_default::<MenuBtnProps>();
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
            transform: Transform {
                pivot: Vec2 { x: 0.5, y: 0.5 },
                scale: Vec2 { x: 0.3, y: 0.3 },
                ..Default::default()
            },
            ..Default::default()
        });
        
        let text_props = Props::new(TextBoxProps {
            height: TextBoxSizeValue::Exact(50.),
            text: menu_btn_props.label,
            alignment: TextBoxAlignment::Center,
            font: TextBoxFont {
                name: "fonts/orbitron.json".to_owned(),
                size: 18.0,
            },
            ..Default::default()
        });
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
            (#{key} button: {btn_props} {
                content = (#{"content"} content_box [
                    (#{"background"} image_box: {background_props})
                    (#{"content"} content_box: {cont_box} [
                        (#{"label"} text_box: {text_props.clone()})
                    ])
                ])
            })
        }
    }
}
