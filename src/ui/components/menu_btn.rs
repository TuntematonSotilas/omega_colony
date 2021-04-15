use oxygengine::user_interface::raui::core::{
    implement_message_data, 
    implement_props_data, 
    prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuBtnSignal {
    NewGame,
    Continue,
}
implement_message_data!(MenuBtnSignal);

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MenuBtnProps {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
}
implement_props_data!(MenuBtnProps);

widget_hook! {
    pub use_menu_btn(life_cycle) {
        life_cycle.change(|context| {
            for msg in context.messenger.messages {
                if let Some(msg) = msg.as_any().downcast_ref::<ButtonNotifyMessage>() {
                    if msg.trigger_start() {
                        context.signals.write(MenuBtnSignal::NewGame);
                    }
                }
            }
        });
    }
}

widget_component! {
    pub menu_btn(id, key, props, state) [use_button_notified_state, use_menu_btn] {
        let btn_props = props.clone()
            .with(NavItemActive)
            .with(ButtonNotifyProps(id.to_owned().into()));
        let menu_btn_props = props.read_cloned_or_default::<MenuBtnProps>();
        let ButtonProps {
            selected,
            trigger,
            ..
        } = state.read_cloned_or_default();

        let background_props = Props::new(ImageBoxProps {
            //content_keep_aspect_ratio: Some(ImageBoxAspectRatio { horizontal_alignment: 0.5 , vertical_alignment: 0.5}),
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
        let btn_size = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Exact(200.), 
            height: SizeBoxSizeValue::Exact(50.),
            ..Default::default()
        });

        let margin = Props::new(ContentBoxItemLayout {
            /*margin: Rect {
                left: 320.,
                right: 320.,
                ..Default::default()    
            },*/
            anchors: Rect {
                left: 0.5,
                //right: 0.5,
                ..Default::default()
            },
            //align: Vec2 { x: 0.5, y:0. },
            ..Default::default()
        });

        widget! {
           /*
           (#{key} button: {btn_props} {
                content = (#{"content"} content_box [
                    (#{"background"} image_box: {background_props})
                    (#{"content"} content_box: {cont_box} [
                        (#{"label"} text_box: {text_props.clone()})
                    ])
                ])
            })
           */
            (#{key} content_box [
                (#{"margin"} content_box: {margin} [
                    (#{"size"} size_box: {btn_size} {
                        content = (#{"button"} button: {btn_props} {
                            content = (#{"content"} content_box [
                                (#{"background"} image_box: {background_props})
                                (#{"label"} text_box: {text_props.clone()})
                            ])
                        })
                    })
                ])
            ])
        }
    }
}
