use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

widget_component! {
	pub res_item(key) {
		let bkg = Props::new(PaperProps { 
			variant: "data".to_string(),
			frame: None, 
			..Default::default() 
		});
		let size = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Exact(80.), 
            height: SizeBoxSizeValue::Exact(20.),
            ..Default::default()
        });

		widget! {
			(#{key} size_box: {size} {
				content = (#{"bkg"} paper: {bkg})
			})
		}
	}
}