use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::{
	resources::{
		stock::Stock,
		player_stock::PlayerStock,
	},
	ui::components::stock::{stock, StockProps},
};

#[derive(MessageData, Debug, Clone, PartialEq, Eq)]
pub enum TopBarSignal {
	Register,
	InitRefeStock(Stock, PlayerStock),
}

#[derive(PropsData, Debug, Default, Clone, Serialize, Deserialize)]
pub struct TopBarState {
	pub stock: Stock,
	pub player_stock: PlayerStock,
}

fn use_tab_bar(context: &mut WidgetContext) {
	context.life_cycle.mount(|context| {
		context.signals.write(TopBarSignal::Register);
	});
	context.life_cycle.change(|context| {
		for msg in context.messenger.messages {
			if let Some(TopBarSignal::InitRefeStock(stock, player_stock)) = msg.as_any().downcast_ref() {
				let mut state = context.state.read_cloned_or_default::<TopBarState>();	
				state.stock = stock.to_owned();
				state.player_stock = player_stock.to_owned();
				drop(context.state.write(state));
			}
		}
	});
}

#[pre_hooks(use_tab_bar)]
pub fn top_bar(mut context: WidgetContext) -> WidgetNode {
	let WidgetContext {
		key,
        state,
        ..
    } = context;

	let size = SizeBoxProps {
		width: SizeBoxSizeValue::Fill, 
		height: SizeBoxSizeValue::Exact(30.),
		..Default::default()
	};
	let bkg = PaperProps { 
		frame: None, 
		..Default::default() 
	};
	let margin = ContentBoxItemLayout {
		margin: Rect {
			top: 6.,
			..Default::default()
		},
		..Default::default()
	};
	let refe_state = state.read_cloned_or_default::<TopBarState>();
	let items_list = refe_state.stock.refe.iter()
        .map(|(code, item)| {
            let cnt = refe_state.player_stock.stock.get(code).cloned().unwrap_or_default();
			widget! {
				(#{item.name} stock: { StockProps { cnt: cnt, img: item.pic.to_owned() }})
            }
        })
        .collect::<Vec<_>>();
		
	widget!{
		(#{key} size_box: {size} {
			content = (#{"bkg"} paper: {bkg} [
				(#{"top_bar_box"} content_box [
					(#{"margin"} content_box : {margin} [
						(#{"h_box"} horizontal_box |[ items_list ]|)
					])
				])
			])
		})
	}
}