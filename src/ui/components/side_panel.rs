use oxygengine::{
	user_interface::raui::{
	core::prelude::*, 
	material::prelude::*
}};

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
}

#[derive(PropsData, Debug, Default, Clone, Serialize, Deserialize)]
pub struct PanelState {
	pub open: bool,
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
	let size_items = SizeBoxProps {
        height: SizeBoxSizeValue::Exact(500.), 
        width: SizeBoxSizeValue::Fill,
        ..Default::default()
    };

	let player_stock = panel_state.player_stock.clone();

	let items_units = panel_state.refe.units;
	let units_list = items_units.iter()
        .map(|(_code, child)| {
			widget! {
                (#{child.name} panel_item: { PanelItemProps { 
					item: child.to_owned(), 
					player_stock: player_stock.to_owned(),
				}})
            }
        })
        .collect::<Vec<_>>();

	let tabs_props = TabsBoxProps {
		tabs_location: TabsBoxTabsLocation::Top,
		tabs_and_content_separation: 5.,
		..Default::default()
	};
	
	let tab_units = TabProps { label: "UNITS".to_owned() };
	let tab_upg = TabProps { label: "UPGRADES".to_owned() };
	
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
				(#{"nav_tabs"} nav_tabs_box : {tabs_props} [
					{WidgetNode::pack_tuple([
						widget! {
							(#{"tab_plate_u"} tab: {tab_units})
						},
						widget! {(#{"cnt_text_u"} size_box: {size_items.to_owned()} {
							content =  (#{"flex_items"} nav_flex_box |[ units_list ]|)
						})},
					])}
					{WidgetNode::pack_tuple([
						widget! {
							(#{"tab_plate_g"} tab: {tab_upg})
						},
						widget! {(#{"cnt_text_g"} size_box: {size_items.to_owned()} {
							content =  ()
						})},
					])}
				])
			])
		])
    }
}