use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

widget_component! {
	pub res_item(key) {
		let size = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Exact(80.), 
            height: SizeBoxSizeValue::Exact(20.),
            ..Default::default()
        });
		let bkg = Props::new(PaperProps { 
			variant: "data".to_string(),
			frame: None, 
			..Default::default() 
		});
		let size_img = Props::new(SizeBoxProps {
            width: SizeBoxSizeValue::Exact(12.), 
            height: SizeBoxSizeValue::Exact(12.),
            ..Default::default()
        });
		let gaz = Props::new(ImageBoxProps {
			width: ImageBoxSizeValue::Fill,
            height: ImageBoxSizeValue::Fill,
            material: ImageBoxMaterial::Image(ImageBoxImage {
                id: "ui/gaz.png".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
		});
		let margin = Props::new(ContentBoxItemLayout {
			margin: Rect {
				top: 4.,
				left: 4.,
				..Default::default()
			},
			..Default::default()
		});
		widget! {
			(#{key} size_box: {size} {
				content = (#{"bkg"} paper: {bkg} [
					(#{"margin"} content_box : {margin} [
						(#{"h-box"} horizontal_box [
							(#{key} size_box: {size_img} {
								content = (#{"bkg"} image_box: {gaz})
							})
						])
					])
				])
			})
		}
	}
}