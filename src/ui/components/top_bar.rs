use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

use crate::ui::components::resource;

fn resources_comp(_context: WidgetContext) -> WidgetNode {
	let bkg = PaperProps { 
		frame: None, 
		..Default::default() 
	};
	let size = Props::new(SizeBoxProps {
		width: SizeBoxSizeValue::Fill, 
		height: SizeBoxSizeValue::Exact(30.),
		..Default::default()
	});
	let margin = Props::new(ContentBoxItemLayout {
		margin: Rect {
			top: 5.,
			left: 50.,
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
							(#{"item-1"} resource::res_item)
							(#{"item-2"} resource::res_item)
							(#{"item-3"} resource::res_item)
						])
					])
				])
			])
		})
	}
}

pub fn resources(context: WidgetContext) -> WidgetNode {
	widget! {
		(#{context.key} resources_comp)
	}
}