use oxygengine::user_interface::raui::{
    core::{
        implement_props_data, 
        prelude::*,
    },
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TabProps {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
}
implement_props_data!(TabProps);

fn use_tab(context: &mut WidgetContext) {
    context.life_cycle.change(|context| {
        for msg in context.messenger.messages {
            if let Some(msg) = msg.as_any().downcast_ref::<ButtonNotifyMessage>() {
				if msg.trigger_start() {
                    debug!("tab clic");
                }
            }
        }
    });
}

#[pre_hooks(use_tab)]
pub fn tab(mut context: WidgetContext) -> WidgetNode {
    let WidgetContext {
        id,
        key,
        props,
        ..
    } = context;

    let btn_props = props.to_owned()
        .with(PaperProps { 
			variant: "tab".to_owned(),
			frame: None, 
			..Default::default() })
        .with(ButtonNotifyProps(id.to_owned().into()));

    let tab_props = props.read_cloned_or_default::<TabProps>();
    
	let text = TextPaperProps {
		text: tab_props.label,
		width: TextBoxSizeValue::Fill,
		height: TextBoxSizeValue::Fill,
		transform: Transform {
			align: Vec2 { x: 0., y: 0.3 },
			..Default::default()
		},
		use_main_color: true,
		..Default::default()
	};

    widget! {
		(#{key} button_paper: {btn_props} {
			content = (#{"label"} text_paper: {text})
		})
    }
}
