use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanelSignal {
	Register,
    HideOrShow,
}
implement_message_data!(PanelSignal);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PanelState {
	pub open: bool,
	pub x_align: Scalar,
}
implement_props_data!(PanelState);

const FRAMES: Scalar = 5.;

fn use_panel(context: &mut WidgetContext) {
	context.life_cycle.mount(|context| {
		drop(context.state.write(PanelState {
			x_align: 1.,
			open: false,
		}));
        context.signals.write(PanelSignal::Register);
    });
	
	context.life_cycle.change(|context| {
		let mut state = context.state.read_cloned_or_default::<PanelState>();
		for msg in context.messenger.messages {
            if let Some(PanelSignal::HideOrShow) = msg.as_any().downcast_ref() {
				state.open = !state.open;
			}
		}
		if state.open && state.x_align > 0. {
			let x = state.x_align - 1. / FRAMES; 
			if x > 0. { state.x_align = x; } else { state.x_align = 0.; }
			
		}
		if !state.open && state.x_align < 1. {
			let x = state.x_align + 1. / FRAMES;
			if x < 1. { state.x_align = x; } else { state.x_align = 0.; }
		}
		drop(context.state.write(state));
	});
}

#[pre_hooks(use_panel)]
pub fn side_panel(mut context: WidgetContext) -> WidgetNode {
	let WidgetContext {
		key,
        state,
        ..
    } = context;

	let bkg = Props::new(PaperProps { 
        frame: None, 
        ..Default::default() 
    });
	let mut x_align = 1.;
	if let Ok(state) = state.read::<PanelState>() {
		x_align = state.x_align;
	}
	let c_box = ContentBoxProps {
        transform: Transform {
			align: Vec2 { x: x_align, y: 0. },
            ..Default::default()
        },
        ..Default::default()
    };

    widget! {
        (#{key} content_box: {c_box} [
            (#{"bkg"} paper: {bkg})
		])
    }
}