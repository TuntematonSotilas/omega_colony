use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

use crate::ui::components::resource;

fn top_bar_comp(_context: WidgetContext) -> WidgetNode {
	let size = Props::new(SizeBoxProps {
		width: SizeBoxSizeValue::Fill, 
		height: SizeBoxSizeValue::Exact(30.),
		..Default::default()
	});
	let bkg = PaperProps { 
		frame: None, 
		..Default::default() 
	};
	let margin = Props::new(ContentBoxItemLayout {
		margin: Rect {
			top: 6.,
			..Default::default()
		},
		..Default::default()
	});
	widget!{
		(#{"size"} size_box: {size} {
			content = (#{"bkg"} paper: {bkg} [
				(#{"cnt"} content_box [
					(#{"margin"} content_box : {margin} [
						(#{"h-box"} horizontal_box [
							(#{"item-1"} resource::resource: { resource::ResourceProps { 
								img: "ui/energy.png".to_string() 
							}})
							(#{"item-2"} resource::resource: { resource::ResourceProps { 
								img: "ui/water.png".to_string() 
							}})
							(#{"item-3"} resource::resource: { resource::ResourceProps { 
								img: "ui/wood.png".to_string() 
							}})
							(#{"item-4"} resource::resource: { resource::ResourceProps { 
								img: "ui/steel.png".to_string() 
							}})
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