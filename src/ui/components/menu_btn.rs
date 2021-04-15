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
        let ButtonProps {
            selected,
            trigger,
            context,
            ..
        } = state.read_cloned_or_default();

        //debug!("{0},{1},{2}", selected, trigger, context);
        let background_props = Props::new(ImageBoxProps {
            width: ImageBoxSizeValue::Fill,
            height: ImageBoxSizeValue::Fill,
            material: ImageBoxMaterial::Image(ImageBoxImage {
                id: "ui/menu_btn.png".to_owned(),
                tint: if trigger {
                    Color { r: 0.0, g: 0.0, b: 0.0, a: 0.8 }
                } else if selected {
                    Color { r: 0.0, g: 0.0, b: 0.0, a: 0.9 }
                } else {
                    Color::default()
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