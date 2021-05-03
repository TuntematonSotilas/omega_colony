use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};

pub fn res_item(context: WidgetContext) -> WidgetNode {
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
		width: SizeBoxSizeValue::Exact(16.), 
		height: SizeBoxSizeValue::Exact(16.),
		..Default::default()
	});
	let steel = Props::new(ImageBoxProps {
		width: ImageBoxSizeValue::Fill,
		height: ImageBoxSizeValue::Fill,
		material: ImageBoxMaterial::Image(ImageBoxImage {
			id: "ui/steel.png".to_owned(),
			..Default::default()
		}),
		..Default::default()
	});
	let margin = Props::new(ContentBoxItemLayout {
		margin: Rect {
			top: 2.,
			left: 4.,
			..Default::default()
		},
		..Default::default()
	});
	widget! {
		(#{context.key} size_box: {size} {
			content = (#{"bkg"} paper: {bkg} [
				(#{"margin"} content_box : {margin} [
					(#{"h-box"} horizontal_box [
						(#{"size"} size_box: {size_img} {
							content = (#{"res_img"} image_box: {steel})
						})
					])
				])
			])
		})
	}
}