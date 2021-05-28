use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum MenuBtnSignal {
    NewGame,
    Continue,
}
implement_message_data!(MenuBtnSignal);

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MenuBtnProps {
    pub id: String,
    pub label: String,
}
implement_props_data!(MenuBtnProps);

fn use_menu_btn(context: &mut WidgetContext) {
    context.life_cycle.change(|context| {
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

#[pre_hooks(use_menu_btn)]
pub fn menu_btn(mut context: WidgetContext) -> WidgetNode {
    let WidgetContext {
        id,
        key,
        props,
        ..
    } = context;

    let btn_props = props.to_owned()
        .with(PaperProps { frame: None, ..Default::default() })
        .with(NavItemActive)
        .with(ButtonNotifyProps(id.to_owned().into()));

	let size = Props::new(ContentBoxItemLayout {
		anchors: Rect {
			left: 0.5,
			right: 0.5,
			top: 0.,
			bottom: 0.,
		},
		offset: Vec2 { x: -75., y: 0. },
		..Default::default()
	}).with(SizeBoxProps {
		height: SizeBoxSizeValue::Exact(40.), 
		width: SizeBoxSizeValue::Exact(150.),
		..Default::default()
	});

    let menu_btn_props = props.read_cloned_or_default::<MenuBtnProps>();
    let text = TextPaperProps {
        text: menu_btn_props.label,
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
		(#{key} content_box [
			(#{"size_btn"} size_box: {size} {
				content = (#{"btn"} button_paper: {btn_props} {
					content = (#{"label"} text_paper: {text.to_owned()})
				})
			})
		])
    }
}
