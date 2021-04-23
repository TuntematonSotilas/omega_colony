use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

use crate::ui::components::res_item;

widget_component! {
    pub resources_comp(key, props, state) {
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
			(#{"bkg-size"} size_box: {size} {
				content = (#{"bkg"} paper: {bkg} [
					(#{"bkg-cnt"} content_box [
						(#{"bkg-margin"} content_box : {margin} [
							(#{"h-box"} horizontal_box [
								(#{"item-1"} res_item::res_item)
								(#{"item-2"} res_item::res_item)
								(#{"item-3"} res_item::res_item)
							])
						])
					])
				])
			})
		}
	}
}

widget_component! {
	pub resources(key) {
		widget! {
			(#{key} resources_comp)
		}
	}
}