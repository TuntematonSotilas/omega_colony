use serde::{Deserialize, Serialize};
use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TabProps {
    pub id: String,
    pub label: String,
	pub is_active: bool,
}
implement_props_data!(TabProps);

#[derive(Debug, Clone, Copy)]
pub enum TabSignal {
    Units,
    Upgrades,
}
implement_message_data!(TabSignal);

fn use_tab(ctx: &mut WidgetContext) {
    ctx.life_cycle.change(|context| {
        for msg in context.messenger.messages {
            if let Some(msg) = msg.as_any().downcast_ref::<ButtonNotifyMessage>() {
				if msg.trigger_start() {
					let props = context.props.read_cloned_or_default::<TabProps>();
					let signal = match props.id.as_str() {
                        "units" => TabSignal::Units,
                        _ => TabSignal::Upgrades,
                    };
                    context.signals.write(signal);
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

	let tab_props = props.read_cloned_or_default::<TabProps>();
    
	let variant = match tab_props.is_active {
		true => "tab_active",
		_ => "tab_inactive",
	};
    let btn_props = props.to_owned()
        .with(PaperProps { 
			variant: variant.to_owned(),
			frame: None, 
			..Default::default() })
		.with(NavItemActive)
        .with(ButtonNotifyProps(id.to_owned().into()));

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
