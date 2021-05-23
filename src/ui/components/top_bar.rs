use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

use crate::ui::components::stock::{ stock, StockProps };

pub fn top_bar(context: WidgetContext) -> WidgetNode {
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
		(#{context.key} size_box: {size} {
			content = (#{"bkg"} paper: {bkg} [
				(#{"top_bar_box"} content_box [
					(#{"margin"} content_box : {margin} [
						(#{"h_box"} horizontal_box [
							(#{"energy"} stock: { StockProps { img: "ui/energy.png".to_string() }})
							(#{"steel"} stock: { StockProps { img: "ui/steel.png".to_string() }})
							(#{"water"} stock: { StockProps { img: "ui/water.png".to_string() }})
							(#{"cereal"} stock: { StockProps { img: "ui/cereal.png".to_string() }})
						])
					])
				])
			])
		})
	}
}