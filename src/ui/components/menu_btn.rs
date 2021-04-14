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
            width: ImageBoxSizeValue::Fill,
            height: ImageBoxSizeValue::Fill,
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
            height: TextBoxSizeValue::Exact(1.),
            text: menu_btn_props.label,
            alignment: TextBoxAlignment::Center,
            font: TextBoxFont {
                name: "fonts/orbitron.json".to_owned(),
                size: 18.0,
            },
            ..Default::default()
        })
        .with(ContentBoxItemLayout {
            margin: Rect {
                top: 15.0,
                ..Default::default()
            },
            ..Default::default()
        });
        let btn_size = SizeBoxProps {
            width: SizeBoxSizeValue::Exact(200.), 
            height: SizeBoxSizeValue::Exact(50.),
            margin: Rect {
                left: 370.0,
                ..Default::default()
            },
            ..Default::default()
        };
        widget! {
            (#{key} size_box: {btn_size} {
                content = (#{key} button: {btn_props} {
                    content = (#{"content"} content_box [
                        (#{"background"} image_box: {background_props})
                        (#{"label"} text_box: {text_props.clone()})
                    ])
                })
            })
        }
    }
}
