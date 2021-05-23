use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

use crate::ui::components::stock::{ stock, StockProps };

fn top_bar_comp(_context: WidgetContext) -> WidgetNode {
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
	widget!{
		(#{"size"} size_box: {size} {
			content = (#{"bkg"} paper: {bkg} [
				(#{"cnt"} content_box [
					(#{"margin"} content_box : {margin} [
						(#{"h-box"} horizontal_box [
							(#{"item-1"} stock: { StockProps { img: "ui/energy.png".to_string() }})
							(#{"item-4"} stock: { StockProps { img: "ui/steel.png".to_string() }})
							(#{"item-2"} stock: { StockProps { img: "ui/water.png".to_string() }})
							(#{"item-3"} stock: { StockProps { img: "ui/cereal.png".to_string() }})
						])
					])
				])
			])
		})
	}
}

pub fn top_bar(context: WidgetContext) -> WidgetNode {
	widget! {
		(#{context.key} top_bar_comp)
	}
}