use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

widget_component! {
    pub resources_comp(key, props, state) {
		let bkg = PaperProps { 
			frame: None, 
			..Default::default() 
		};
		let bkg_size = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Fill, 
            height: SizeBoxSizeValue::Exact(30.),
            ..Default::default()
        });
		let bkg_margin = Props::new(ContentBoxItemLayout {
			margin: Rect {
				top: 5.,
				left: 50.,
				..Default::default()
			},
			..Default::default()
		});
		let res_bkg = Props::new(PaperProps { 
			variant: "data".to_string(),
			frame: None, 
			..Default::default() 
		});
		let res_size = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Exact(80.), 
            height: SizeBoxSizeValue::Exact(20.),
            ..Default::default()
        });

		widget!{
			(#{"bkg-size"} size_box: {bkg_size} {
				content = (#{"bkg"} paper: {bkg} [
					(#{"bkg-cnt"} content_box [
						(#{"bkg-margin"} content_box : {bkg_margin} [
							(#{"h-box"} horizontal_box [
								(#{"res-size-1"} size_box: {&res_size} {
									content = (#{"res-bkg-1"} paper: {&res_bkg})
								})
								(#{"res-size-2"} size_box: {&res_size} {
									content = (#{"res-bkg-2"} paper: {&res_bkg})
								})
								(#{"res-size-3"} size_box: {&res_size} {
									content = (#{"res-bkg-3"} paper: {&res_bkg})
								})
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