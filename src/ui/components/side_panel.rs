use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};
use serde::{Deserialize, Serialize};

use crate::{
	ui::components::panel_item::{ panel_item, PanelItemProps },
	resources::referential::RefeItem
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PanelSignal {
	Register,
    HideOrShow(RefeItem),
}
implement_message_data!(PanelSignal);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PanelState {
	pub open: bool,
	pub x_align: Scalar,
	pub refe: Option<RefeItem>,
}
implement_props_data!(PanelState);

const FRAMES: Scalar = 5.;

fn use_panel(context: &mut WidgetContext) {
	context.life_cycle.mount(|context| {
		drop(context.state.write(PanelState {
			x_align: 1.,
			open: false,
			refe: None,
		}));
        context.signals.write(PanelSignal::Register);
    });
	
	context.life_cycle.change(|context| {
		let mut state = context.state.read_cloned_or_default::<PanelState>();
		for msg in context.messenger.messages {
            if let Some(PanelSignal::HideOrShow(refe)) = msg.as_any().downcast_ref() {
				state.open = !state.open;
				if state.open {
					state.refe = Some(refe.to_owned());
				}
			}
		}
		if state.open && state.x_align > 0. {
			let x = state.x_align - 1. / FRAMES; 
			if x > 0. { state.x_align = x; } else { state.x_align = 0.; }
			
		}
		if !state.open && state.x_align < 1. {
			let x = state.x_align + 1. / FRAMES;
			if x < 1. { state.x_align = x; } else { state.x_align = 1.; }
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

	let bkg = PaperProps { 
        frame: None, 
        ..Default::default() 
    };
	let mut x_align = 1.;
	let mut alpha = 0.;
	let mut title_txt = String::new();
	let mut preview_pic = String::new();
	let mut refe = RefeItem::default();

	if let Ok(state) = state.read::<PanelState>() {
		x_align = state.x_align;
		alpha = match state.open {
			true => 1.,
			false => 0.
		};
		if let Some(refe_item) = &state.refe {
			refe = refe_item.to_owned();
			title_txt = refe_item.name.to_owned();
			preview_pic = refe_item.preview.to_owned();
		};
	}
	let c_box = ContentBoxProps {
        transform: Transform {
			align: Vec2 { x: x_align, y: 0. },
            ..Default::default()
        },
        ..Default::default()
    };
	let size_title = SizeBoxProps {
        height: SizeBoxSizeValue::Exact(60.), 
        width: SizeBoxSizeValue::Fill,
        ..Default::default()
    };
	let margin_panel = ContentBoxItemLayout {
		margin: Rect {
            left: 10.,
            right: 10.,
            top: 10.,
            bottom: 10.,
        },
		..Default::default()
	};
	let margin_title = ContentBoxItemLayout {
		margin: Rect {
            left: 80.,
            right: 80.,
			top: 10.,
            bottom: 10.,
        },
		..Default::default()
	};
	let bkg_title = PaperProps { 
        frame: None, 
        variant: "bkg_title".to_owned(),
        ..Default::default() 
    };
	let preview = ImageBoxProps {
		width: ImageBoxSizeValue::Exact(32.),
		height: ImageBoxSizeValue::Exact(32.),
		material: ImageBoxMaterial::Image(ImageBoxImage {
			id: preview_pic,
			..Default::default()
		}),
		..Default::default()
	};
	let title = TextPaperProps {
        text: title_txt,
        width: TextBoxSizeValue::Fill,
        height: TextBoxSizeValue::Fill,
        use_main_color: true,
		transform: Transform {
            align: Vec2 { x: 0., y: 0.3 },
            ..Default::default()
        },
		..Default::default()
    };
	let size_tabs = SizeBoxProps {
        height: SizeBoxSizeValue::Exact(30.), 
        width: SizeBoxSizeValue::Fill,
        ..Default::default()
    };
	let size_items = SizeBoxProps {
        height: SizeBoxSizeValue::Exact(480.), 
        width: SizeBoxSizeValue::Fill,
        ..Default::default()
    };
	let btn = Props::new(PaperProps {
		variant: "tab".to_owned(),
		frame: None, 
		..Default::default() 
	}).with(NavItemActive);
	
	let items_list = refe.childs.iter()
        .map(|(_code, child)| {
            widget! {
                (#{child.name} panel_item: { PanelItemProps { item: child.to_owned() }})
            }
        })
        .collect::<Vec<_>>();

    widget! {
        (#{key} content_box: {c_box} | {WidgetAlpha(alpha)} [
            (#{"bkg"} paper: {bkg})
			(#{"v_box"} vertical_box: {margin_panel} [
				(#{"title"} size_box: {size_title} {
					content = (#{"box_title"} paper: {bkg_title} [
						(#{"margin_title"} content_box : {margin_title} [
							(#{"h_title"} horizontal_box [
								(#{"img"} image_box: {preview})
								(#{"bkg"} text_paper: {title})
							])
						])
					])
				})
				(#{"tabs"} size_box: {size_tabs} {
					content = (#{"v_box"} nav_horizontal_box [
						(#{"units"} button_paper: {btn.to_owned()})
						(#{"upgrades"} button_paper: {btn.to_owned()})
					])
				})
				(#{"items"} size_box: {size_items} {
					content =  (#{key} flex_box |[ items_list ]|)
				})
			])
		])
    }
}