use oxygengine::user_interface::raui::{
    core::prelude::*,
    material::prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(PropsData, Default, Debug, Clone, Serialize, Deserialize)]
pub struct StockProps {
    pub img: String,
	pub cnt: u32,
}

pub fn stock(context: WidgetContext) -> WidgetNode {
	let props = context.props.read_cloned_or_default::<StockProps>();
	let size = SizeBoxProps {
		width: SizeBoxSizeValue::Exact(80.), 
		height: SizeBoxSizeValue::Exact(20.),
		..Default::default()
	};
	let bkg = PaperProps { 
		variant: "bkg_stock".to_string(),
		frame: None, 
		..Default::default() 
	};
	let margin = ContentBoxItemLayout {
		margin: Rect {
			top: 4.,
			left: 4.,
			bottom: 2.,
			..Default::default()
		},
		..Default::default()
	};
	let img = ImageBoxProps {
		width: ImageBoxSizeValue::Exact(12.),
		height: ImageBoxSizeValue::Exact(12.),
		material: ImageBoxMaterial::Image(ImageBoxImage {
			id: props.img,
			..Default::default()
		}),
		..Default::default()
	};
	let text = TextPaperProps {
        text: props.cnt.to_string(),
        width: TextBoxSizeValue::Exact(60.),
        height: TextBoxSizeValue::Fill,
        use_main_color: true,
		alignment_override: Some(TextBoxAlignment::Right),
        ..Default::default()
    };
	widget! {
		(#{context.key} size_box: {size} {
			content = (#{"bkg"} paper: {bkg} [
				(#{"h_box"} horizontal_box: {margin} [
					(#{"img"} image_box: {img})
					(#{"text"} text_paper: {text})
				])
			])
		})
	}
}