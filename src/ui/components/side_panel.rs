use oxygengine::user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
};
use serde::{Deserialize, Serialize};

use crate::{
	ui::components::{
		panel_item::{panel_item, PanelItemProps},
		tab::{tab, TabProps},
	},
	resources::referential::RefeItem
};

#[derive(MessageData, Debug, Clone, PartialEq, Eq)]
pub enum PanelSignal {
	Register,
    HideOrShow(RefeItem),
	ActiveTab,
}

#[derive(PropsData, Debug, Default, Clone, Serialize, Deserialize)]
pub struct PanelState {
	pub open: bool,
	pub refe: Option<RefeItem>,
	pub is_tab_units: bool,
}

fn use_panel(context: &mut WidgetContext) {
	context.life_cycle.mount(|context| {
		drop(context.state.write(PanelState {
			open: false,
			refe: None,
			is_tab_units: true,
		}));
        context.signals.write(PanelSignal::Register);
    });
	
	context.life_cycle.change(|context| {
		for msg in context.messenger.messages {
			let mut state = context.state.read_cloned_or_default::<PanelState>();
			if let Some(PanelSignal::HideOrShow(refe)) = msg.as_any().downcast_ref() {
				state.open = !state.open;
				if state.open {
					state.refe = Some(refe.to_owned());
				}
			}
			if let Some(PanelSignal::ActiveTab) = msg.as_any().downcast_ref() {
				state.is_tab_units = !state.is_tab_units; 
			}	
			drop(context.state.write(state));
		}
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
	
	let panel_state = state.read_cloned_or_default::<PanelState>();

	let alpha = match panel_state.open {
		true => 1.,
		false => 0.,
	};

	let refe = panel_state.refe.unwrap_or_default();
	
	let size_title = SizeBoxProps {
        height: SizeBoxSizeValue::Exact(45.), 
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
			top: 5.,
            bottom: 5.,
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
			id: refe.preview,
			..Default::default()
		}),
		..Default::default()
	};
	let title = TextPaperProps {
        text: refe.name,
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
        height: SizeBoxSizeValue::Exact(500.), 
        width: SizeBoxSizeValue::Fill,
        ..Default::default()
    };
	
	let items = match panel_state.is_tab_units {
		true => refe.units,
		false => refe.upgrades,
	};
	let items_list = items.iter()
        .map(|(_code, child)| {
            widget! {
                (#{child.name} panel_item: { PanelItemProps { item: child.to_owned() }})
            }
        })
        .collect::<Vec<_>>();

    widget! {
        (#{key} content_box | {WidgetAlpha(alpha)} [
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
					content = (#{"h_tabs"} nav_horizontal_box [
						(#{"units"} tab: { TabProps {
							id: "units".to_string(),
							label: "UNITS".to_string(),
							is_active: panel_state.is_tab_units,
						}})
						(#{"upg"} tab: { TabProps {
							id: "upgrades".to_string(),
							label: "UPGRADES".to_string(),
							is_active: !panel_state.is_tab_units,
						}})
					])
				})
				(#{"items"} size_box: {size_items} {
					content =  (#{"flex_items"} nav_flex_box |[ items_list ]|)
				})
			])
		])
    }
}