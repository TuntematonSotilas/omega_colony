use oxygengine::user_interface::raui::core::prelude::*;

widget_component! {
    pub resources_comp(key, props, state) {

		let background_props2 = Props::new(ImageBoxProps {
			material: ImageBoxMaterial::Image(ImageBoxImage {
			  id: "image.png".to_owned(),
			  scaling: ImageBoxImageScaling::Frame(ImageBoxFrame {
				// each field of a rect defines the size of a frame on each side of the source image
				source: Rect { left: 16.0, right: 16.0, top: 16.0, bottom: 16.0},
				// destination maps source rect frame to the ui space values so if you don't want to "rescale" the frame just make it the same as source.
				// if you double the values here then all frame sides would have doubled size in ui space.
				destination: Rect { left: 16.0, right: 16.0, top: 16.0, bottom: 16.0},
				..Default::default()
			  }),
			  ..Default::default()
			}),
			..Default::default()
		  });

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