use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelSignal {
	Register,
    HideOrShow,
}
implement_message_data!(PanelSignal);

fn use_panel(context: &mut WidgetContext) {
	context.life_cycle.mount(|context| {
        context.signals.write(PanelSignal::Register);
    });
	
	context.life_cycle.change(|context| {
		for msg in context.messenger.messages {
            if let Some(PanelSignal::HideOrShow) = msg.as_any().downcast_ref() {
				debug!("PanelSignal::HideOrShow");
			}
		}
	});
}

#[pre_hooks(use_panel)]
pub fn side_panel(mut context: WidgetContext) -> WidgetNode {
	let bkg = Props::new(PaperProps { 
        frame: None, 
        ..Default::default() 
    });

	let c_box = ContentBoxProps {
        transform: Transform {
			align: Vec2 { x: 0.9, y: 0. },
            ..Default::default()
        },
        ..Default::default()
    };

    widget! {
        (#{context.key} content_box: {c_box} [
            (#{"bkg"} paper: {bkg})
		])
    }
}