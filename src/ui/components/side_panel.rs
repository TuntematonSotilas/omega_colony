use oxygengine::{user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
}, widget::component::containers::tabs_box};

use serde::{Deserialize, Serialize};

use crate::{
	ui::components::{
		panel_item::{panel_item, PanelItemProps},
		tab::{tab, TabProps},
	},
	resources::{
		referential::RefeItem,
		player_stock::PlayerStock,
	},
};

#[derive(MessageData, Debug, Clone, PartialEq, Eq)]
pub enum PanelSignal {
	Register,
    HideOrShow(RefeItem, PlayerStock),
	ActiveTab,
}

#[derive(PropsData, Debug, Default, Clone, Serialize, Deserialize)]
pub struct PanelState {
	pub open: bool,
	pub is_tab_upg: bool,
	pub refe: RefeItem,
	pub player_stock: PlayerStock,
}

fn use_panel(context: &mut WidgetContext) {
	context.life_cycle.mount(|context| {
		context.signals.write(PanelSignal::Register);
    });
	
	context.life_cycle.change(|context| {
		for msg in context.messenger.messages {
			let mut state = context.state.read_cloned_or_default::<PanelState>();
			if let Some(PanelSignal::HideOrShow(refe, player_stock)) = msg.as_any().downcast_ref() {
				state.open = !state.open;
				if state.open {
					state.refe = refe.to_owned();
					state.player_stock = player_stock.to_owned();
				}
			}
			if let Some(PanelSignal::ActiveTab) = msg.as_any().downcast_ref() {
				state.is_tab_upg = !state.is_tab_upg; 
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
			id: panel_state.refe.preview,
			..Default::default()
		}),
		..Default::default()
	};
	let title = TextPaperProps {
        text: panel_state.refe.name,
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
	
	let items = match panel_state.is_tab_upg {
		true => panel_state.refe.upgrades,
		false => panel_state.refe.units,
	};

	let player_stock = panel_state.player_stock.clone();

	let items_list = items.iter()
        .map(|(_code, child)| {
			widget! {
                (#{child.name} panel_item: { PanelItemProps { 
					item: child.to_owned(), 
					player_stock: player_stock.to_owned(),
				}})
            }
        })
        .collect::<Vec<_>>();

	let t1 = TextPaperProps {
		text: "UNITS".to_owned(),
		width: TextBoxSizeValue::Fill,
		height: TextBoxSizeValue::Fill,
		use_main_color: true,
		..Default::default()
	};
	let c1 = TextPaperProps {
		text: "c_UNITS".to_owned(),
		width: TextBoxSizeValue::Fill,
		height: TextBoxSizeValue::Fill,
		use_main_color: true,
		..Default::default()
	};
	let t2 = TextPaperProps {
		text: "UPGRADES".to_owned(),
		width: TextBoxSizeValue::Fill,
		height: TextBoxSizeValue::Fill,
		use_main_color: true,
		..Default::default()
	};
	let c2 = TextPaperProps {
		text: "c_UPGRADES".to_owned(),
		width: TextBoxSizeValue::Fill,
		height: TextBoxSizeValue::Fill,
		use_main_color: true,
		..Default::default()
	};

	let tab_paper = PaperProps { 
		variant: "tab_inactive".to_owned(),
        frame: None, 
        ..Default::default() 
    };

	let tab_props = TabsBoxProps {
		tabs_location: TabsBoxTabsLocation::Top,
		..Default::default()
	};
	
	let size_tab = SizeBoxProps {
		height: SizeBoxSizeValue::Exact(20.), 
		width: SizeBoxSizeValue::Exact(30.),
		..Default::default()
	};

    widget! {
        (#{key} content_box | {WidgetAlpha(alpha)} [
            (#{"bkg"} paper: {bkg})
			(#{"v_box"} vertical_box: {margin_panel} [
				(#{"title"} size_box: {size_title} {
					content = (#{"box_title"} paper: {bkg_title} [
						(#{"margin_title"} content_box : {margin_title} [
							(#{"h_title"} horizontal_box [
								(#{"img"} image_box: {preview})
								(#{"title"} text_paper: {title})
							])
						])
					])
				})
				// (#{"tabs"} size_box: {size_tabs} {
				// 	content = (#{"h_tabs"} nav_horizontal_box [
				// 		(#{"units"} tab: { TabProps {
				// 			id: "units".to_string(),
				// 			label: "UNITS".to_string(),
				// 			is_active: !panel_state.is_tab_upg,
				// 		}})
				// 		(#{"upg"} tab: { TabProps {
				// 			id: "upgrades".to_string(),
				// 			label: "UPGRADES".to_string(),
				// 			is_active: panel_state.is_tab_upg,
				// 		}})
				// 	])
				// })
				// (#{"items"} size_box: {size_items} {
				// 	content =  (#{"flex_items"} nav_flex_box |[ items_list ]|)
				// })

				(#{"tabs"} size_box: {size_tabs} {
					content = (#{"nav_tabs"} nav_tabs_box : {tab_props} [
						{WidgetNode::pack_tuple([
							widget! {
								(#{"s1"} size_box: {size_tab.to_owned()} {
									content = (#{"p1"} paper: {tab_paper.to_owned()} [
										(#{"t1"} text_paper: {t1})
									])
								})
							}, 
							widget! {
								(#{"c1"} text_paper: {c1})
							},
						])}
						{WidgetNode::pack_tuple([
							widget! {
								(#{"s2"} size_box: {size_tab.to_owned()} {
									content = (#{"p2"} paper: {tab_paper.to_owned()} [
										(#{"t1"} text_paper: {t2})
									])
								})
							}, 
							widget! {
								(#{"c2"} text_paper: {c2})
							},
						])}
					])
				})
			])
		])
    }
}