use oxygengine::user_interface::raui::{
    core::{
        implement_props_data, 
        prelude::*,
    },
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProps {
    #[serde(default)]
    pub img: String,
}
implement_props_data!(ResourceProps);

pub fn resource(context: WidgetContext) -> WidgetNode {
	let props = context.props.read_cloned_or_default::<ResourceProps>();
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
	let margin = Props::new(ContentBoxItemLayout {
		margin: Rect {
			top: 1.,
			left: 4.,
			..Default::default()
		},
		..Default::default()
	});
	let size_img = Props::new(SizeBoxProps {
		width: SizeBoxSizeValue::Exact(16.), 
		height: SizeBoxSizeValue::Exact(16.),
		..Default::default()
	});
	let img = Props::new(ImageBoxProps {
		width: ImageBoxSizeValue::Fill,
		height: ImageBoxSizeValue::Fill,
		material: ImageBoxMaterial::Image(ImageBoxImage {
			id: props.img.to_owned(),
			..Default::default()
		}),
		..Default::default()
	});

	let text = Props::new(TextPaperProps {
        //variant: "btn".to_string(),
        text: "0".to_owned(),
        width: TextBoxSizeValue::Fill,
        height: TextBoxSizeValue::Fill,
        use_main_color: true,
        ..Default::default()
    });

	widget! {
		(#{context.key} size_box: {size} {
			content = (#{"bkg"} paper: {bkg} [
				(#{"margin"} content_box : {margin} [
					(#{"size"} size_box: {size_img} {
						content = (#{"h-box"} horizontal_box [
							(#{"img"} image_box: {img})
							(#{"text"} text_paper: {text.clone()})
						])
					})
				])
			])
		})
	}
}