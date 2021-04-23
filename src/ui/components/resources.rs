use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

widget_component! {
    pub resources_comp(key, props, state) {
		let bkg = props.clone()
            .with(PaperProps { frame: None, ..Default::default() });
		let size = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Fill, 
            height: SizeBoxSizeValue::Exact(30.),
            ..Default::default()
        });
		widget!{
			(#{"size"} size_box: {size} {
				content = (#{"paper"} paper: {bkg})
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