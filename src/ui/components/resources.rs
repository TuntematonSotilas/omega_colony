use oxygengine::user_interface::raui::core::prelude::*;

widget_component! {
    pub resources_comp(key, props, state) {
		let size = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Fill, 
            height: SizeBoxSizeValue::Exact(50.),
            ..Default::default()
        });
		let background_props = Props::new(ImageBoxProps {
            width: ImageBoxSizeValue::Fill,
            height: ImageBoxSizeValue::Fill,
            material: ImageBoxMaterial::Image(ImageBoxImage {
                id: "ui/menu_btn.png".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        });
		widget!{
			(#{"size"} size_box: {size} {
				content = (#{"content"} content_box [
					(#{"background"} image_box: {background_props})
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