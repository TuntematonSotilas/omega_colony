use oxygengine::user_interface::raui::{
    core::{
        implement_message_data, 
        implement_props_data, 
        prelude::*,
    },
    material::prelude::*,
};
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
                        let props = context.props.read_cloned_or_default::<MenuBtnProps>();
                        let signal = match props.id.as_str() {
                            "new_game" => MenuBtnSignal::NewGame,
                            _ => MenuBtnSignal::Continue,
                        };
                        context.signals.write(signal);
                    }
                }
            }
        });
    }
}

widget_component! {
    pub menu_btn(id, key, props, state) [use_button_notified_state, use_menu_btn] {
        let btn_props = props.clone()
            .with(PaperProps { frame: None, ..Default::default() })
            .with(NavItemActive)
            .with(ButtonNotifyProps(id.to_owned().into()));
        /*
        let size = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Exact(200.), 
            height: SizeBoxSizeValue::Exact(50.),
            ..Default::default()
        });
        let anchor = Props::new(ContentBoxItemLayout {
            anchors: Rect {
                left: 0.42,
                ..Default::default()
            },
            ..Default::default()
        });*/
        let menu_btn_props = props.read_cloned_or_default::<MenuBtnProps>();
        let text = Props::new(TextPaperProps {
            text: menu_btn_props.label,
            width: TextBoxSizeValue::Fill,
            height: TextBoxSizeValue::Fill,
            transform: Transform {
                align: Vec2 { x: 0., y: 0.3},
                ..Default::default()
            },
            use_main_color: true,
            ..Default::default()
        });

        widget! {
            (#{key} button_paper: {btn_props} {
                content = (#{"label"} text_paper: {text.clone()})
            })
        }
    }
}
